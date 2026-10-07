use axum::{
    Extension, Json,
    extract::State,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    productos::{
        model::{RegistrarVentaRequest, VentaResponse},
        service::VentaService,
    },
    usuarios::model::Claims,
};

pub async fn registrar_venta_vendedor(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RegistrarVentaRequest>,
) -> Result<Json<ApiResponse<VentaResponse>>, AppError> {
    let res = VentaService::registrar_venta(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Venta procesada exitosamente",
    )))
}
