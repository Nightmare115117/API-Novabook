use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Autor {
    pub id_autor: i32,
    pub nombre: String,
    pub apellidos: String,
    pub nacionalidad: Option<String>,
    pub biografia: Option<String>,
}

impl Autor {
    pub fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellidos)
            .trim()
            .to_string()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrearAutorRequest {
    pub nombre: String,
    pub apellidos: String,
    pub nacionalidad: Option<String>,
    pub biografia: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActualizarAutorRequest {
    pub nombre: String,
    pub apellidos: String,
    pub nacionalidad: Option<String>,
    pub biografia: Option<String>,
}
