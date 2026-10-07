use axum::{
    Extension, Json,
    extract::{Path, State},
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError, MessageResponse},
    productos::{
        model::{ActualizarProveedorRequest, CrearProveedorRequest, Proveedor},
        service::ProveedorService,
    },
    usuarios::model::Claims,
};

pub async fn listar_proveedores_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Proveedor>>>, AppError> {
    let res = ProveedorService::listar_proveedores(&state.pool).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn obtener_proveedor_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Proveedor>>, AppError> {
    let res = ProveedorService::obtener_proveedor(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn crear_proveedor_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CrearProveedorRequest>,
) -> Result<Json<ApiResponse<Proveedor>>, AppError> {
    let res = ProveedorService::crear_proveedor(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Proveedor registrado exitosamente",
    )))
}

pub async fn actualizar_proveedor_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarProveedorRequest>,
) -> Result<Json<ApiResponse<Proveedor>>, AppError> {
    let res =
        ProveedorService::actualizar_proveedor(&state.pool, claims.id_usuario, id, payload).await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Proveedor actualizado exitosamente",
    )))
}

pub async fn eliminar_proveedor_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<MessageResponse>, AppError> {
    ProveedorService::eliminar_proveedor(&state.pool, claims.id_usuario, id).await?;
    Ok(Json(MessageResponse::new(format!(
        "Proveedor con ID {} procesado para baja",
        id
    ))))
}
