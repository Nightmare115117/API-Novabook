use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{CrearMotivoRequest, Motivo},
        repo::MotivoRepo,
    },
};

pub struct MotivoService;

impl MotivoService {
    pub async fn listar_motivos(pool: &DbPool) -> Result<Vec<Motivo>, AppError> {
        MotivoRepo::listar_motivos(pool).await
    }

    fn validar(req: &CrearMotivoRequest) -> Result<(), AppError> {
        if req.motivo.trim().is_empty() {
            return Err(AppError::BadRequest("El motivo es obligatorio".to_string()));
        }
        Ok(())
    }

    pub async fn crear_motivo(
        pool: &DbPool,
        actor_id: i64,
        req: CrearMotivoRequest,
    ) -> Result<Motivo, AppError> {
        Self::validar(&req)?;
        let m = MotivoRepo::crear_motivo(pool, &req).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR MOTIVO",
            &format!("Motivo registrado: {} (ID: {})", m.motivo, m.id_motivo),
        )
        .await;
        Ok(m)
    }

    pub async fn actualizar_motivo(
        pool: &DbPool,
        actor_id: i64,
        id: i32,
        req: CrearMotivoRequest,
    ) -> Result<Motivo, AppError> {
        Self::validar(&req)?;
        let m = MotivoRepo::actualizar_motivo(pool, id, &req).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "ACTUALIZAR MOTIVO",
            &format!("Motivo actualizado: {} (ID: {})", m.motivo, id),
        )
        .await;
        Ok(m)
    }

    pub async fn eliminar_motivo(pool: &DbPool, actor_id: i64, id: i32) -> Result<(), AppError> {
        MotivoRepo::eliminar_motivo(pool, id).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "ELIMINAR MOTIVO",
            &format!("Motivo eliminado con ID: {}", id),
        )
        .await;
        Ok(())
    }
}
