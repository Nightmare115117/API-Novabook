use axum::{
    Extension, Json,
    extract::{Query, State},
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    productos::{
        model::{ExistenciaInventario, FiltroExistencias},
        service::InventarioService,
    },
    usuarios::model::Claims,
};

pub async fn consultar_existencias_vendedor(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(params): Query<FiltroExistencias>,
) -> Result<Json<ApiResponse<Vec<ExistenciaInventario>>>, AppError> {
    let res = InventarioService::consultar_existencias(&state.pool, params).await?;
    Ok(Json(ApiResponse::success(res)))
}
