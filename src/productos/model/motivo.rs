use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Motivo {
    pub id_motivo: i32,
    #[serde(rename = "motivo")]
    pub nombre_motivo: String,
    pub descripcion: Option<String>,
}

// Sirve para POST y PUT (mismos campos).
#[derive(Debug, Clone, Deserialize)]
pub struct CrearMotivoRequest {
    #[serde(alias = "motivo")]
    pub nombre_motivo: String,
    pub descripcion: Option<String>,
}
