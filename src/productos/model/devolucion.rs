use serde::{Deserialize, Serialize};
use super::producto::TipoProducto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDevolucion {
    pub codigo_ean: i64,
    pub sku: Option<i64>,
    pub titulo: String,
    pub cantidad: i32,
    pub motivo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devolucion {
    pub id_devolucion: i64,
    pub fecha: String,
    pub total_piezas: i32,
    pub estado: String,
    pub autorizado_por: Option<String>,
    pub id_usuarios: i64,
    pub vendedor_nombre: Option<String>,
    pub id_proveedor: i32,
    pub nombre_proveedor: Option<String>,
    pub tipo_producto: TipoProducto,
    pub items: Vec<ItemDevolucion>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemDevolucionRequest {
    pub codigo_ean: i64,
    pub cantidad: i32,
    pub motivo: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrearDevolucionRequest {
    pub id_proveedor: i32,
    #[serde(default = "default_tipo_libro")]
    pub tipo_producto: TipoProducto,
    pub items: Vec<ItemDevolucionRequest>,
}

fn default_tipo_libro() -> TipoProducto {
    TipoProducto::Libro
}

#[derive(Debug, Clone, Deserialize)]
pub struct AprobarDevolucionRequest {
    #[serde(default = "default_true")]
    pub aprobar: bool,
    pub notas: Option<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct HistorialDevolucionQuery {
    pub fecha: Option<String>,
    pub estado: Option<String>,
    pub id_proveedor: Option<i32>,
}
