use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{CrearMuebleRequest, Mueble},
        repo::MuebleRepo,
    },
};

pub struct MuebleService;

impl MuebleService {
    pub async fn listar_muebles(pool: &DbPool) -> Result<Vec<Mueble>, AppError> {
        MuebleRepo::listar_muebles(pool).await
    }

    pub async fn crear_mueble(
        pool: &DbPool,
        actor_id: i64,
        req: CrearMuebleRequest,
    ) -> Result<Mueble, AppError> {
        let m = MuebleRepo::crear_mueble(pool, &req).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR MUEBLE",
            &format!("Mueble registrado: ID {} (género: {})", m.id_mueble, m.genero_literario),
        )
        .await;
        Ok(m)
    }

    pub async fn actualizar_mueble(
        pool: &DbPool,
        actor_id: i64,
        id: i32,
        req: CrearMuebleRequest,
    ) -> Result<Mueble, AppError> {
        let m = MuebleRepo::actualizar_mueble(pool, id, &req).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "ACTUALIZAR MUEBLE",
            &format!("Mueble ID {} ahora asignado al género: {}", id, m.genero_literario),
        )
        .await;
        Ok(m)
    }

    pub async fn eliminar_mueble(pool: &DbPool, actor_id: i64, id: i32) -> Result<(), AppError> {
        MuebleRepo::eliminar_mueble(pool, id).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "ELIMINAR MUEBLE",
            &format!("Mueble eliminado con ID: {}", id),
        )
        .await;
        Ok(())
    }
}
