use axum::{Extension, Json, extract::State};

use crate::{
    db::{AppState, log_bitacora_tipo},
    error::{ApiResponse, AppError},
    usuarios::{
        model::{Claims, LoginRequest, LoginResponse, UsuarioDto},
        repo::UsuarioRepo,
        service::UsuarioService,
    },
};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let id_usuario = payload.id_usuario;
    let res = match UsuarioService::login(&state.pool, &state.config, payload).await {
        Ok(response) => response,
        Err(error @ AppError::Unauthorized(_)) => {
            match UsuarioRepo::existe(&state.pool, id_usuario).await {
                Ok(true) => {
                    if let Err(audit_error) = log_bitacora_tipo(
                        &state.pool,
                        id_usuario,
                        2,
                        "INICIO DE SESIÓN FALLIDO",
                        "Intento de autenticación rechazado",
                    )
                    .await
                    {
                        eprintln!(
                            "[AUDIT ERROR] No se pudo registrar intento fallido de inicio de sesión para usuario {id_usuario}: {audit_error}"
                        );
                    }
                }
                Ok(false) => {}
                Err(audit_error) => {
                    eprintln!(
                        "[AUDIT ERROR] No se pudo verificar usuario {id_usuario} para registrar intento de inicio de sesión: {audit_error}"
                    );
                }
            }
            return Err(error);
        }
        Err(error) => return Err(error),
    };
    Ok(Json(ApiResponse::success_msg(
        res,
        "Inicio de sesión exitoso",
    )))
}

pub async fn perfil_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<UsuarioDto>>, AppError> {
    let res = UsuarioService::obtener_perfil(&state.pool, claims.id_usuario).await?;
    Ok(Json(ApiResponse::success(res)))
}
