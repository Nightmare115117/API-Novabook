use serde::{Deserialize, Serialize};
use super::producto::TipoProducto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemVenta {
    pub codigo_ean: i64,
    pub tipo_producto: TipoProducto,
    pub nombre_producto: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venta {
    pub id_venta: i64,
    pub id_vendedor: i64,
    pub fecha_hora: String,
    pub total: f64,
    pub items: Vec<ItemVenta>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemVentaRequest {
    pub codigo_ean: i64,
    #[serde(default = "default_tipo_libro")]
    pub tipo_producto: TipoProducto,
    pub cantidad: i32,
}

fn default_tipo_libro() -> TipoProducto {
    TipoProducto::Libro
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistrarVentaRequest {
    pub items: Vec<ItemVentaRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VentaResponse {
    pub id_venta: i64,
    pub id_vendedor: i64,
    pub fecha_hora: String,
    pub total: f64,
    pub total_articulos: i32,
    pub items: Vec<ItemVenta>,
    pub mensaje: String,
}
