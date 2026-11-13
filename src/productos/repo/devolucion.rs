use sqlx::{MySql, MySqlPool, QueryBuilder, Row};

use crate::{
    db::{DbPool, row_get_i32, row_get_i64, row_get_opt_i64},
    error::AppError,
    productos::model::{
        Devolucion, HistorialDevolucionQuery, ItemDevolucion, ItemDevolucionRequest, TipoProducto,
    },
};

pub struct DevolucionRepo;

const DEVOLUCION_SELECT: &str = r#"
    SELECT d.id_devolucion,
           DATE_FORMAT(d.fecha_solicitud, '%Y-%m-%d') AS fecha,
           d.total_piezas,
           d.estado,
           CONCAT(ap.nombre, ' ', COALESCE(ap.apellido_paterno, '')) AS autorizado_por,
           d.id_usuarios,
           CONCAT(u.nombre, ' ', COALESCE(u.apellido_paterno, '')) AS vendedor_nombre,
           d.id_proveedor,
           p.nombre_proveedor,
           p.rfc AS proveedor_rfc,
           p.telefono AS proveedor_telefono,
           p.correo AS proveedor_correo,
           p.direccion AS proveedor_direccion,
           p.persona_contacto AS proveedor_contacto
    FROM devolucion d
    JOIN usuarios u ON d.id_usuarios = u.id_usuarios
    LEFT JOIN usuarios ap ON d.id_usuario_aprueba = ap.id_usuarios
    JOIN proveedor p ON d.id_proveedor = p.id_proveedor
"#;

fn map_devolucion(
    row: &sqlx::mysql::MySqlRow,
    tipo_producto: TipoProducto,
    items: Vec<ItemDevolucion>,
) -> Devolucion {
    Devolucion {
        id_devolucion: row_get_i64(row, "id_devolucion"),
        fecha: row.get("fecha"),
        total_piezas: row_get_i32(row, "total_piezas"),
        estado: row.get("estado"),
        autorizado_por: row.get("autorizado_por"),
        id_usuarios: row_get_i64(row, "id_usuarios"),
        vendedor_nombre: row.get("vendedor_nombre"),
        id_proveedor: row_get_i32(row, "id_proveedor"),
        nombre_proveedor: row.get("nombre_proveedor"),
        proveedor_rfc: row.get("proveedor_rfc"),
        proveedor_telefono: row.get("proveedor_telefono"),
        proveedor_correo: row.get("proveedor_correo"),
        proveedor_direccion: row.get("proveedor_direccion"),
        proveedor_contacto: row.get("proveedor_contacto"),
        tipo_producto,
        items,
    }
}

async fn cargar_items(
    pool: &MySqlPool,
    id_devolucion: i64,
) -> Result<(Vec<ItemDevolucion>, TipoProducto), AppError> {
    let rows = sqlx::query(
        r#"
        SELECT dd.codigo_ean, dd.id_motivo, dd.cantidad,
               p.SKU AS sku, p.tipo_producto,
               COALESCE(l.nombre_libro, r.nombre_revista, 'Producto Desconocido') AS titulo,
               m.nombre_motivo AS motivo
        FROM detalle_devolucion dd
        JOIN productos p ON p.codigo_ean = dd.codigo_ean
        LEFT JOIN libros l ON l.codigo_ean = dd.codigo_ean
        LEFT JOIN revistas r ON r.codigo_ean = dd.codigo_ean
        JOIN motivo m ON m.id_motivo = dd.id_motivo
        WHERE dd.id_devolucion = ?
        ORDER BY dd.id_detalle_dev
        "#,
    )
    .bind(id_devolucion)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    let mut tipo_producto = TipoProducto::Libro;
    let items = rows
        .into_iter()
        .enumerate()
        .map(|(index, row)| {
            let tipo: String = row.get("tipo_producto");
            if index == 0 && tipo.eq_ignore_ascii_case("revista") {
                tipo_producto = TipoProducto::Revista;
            }
            ItemDevolucion {
                codigo_ean: row_get_i64(&row, "codigo_ean"),
                id_motivo: row_get_i32(&row, "id_motivo"),
                sku: row_get_opt_i64(&row, "sku"),
                titulo: row.get("titulo"),
                cantidad: row_get_i32(&row, "cantidad"),
                motivo: row.get("motivo"),
            }
        })
        .collect();

    Ok((items, tipo_producto))
}

