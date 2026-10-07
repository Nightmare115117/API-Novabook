use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    productos::{
        model::{
            AprobarDevolucionRequest, CrearDevolucionRequest, Devolucion, HistorialDevolucionQuery,
            TipoProducto,
        },
        service::DevolucionService,
    },
    usuarios::model::Claims,
};

pub async fn crear_devolucion_vendedor(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CrearDevolucionRequest>,
) -> Result<Json<ApiResponse<Devolucion>>, AppError> {
    let res =
        DevolucionService::crear_devolucion(&state.pool, claims.id_usuario, &claims.nombre, payload)
            .await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Devolución registrada exitosamente (Pendiente de autorización)",
    )))
}

pub async fn generar_pdf_libros_vendedor(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    let (pdf_bytes, filename) =
        DevolucionService::generar_pdf_devolucion(&state.pool, id, TipoProducto::Libro).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/pdf"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
    );

    Ok((StatusCode::OK, headers, Body::from(pdf_bytes)).into_response())
}

pub async fn generar_pdf_revistas_vendedor(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    let (pdf_bytes, filename) =
        DevolucionService::generar_pdf_devolucion(&state.pool, id, TipoProducto::Revista).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/pdf"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
    );

    Ok((StatusCode::OK, headers, Body::from(pdf_bytes)).into_response())
}

pub async fn consultar_historial_devoluciones_jefe(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(params): Query<HistorialDevolucionQuery>,
) -> Result<Json<ApiResponse<Vec<Devolucion>>>, AppError> {
    let res = DevolucionService::consultar_historial_devoluciones(&state.pool, params).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn aprobar_devolucion_jefe(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(payload): Json<AprobarDevolucionRequest>,
) -> Result<Json<ApiResponse<Devolucion>>, AppError> {
    let res = DevolucionService::aprobar_devolucion(
        &state.pool,
        claims.id_usuario,
        &claims.nombre,
        id,
        payload,
    )
    .await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Devolución evaluada exitosamente",
    )))
}
