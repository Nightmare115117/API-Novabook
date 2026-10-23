use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{CrearGeneroRequest, Genero},
        repo::GeneroRepo,
    },
};

pub struct GeneroService;

impl GeneroService {
    pub async fn listar_generos(pool: &DbPool) -> Result<Vec<Genero>, AppError> {
        GeneroRepo::listar_generos(pool).await
    }

    fn validar(req: &CrearGeneroRequest) -> Result<(), AppError> {
        if req.genero_literario.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre del género es obligatorio".to_string()));
        }
        Ok(())
    }

    pub async fn crear_genero(
        pool: &DbPool,
        actor_id: i64,
        req: CrearGeneroRequest,
    ) -> Result<Genero, AppError> {
        Self::validar(&req)?;
        let g = GeneroRepo::crear_genero(pool, &req).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR GENERO",
            &format!("Género registrado: {} (ID: {})", g.genero_literario, g.id_genero),
        )
        .await;
        Ok(g)
    }

    pub async fn actualizar_genero(
        pool: &DbPool,
        actor_id: i64,
        id: i32,
        req: CrearGeneroRequest,
    ) -> Result<Genero, AppError> {
        Self::validar(&req)?;
        let g = GeneroRepo::actualizar_genero(pool, id, &req).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "ACTUALIZAR GENERO",
            &format!("Género actualizado: {} (ID: {})", g.genero_literario, id),
        )
        .await;
        Ok(g)
    }

    pub async fn eliminar_genero(pool: &DbPool, actor_id: i64, id: i32) -> Result<(), AppError> {
        GeneroRepo::eliminar_genero(pool, id).await?;
        let _ = log_bitacora(
            pool,
            actor_id,
            "ELIMINAR GENERO",
            &format!("Género eliminado con ID: {}", id),
        )
        .await;
        Ok(())
    }
}
