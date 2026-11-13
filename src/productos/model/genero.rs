use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Genero {
    pub id_genero: i32,
    pub genero_literario: String,
    pub total_libros: i64,
    pub total_revistas: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrearGeneroRequest {
    pub genero_literario: String,
}
