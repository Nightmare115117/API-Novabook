use axum::{
    Extension, Json,
    extract::State,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    productos::{
        model::Genero,
        service::GeneroService,
    },
    usuarios::model::Claims,
};

pub async fn listar_generos_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Genero>>>, AppError> {
    let res = GeneroService::listar_generos(&state.pool).await?;
    Ok(Json(ApiResponse::success(res)))
}
