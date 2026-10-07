use axum::{
    Extension, Json,
    extract::State,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    productos::{
        model::{TrasladoRequest, TrasladoResponse},
        service::MovimientoService,
    },
    usuarios::model::Claims,
};

pub async fn movimiento_bodega_tienda_libros(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrasladoRequest>,
) -> Result<Json<ApiResponse<TrasladoResponse>>, AppError> {
    let res =
        MovimientoService::traslado_bodega_a_tienda_libros(&state.pool, claims.id_usuario, payload)
            .await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn movimiento_bodega_tienda_revistas(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrasladoRequest>,
) -> Result<Json<ApiResponse<TrasladoResponse>>, AppError> {
    let res =
        MovimientoService::traslado_bodega_a_tienda_revistas(&state.pool, claims.id_usuario, payload)
            .await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn movimiento_tienda_bodega_libros(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrasladoRequest>,
) -> Result<Json<ApiResponse<TrasladoResponse>>, AppError> {
    let res =
        MovimientoService::traslado_tienda_a_bodega_libros(&state.pool, claims.id_usuario, payload)
            .await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn movimiento_tienda_bodega_revistas(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrasladoRequest>,
) -> Result<Json<ApiResponse<TrasladoResponse>>, AppError> {
    let res =
        MovimientoService::traslado_tienda_a_bodega_revistas(&state.pool, claims.id_usuario, payload)
            .await?;
    Ok(Json(ApiResponse::success(res)))
}
