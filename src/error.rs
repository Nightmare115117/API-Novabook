use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("No autenticado: {0}")]
    Unauthorized(String),

    #[error("Acceso denegado: {0}")]
    Forbidden(String),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Solicitud inválida: {0}")]
    BadRequest(String),

    #[error("Conflicto: {0}")]
    Conflict(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de token JWT: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Error de hash: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("Error interno del servidor: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: ErrorDetail,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::Database(err) => {
                eprintln!("[DB ERROR]: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "Error al procesar la operación en la base de datos".to_string(),
                )
            }
            AppError::Jwt(err) => (
                StatusCode::UNAUTHORIZED,
                "INVALID_TOKEN",
                format!("Token inválido o expirado: {}", err),
            ),
            AppError::Bcrypt(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "BCRYPT_ERROR",
                "Error en el procesamiento seguro de credenciales".to_string(),
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                msg.clone(),
            ),
        };

        let body = Json(ErrorResponse {
            success: false,
            error: ErrorDetail { code, message },
        });

        (status, body).into_response()
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn success_msg(data: T, msg: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(msg.into()),
        }
    }
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

impl MessageResponse {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            success: true,
            message: msg.into(),
        }
    }
}
