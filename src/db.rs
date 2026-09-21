use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;
use crate::{config::Config, error::AppError};

pub type DbPool = MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: Config,
}

pub async fn create_pool(database_url: &str) -> Result<DbPool, AppError> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await
        .map_err(AppError::Database)
}

/// Registra una acción en la tabla Bitacora para auditoría
pub async fn log_bitacora(
    pool: &DbPool,
    id_usuarios: i64,
    accion: &str,
    detalle: &str,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO bitacora (fecha_hora, accion, detalle, id_usuarios) VALUES (NOW(), ?, ?, ?)"
    )
    .bind(accion)
    .bind(detalle)
    .bind(id_usuarios)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(())
}

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::Row;

/// Extrae de forma segura un i64 de una columna de MySQL sin importar si el tipo en BD es DECIMAL, BIGINT, INT o VARCHAR
pub fn row_get_i64(row: &sqlx::mysql::MySqlRow, col: &str) -> i64 {
    if let Ok(v) = row.try_get::<i64, _>(col) {
        v
    } else if let Ok(d) = row.try_get::<Decimal, _>(col) {
        d.to_i64().unwrap_or_default()
    } else if let Ok(v) = row.try_get::<i32, _>(col) {
        v as i64
    } else if let Ok(s) = row.try_get::<String, _>(col) {
        s.parse::<i64>().unwrap_or_default()
    } else {
        0
    }
}

/// Extrae de forma segura un Option<i64> de una columna de MySQL
pub fn row_get_opt_i64(row: &sqlx::mysql::MySqlRow, col: &str) -> Option<i64> {
    if let Ok(Some(v)) = row.try_get::<Option<i64>, _>(col) {
        Some(v)
    } else if let Ok(Some(d)) = row.try_get::<Option<Decimal>, _>(col) {
        d.to_i64()
    } else if let Ok(Some(v)) = row.try_get::<Option<i32>, _>(col) {
        Some(v as i64)
    } else if let Ok(Some(s)) = row.try_get::<Option<String>, _>(col) {
        s.parse::<i64>().ok()
    } else {
        None
    }
}

/// Extrae de forma segura un i32 de una columna de MySQL (soporta DECIMAL, INT, BIGINT, VARCHAR)
pub fn row_get_i32(row: &sqlx::mysql::MySqlRow, col: &str) -> i32 {
    if let Ok(v) = row.try_get::<i32, _>(col) {
        v
    } else if let Ok(d) = row.try_get::<Decimal, _>(col) {
        d.to_i32().unwrap_or_default()
    } else if let Ok(v) = row.try_get::<i64, _>(col) {
        v as i32
    } else if let Ok(s) = row.try_get::<String, _>(col) {
        s.parse::<i32>().unwrap_or_default()
    } else {
        0
    }
}

/// Extrae de forma segura un Option<i32> de una columna de MySQL
#[allow(dead_code)]
pub fn row_get_opt_i32(row: &sqlx::mysql::MySqlRow, col: &str) -> Option<i32> {
    if let Ok(Some(v)) = row.try_get::<Option<i32>, _>(col) {
        Some(v)
    } else if let Ok(Some(d)) = row.try_get::<Option<Decimal>, _>(col) {
        d.to_i32()
    } else if let Ok(Some(v)) = row.try_get::<Option<i64>, _>(col) {
        Some(v as i32)
    } else if let Ok(Some(s)) = row.try_get::<Option<String>, _>(col) {
        s.parse::<i32>().ok()
    } else {
        None
    }
}

/// Extrae de forma segura un f64 de una columna de MySQL (soporta DECIMAL, DOUBLE, FLOAT, VARCHAR)
pub fn row_get_f64(row: &sqlx::mysql::MySqlRow, col: &str) -> f64 {
    if let Ok(v) = row.try_get::<f64, _>(col) {
        v
    } else if let Ok(d) = row.try_get::<Decimal, _>(col) {
        d.to_f64().unwrap_or_default()
    } else if let Ok(s) = row.try_get::<String, _>(col) {
        s.parse::<f64>().unwrap_or_default()
    } else if let Ok(v) = row.try_get::<i64, _>(col) {
        v as f64
    } else {
        0.0
    }
}
