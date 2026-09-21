use axum::{
    extract::State,
    middleware,
    routing::post,
    Extension, Json, Router,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    middleware::{auth_middleware, require_bodega},
    productos::{
        model::{
            Libro, RegistrarLibroRequest, RegistrarRevistaRequest, Revista, TrasladoRequest,
            TrasladoResponse,
        },
        service::ProductoService,
    },
    usuarios::model::Claims,
};

pub async fn registrar_libro_bodega(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RegistrarLibroRequest>,
) -> Result<Json<ApiResponse<Libro>>, AppError> {
    let res = ProductoService::registrar_libro(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Libro registrado exitosamente en bodega")))
}

pub async fn registrar_revista_bodega(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RegistrarRevistaRequest>,
) -> Result<Json<ApiResponse<Revista>>, AppError> {
    let res = ProductoService::registrar_revista(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Revista registrada exitosamente en bodega")))
}

pub async fn movimiento_bodega_tienda_libros(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrasladoRequest>,
) -> Result<Json<ApiResponse<TrasladoResponse>>, AppError> {
    let res = ProductoService::traslado_bodega_a_tienda_libros(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn movimiento_bodega_tienda_revistas(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrasladoRequest>,
) -> Result<Json<ApiResponse<TrasladoResponse>>, AppError> {
    let res = ProductoService::traslado_bodega_a_tienda_revistas(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/libros", post(registrar_libro_bodega))
        .route("/revistas", post(registrar_revista_bodega))
        .route("/movimientos/libros", post(movimiento_bodega_tienda_libros))
        .route("/movimientos/revistas", post(movimiento_bodega_tienda_revistas))
        .layer(middleware::from_fn(require_bodega))
        .layer(middleware::from_fn_with_state(state.config.clone(), auth_middleware))
}
