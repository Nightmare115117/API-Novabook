use axum::{
    Extension, Json,
    extract::{Path, State},
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError, MessageResponse},
    productos::{
        model::{CrearGeneroRequest, Genero},
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

pub async fn crear_genero_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CrearGeneroRequest>,
) -> Result<Json<ApiResponse<Genero>>, AppError> {
    let res = GeneroService::crear_genero(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Género registrado exitosamente")))
}

pub async fn actualizar_genero_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<CrearGeneroRequest>,
) -> Result<Json<ApiResponse<Genero>>, AppError> {
    let res = GeneroService::actualizar_genero(&state.pool, claims.id_usuario, id, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Género actualizado exitosamente")))
}

pub async fn eliminar_genero_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<MessageResponse>, AppError> {
    GeneroService::eliminar_genero(&state.pool, claims.id_usuario, id).await?;
    Ok(Json(MessageResponse::new(format!("Género con ID {} eliminado exitosamente", id))))
}
