use sqlx::Row;

use crate::{
    db::{DbPool, row_get_i32},
    error::AppError,
    productos::model::{CrearMotivoRequest, Motivo},
};

pub struct MotivoRepo;

impl MotivoRepo {
    pub async fn listar_motivos(pool: &DbPool) -> Result<Vec<Motivo>, AppError> {
        let rows = sqlx::query("SELECT id_motivo, motivo FROM motivos ORDER BY motivo ASC")
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(rows
            .into_iter()
            .map(|r| Motivo {
                id_motivo: row_get_i32(&r, "id_motivo"),
                motivo: r.get("motivo"),
            })
            .collect())
    }

    pub async fn crear_motivo(pool: &DbPool, req: &CrearMotivoRequest) -> Result<Motivo, AppError> {
        let res = sqlx::query("INSERT INTO motivos (motivo) VALUES (?)")
            .bind(&req.motivo)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(Motivo {
            id_motivo: res.last_insert_id() as i32,
            motivo: req.motivo.clone(),
        })
    }

    pub async fn actualizar_motivo(
        pool: &DbPool,
        id: i32,
        req: &CrearMotivoRequest,
    ) -> Result<Motivo, AppError> {
        let res = sqlx::query("UPDATE motivos SET motivo = ? WHERE id_motivo = ?")
            .bind(&req.motivo)
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Motivo con ID {} no encontrado", id)));
        }
        Ok(Motivo { id_motivo: id, motivo: req.motivo.clone() })
    }

    // Ninguna tabla referencia motivos por FK (devolucion/movimientos guardan texto), se borra directo.
    pub async fn eliminar_motivo(pool: &DbPool, id: i32) -> Result<(), AppError> {
        let res = sqlx::query("DELETE FROM motivos WHERE id_motivo = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Motivo con ID {} no encontrado", id)));
        }
        Ok(())
    }
}
