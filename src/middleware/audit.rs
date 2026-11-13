use axum::{
    extract::State,
    http::{Request, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::{
    db::{AppState, log_bitacora},
    middleware::verify_jwt,
};

pub async fn audit_middleware(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let actor_id = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .and_then(|token| verify_jwt(token.trim(), &state.config.jwt_secret).ok())
        .map(|claims| claims.id_usuario);

    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let response = next.run(request).await;

    if let Some(actor_id) = actor_id {
        let detail: String = format!("{method} {path} -> {}", response.status())
            .chars()
            .take(255)
            .collect();

        if let Err(error) = log_bitacora(
            &state.pool,
            actor_id,
            &format!("API {method}"),
            &detail,
        )
        .await
        {
            eprintln!(
                "[AUDIT ERROR] No se pudo registrar {method} {path} para usuario {actor_id}: {error}"
            );
        }
    }

    response
}
