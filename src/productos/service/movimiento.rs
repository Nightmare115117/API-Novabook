use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{TipoMovimiento, TipoProducto, TrasladoRequest, TrasladoResponse},
        repo::{MovimientoRepo, UBICACION_BODEGA, UBICACION_TIENDA},
    },
};

pub struct MovimientoService;

impl MovimientoService {
    pub async fn traslado_bodega_a_tienda_libros(
        pool: &DbPool,
        actor_id: i64,
        req: TrasladoRequest,
    ) -> Result<TrasladoResponse, AppError> {
        if req.codigo_ean <= 0 {
            return Err(AppError::BadRequest("Código EAN inválido".to_string()));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest(
                "La cantidad a trasladar debe ser mayor a 0".to_string(),
            ));
        }

        let (restante_bodega, nuevo_tienda) = MovimientoRepo::trasladar_stock_libro(
            pool,
            req.codigo_ean,
            req.cantidad,
            UBICACION_BODEGA,
            UBICACION_TIENDA,
            actor_id,
            req.observaciones.as_deref(),
        )
        .await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "TRASLADO BODEGA A TIENDA (LIBROS)",
            &format!(
                "EAN: {}, Cantidad: {}. Bodega restan: {}, Tienda: {}",
                req.codigo_ean, req.cantidad, restante_bodega, nuevo_tienda
            ),
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
            return Err(AppError::BadRequest(
                "Código EAN / ISSN inválido".to_string(),
            ));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest(
                "La cantidad a trasladar debe ser mayor a 0".to_string(),
            ));
        }

        let (restante_bodega, nuevo_tienda) = MovimientoRepo::trasladar_stock_revista(
            pool,
            req.codigo_ean,
            req.cantidad,
            UBICACION_BODEGA,
            UBICACION_TIENDA,
            actor_id,
            req.observaciones.as_deref(),
        )
        .await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "TRASLADO BODEGA A TIENDA (REVISTAS)",
            &format!(
                "EAN: {}, Cantidad trasladada: {}. Bodega restan: {}, Tienda: {}",
                req.codigo_ean, req.cantidad, restante_bodega, nuevo_tienda
            ),
        )
        .await;

        Ok(TrasladoResponse {
            codigo_ean: req.codigo_ean,
            tipo_producto: TipoProducto::Revista,
            tipo_movimiento: TipoMovimiento::BodegaATienda,
            cantidad_trasladada: req.cantidad,
            stock_origen_restante: restante_bodega,
            stock_destino_nuevo: nuevo_tienda,
            mensaje: "Traslado de revistas de bodega a tienda completado exitosamente".to_string(),
        })
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
            return Err(AppError::BadRequest(
                "La cantidad a trasladar debe ser mayor a 0".to_string(),
            ));
        }

        let (restante_tienda, nuevo_bodega) = MovimientoRepo::trasladar_stock_libro(
            pool,
            req.codigo_ean,
            req.cantidad,
            UBICACION_TIENDA,
            UBICACION_BODEGA,
            actor_id,
            req.observaciones.as_deref(),
        )
        .await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "TRASLADO TIENDA A BODEGA (LIBROS)",
            &format!(
                "EAN: {}, Cantidad: {}. Tienda restan: {}, Bodega: {}",
                req.codigo_ean, req.cantidad, restante_tienda, nuevo_bodega
            ),
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
            return Err(AppError::BadRequest(
                "Código EAN / ISSN inválido".to_string(),
            ));
        }
        if req.cantidad <= 0 {
            return Err(AppError::BadRequest(
                "La cantidad a trasladar debe ser mayor a 0".to_string(),
            ));
        }

        let (restante_tienda, nuevo_bodega) = MovimientoRepo::trasladar_stock_revista(
            pool,
            req.codigo_ean,
            req.cantidad,
            UBICACION_TIENDA,
            UBICACION_BODEGA,
            actor_id,
            req.observaciones.as_deref(),
        )
        .await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "TRASLADO TIENDA A BODEGA (REVISTAS)",
            &format!(
                "EAN: {}, Cantidad trasladada: {}. Tienda restan: {}, Bodega: {}",
                req.codigo_ean, req.cantidad, restante_tienda, nuevo_bodega
            ),
        )
        .await;

        Ok(TrasladoResponse {
            codigo_ean: req.codigo_ean,
            tipo_producto: TipoProducto::Revista,
            tipo_movimiento: TipoMovimiento::TiendaABodega,
            cantidad_trasladada: req.cantidad,
            stock_origen_restante: restante_tienda,
            stock_destino_nuevo: nuevo_bodega,
            mensaje: "Traslado de revistas de tienda a bodega completado exitosamente".to_string(),
        })
    }
}
