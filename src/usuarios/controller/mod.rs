pub mod auth;
pub mod usuario;

use crate::{
    db::AppState,
    middleware::{auth_middleware, require_gerente_o_jefe},
};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

pub fn router(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/login", post(auth::login_handler))
        .route(
            "/perfil",
            get(auth::perfil_handler).layer(middleware::from_fn_with_state(
                state.config.clone(),
                auth_middleware,
            )),
        );

    let crud_routes = Router::new()
        .route("/", get(usuario::listar_usuarios_handler))
        .route("/", post(usuario::crear_usuario_handler))
        .route("/{id}", get(usuario::obtener_usuario_handler))
        .route("/{id}", put(usuario::actualizar_usuario_handler))
        .route("/{id}", delete(usuario::eliminar_usuario_handler))
        .layer(middleware::from_fn(require_gerente_o_jefe))
        .layer(middleware::from_fn_with_state(
            state.config.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api/usuarios", crud_routes)
}
