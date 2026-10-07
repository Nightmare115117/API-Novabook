use sqlx::Row;

use crate::{
    db::{DbPool, row_get_f64, row_get_i32, row_get_i64, row_get_opt_i32, row_get_opt_i64},
    error::AppError,
    productos::{
        model::{CompraResponse, ItemCompraDto, RegistrarCompraRequest, TipoProducto},
        repo::UBICACION_BODEGA,
    },
};

pub struct CompraRepo;

impl CompraRepo {
    pub async fn registrar_compra_transaccional(
        pool: &DbPool,
        actor_id: i64,
        req: &RegistrarCompraRequest,
    ) -> Result<CompraResponse, AppError> {
        let mut tx = pool.begin().await.map_err(AppError::Database)?;

        // Verificar que proveedor existe
        let prov_row =
            sqlx::query("SELECT nombre_proveedor FROM proveedores WHERE id_proveedor = ?")
                .bind(req.id_proveedor)
                .fetch_optional(&mut *tx)
                .await
                .map_err(AppError::Database)?
                .ok_or_else(|| {
                    AppError::NotFound(format!(
                        "Proveedor con ID {} no encontrado",
                        req.id_proveedor
                    ))
                })?;

        let prov_nombre: String = prov_row.get("nombre_proveedor");

        // Insertar cabecera de compra
        let insert_res = sqlx::query(
            "INSERT INTO compras_proveedor (id_proveedor, id_usuarios, fecha_hora, total, observaciones) VALUES (?, ?, NOW(), 0.0, ?)"
        )
        .bind(req.id_proveedor)
        .bind(actor_id)
        .bind(&req.observaciones)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        let id_compra = insert_res.last_insert_id() as i64;
        let mut total_compra = 0.0;
        let mut total_articulos = 0;
        let mut items_procesados = Vec::new();

        for item in &req.items {
            if item.cantidad <= 0 {
                return Err(AppError::BadRequest(
                    "La cantidad comprada debe ser mayor a 0".to_string(),
                ));
            }
            if item.costo_unitario < 0.0 {
                return Err(AppError::BadRequest(
                    "El costo unitario no puede ser negativo".to_string(),
                ));
            }

            let subtotal = item.costo_unitario * (item.cantidad as f64);
            total_compra += subtotal;
            total_articulos += item.cantidad;

            let nombre_prod = match item.tipo_producto {
                TipoProducto::Libro => {
                    let row_libro = sqlx::query(
                        "SELECT nombre_libro, cantidad FROM libros WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
                    )
                    .bind(item.codigo_ean)
                    .bind(UBICACION_BODEGA)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(AppError::Database)?;

                    match row_libro {
                        Some(r) => {
                            let nom: String = r.get("nombre_libro");
                            sqlx::query("UPDATE libros SET cantidad = cantidad + ? WHERE codigo_ean = ? AND id_ubicacion = ?")
                                .bind(item.cantidad)
                                .bind(item.codigo_ean)
                                .bind(UBICACION_BODEGA)
                                .execute(&mut *tx)
                                .await
                                .map_err(AppError::Database)?;
                            nom
                        }
                        None => {
                            let row_tienda = sqlx::query(
                                "SELECT id_mueble, id_proveedor, nombre_libro, precio, SKU FROM libros WHERE codigo_ean = ? LIMIT 1"
                            )
                            .bind(item.codigo_ean)
                            .fetch_optional(&mut *tx)
                            .await
                            .map_err(AppError::Database)?;

                            if let Some(rt) = row_tienda {
                                let nom: String = rt.get("nombre_libro");
                                sqlx::query(
                                    r#"
                                    INSERT INTO libros (codigo_ean, id_ubicacion, id_mueble, id_proveedor,
                                                       nombre_libro, precio, cantidad, SKU)
                                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                                    "#,
                                )
                                .bind(item.codigo_ean)
                                .bind(UBICACION_BODEGA)
                                .bind(row_get_i32(&rt, "id_mueble"))
                                .bind(req.id_proveedor)
                                .bind(&nom)
                                .bind(row_get_f64(&rt, "precio"))
                                .bind(item.cantidad)
                                .bind(row_get_opt_i64(&rt, "SKU"))
                                .execute(&mut *tx)
                                .await
                                .map_err(AppError::Database)?;
                                nom
                            } else {
                                format!("Libro EAN {}", item.codigo_ean)
                            }
                        }
                    }
                }
                TipoProducto::Revista => {
                    let row_rev = sqlx::query(
                        "SELECT nombre_revista, cantidad FROM revistas WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
                    )
                    .bind(item.codigo_ean)
                    .bind(UBICACION_BODEGA)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(AppError::Database)?;

                    match row_rev {
                        Some(r) => {
                            let nom: String = r.get("nombre_revista");
                            sqlx::query("UPDATE revistas SET cantidad = cantidad + ? WHERE codigo_ean = ? AND id_ubicacion = ?")
                                .bind(item.cantidad)
                                .bind(item.codigo_ean)
                                .bind(UBICACION_BODEGA)
                                .execute(&mut *tx)
                                .await
                                .map_err(AppError::Database)?;
                            nom
                        }
                        None => {
                            let row_tienda = sqlx::query(
                                "SELECT id_mueble, id_proveedor, nombre_revista, numero_edicion, periodicidad, precio, SKU FROM revistas WHERE codigo_ean = ? LIMIT 1"
                            )
                            .bind(item.codigo_ean)
                            .fetch_optional(&mut *tx)
                            .await
                            .map_err(AppError::Database)?;

                            if let Some(rt) = row_tienda {
                                let nom: String = rt.get("nombre_revista");
                                sqlx::query(
                                    r#"
                                    INSERT INTO revistas (codigo_ean, id_ubicacion, id_mueble, id_proveedor,
                                                         nombre_revista, numero_edicion, periodicidad, precio, cantidad, SKU)
                                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                                    "#,
                                )
                                .bind(item.codigo_ean)
                                .bind(UBICACION_BODEGA)
                                .bind(row_get_i32(&rt, "id_mueble"))
                                .bind(req.id_proveedor)
                                .bind(&nom)
                                .bind(row_get_opt_i32(&rt, "numero_edicion"))
                                .bind(rt.try_get::<Option<String>, _>("periodicidad").ok().flatten())
                                .bind(row_get_f64(&rt, "precio"))
                                .bind(item.cantidad)
                                .bind(row_get_opt_i64(&rt, "SKU"))
                                .execute(&mut *tx)
                                .await
                                .map_err(AppError::Database)?;
                                nom
                            } else {
                                format!("Revista EAN {}", item.codigo_ean)
                            }
                        }
                    }
                }
            };

            let ins_det = sqlx::query(
                r#"
                INSERT INTO detalle_compra (id_compra, codigo_ean, tipo_producto, cantidad, costo_unitario, subtotal)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(id_compra)
            .bind(item.codigo_ean)
            .bind(item.tipo_producto.as_str())
            .bind(item.cantidad)
            .bind(item.costo_unitario)
            .bind(subtotal)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

            items_procesados.push(ItemCompraDto {
                id_detalle_compra: ins_det.last_insert_id() as i64,
                codigo_ean: item.codigo_ean,
                tipo_producto: item.tipo_producto,
                nombre_producto: nombre_prod,
                cantidad: item.cantidad,
                costo_unitario: item.costo_unitario,
                subtotal,
            });
        }

        // Actualizar total en `compras_proveedor`
        sqlx::query("UPDATE compras_proveedor SET total = ? WHERE id_compra = ?")
            .bind(total_compra)
            .bind(id_compra)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        Ok(CompraResponse {
            id_compra,
            id_proveedor: req.id_proveedor,
            nombre_proveedor: prov_nombre,
            id_usuarios: actor_id,
            comprador_nombre: None,
            fecha_hora: "Hoy".to_string(),
            total: total_compra,
            total_articulos,
            observaciones: req.observaciones.clone(),
            items: items_procesados,
            mensaje: "Compra a proveedor registrada e inventario en bodega actualizado".to_string(),
        })
    }

    pub async fn listar_compras(pool: &DbPool) -> Result<Vec<CompraResponse>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT c.id_compra, c.id_proveedor, p.nombre_proveedor, c.id_usuarios,
                   CONCAT(u.nombre, ' ', COALESCE(u.apellido_paterno, '')) as comprador_nombre,
                   DATE_FORMAT(c.fecha_hora, '%Y-%m-%d %H:%i:%s') as fecha_hora,
                   c.total, c.observaciones
            FROM compras_proveedor c
            JOIN proveedores p ON c.id_proveedor = p.id_proveedor
            JOIN usuarios u ON c.id_usuarios = u.id_usuarios
            ORDER BY c.id_compra DESC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

        let mut list = Vec::new();
        for r in rows {
            let id_compra = row_get_i64(&r, "id_compra");
            let det_rows = sqlx::query(
                "SELECT id_detalle_compra, codigo_ean, tipo_producto, cantidad, costo_unitario, subtotal FROM detalle_compra WHERE id_compra = ?"
            )
            .bind(id_compra)
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;

            let mut total_articulos = 0;
            let items: Vec<ItemCompraDto> = det_rows
                .into_iter()
                .map(|dr| {
                    let cant: i32 = row_get_i32(&dr, "cantidad");
                    total_articulos += cant;
                    let tipo_str: String = dr.get("tipo_producto");
                    let tipo = if tipo_str.eq_ignore_ascii_case("revista") {
                        TipoProducto::Revista
                    } else {
                        TipoProducto::Libro
                    };
                    ItemCompraDto {
                        id_detalle_compra: row_get_i64(&dr, "id_detalle_compra"),
                        codigo_ean: row_get_i64(&dr, "codigo_ean"),
                        tipo_producto: tipo,
                        nombre_producto: format!("Artículo {}", row_get_i64(&dr, "codigo_ean")),
                        cantidad: cant,
                        costo_unitario: row_get_f64(&dr, "costo_unitario"),
                        subtotal: row_get_f64(&dr, "subtotal"),
                    }
                })
                .collect();

            list.push(CompraResponse {
                id_compra,
                id_proveedor: row_get_i32(&r, "id_proveedor"),
                nombre_proveedor: r.get("nombre_proveedor"),
                id_usuarios: row_get_i64(&r, "id_usuarios"),
                comprador_nombre: r.get("comprador_nombre"),
                fecha_hora: r.get("fecha_hora"),
                total: row_get_f64(&r, "total"),
                total_articulos,
                observaciones: r.get("observaciones"),
                items,
                mensaje: "OK".to_string(),
            });
        }

        Ok(list)
    }
}
