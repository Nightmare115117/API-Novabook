use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    db::{log_bitacora, DbPool},
    error::AppError,
    productos::{
        model::{
            AprobarDevolucionRequest, CrearDevolucionRequest, Devolucion, ExistenciaInventario,
            FiltroExistencias, HistorialDevolucionQuery, ItemDevolucion, ItemVenta, Libro,
            RegistrarLibroRequest, RegistrarRevistaRequest, RegistrarVentaRequest,
            ResumenMovimientoDiario, Revista, TipoMovimiento, TipoProducto, TrasladoRequest,
            TrasladoResponse, VentaResponse,
        },
        repo::{ProductoRepo, UBICACION_BODEGA, UBICACION_TIENDA},
    },
    services::PdfService,
};

pub struct ProductoService;

impl ProductoService {
    // ========================================================================
    // 3) Personal de Bodega
    // ========================================================================

    pub async fn registrar_libro(
        pool: &DbPool,
        actor_id: i64,
        req: RegistrarLibroRequest,
    ) -> Result<Libro, AppError> {
        if req.codigo_ean <= 0 {
            return Err(AppError::BadRequest("Código EAN inválido".to_string()));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest("La cantidad debe ser mayor a 0".to_string()));
        }
        if req.precio < 0.0 {
            return Err(AppError::BadRequest("El precio no puede ser negativo".to_string()));
        }

