use sqlx::Row;

use crate::{
    db::{DbPool, row_get_f64, row_get_i32},
    error::AppError,
    productos::{
        model::{ItemVenta, RegistrarVentaRequest, TipoProducto, VentaResponse},
        repo::UBICACION_TIENDA,
    },
};

pub struct VentaRepo;

impl VentaRepo {
    pub async fn registrar_venta_transaccional(
        pool: &DbPool,
        actor_id: i64,
        req: &RegistrarVentaRequest,
    ) -> Result<VentaResponse, AppError> {
        let mut tx = pool.begin().await.map_err(AppError::Database)?;

        let nombre_cli = if req.cliente.trim().is_empty() {
            "Público en General"
        } else {
            req.cliente.trim()
        };

        let insert_res = sqlx::query(
            "INSERT INTO ventas (id_usuarios, nombre_cliente, fecha_hora, total) VALUES (?, ?, NOW(), 0.0)"
        )
        .bind(actor_id)
        .bind(nombre_cli)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        let id_venta = insert_res.last_insert_id() as i64;
        let mut total_venta = 0.0;
        let mut total_articulos = 0;
        let mut items_procesados = Vec::new();

        for item in &req.items {
            if item.cantidad <= 0 {
                return Err(AppError::BadRequest(
                    "La cantidad vendida debe ser mayor a 0".to_string(),
                ));
            }

            let (nombre, precio) = match item.tipo_producto {
                TipoProducto::Libro => {
                    let row = sqlx::query(
                        "SELECT nombre_libro, precio, cantidad FROM libros WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
                    )
                    .bind(item.codigo_ean)
                    .bind(UBICACION_TIENDA)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(AppError::Database)?
                    .ok_or_else(|| {
                        AppError::NotFound(format!("El libro con EAN {} no existe en Piso de Ventas", item.codigo_ean))
                    })?;

                    let nombre: String = row.get("nombre_libro");
                    let precio: f64 = row.get("precio");
                    let stock_actual: i32 = row.get("cantidad");

                    if stock_actual < item.cantidad {
                        return Err(AppError::BadRequest(format!(
                            "Stock insuficiente en tienda para '{}'. Disponible: {}, Solicitado: {}",
                            nombre, stock_actual, item.cantidad
                        )));
                    }

                    sqlx::query("UPDATE libros SET cantidad = cantidad - ? WHERE codigo_ean = ? AND id_ubicacion = ?")
                        .bind(item.cantidad)
                        .bind(item.codigo_ean)
                        .bind(UBICACION_TIENDA)
                        .execute(&mut *tx)
                        .await
                        .map_err(AppError::Database)?;

                    (nombre, precio)
                }
                TipoProducto::Revista => {
                    let row = sqlx::query(
                        "SELECT nombre_revista, precio, cantidad FROM revistas WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
                    )
                    .bind(item.codigo_ean)
                    .bind(UBICACION_TIENDA)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(AppError::Database)?
                    .ok_or_else(|| {
                        AppError::NotFound(format!("La revista con EAN {} no existe en Piso de Ventas", item.codigo_ean))
                    })?;

                    let nombre: String = row.get("nombre_revista");
                    let precio: f64 = row_get_f64(&row, "precio");
                    let stock_actual: i32 = row_get_i32(&row, "cantidad");

                    if stock_actual < item.cantidad {
                        return Err(AppError::BadRequest(format!(
                            "Stock insuficiente en tienda para revista '{}'. Disponible: {}, Solicitado: {}",
                            nombre, stock_actual, item.cantidad
                        )));
                    }

                    sqlx::query("UPDATE revistas SET cantidad = cantidad - ? WHERE codigo_ean = ? AND id_ubicacion = ?")
                        .bind(item.cantidad)
                        .bind(item.codigo_ean)
                        .bind(UBICACION_TIENDA)
                        .execute(&mut *tx)
                        .await
                        .map_err(AppError::Database)?;

                    (nombre, precio)
                }
            };

            let subtotal = precio * (item.cantidad as f64);
            total_venta += subtotal;
            total_articulos += item.cantidad;

            sqlx::query(
                r#"
                INSERT INTO detalle_ventas (id_venta, codigo_ean, tipo_producto, cantidad, precio_unitario, subtotal)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(id_venta)
            .bind(item.codigo_ean)
            .bind(item.tipo_producto.as_str())
            .bind(item.cantidad)
            .bind(precio)
            .bind(subtotal)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

            items_procesados.push(ItemVenta {
                codigo_ean: item.codigo_ean,
                tipo_producto: item.tipo_producto,
                nombre_producto: nombre,
                cantidad: item.cantidad,
                precio_unitario: precio,
                subtotal,
            });
        }

        sqlx::query("UPDATE ventas SET total = ? WHERE id_venta = ?")
            .bind(total_venta)
            .bind(id_venta)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        Ok(VentaResponse {
            id_venta,
            id_vendedor: actor_id,
            cliente: nombre_cli.to_string(),
            fecha_hora: "Hoy".to_string(),
            total: total_venta,
            total_articulos,
            items: items_procesados,
            mensaje: "Venta registrada y procesada exitosamente".to_string(),
        })
    }
}
