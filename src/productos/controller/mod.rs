pub mod autor;
pub mod bitacora;
pub mod bodega;
pub mod compra;
pub mod devolucion;
pub mod genero;
pub mod inventario;
pub mod motivo;
pub mod movimiento;
pub mod mueble;
pub mod producto;
pub mod proveedor;
pub mod venta;

pub use autor::*;
pub use bitacora::*;
pub use compra::*;
pub use devolucion::*;
pub use genero::*;
pub use inventario::*;
pub use motivo::*;
pub use movimiento::*;
pub use mueble::*;
pub use producto::*;
pub use proveedor::*;
pub use venta::*;

use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::{
    db::AppState,
    middleware::{
        auth_middleware, require_bodega, require_bodega_o_gerente, require_jefe, require_vendedor,
    },
};

// ============================================================================
// Router Ensamblado para Productos, Catálogos, Bodega, Vendedor y Jefe
// ============================================================================

pub fn router(state: AppState) -> Router<AppState> {
    // Rutas de Catálogos (Géneros, Motivos, Muebles, Autores, Proveedores)
    let escritura = middleware::from_fn(require_bodega_o_gerente);
    let catalogos_routes = Router::new()
        .route(
            "/generos",
            get(genero::listar_generos_handler)
                .merge(post(genero::crear_genero_handler).layer(escritura.clone())),
        )
        .route(
            "/generos/{id}",
            put(genero::actualizar_genero_handler)
                .delete(genero::eliminar_genero_handler)
                .layer(escritura.clone()),
        )
        .route(
            "/motivos",
            get(motivo::listar_motivos_handler)
                .merge(post(motivo::crear_motivo_handler).layer(escritura.clone())),
        )
        .route(
            "/motivos/{id}",
            put(motivo::actualizar_motivo_handler)
                .delete(motivo::eliminar_motivo_handler)
                .layer(escritura.clone()),
        )
        .route(
            "/muebles",
            get(mueble::listar_muebles_handler)
                .merge(post(mueble::crear_mueble_handler).layer(escritura.clone())),
        )
        .route(
            "/muebles/{id}",
            put(mueble::actualizar_mueble_handler)
                .delete(mueble::eliminar_mueble_handler)
                .layer(escritura),
        )
        .route("/autores", get(autor::listar_autores_handler))
        .route("/autores/{id}", get(autor::obtener_autor_handler))
        .route(
            "/autores",
            post(autor::crear_autor_handler).layer(middleware::from_fn(require_bodega_o_gerente)),
        )
        .route(
            "/autores/{id}",
            put(autor::actualizar_autor_handler).layer(middleware::from_fn(require_bodega_o_gerente)),
        )
        .route(
            "/autores/{id}",
            delete(autor::eliminar_autor_handler).layer(middleware::from_fn(require_bodega_o_gerente)),
        )
        .route("/proveedores", get(proveedor::listar_proveedores_handler))
        .route("/proveedores/{id}", get(proveedor::obtener_proveedor_handler))
        .route(
            "/proveedores",
            post(proveedor::crear_proveedor_handler).layer(middleware::from_fn(require_bodega_o_gerente)),
        )
        .route(
            "/proveedores/{id}",
            put(proveedor::actualizar_proveedor_handler).layer(middleware::from_fn(require_bodega_o_gerente)),
        )
        .route(
            "/proveedores/{id}",
            delete(proveedor::eliminar_proveedor_handler).layer(middleware::from_fn(require_bodega_o_gerente)),
        )
        .layer(middleware::from_fn_with_state(
            state.config.clone(),
            auth_middleware,
        ));

    let bodega_routes = Router::new()
        .route("/libros", post(producto::registrar_libro_bodega))
        .route("/revistas", post(producto::registrar_revista_bodega))
        .route("/movimientos/libros", post(movimiento::movimiento_bodega_tienda_libros))
        .route(
            "/movimientos/revistas",
            post(movimiento::movimiento_bodega_tienda_revistas),
        )
        .route("/compras", post(compra::registrar_compra_bodega))
        .route("/compras", get(compra::listar_compras_bodega))
        .layer(middleware::from_fn(require_bodega))
        .layer(middleware::from_fn_with_state(
            state.config.clone(),
            auth_middleware,
        ));

    let vendedor_routes = Router::new()
        .route("/ventas", post(venta::registrar_venta_vendedor))
        .route("/existencias", get(inventario::consultar_existencias_vendedor))
        .route("/movimientos/libros", post(movimiento::movimiento_tienda_bodega_libros))
        .route(
            "/movimientos/revistas",
            post(movimiento::movimiento_tienda_bodega_revistas),
        )
        .route("/devoluciones", post(devolucion::crear_devolucion_vendedor))
        .route(
            "/devoluciones/{id}/pdf/libros",
            get(devolucion::generar_pdf_libros_vendedor),
        )
        .route(
            "/devoluciones/{id}/pdf/revistas",
            get(devolucion::generar_pdf_revistas_vendedor),
        )
        .layer(middleware::from_fn(require_vendedor))
        .layer(middleware::from_fn_with_state(
            state.config.clone(),
            auth_middleware,
        ));

    let jefe_routes = Router::new()
        .route(
            "/devoluciones/historial",
            get(devolucion::consultar_historial_devoluciones_jefe),
        )
        .route(
            "/movimientos/diarios",
            get(bitacora::consultar_movimiento_diario_jefe),
        )
        .route("/devoluciones/{id}/aprobar", put(devolucion::aprobar_devolucion_jefe))
        .layer(middleware::from_fn(require_jefe))
        .layer(middleware::from_fn_with_state(
            state.config.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/api", catalogos_routes)
        .nest("/api/bodega", bodega_routes)
        .nest("/api/vendedor", vendedor_routes)
        .nest("/api/jefe", jefe_routes)
}