impl DevolucionRepo {
    pub async fn registrar_devolucion(
        pool: &DbPool,
        id_usuario: i64,
        id_proveedor: i32,
        tipo_producto: TipoProducto,
        items: &[ItemDevolucionRequest],
    ) -> Result<i64, AppError> {
        if items.is_empty() {
            return Err(AppError::BadRequest(
                "Debe especificar al menos un artículo para devolución".to_string(),
            ));
        }

        let total_piezas = items.iter().try_fold(0_i32, |total, item| {
            total.checked_add(item.cantidad).ok_or_else(|| {
                AppError::BadRequest("El total de piezas excede el límite permitido".to_string())
            })
        })?;

        let mut tx = pool.begin().await.map_err(AppError::Database)?;

        let proveedor = sqlx::query("SELECT id_proveedor FROM proveedor WHERE id_proveedor = ?")
            .bind(id_proveedor)
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::Database)?;
        if proveedor.is_none() {
            return Err(AppError::NotFound(format!(
                "Proveedor con ID {} no encontrado",
                id_proveedor
            )));
        }

        let insert = sqlx::query(
            "INSERT INTO devolucion (id_proveedor, id_usuarios, total_piezas) VALUES (?, ?, ?)",
        )
        .bind(id_proveedor)
        .bind(id_usuario)
        .bind(total_piezas)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;
        let id_devolucion = insert.last_insert_id() as i64;

