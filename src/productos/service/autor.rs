use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{ActualizarAutorRequest, Autor, CrearAutorRequest},
        repo::AutorRepo,
    },
};

pub struct AutorService;

impl AutorService {
    pub async fn listar_autores(pool: &DbPool) -> Result<Vec<Autor>, AppError> {
        AutorRepo::listar_autores(pool).await
    }

    pub async fn obtener_autor(pool: &DbPool, id: i32) -> Result<Autor, AppError> {
        AutorRepo::buscar_autor(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Autor con ID {} no encontrado", id)))
    }

    pub async fn crear_autor(
        pool: &DbPool,
        actor_id: i64,
        req: CrearAutorRequest,
    ) -> Result<Autor, AppError> {
        if req.nombre.trim().is_empty() || req.apellidos.trim().is_empty() {
            return Err(AppError::BadRequest(
                "Nombre y apellidos del autor son obligatorios".to_string(),
            ));
        }

        let autor = AutorRepo::crear_autor(pool, &req).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR AUTOR",
            &format!(
                "Autor registrado: {} (ID: {})",
                autor.nombre_completo(),
                autor.id_autor
            ),
        )
        .await;

        Ok(autor)
    }

    pub async fn actualizar_autor(
        pool: &DbPool,
        actor_id: i64,
        id: i32,
        req: ActualizarAutorRequest,
    ) -> Result<Autor, AppError> {
        if req.nombre.trim().is_empty() || req.apellidos.trim().is_empty() {
            return Err(AppError::BadRequest(
                "Nombre y apellidos del autor son obligatorios".to_string(),
            ));
        }

        let autor = AutorRepo::actualizar_autor(pool, id, &req).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "ACTUALIZAR AUTOR",
            &format!(
                "Autor actualizado: {} (ID: {})",
                autor.nombre_completo(),
                id
            ),
        )
        .await;

        Ok(autor)
    }

    pub async fn eliminar_autor(pool: &DbPool, actor_id: i64, id: i32) -> Result<(), AppError> {
        AutorRepo::eliminar_autor(pool, id).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "ELIMINAR AUTOR",
            &format!("Autor eliminado con ID: {}", id),
        )
        .await;

        Ok(())
    }
}
