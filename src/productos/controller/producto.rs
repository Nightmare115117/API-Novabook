use axum::{
    Extension, Json,
    extract::State,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    productos::{
        model::{Libro, RegistrarLibroRequest, RegistrarRevistaRequest, Revista},
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
    Ok(Json(ApiResponse::success_msg(
        res,
        "Libro registrado exitosamente en bodega",
    )))
}

pub async fn registrar_revista_bodega(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RegistrarRevistaRequest>,
) -> Result<Json<ApiResponse<Revista>>, AppError> {
    let res = ProductoService::registrar_revista(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Revista registrada exitosamente en bodega",
    )))
}