        for item in items {
            if item.codigo_ean <= 0 || item.cantidad <= 0 || item.id_motivo <= 0 {
                return Err(AppError::BadRequest(
                    "Cada artículo debe tener EAN, cantidad e id_motivo válidos".to_string(),
                ));
            }

            let producto = sqlx::query(
                "SELECT tipo_producto, id_proveedor FROM productos WHERE codigo_ean = ?",
            )
            .bind(item.codigo_ean)
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Producto con EAN {} no encontrado",
                    item.codigo_ean
                ))
            })?;
            let tipo: String = producto.get("tipo_producto");
            let proveedor_producto = row_get_i32(&producto, "id_proveedor");
            if proveedor_producto != id_proveedor {
                return Err(AppError::BadRequest(format!(
                    "El producto {} pertenece al proveedor {}, no al proveedor {} de la devolución",
                    item.codigo_ean, proveedor_producto, id_proveedor
                )));
            }
            if !tipo.eq_ignore_ascii_case(tipo_producto.as_str()) {
                return Err(AppError::BadRequest(format!(
                    "El producto {} no es de tipo {}",
                    item.codigo_ean,
                    tipo_producto.as_str()
                )));
            }

            let motivo = sqlx::query("SELECT id_motivo FROM motivo WHERE id_motivo = ?")
                .bind(item.id_motivo)
                .fetch_optional(&mut *tx)
                .await
                .map_err(AppError::Database)?;
            if motivo.is_none() {
                return Err(AppError::BadRequest(format!(
                    "El motivo con ID {} no existe",
                    item.id_motivo
                )));
            }

            sqlx::query(
                "INSERT INTO detalle_devolucion (id_devolucion, codigo_ean, id_motivo, cantidad) \
                 VALUES (?, ?, ?, ?)",
            )
            .bind(id_devolucion)
            .bind(item.codigo_ean)
            .bind(item.id_motivo)
            .bind(item.cantidad)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;
        }

        tx.commit().await.map_err(AppError::Database)?;
        Ok(id_devolucion)
    }

    pub async fn buscar_devolucion(
        pool: &DbPool,
        id: i64,
    ) -> Result<Option<Devolucion>, AppError> {
        let mut query: QueryBuilder<MySql> = QueryBuilder::new(DEVOLUCION_SELECT);
        query.push(" WHERE d.id_devolucion = ").push_bind(id);
        let row = query
            .build()
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?;

        match row {
            Some(row) => {
                let id = row_get_i64(&row, "id_devolucion");
                let (items, tipo_producto) = cargar_items(pool, id).await?;
                Ok(Some(map_devolucion(&row, tipo_producto, items)))
            }
            None => Ok(None),
        }
    }

    pub async fn listar_historial_devoluciones(
        pool: &DbPool,
        params: &HistorialDevolucionQuery,
    ) -> Result<Vec<Devolucion>, AppError> {
        let mut builder: QueryBuilder<MySql> =
            QueryBuilder::new(format!("{DEVOLUCION_SELECT} WHERE 1 = 1"));

        if let Some(fecha) = params.fecha.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            builder.push(" AND DATE(d.fecha_solicitud) = ").push_bind(fecha);
        }
        if let Some(estado) = params
            .estado
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty() && !s.eq_ignore_ascii_case("todos"))
        {
            builder.push(" AND d.estado = ").push_bind(estado);
        }
        if let Some(id_proveedor) = params.id_proveedor.filter(|id| *id > 0) {
            builder
                .push(" AND d.id_proveedor = ")
                .push_bind(id_proveedor);
        }
        builder.push(" ORDER BY d.fecha_solicitud DESC, d.id_devolucion DESC");

        let rows = builder
            .build()
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;
        let mut devoluciones = Vec::with_capacity(rows.len());
        for row in rows {
            let id = row_get_i64(&row, "id_devolucion");
            let (items, tipo_producto) = cargar_items(pool, id).await?;
            devoluciones.push(map_devolucion(&row, tipo_producto, items));
        }
        Ok(devoluciones)
    }

    pub async fn actualizar_estado_devolucion(
        pool: &DbPool,
        id: i64,
        id_usuario_aprueba: i64,
        nuevo_estado: &str,
        aprobar: bool,
    ) -> Result<(), AppError> {
        let mut tx = pool.begin().await.map_err(AppError::Database)?;
        let row = sqlx::query(
            "SELECT estado FROM devolucion WHERE id_devolucion = ? FOR UPDATE",
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Devolución #{} no encontrada", id)))?;
        let estado: String = row.get("estado");
        if !estado.eq_ignore_ascii_case("PENDIENTE") {
            return Err(AppError::Conflict(format!(
                "La devolución #{} ya fue evaluada (estado: {})",
                id, estado
            )));
        }

        if aprobar {
            let detalles = sqlx::query(
                "SELECT codigo_ean, cantidad FROM detalle_devolucion \
                 WHERE id_devolucion = ? ORDER BY codigo_ean, id_detalle_dev",
            )
            .bind(id)
            .fetch_all(&mut *tx)
            .await
            .map_err(AppError::Database)?;

            for detalle in detalles {
                let codigo_ean = row_get_i64(&detalle, "codigo_ean");
                let mut restante = row_get_i32(&detalle, "cantidad");
                let existencias = sqlx::query(
                    r#"
                    SELECT e.id_mueble, e.cantidad
                    FROM existencias e
                    JOIN mueble m ON m.id_mueble = e.id_mueble
                    WHERE e.codigo_ean = ? AND m.id_ubicacion = 2 AND e.cantidad > 0
                    ORDER BY e.cantidad DESC
                    FOR UPDATE
                    "#,
                )
                .bind(codigo_ean)
                .fetch_all(&mut *tx)
                .await
                .map_err(AppError::Database)?;

                let disponible: i64 = existencias
                    .iter()
                    .map(|row| row_get_i32(row, "cantidad") as i64)
                    .sum();
                if disponible < restante as i64 {
                    return Err(AppError::BadRequest(format!(
                        "Stock insuficiente en bodega para EAN {}. Disponible: {}, solicitado: {}",
                        codigo_ean, disponible, restante
                    )));
                }

                for existencia in existencias {
                    if restante == 0 {
                        break;
                    }
                    let cantidad = row_get_i32(&existencia, "cantidad");
                    let descontar = cantidad.min(restante);
                    sqlx::query(
                        "UPDATE existencias SET cantidad = cantidad - ? \
                         WHERE codigo_ean = ? AND id_mueble = ?",
                    )
                    .bind(descontar)
                    .bind(codigo_ean)
                    .bind(row_get_i32(&existencia, "id_mueble"))
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::Database)?;
                    restante -= descontar;
                }
            }
        }

        let update = sqlx::query(
            "UPDATE devolucion SET estado = ?, id_usuario_aprueba = ?, \
             fecha_aprobacion = CURRENT_TIMESTAMP WHERE id_devolucion = ?",
        )
        .bind(nuevo_estado)
        .bind(id_usuario_aprueba)
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;
        if update.rows_affected() != 1 {
            return Err(AppError::NotFound(format!(
                "Devolución #{} no encontrada",
                id
            )));
        }

        tx.commit().await.map_err(AppError::Database)?;
        Ok(())
    }
}