        let ubicacion = req.id_ubicacion.unwrap_or(UBICACION_BODEGA);
        let libro = ProductoRepo::guardar_o_actualizar_libro(pool, &req, ubicacion).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "REGISTRO LIBRO BODEGA",
            &format!("EAN: {}, Título: '{}', Cant: {}", req.codigo_ean, req.nombre_libro, req.cantidad),
        )
        .await;

        Ok(libro)
    }

    pub async fn registrar_revista(
        pool: &DbPool,
        actor_id: i64,
        req: RegistrarRevistaRequest,
    ) -> Result<Revista, AppError> {
        if req.codigo_ean <= 0 {
            return Err(AppError::BadRequest("Código EAN / ISSN inválido".to_string()));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest("La cantidad debe ser mayor a 0".to_string()));
        }

        let ubicacion = req.id_ubicacion.unwrap_or(UBICACION_BODEGA);

        let _ = log_bitacora(
            pool,
            actor_id,
            "REGISTRO REVISTA BODEGA",
            &format!("EAN: {}, Título: '{}', Edición: {:?}, Cant: {}", req.codigo_ean, req.nombre_revista, req.numero_edicion, req.cantidad),
        )
        .await;

        Ok(Revista {
            codigo_ean: req.codigo_ean,
            id_mueble: req.id_mueble,
            id_proveedor: req.id_proveedor,
            id_ubicacion: ubicacion,
            nombre_revista: req.nombre_revista,
            numero_edicion: req.numero_edicion,
            periodicidad: req.periodicidad,
            precio: req.precio,
            cantidad: req.cantidad,
            sku: req.sku,
        })
    }

    pub async fn traslado_bodega_a_tienda_libros(
        pool: &DbPool,
        actor_id: i64,
        req: TrasladoRequest,
    ) -> Result<TrasladoResponse, AppError> {
        if req.codigo_ean <= 0 {
            return Err(AppError::BadRequest("Código EAN inválido".to_string()));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest("La cantidad a trasladar debe ser mayor a 0".to_string()));
        }

        let (restante_bodega, nuevo_tienda) = ProductoRepo::trasladar_stock_libro(
            pool,
            req.codigo_ean,
            req.cantidad,
            UBICACION_BODEGA,
            UBICACION_TIENDA,
        )
        .await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "TRASLADO BODEGA A TIENDA (LIBROS)",
            &format!("EAN: {}, Cantidad: {}. Bodega restan: {}, Tienda: {}", req.codigo_ean, req.cantidad, restante_bodega, nuevo_tienda),
        )
        .await;

        Ok(TrasladoResponse {
            codigo_ean: req.codigo_ean,
            tipo_producto: TipoProducto::Libro,
            tipo_movimiento: TipoMovimiento::BodegaATienda,
            cantidad_trasladada: req.cantidad,
            stock_origen_restante: restante_bodega,
            stock_destino_nuevo: nuevo_tienda,
            mensaje: "Traslado de libros de bodega a tienda completado exitosamente".to_string(),
        })
    }

    pub async fn traslado_bodega_a_tienda_revistas(
        pool: &DbPool,
        actor_id: i64,
        req: TrasladoRequest,
    ) -> Result<TrasladoResponse, AppError> {
        if req.codigo_ean <= 0 {
            return Err(AppError::BadRequest("Código EAN / ISSN inválido".to_string()));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest("La cantidad a trasladar debe ser mayor a 0".to_string()));
        }

        let _ = log_bitacora(
            pool,
            actor_id,
            "TRASLADO BODEGA A TIENDA (REVISTAS)",
            &format!("EAN: {}, Cantidad trasladada: {}", req.codigo_ean, req.cantidad),
        )
        .await;

        Ok(TrasladoResponse {
            codigo_ean: req.codigo_ean,
            tipo_producto: TipoProducto::Revista,
            tipo_movimiento: TipoMovimiento::BodegaATienda,
            cantidad_trasladada: req.cantidad,
            stock_origen_restante: 0,
            stock_destino_nuevo: req.cantidad,
            mensaje: "Traslado de revistas de bodega a tienda completado exitosamente".to_string(),
        })
    }

    // ========================================================================
    // 4) Vendedor
    // ========================================================================

    pub async fn registrar_venta(
        pool: &DbPool,
        actor_id: i64,
        req: RegistrarVentaRequest,
    ) -> Result<VentaResponse, AppError> {
        if req.items.is_empty() {
            return Err(AppError::BadRequest("El carrito de venta no puede estar vacío".to_string()));
        }

        let mut total_venta = 0.0;
        let mut total_articulos = 0;
        let mut items_procesados = Vec::new();

        for item in &req.items {
            if item.cantidad <= 0 {
                return Err(AppError::BadRequest("La cantidad de venta debe ser mayor a 0".to_string()));
            }

            let (nombre, precio) = ProductoRepo::descontar_stock_venta(pool, item.codigo_ean, item.cantidad).await?;
            let subtotal = precio * (item.cantidad as f64);
            total_venta += subtotal;
            total_articulos += item.cantidad;

            items_procesados.push(ItemVenta {
                codigo_ean: item.codigo_ean,
                tipo_producto: item.tipo_producto,
                nombre_producto: nombre,
                cantidad: item.cantidad,
                precio_unitario: precio,
                subtotal,
            });
        }

        let venta_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(1);

        let _ = log_bitacora(
            pool,
            actor_id,
            "BAJA POR VENTA",
            &format!("Venta #{} completada. Piezas: {}, Total: ${:.2}", venta_id, total_articulos, total_venta),
        )
        .await;

        Ok(VentaResponse {
            id_venta: venta_id,
            id_vendedor: actor_id,
            fecha_hora: "Hoy".to_string(),
            total: total_venta,
            total_articulos,
            items: items_procesados,
            mensaje: "Venta registrada exitosamente".to_string(),
        })
    }

    pub async fn consultar_existencias(
        pool: &DbPool,
        params: FiltroExistencias,
    ) -> Result<Vec<ExistenciaInventario>, AppError> {
        ProductoRepo::consultar_existencias(pool, &params).await
    }

    pub async fn traslado_tienda_a_bodega_libros(
        pool: &DbPool,
        actor_id: i64,
        req: TrasladoRequest,
    ) -> Result<TrasladoResponse, AppError> {
        if req.codigo_ean <= 0 {
            return Err(AppError::BadRequest("Código EAN inválido".to_string()));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest("La cantidad a trasladar debe ser mayor a 0".to_string()));
        }

        let (restante_tienda, nuevo_bodega) = ProductoRepo::trasladar_stock_libro(
            pool,
            req.codigo_ean,
            req.cantidad,
            UBICACION_TIENDA,
            UBICACION_BODEGA,
        )
        .await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "TRASLADO TIENDA A BODEGA (LIBROS)",
            &format!("EAN: {}, Cantidad: {}. Tienda restan: {}, Bodega: {}", req.codigo_ean, req.cantidad, restante_tienda, nuevo_bodega),
        )
        .await;

        Ok(TrasladoResponse {
            codigo_ean: req.codigo_ean,
            tipo_producto: TipoProducto::Libro,
            tipo_movimiento: TipoMovimiento::TiendaABodega,
            cantidad_trasladada: req.cantidad,
            stock_origen_restante: restante_tienda,
            stock_destino_nuevo: nuevo_bodega,
            mensaje: "Traslado de libros de tienda a bodega completado exitosamente".to_string(),
        })
    }

    pub async fn traslado_tienda_a_bodega_revistas(
        pool: &DbPool,
        actor_id: i64,
        req: TrasladoRequest,
    ) -> Result<TrasladoResponse, AppError> {
        if req.codigo_ean <= 0 {
            return Err(AppError::BadRequest("Código EAN / ISSN inválido".to_string()));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest("La cantidad a trasladar debe ser mayor a 0".to_string()));
        }

        let _ = log_bitacora(
            pool,
            actor_id,
            "TRASLADO TIENDA A BODEGA (REVISTAS)",
            &format!("EAN: {}, Cantidad trasladada: {}", req.codigo_ean, req.cantidad),
        )
        .await;

        Ok(TrasladoResponse {
            codigo_ean: req.codigo_ean,
            tipo_producto: TipoProducto::Revista,
            tipo_movimiento: TipoMovimiento::TiendaABodega,
            cantidad_trasladada: req.cantidad,
            stock_origen_restante: 0,
            stock_destino_nuevo: req.cantidad,
            mensaje: "Traslado de revistas de tienda a bodega completado exitosamente".to_string(),
        })
    }

    pub async fn crear_devolucion(
        pool: &DbPool,
        actor_id: i64,
        actor_nombre: &str,
        req: CrearDevolucionRequest,
    ) -> Result<Devolucion, AppError> {
        if req.items.is_empty() {
            return Err(AppError::BadRequest("Debe especificar al menos un artículo para devolución".to_string()));
        }

        let total_piezas: i32 = req.items.iter().map(|i| i.cantidad).sum();
        let id_devolucion = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(1);

        ProductoRepo::registrar_devolucion(pool, id_devolucion, total_piezas, actor_id, req.id_proveedor).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR DEVOLUCIÓN",
            &format!("Devolución #{} creada. Proveedor ID: {}, Piezas: {}", id_devolucion, req.id_proveedor, total_piezas),
        )
        .await;

        let items: Vec<ItemDevolucion> = req
            .items
            .into_iter()
            .map(|i| ItemDevolucion {
                codigo_ean: i.codigo_ean,
                sku: None,
                titulo: format!("Artículo {}", i.codigo_ean),
                cantidad: i.cantidad,
                motivo: i.motivo,
            })
            .collect();

        Ok(Devolucion {
            id_devolucion,
            fecha: "Hoy".to_string(),
            total_piezas,
            estado: "Pendiente".to_string(),
            autorizado_por: None,
            id_usuarios: actor_id,
            vendedor_nombre: Some(actor_nombre.to_string()),
            id_proveedor: req.id_proveedor,
            nombre_proveedor: None,
            tipo_producto: req.tipo_producto,
            items,
        })
    }

    pub async fn generar_pdf_devolucion(
        pool: &DbPool,
        id_devolucion: i64,
        tipo: TipoProducto,
    ) -> Result<(Vec<u8>, String), AppError> {
        let devolucion = ProductoRepo::buscar_devolucion(pool, id_devolucion)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Devolución #{} no encontrada", id_devolucion)))?;

        let pdf_bytes = PdfService::generar_pdf_devolucion(&devolucion, tipo);
        let filename = format!("devolucion_{}_{}.pdf", id_devolucion, tipo.as_str().to_lowercase());

        Ok((pdf_bytes, filename))
    }

    // ========================================================================
    // 2) Jefe de Departamento
    // ========================================================================

    pub async fn consultar_historial_devoluciones(
        pool: &DbPool,
        params: HistorialDevolucionQuery,
    ) -> Result<Vec<Devolucion>, AppError> {
        ProductoRepo::listar_historial_devoluciones(pool, &params).await
    }

    pub async fn consultar_movimiento_diario(
        pool: &DbPool,
        fecha: Option<String>,
    ) -> Result<ResumenMovimientoDiario, AppError> {
        let movimientos = ProductoRepo::consultar_movimiento_diario(pool, fecha.as_deref()).await?;
        let total = movimientos.len();
        let fecha_str = fecha.unwrap_or_else(|| "Hoy".to_string());

        Ok(ResumenMovimientoDiario {
            fecha: fecha_str,
            total_eventos: total,
            movimientos,
        })
    }

    pub async fn aprobar_devolucion(
        pool: &DbPool,
        actor_id: i64,
        actor_nombre: &str,
        id_devolucion: i64,
        req: AprobarDevolucionRequest,
    ) -> Result<Devolucion, AppError> {
        let actual = ProductoRepo::buscar_devolucion(pool, id_devolucion)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Devolución #{} no encontrada", id_devolucion)))?;

        let nuevo_estado = if req.aprobar { "Autorizado" } else { "Rechazado" };
        ProductoRepo::actualizar_estado_devolucion(pool, id_devolucion, nuevo_estado, actor_nombre).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "APROBAR DEVOLUCIÓN",
            &format!("Devolución #{} actualizada a estado '{}'", id_devolucion, nuevo_estado),
        )
        .await;

        Ok(Devolucion {
            estado: nuevo_estado.to_string(),
            autorizado_por: Some(actor_nombre.to_string()),
            ..actual
        })
    }
}
