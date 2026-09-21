use axum::{
    extract::State,
    Extension, Json,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    usuarios::{
        model::{Claims, LoginRequest, LoginResponse, UsuarioDto},
        service::UsuarioService,
    },
};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let res = UsuarioService::login(&state.pool, &state.config, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Inicio de sesión exitoso")))
}

pub async fn perfil_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<UsuarioDto>>, AppError> {
    let res = UsuarioService::obtener_perfil(&state.pool, claims.id_usuario).await?;
    Ok(Json(ApiResponse::success(res)))
}
