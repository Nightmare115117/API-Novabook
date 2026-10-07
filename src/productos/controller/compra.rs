use axum::{
    Extension, Json,
    extract::State,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    productos::{
        model::{CompraResponse, RegistrarCompraRequest},
        service::CompraService,
    },
    usuarios::model::Claims,
};

pub async fn registrar_compra_bodega(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RegistrarCompraRequest>,
) -> Result<Json<ApiResponse<CompraResponse>>, AppError> {
    let res = CompraService::registrar_compra_proveedor(&state.pool, claims.id_usuario, payload)
        .await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Compra a proveedor registrada e inventario actualizado",
    )))
}

pub async fn listar_compras_bodega(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<CompraResponse>>>, AppError> {
    let res = CompraService::listar_compras_proveedor(&state.pool).await?;
    Ok(Json(ApiResponse::success(res)))
}
