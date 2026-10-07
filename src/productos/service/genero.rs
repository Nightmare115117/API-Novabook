use crate::{
    db::DbPool,
    error::AppError,
    productos::{model::Genero, repo::GeneroRepo},
};

pub struct GeneroService;

impl GeneroService {
    pub async fn listar_generos(pool: &DbPool) -> Result<Vec<Genero>, AppError> {
        GeneroRepo::listar_generos(pool).await
    }
}
