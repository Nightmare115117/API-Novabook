use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoProducto {
    Libro,
    Revista,
}

impl TipoProducto {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Libro => "Libro",
            Self::Revista => "Revista",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Libro {
    pub codigo_ean: i64,
    pub id_genero: i32,
    pub id_mueble: i32,
    pub id_proveedor: i32,
    pub id_ubicacion: i32,
    pub nombre_libro: String,
    pub precio: f64,
    pub cantidad: i32,
    pub autor: Option<String>,
    pub sku: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Revista {
    pub codigo_ean: i64,
    pub id_mueble: i32,
    pub id_proveedor: i32,
    pub id_ubicacion: i32,
    pub nombre_revista: String,
    pub numero_edicion: Option<i32>,
    pub periodicidad: Option<String>,
    pub precio: f64,
    pub cantidad: i32,
    pub sku: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistrarLibroRequest {
    pub codigo_ean: i64,
    pub id_genero: i32,
    pub id_mueble: i32,
    pub id_proveedor: i32,
    pub nombre_libro: String,
    pub precio: f64,
    pub cantidad: i32,
    pub autor: Option<String>,
    pub sku: Option<i64>,
    pub id_ubicacion: Option<i32>, // Default: 2 (Bodega)
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistrarRevistaRequest {
    pub codigo_ean: i64,
    pub id_mueble: i32,
    pub id_proveedor: i32,
    pub nombre_revista: String,
    pub numero_edicion: Option<i32>,
    pub periodicidad: Option<String>,
    pub precio: f64,
    pub cantidad: i32,
    pub sku: Option<i64>,
    pub id_ubicacion: Option<i32>, // Default: 2 (Bodega)
}
