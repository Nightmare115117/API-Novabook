use axum::{
    Extension, Json,
    extract::{Path, State},
};

use crate::{
    db::AppState,
    error::{ApiResponse, AppError, MessageResponse},
    productos::{
        model::{ActualizarAutorRequest, Autor, CrearAutorRequest},
        service::AutorService,
    },
    usuarios::model::Claims,
};

pub async fn listar_autores_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Autor>>>, AppError> {
    let res = AutorService::listar_autores(&state.pool).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn obtener_autor_handler(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Autor>>, AppError> {
    let res = AutorService::obtener_autor(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(res)))
}

pub async fn crear_autor_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CrearAutorRequest>,
) -> Result<Json<ApiResponse<Autor>>, AppError> {
    let res = AutorService::crear_autor(&state.pool, claims.id_usuario, payload).await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Autor registrado exitosamente",
    )))
}

pub async fn actualizar_autor_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarAutorRequest>,
) -> Result<Json<ApiResponse<Autor>>, AppError> {
    let res =
        AutorService::actualizar_autor(&state.pool, claims.id_usuario, id, payload).await?;
    Ok(Json(ApiResponse::success_msg(
        res,
        "Autor actualizado exitosamente",
    )))
}

pub async fn eliminar_autor_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<MessageResponse>, AppError> {
    AutorService::eliminar_autor(&state.pool, claims.id_usuario, id).await?;
    Ok(Json(MessageResponse::new(format!(
        "Autor con ID {} eliminado exitosamente",
        id
    ))))
}
