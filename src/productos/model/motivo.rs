use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Motivo {
    pub id_motivo: i32,
    pub motivo: String,
}

// Sirve para POST y PUT (mismos campos).
#[derive(Debug, Clone, Deserialize)]
pub struct CrearMotivoRequest {
    pub motivo: String,
}
