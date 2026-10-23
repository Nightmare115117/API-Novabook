use axum::{
    Extension, Json,
    extract::{Path, State},
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError, MessageResponse},
    productos::{
        model::{CrearMotivoRequest, Motivo},
        service::MotivoService,
    },
    usuarios::model::Claims,
};

pub async fn listar_motivos_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Motivo>>>, AppError> {
    let res = MotivoService::listar_motivos(&state.pool).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn crear_motivo_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CrearMotivoRequest>,
) -> Result<Json<ApiResponse<Motivo>>, AppError> {
    let res = MotivoService::crear_motivo(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Motivo registrado exitosamente")))
}

pub async fn actualizar_motivo_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<CrearMotivoRequest>,
) -> Result<Json<ApiResponse<Motivo>>, AppError> {
    let res = MotivoService::actualizar_motivo(&state.pool, claims.id_usuario, id, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Motivo actualizado exitosamente")))
}

pub async fn eliminar_motivo_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<MessageResponse>, AppError> {
    MotivoService::eliminar_motivo(&state.pool, claims.id_usuario, id).await?;
    Ok(Json(MessageResponse::new(format!("Motivo con ID {} eliminado exitosamente", id))))
}
