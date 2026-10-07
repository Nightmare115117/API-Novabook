use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Proveedor {
    pub id_proveedor: i32,
    pub nombre_proveedor: String,
    pub rfc: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub persona_contacto: Option<String>,
    pub estatus: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrearProveedorRequest {
    pub nombre_proveedor: String,
    pub rfc: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub persona_contacto: Option<String>,
    #[serde(default = "default_estatus_activo")]
    pub estatus: String,
}

fn default_estatus_activo() -> String {
    "ACTIVO".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActualizarProveedorRequest {
    pub nombre_proveedor: String,
    pub rfc: String,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub direccion: Option<String>,
    pub persona_contacto: Option<String>,
    #[serde(default = "default_estatus_activo")]
    pub estatus: String,
}
