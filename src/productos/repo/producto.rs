use sqlx::Row;

use crate::{
    db::{row_get_f64, row_get_i32, row_get_i64, row_get_opt_i64, DbPool},
    error::AppError,
    productos::model::{
        Devolucion, ExistenciaInventario, FiltroExistencias, HistorialDevolucionQuery, Libro,
        MovimientoDiarioItem, RegistrarLibroRequest, TipoProducto,
    },
};

pub const UBICACION_TIENDA: i32 = 1;
pub const UBICACION_BODEGA: i32 = 2;

pub struct ProductoRepo;

impl ProductoRepo {
    // ------------------------------------------------------------------------
    // Operaciones de Libros y Stock
    // ------------------------------------------------------------------------

    pub async fn buscar_libro(
        pool: &DbPool,
        codigo_ean: i64,
        id_ubicacion: i32,
    ) -> Result<Option<Libro>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT codigo_ean, id_genero, id_mueble, id_proveedor, id_ubicacion,
                   nombre_libro, precio, cantidad, autor, SKU
            FROM libros
            WHERE codigo_ean = ? AND id_ubicacion = ?
            "#,
        )
        .bind(codigo_ean)
        .bind(id_ubicacion)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row.map(|r| Libro {
            codigo_ean: row_get_i64(&r, "codigo_ean"),
            id_genero: row_get_i32(&r, "id_genero"),
            id_mueble: row_get_i32(&r, "id_mueble"),
            id_proveedor: row_get_i32(&r, "id_proveedor"),
            id_ubicacion: row_get_i32(&r, "id_ubicacion"),
            nombre_libro: r.get("nombre_libro"),
            precio: row_get_f64(&r, "precio"),
            cantidad: row_get_i32(&r, "cantidad"),
            autor: r.get("autor"),
            sku: row_get_opt_i64(&r, "SKU"),
        }))
    }

    pub async fn guardar_o_actualizar_libro(
        pool: &DbPool,
        req: &RegistrarLibroRequest,
        ubicacion: i32,
    ) -> Result<Libro, AppError> {
        let existing = Self::buscar_libro(pool, req.codigo_ean, ubicacion).await?;

        if let Some(libro_actual) = existing {
            let nueva_cantidad = libro_actual.cantidad + req.cantidad;
            sqlx::query(
                "UPDATE libros SET cantidad = ?, precio = ? WHERE codigo_ean = ? AND id_ubicacion = ?"
            )
            .bind(nueva_cantidad)
            .bind(req.precio)
            .bind(req.codigo_ean)
            .bind(ubicacion)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

            Ok(Libro {
                cantidad: nueva_cantidad,
                precio: req.precio,
                ..libro_actual
            })
        } else {
            sqlx::query(
                r#"
                INSERT INTO libros (codigo_ean, id_genero, id_mueble, id_proveedor, id_ubicacion,
                                   nombre_libro, precio, cantidad, autor, SKU)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(req.codigo_ean)
            .bind(req.id_genero)
            .bind(req.id_mueble)
            .bind(req.id_proveedor)
            .bind(ubicacion)
            .bind(&req.nombre_libro)
            .bind(req.precio)
            .bind(req.cantidad)
            .bind(&req.autor)
            .bind(req.sku)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

            Ok(Libro {
                codigo_ean: req.codigo_ean,
                id_genero: req.id_genero,
                id_mueble: req.id_mueble,
                id_proveedor: req.id_proveedor,
                id_ubicacion: ubicacion,
                nombre_libro: req.nombre_libro.clone(),
                precio: req.precio,
                cantidad: req.cantidad,
                autor: req.autor.clone(),
                sku: req.sku,
            })
        }
    }

    /// Ejecuta una transferencia atómica de stock entre dos ubicaciones (ej. Bodega <-> Tienda)
    pub async fn trasladar_stock_libro(
        pool: &DbPool,
        codigo_ean: i64,
        cantidad: i32,
        origen: i32,
        destino: i32,
    ) -> Result<(i32, i32), AppError> {
        let mut tx = pool.begin().await.map_err(AppError::Database)?;

        // 1. Bloquear y verificar origen
        let row_origen = sqlx::query(
            "SELECT id_genero, id_mueble, id_proveedor, nombre_libro, precio, cantidad, autor, SKU FROM libros WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
        )
        .bind(codigo_ean)
        .bind(origen)
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| {
            let lugar = if origen == UBICACION_BODEGA { "Bodega" } else { "Piso de Ventas" };
            AppError::NotFound(format!("El artículo con EAN {} no existe en {}", codigo_ean, lugar))
        })?;

        let stock_actual_origen: i32 = row_origen.get("cantidad");
        if stock_actual_origen < cantidad {
            let lugar = if origen == UBICACION_BODEGA { "Bodega" } else { "Piso de Ventas" };
            return Err(AppError::BadRequest(format!(
                "Stock insuficiente en {}. Disponible: {}, Solicitado: {}",
                lugar, stock_actual_origen, cantidad
            )));
        }

        let nuevo_origen = stock_actual_origen - cantidad;
        sqlx::query("UPDATE libros SET cantidad = ? WHERE codigo_ean = ? AND id_ubicacion = ?")
            .bind(nuevo_origen)
            .bind(codigo_ean)
            .bind(origen)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        // 2. Incrementar o insertar en destino
        let row_destino = sqlx::query(
            "SELECT cantidad FROM libros WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
        )
        .bind(codigo_ean)
        .bind(destino)
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        let nuevo_destino = match row_destino {
            Some(r) => {
                let actual: i32 = row_get_i32(&r, "cantidad");
                let suma = actual + cantidad;
                sqlx::query("UPDATE libros SET cantidad = ? WHERE codigo_ean = ? AND id_ubicacion = ?")
                    .bind(suma)
                    .bind(codigo_ean)
                    .bind(destino)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::Database)?;
                suma
            }
            None => {
                sqlx::query(
                    r#"
                    INSERT INTO libros (codigo_ean, id_genero, id_mueble, id_proveedor, id_ubicacion,
                                       nombre_libro, precio, cantidad, autor, SKU)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(codigo_ean)
                .bind(row_origen.get::<i32, _>("id_genero"))
                .bind(row_origen.get::<i32, _>("id_mueble"))
                .bind(row_origen.get::<i32, _>("id_proveedor"))
                .bind(destino)
                .bind(row_origen.get::<String, _>("nombre_libro"))
                .bind(row_origen.get::<f64, _>("precio"))
                .bind(cantidad)
                .bind(row_origen.get::<Option<String>, _>("autor"))
                .bind(row_origen.get::<Option<i64>, _>("SKU"))
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;

                cantidad
            }
        };

        tx.commit().await.map_err(AppError::Database)?;

        Ok((nuevo_origen, nuevo_destino))
    }

    /// Descuenta inventario de la tienda para una venta
    pub async fn descontar_stock_venta(
        pool: &DbPool,
        codigo_ean: i64,
        cantidad: i32,
    ) -> Result<(String, f64), AppError> {
        let mut tx = pool.begin().await.map_err(AppError::Database)?;

        let row = sqlx::query(
            "SELECT nombre_libro, precio, cantidad FROM libros WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
        )
        .bind(codigo_ean)
        .bind(UBICACION_TIENDA)
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| {
            AppError::NotFound(format!("El artículo con EAN {} no existe en Piso de Ventas", codigo_ean))
        })?;

        let nombre: String = row.get("nombre_libro");
        let precio: f64 = row.get("precio");
        let stock_actual: i32 = row.get("cantidad");

        if stock_actual < cantidad {
            return Err(AppError::BadRequest(format!(
                "Stock insuficiente para '{}'. Disponible: {}, Solicitado: {}",
                nombre, stock_actual, cantidad
            )));
        }

        let nuevo_stock = stock_actual - cantidad;
        sqlx::query("UPDATE libros SET cantidad = ? WHERE codigo_ean = ? AND id_ubicacion = ?")
            .bind(nuevo_stock)
            .bind(codigo_ean)
            .bind(UBICACION_TIENDA)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        Ok((nombre, precio))
    }

    // ------------------------------------------------------------------------
    // Existencias e Inventario Consolidado
    // ------------------------------------------------------------------------

    pub async fn consultar_existencias(
        pool: &DbPool,
        params: &FiltroExistencias,
    ) -> Result<Vec<ExistenciaInventario>, AppError> {
        let mut builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
            r#"
            SELECT l.codigo_ean, l.SKU as sku, l.nombre_libro as titulo, l.autor,
                   l.precio, p.nombre_proveedor,
                   COALESCE(SUM(CASE WHEN l.id_ubicacion = 1 THEN l.cantidad ELSE 0 END), 0) as stock_tienda,
                   COALESCE(SUM(CASE WHEN l.id_ubicacion = 2 THEN l.cantidad ELSE 0 END), 0) as stock_bodega
            FROM libros l
            LEFT JOIN proveedor p ON l.id_proveedor = p.id_proveedor
            WHERE 1 = 1
            "#,
        );

        if let Some(ref q) = params.q {
            let q_clean = q.trim();
            if !q_clean.is_empty() {
                let pattern = format!("%{}%", q_clean);
                builder.push(" AND (l.nombre_libro LIKE ");
                builder.push_bind(pattern.clone());
                builder.push(" OR l.autor LIKE ");
                builder.push_bind(pattern.clone());
                builder.push(" OR CAST(l.codigo_ean AS CHAR) LIKE ");
                builder.push_bind(pattern);
                builder.push(")");
            }
        }

        builder.push(" GROUP BY l.codigo_ean, l.SKU, l.nombre_libro, l.autor, l.precio, p.nombre_proveedor");
        builder.push(" ORDER BY l.nombre_libro ASC");

        let rows = builder.build().fetch_all(pool).await.map_err(AppError::Database)?;

        let list = rows
            .into_iter()
            .map(|r| {
                let stock_tienda: i64 = r.try_get("stock_tienda").unwrap_or(0);
                let stock_bodega: i64 = r.try_get("stock_bodega").unwrap_or(0);
                let stock_total = (stock_tienda + stock_bodega) as i32;

                ExistenciaInventario {
                    codigo_ean: row_get_i64(&r, "codigo_ean"),
                    sku: row_get_opt_i64(&r, "sku"),
                    titulo: r.get("titulo"),
                    tipo_producto: "Libro".to_string(),
                    stock_tienda: stock_tienda as i32,
                    stock_bodega: stock_bodega as i32,
                    stock_total,
                    precio: row_get_f64(&r, "precio"),
                    proveedor: r.get("nombre_proveedor"),
                    autor_o_editorial: r.get("autor"),
                }
            })
            .collect();

        Ok(list)
    }

    // ------------------------------------------------------------------------
    // Devoluciones a Proveedor
    // ------------------------------------------------------------------------

    pub async fn registrar_devolucion(
        pool: &DbPool,
        id_devolucion: i64,
        total_piezas: i32,
        id_usuario: i64,
        id_proveedor: i32,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO devolucion (id_devolucion, fecha, total_piezas, estado, autorizado_por, id_usuarios, id_proveedor)
            VALUES (?, CURDATE(), ?, 'Pendiente', NULL, ?, ?)
            "#,
        )
        .bind(id_devolucion)
        .bind(total_piezas)
        .bind(id_usuario)
        .bind(id_proveedor)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn buscar_devolucion(
        pool: &DbPool,
        id: i64,
    ) -> Result<Option<Devolucion>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT d.id_devolucion, DATE_FORMAT(d.fecha, '%Y-%m-%d') as fecha,
                   d.total_piezas, d.estado, d.autorizado_por, d.id_usuarios,
                   CONCAT(u.nombre, ' ', COALESCE(u.apellido_paterno, '')) as vendedor_nombre,
                   d.id_proveedor, p.nombre_proveedor
            FROM devolucion d
            JOIN usuarios u ON d.id_usuarios = u.id_usuarios
            JOIN proveedor p ON d.id_proveedor = p.id_proveedor
            WHERE d.id_devolucion = ?
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row.map(|r| {
            Devolucion {
                id_devolucion: row_get_i64(&r, "id_devolucion"),
                fecha: r.get("fecha"),
                total_piezas: row_get_i32(&r, "total_piezas"),
                estado: r.get::<Option<String>, _>("estado").unwrap_or_else(|| "Pendiente".to_string()),
                autorizado_por: r.get("autorizado_por"),
                id_usuarios: row_get_i64(&r, "id_usuarios"),
                vendedor_nombre: r.get("vendedor_nombre"),
                id_proveedor: row_get_i32(&r, "id_proveedor"),
                nombre_proveedor: r.get("nombre_proveedor"),
                tipo_producto: TipoProducto::Libro,
                items: Vec::new(),
            }
        }))
    }

    pub async fn listar_historial_devoluciones(
        pool: &DbPool,
        params: &HistorialDevolucionQuery,
    ) -> Result<Vec<Devolucion>, AppError> {
        let mut builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
            r#"
            SELECT d.id_devolucion, DATE_FORMAT(d.fecha, '%Y-%m-%d') as fecha,
                    d.total_piezas, d.estado, d.autorizado_por, d.id_usuarios,
                    CONCAT(u.nombre, ' ', COALESCE(u.apellido_paterno, '')) AS vendedor_nombre,
                    d.id_proveedor, p.nombre_proveedor
            FROM devolucion d
            JOIN usuarios u ON d.id_usuarios = u.id_usuarios
            JOIN proveedor p ON d.id_proveedor = p.id_proveedor
            WHERE 1 = 1
            "#,
        );

        if let Some(ref fecha) = params.fecha {
            let f = fecha.trim();
            if !f.is_empty() {
                builder.push(" AND d.fecha = ");
                builder.push_bind(f);
            }
        }
        if let Some(ref estado) = params.estado {
            let e = estado.trim();
            if !e.is_empty() {
                builder.push(" AND d.estado = ");
                builder.push_bind(e);
            }
        }
        if let Some(prov_id) = params.id_proveedor {
            builder.push(" AND d.id_proveedor = ");
            builder.push_bind(prov_id);
        }

        builder.push(" ORDER BY d.fecha DESC, d.id_devolucion DESC");

        let rows = builder.build().fetch_all(pool).await.map_err(AppError::Database)?;

        let list = rows
            .into_iter()
            .map(|r| {
                Devolucion {
                    id_devolucion: row_get_i64(&r, "id_devolucion"),
                    fecha: r.get("fecha"),
                    total_piezas: row_get_i32(&r, "total_piezas"),
                    estado: r.get::<Option<String>, _>("estado").unwrap_or_else(|| "Pendiente".to_string()),
                    autorizado_por: r.get("autorizado_por"),
                    id_usuarios: row_get_i64(&r, "id_usuarios"),
                    vendedor_nombre: r.get("vendedor_nombre"),
                    id_proveedor: row_get_i32(&r, "id_proveedor"),
                    nombre_proveedor: r.get("nombre_proveedor"),
                    tipo_producto: TipoProducto::Libro,
                    items: Vec::new(),
                }
            })
            .collect();

        Ok(list)
    }

    pub async fn actualizar_estado_devolucion(
        pool: &DbPool,
        id: i64,
        nuevo_estado: &str,
        autorizado_por: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE devolucion SET estado = ?, autorizado_por = ? WHERE id_devolucion = ?"
        )
        .bind(nuevo_estado)
        .bind(autorizado_por)
        .bind(id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Bitácora / Movimientos Diarios
    // ------------------------------------------------------------------------

    pub async fn consultar_movimiento_diario(
        pool: &DbPool,
        fecha: Option<&str>,
    ) -> Result<Vec<MovimientoDiarioItem>, AppError> {
        let is_curdate = fecha.map(|f| f.trim().is_empty()).unwrap_or(true);

        let query_str = if is_curdate {
            r#"
            SELECT DATE_FORMAT(b.fecha_hora, '%Y-%m-%d %H:%i:%s') as fecha_hora,
                   CONCAT(u.nombre, ' ', COALESCE(u.apellido_paterno, '')) AS usuario,
                   COALESCE(r.nombre_rol, 'Sin Rol') AS rol,
                   b.accion,
                   b.detalle
            FROM bitacora b
            JOIN usuarios u ON b.id_usuarios = u.id_usuarios
            LEFT JOIN roles r ON u.id_roles = r.id_roles
            WHERE DATE(b.fecha_hora) = CURDATE()
            ORDER BY b.fecha_hora DESC
            "#
        } else {
            r#"
            SELECT DATE_FORMAT(b.fecha_hora, '%Y-%m-%d %H:%i:%s') as fecha_hora,
                   CONCAT(u.nombre, ' ', COALESCE(u.apellido_paterno, '')) AS usuario,
                   COALESCE(r.nombre_rol, 'Sin Rol') AS rol,
                   b.accion,
                   b.detalle
            FROM bitacora b
            JOIN usuarios u ON b.id_usuarios = u.id_usuarios
            LEFT JOIN roles r ON u.id_roles = r.id_roles
            WHERE DATE(b.fecha_hora) = ?
            ORDER BY b.fecha_hora DESC
            "#
        };

        let rows = if is_curdate {
            sqlx::query(query_str).fetch_all(pool).await
        } else {
            sqlx::query(query_str)
                .bind(fecha.unwrap())
                .fetch_all(pool)
                .await
        }
        .map_err(AppError::Database)?;

        let list = rows
            .into_iter()
            .map(|r| MovimientoDiarioItem {
                fecha_hora: r.get("fecha_hora"),
                usuario: r.get("usuario"),
                rol: r.get("rol"),
                accion: r.get("accion"),
                detalle: r.get("detalle"),
            })
            .collect();

        Ok(list)
    }
}
