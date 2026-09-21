use axum::{
    extract::{Path, State},
    Extension, Json,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError, MessageResponse},
    usuarios::{
        model::{
            ActualizarUsuarioRequest, Claims, CrearUsuarioRequest, UsuarioDto,
        },
        service::UsuarioService,
    },
};

pub async fn listar_usuarios_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<UsuarioDto>>>, AppError> {
    let res = UsuarioService::listar_usuarios(&state.pool).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn obtener_usuario_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<UsuarioDto>>, AppError> {
    let res = UsuarioService::obtener_usuario(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn crear_usuario_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CrearUsuarioRequest>,
) -> Result<Json<ApiResponse<UsuarioDto>>, AppError> {
    let res = UsuarioService::crear_usuario(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Usuario creado exitosamente")))
}

pub async fn actualizar_usuario_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(payload): Json<ActualizarUsuarioRequest>,
) -> Result<Json<ApiResponse<UsuarioDto>>, AppError> {
    let res = UsuarioService::actualizar_usuario(&state.pool, claims.id_usuario, id, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Usuario actualizado exitosamente")))
}

pub async fn eliminar_usuario_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    UsuarioService::eliminar_usuario(&state.pool, claims.id_usuario, id).await?;
    Ok(Json(MessageResponse::new(format!("Usuario con ID {} eliminado exitosamente", id))))
}
