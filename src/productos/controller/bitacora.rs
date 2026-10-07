use axum::{
    Extension, Json,
    extract::{Query, State},
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    productos::{
        model::{FechaQuery, ResumenMovimientoDiario},
        service::BitacoraService,
    },
    usuarios::model::Claims,
};

pub async fn consultar_movimiento_diario_jefe(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(query): Query<FechaQuery>,
) -> Result<Json<ApiResponse<ResumenMovimientoDiario>>, AppError> {
    let res = BitacoraService::consultar_movimiento_diario(&state.pool, query.fecha).await?;
    Ok(Json(ApiResponse::success(res)))
}
