use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Mueble {
    pub id_mueble: i32,
    pub id_genero: i32,
    pub id_ubicacion: i32,
    pub codigo_mueble: String,
    pub nombre_mueble: String,
    pub capacidad: Option<i32>,
    pub capacidad_actual: i32,
    pub descripcion: Option<String>,
    pub genero_literario: String,
}

// Sirve para POST y PUT (mismos campos).
#[derive(Debug, Clone, Deserialize)]
pub struct CrearMuebleRequest {
    pub id_genero: i32,
    pub id_ubicacion: i32,
    pub codigo_mueble: String,
    pub nombre_mueble: String,
    pub capacidad: Option<i32>,
    pub descripcion: Option<String>,
}
