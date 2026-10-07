use sqlx::Row;

use crate::{
    db::DbPool,
    error::AppError,
    productos::model::MovimientoDiarioItem,
};

pub struct BitacoraRepo;

impl BitacoraRepo {
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
