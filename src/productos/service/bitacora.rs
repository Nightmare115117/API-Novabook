use crate::{
    db::DbPool,
    error::AppError,
    productos::{model::ResumenMovimientoDiario, repo::BitacoraRepo},
};

pub struct BitacoraService;

impl BitacoraService {
    pub async fn consultar_movimiento_diario(
        pool: &DbPool,
        fecha: Option<String>,
    ) -> Result<ResumenMovimientoDiario, AppError> {
        let movimientos = BitacoraRepo::consultar_movimiento_diario(pool, fecha.as_deref()).await?;
        let total = movimientos.len();
        let fecha_str = fecha.unwrap_or_else(|| "Hoy".to_string());

        Ok(ResumenMovimientoDiario {
            fecha: fecha_str,
            total_eventos: total,
            movimientos,
        })
    }
}
