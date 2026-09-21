use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MovimientoDiarioItem {
    pub fecha_hora: String,
    pub usuario: String,
    pub rol: String,
    pub accion: String,
    pub detalle: String,
}

#[derive(Debug, Serialize)]
pub struct ResumenMovimientoDiario {
    pub fecha: String,
    pub total_eventos: usize,
    pub movimientos: Vec<MovimientoDiarioItem>,
}

#[derive(Debug, Deserialize)]
pub struct FechaQuery {
    pub fecha: Option<String>,
}
