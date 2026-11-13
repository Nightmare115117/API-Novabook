use sqlx::Row;

use crate::{
    db::{DbPool, row_get_i32},
    error::AppError,
    productos::model::{CrearMotivoRequest, Motivo},
};

pub struct MotivoRepo;

impl MotivoRepo {
    pub async fn listar_motivos(pool: &DbPool) -> Result<Vec<Motivo>, AppError> {
        let rows =
            sqlx::query("SELECT id_motivo, nombre_motivo, descripcion FROM motivo ORDER BY nombre_motivo ASC")
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?;

        Ok(rows
            .into_iter()
            .map(|r| Motivo {
                id_motivo: row_get_i32(&r, "id_motivo"),
                nombre_motivo: r.get("nombre_motivo"),
                descripcion: r.get("descripcion"),
            })
            .collect())
    }

    pub async fn crear_motivo(pool: &DbPool, req: &CrearMotivoRequest) -> Result<Motivo, AppError> {
        let res = sqlx::query("INSERT INTO motivo (nombre_motivo, descripcion) VALUES (?, ?)")
            .bind(&req.nombre_motivo)
            .bind(&req.descripcion)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(Motivo {
            id_motivo: res.last_insert_id() as i32,
            nombre_motivo: req.nombre_motivo.clone(),
            descripcion: req.descripcion.clone(),
        })
    }

    pub async fn actualizar_motivo(
        pool: &DbPool,
        id: i32,
        req: &CrearMotivoRequest,
    ) -> Result<Motivo, AppError> {
        let res = sqlx::query("UPDATE motivo SET nombre_motivo = ?, descripcion = ? WHERE id_motivo = ?")
            .bind(&req.nombre_motivo)
            .bind(&req.descripcion)
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Motivo con ID {} no encontrado",
                id
            )));
        }
        Ok(Motivo {
            id_motivo: id,
            nombre_motivo: req.nombre_motivo.clone(),
            descripcion: req.descripcion.clone(),
        })
    }

    pub async fn eliminar_motivo(pool: &DbPool, id: i32) -> Result<(), AppError> {
        let res = sqlx::query("DELETE FROM motivo WHERE id_motivo = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Motivo con ID {} no encontrado",
                id
            )));
        }
        Ok(())
    }
}
