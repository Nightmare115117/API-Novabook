use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Extension, Json, Router,
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError},
    middleware::{auth_middleware, require_bodega, require_jefe, require_vendedor},
    productos::{
        model::{
            AprobarDevolucionRequest, CrearDevolucionRequest, Devolucion, ExistenciaInventario,
            FechaQuery, FiltroExistencias, HistorialDevolucionQuery, Libro, RegistrarLibroRequest,
            RegistrarRevistaRequest, RegistrarVentaRequest, ResumenMovimientoDiario, Revista,
            TipoProducto, TrasladoRequest, TrasladoResponse, VentaResponse,
        },
        service::ProductoService,
    },
    usuarios::model::Claims,
};

// ============================================================================
// Handlers: Personal de Bodega
// ============================================================================

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

// ============================================================================
// Handlers: Vendedor
// ============================================================================

pub async fn registrar_venta_vendedor(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RegistrarVentaRequest>,
) -> Result<Json<ApiResponse<VentaResponse>>, AppError> {
    let res = ProductoService::registrar_venta(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Venta procesada exitosamente")))
}

pub async fn consultar_existencias_vendedor(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(params): Query<FiltroExistencias>,
) -> Result<Json<ApiResponse<Vec<ExistenciaInventario>>>, AppError> {
    let res = ProductoService::consultar_existencias(&state.pool, params).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn movimiento_tienda_bodega_libros(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrasladoRequest>,
) -> Result<Json<ApiResponse<TrasladoResponse>>, AppError> {
    let res = ProductoService::traslado_tienda_a_bodega_libros(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn movimiento_tienda_bodega_revistas(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TrasladoRequest>,
) -> Result<Json<ApiResponse<TrasladoResponse>>, AppError> {
    let res = ProductoService::traslado_tienda_a_bodega_revistas(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn crear_devolucion_vendedor(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CrearDevolucionRequest>,
) -> Result<Json<ApiResponse<Devolucion>>, AppError> {
    let res = ProductoService::crear_devolucion(&state.pool, claims.id_usuario, &claims.nombre, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Devolución registrada exitosamente (Pendiente de autorización)")))
}

pub async fn generar_pdf_libros_vendedor(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    let (pdf_bytes, filename) = ProductoService::generar_pdf_devolucion(&state.pool, id, TipoProducto::Libro).await?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
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
    let (pdf_bytes, filename) = ProductoService::generar_pdf_devolucion(&state.pool, id, TipoProducto::Revista).await?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
    );

    Ok((StatusCode::OK, headers, Body::from(pdf_bytes)).into_response())
}

// ============================================================================
// Handlers: Jefe de Departamento
// ============================================================================

pub async fn consultar_historial_devoluciones_jefe(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(params): Query<HistorialDevolucionQuery>,
) -> Result<Json<ApiResponse<Vec<Devolucion>>>, AppError> {
    let res = ProductoService::consultar_historial_devoluciones(&state.pool, params).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn consultar_movimiento_diario_jefe(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(query): Query<FechaQuery>,
) -> Result<Json<ApiResponse<ResumenMovimientoDiario>>, AppError> {
    let res = ProductoService::consultar_movimiento_diario(&state.pool, query.fecha).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn aprobar_devolucion_jefe(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(payload): Json<AprobarDevolucionRequest>,
) -> Result<Json<ApiResponse<Devolucion>>, AppError> {
    let res = ProductoService::aprobar_devolucion(&state.pool, claims.id_usuario, &claims.nombre, id, payload).await?;
    Ok(Json(ApiResponse::success_msg(res, "Devolución evaluada exitosamente")))
}

// ============================================================================
// Router Ensamblado para Productos (Bodega, Vendedor, Jefe)
// ============================================================================

pub fn router(state: AppState) -> Router<AppState> {
    let bodega_routes = Router::new()
        .route("/libros", post(registrar_libro_bodega))
        .route("/revistas", post(registrar_revista_bodega))
        .route("/movimientos/libros", post(movimiento_bodega_tienda_libros))
        .route("/movimientos/revistas", post(movimiento_bodega_tienda_revistas))
        .layer(middleware::from_fn(require_bodega))
        .layer(middleware::from_fn_with_state(state.config.clone(), auth_middleware));

    let vendedor_routes = Router::new()
        .route("/ventas", post(registrar_venta_vendedor))
        .route("/existencias", get(consultar_existencias_vendedor))
        .route("/movimientos/libros", post(movimiento_tienda_bodega_libros))
        .route("/movimientos/revistas", post(movimiento_tienda_bodega_revistas))
        .route("/devoluciones", post(crear_devolucion_vendedor))
        .route("/devoluciones/{id}/pdf/libros", get(generar_pdf_libros_vendedor))
        .route("/devoluciones/{id}/pdf/revistas", get(generar_pdf_revistas_vendedor))
        .layer(middleware::from_fn(require_vendedor))
        .layer(middleware::from_fn_with_state(state.config.clone(), auth_middleware));

    let jefe_routes = Router::new()
        .route("/devoluciones/historial", get(consultar_historial_devoluciones_jefe))
        .route("/movimientos/diarios", get(consultar_movimiento_diario_jefe))
        .route("/devoluciones/{id}/aprobar", put(aprobar_devolucion_jefe))
        .layer(middleware::from_fn(require_jefe))
        .layer(middleware::from_fn_with_state(state.config.clone(), auth_middleware));

    Router::new()
        .nest("/api/bodega", bodega_routes)
        .nest("/api/vendedor", vendedor_routes)
        .nest("/api/jefe", jefe_routes)
}
