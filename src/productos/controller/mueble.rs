use axum::{
    Extension, Json,
    extract::{Path, State},
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError, MessageResponse},
    productos::{
        model::{CrearMuebleRequest, Mueble},
        service::MuebleService,
    },
    usuarios::model::Claims,
};

pub async fn listar_muebles_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Mueble>>>, AppError> {
    let res = MuebleService::listar_muebles(&state.pool).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn crear_mueble_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CrearMuebleRequest>,
) -> Result<Json<ApiResponse<Mueble>>, AppError> {
    let res = MuebleService::crear_mueble(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Mueble registrado exitosamente")))
}

pub async fn actualizar_mueble_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<CrearMuebleRequest>,
) -> Result<Json<ApiResponse<Mueble>>, AppError> {
    let res = MuebleService::actualizar_mueble(&state.pool, claims.id_usuario, id, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Mueble actualizado exitosamente")))
}

pub async fn eliminar_mueble_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<MessageResponse>, AppError> {
    MuebleService::eliminar_mueble(&state.pool, claims.id_usuario, id).await?;
    Ok(Json(MessageResponse::new(format!("Mueble con ID {} eliminado exitosamente", id))))
}
