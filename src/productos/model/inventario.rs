use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistenciaInventario {
    pub codigo_ean: i64,
    pub sku: Option<i64>,
    pub titulo: String,
    pub tipo_producto: String,
    pub stock_tienda: i32,
    pub stock_bodega: i32,
    pub stock_total: i32,
    pub precio: f64,
    pub proveedor: Option<String>,
    pub autor_o_editorial: Option<String>,
    pub generos: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FiltroExistencias {
    pub q: Option<String>,
    pub ubicacion: Option<i32>,
    pub tipo: Option<String>,
}
