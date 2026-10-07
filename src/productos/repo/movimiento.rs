use sqlx::Row;

use crate::{
    db::{DbPool, row_get_f64, row_get_i32, row_get_opt_i32, row_get_opt_i64},
    error::AppError,
    productos::repo::UBICACION_BODEGA,
};

pub struct MovimientoRepo;

impl MovimientoRepo {
    pub async fn trasladar_stock_libro(
        pool: &DbPool,
        codigo_ean: i64,
        cantidad: i32,
        origen: i32,
        destino: i32,
        actor_id: i64,
        observaciones: Option<&str>,
    ) -> Result<(i32, i32), AppError> {
        let mut tx = pool.begin().await.map_err(AppError::Database)?;

        let row_origen = sqlx::query(
            "SELECT id_mueble, id_proveedor, nombre_libro, precio, cantidad, SKU FROM libros WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
        )
        .bind(codigo_ean)
        .bind(origen)
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| {
            let lugar = if origen == UBICACION_BODEGA { "Bodega" } else { "Piso de Ventas" };
            AppError::NotFound(format!("El libro con EAN {} no existe en {}", codigo_ean, lugar))
        })?;

        let stock_actual_origen: i32 = row_get_i32(&row_origen, "cantidad");
        if stock_actual_origen < cantidad {
            let lugar = if origen == UBICACION_BODEGA {
                "Bodega"
            } else {
                "Piso de Ventas"
            };
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

        let row_destino = sqlx::query(
            "SELECT cantidad FROM libros WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE",
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
                sqlx::query(
                    "UPDATE libros SET cantidad = ? WHERE codigo_ean = ? AND id_ubicacion = ?",
                )
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
                    INSERT INTO libros (codigo_ean, id_ubicacion, id_mueble, id_proveedor,
                                       nombre_libro, precio, cantidad, SKU)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(codigo_ean)
                .bind(destino)
                .bind(row_get_i32(&row_origen, "id_mueble"))
                .bind(row_get_i32(&row_origen, "id_proveedor"))
                .bind(row_origen.get::<String, _>("nombre_libro"))
                .bind(row_get_f64(&row_origen, "precio"))
                .bind(cantidad)
                .bind(row_get_opt_i64(&row_origen, "SKU"))
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;

                cantidad
            }
        };

        sqlx::query(
            r#"
            INSERT INTO movimientos (id_usuarios, codigo_ean, tipo_producto, origen_ubicacion, destino_ubicacion, cantidad, fecha_hora, observaciones)
            VALUES (?, ?, 'Libro', ?, ?, ?, NOW(), ?)
            "#,
        )
        .bind(actor_id)
        .bind(codigo_ean)
        .bind(origen)
        .bind(destino)
        .bind(cantidad)
        .bind(observaciones)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        Ok((nuevo_origen, nuevo_destino))
    }

    pub async fn trasladar_stock_revista(
        pool: &DbPool,
        codigo_ean: i64,
        cantidad: i32,
        origen: i32,
        destino: i32,
        actor_id: i64,
        observaciones: Option<&str>,
    ) -> Result<(i32, i32), AppError> {
        let mut tx = pool.begin().await.map_err(AppError::Database)?;

        let row_origen = sqlx::query(
            "SELECT id_mueble, id_proveedor, nombre_revista, numero_edicion, periodicidad, precio, cantidad, SKU FROM revistas WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE"
        )
        .bind(codigo_ean)
        .bind(origen)
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| {
            let lugar = if origen == UBICACION_BODEGA { "Bodega" } else { "Piso de Ventas" };
            AppError::NotFound(format!("La revista con EAN {} no existe en {}", codigo_ean, lugar))
        })?;

        let stock_actual_origen: i32 = row_get_i32(&row_origen, "cantidad");
        if stock_actual_origen < cantidad {
            let lugar = if origen == UBICACION_BODEGA {
                "Bodega"
            } else {
                "Piso de Ventas"
            };
            return Err(AppError::BadRequest(format!(
                "Stock insuficiente en {}. Disponible: {}, Solicitado: {}",
                lugar, stock_actual_origen, cantidad
            )));
        }

        let nuevo_origen = stock_actual_origen - cantidad;
        sqlx::query("UPDATE revistas SET cantidad = ? WHERE codigo_ean = ? AND id_ubicacion = ?")
            .bind(nuevo_origen)
            .bind(codigo_ean)
            .bind(origen)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        let row_destino = sqlx::query(
            "SELECT cantidad FROM revistas WHERE codigo_ean = ? AND id_ubicacion = ? FOR UPDATE",
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
                sqlx::query(
                    "UPDATE revistas SET cantidad = ? WHERE codigo_ean = ? AND id_ubicacion = ?",
                )
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
                    INSERT INTO revistas (codigo_ean, id_ubicacion, id_mueble, id_proveedor,
                                         nombre_revista, numero_edicion, periodicidad, precio, cantidad, SKU)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(codigo_ean)
                .bind(destino)
                .bind(row_get_i32(&row_origen, "id_mueble"))
                .bind(row_get_i32(&row_origen, "id_proveedor"))
                .bind(row_origen.get::<String, _>("nombre_revista"))
                .bind(row_get_opt_i32(&row_origen, "numero_edicion"))
                .bind(row_origen.try_get::<Option<String>, _>("periodicidad").ok().flatten())
                .bind(row_get_f64(&row_origen, "precio"))
                .bind(cantidad)
                .bind(row_get_opt_i64(&row_origen, "SKU"))
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;

                cantidad
            }
        };

        sqlx::query(
            r#"
            INSERT INTO movimientos (id_usuarios, codigo_ean, tipo_producto, origen_ubicacion, destino_ubicacion, cantidad, fecha_hora, observaciones)
            VALUES (?, ?, 'Revista', ?, ?, ?, NOW(), ?)
            "#,
        )
        .bind(actor_id)
        .bind(codigo_ean)
        .bind(origen)
        .bind(destino)
        .bind(cantidad)
        .bind(observaciones)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        Ok((nuevo_origen, nuevo_destino))
    }
}
