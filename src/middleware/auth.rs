use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    config::Config,
    error::AppError,
    usuarios::model::{Claims, RolEnum},
};

/// Genera un token JWT firmado con HMAC-SHA256
pub fn generate_jwt(
    usuario_id: i64,
    rol: RolEnum,
    nombre: &str,
    secret: &str,
    expires_in_hours: i64,
) -> Result<String, AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .as_secs();

    let exp = (now + (expires_in_hours as u64 * 3600)) as usize;

    let claims = Claims {
        sub: usuario_id.to_string(),
        id_usuario: usuario_id,
        rol,
        nombre: nombre.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AppError::Jwt)
}

/// Decodifica y valida la firma y vigencia del token JWT
pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(AppError::Jwt)?;

    Ok(token_data.claims)
}

/// Middleware que extrae el token Bearer, valida el JWT e inyecta Claims en Request Extensions
pub async fn auth_middleware(
    State(config): State<Config>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| {
            AppError::Unauthorized("Cabecera Authorization no proporcionada".to_string())
        })?;

    let token = if let Some(token) = auth_header.strip_prefix("Bearer ") {
        token.trim()
    } else {
        return Err(AppError::Unauthorized(
            "Formato de token inválido. Se espera: Bearer <token>".to_string(),
        ));
    };

    let claims = verify_jwt(token, &config.jwt_secret)?;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Helper para validar si los Claims contienen uno de los roles permitidos
fn validate_role(req: &Request, allowed: &[RolEnum]) -> Result<(), AppError> {
    let claims = req.extensions().get::<Claims>().ok_or_else(|| {
        AppError::Unauthorized("No se encontraron claims de autenticación".to_string())
    })?;

    if allowed.contains(&claims.rol) {
        Ok(())
    } else {
        let allowed_str: Vec<&str> = allowed.iter().map(|r| r.as_str()).collect();
        Err(AppError::Forbidden(format!(
            "Acceso denegado para el rol '{}'. Se requiere uno de: [{}]",
            claims.rol.as_str(),
            allowed_str.join(", ")
        )))
    }
}

/// Middleware de autorización: Permite a Gerente y Jefe de Departamento (hereda de Gerente)
pub async fn require_gerente_o_jefe(req: Request, next: Next) -> Result<Response, AppError> {
    validate_role(&req, &[RolEnum::Gerente, RolEnum::JefeDepartamento])?;
    Ok(next.run(req).await)
}

/// Middleware de autorización: Permite a Bodega, Jefe y Gerente
pub async fn require_bodega_o_gerente(req: Request, next: Next) -> Result<Response, AppError> {
    validate_role(
        &req,
        &[
            RolEnum::PersonalBodega,
            RolEnum::Gerente,
            RolEnum::JefeDepartamento,
        ],
    )?;
    Ok(next.run(req).await)
}

/// Middleware de autorización: Exclusivo para Jefe de Departamento
pub async fn require_jefe(req: Request, next: Next) -> Result<Response, AppError> {
    validate_role(&req, &[RolEnum::JefeDepartamento])?;
    Ok(next.run(req).await)
}

/// Middleware de autorización: Exclusivo para Personal de Bodega
pub async fn require_bodega(req: Request, next: Next) -> Result<Response, AppError> {
    validate_role(&req, &[RolEnum::PersonalBodega])?;
    Ok(next.run(req).await)
}

/// Middleware de autorización: Exclusivo para Vendedor
pub async fn require_vendedor(req: Request, next: Next) -> Result<Response, AppError> {
    validate_role(&req, &[RolEnum::Vendedor])?;
    Ok(next.run(req).await)
}
