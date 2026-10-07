use crate::{
    db::DbPool,
    error::AppError,
    productos::{
        model::{ExistenciaInventario, FiltroExistencias},
        repo::InventarioRepo,
    },
};

pub struct InventarioService;

impl InventarioService {
    pub async fn consultar_existencias(
        pool: &DbPool,
        params: FiltroExistencias,
    ) -> Result<Vec<ExistenciaInventario>, AppError> {
        InventarioRepo::consultar_existencias(pool, &params).await
    }
}
