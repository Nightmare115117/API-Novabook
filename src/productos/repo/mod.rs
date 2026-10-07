pub mod autor;
pub mod bitacora;
pub mod compra;
pub mod devolucion;
pub mod genero;
pub mod inventario;
pub mod movimiento;
pub mod producto;
pub mod proveedor;
pub mod venta;

pub use autor::*;
pub use bitacora::*;
pub use compra::*;
pub use devolucion::*;
pub use genero::*;
pub use inventario::*;
pub use movimiento::*;
pub use producto::*;
pub use proveedor::*;
pub use venta::*;

pub const UBICACION_TIENDA: i32 = 1;
pub const UBICACION_BODEGA: i32 = 2;
