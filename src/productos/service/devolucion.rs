use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{
            AprobarDevolucionRequest, CrearDevolucionRequest, Devolucion, HistorialDevolucionQuery,
            ItemDevolucion, TipoProducto,
        },
        repo::DevolucionRepo,
    },
    services::PdfService,
};

pub struct DevolucionService;

impl DevolucionService {
    pub async fn crear_devolucion(
        pool: &DbPool,
        actor_id: i64,
        actor_nombre: &str,
        req: CrearDevolucionRequest,
    ) -> Result<Devolucion, AppError> {
        if req.items.is_empty() {
            return Err(AppError::BadRequest(
                "Debe especificar al menos un artículo para devolución".to_string(),
            ));
        }

        let total_piezas: i32 = req.items.iter().map(|i| i.cantidad).sum();
        let id_devolucion = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(1);

        DevolucionRepo::registrar_devolucion(
            pool,
            id_devolucion,
            total_piezas,
            actor_id,
            req.id_proveedor,
        )
        .await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR DEVOLUCIÓN",
            &format!(
                "Devolución #{} creada. Proveedor ID: {}, Piezas: {}",
                id_devolucion, req.id_proveedor, total_piezas
            ),
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
            proveedor_rfc: None,
            proveedor_telefono: None,
            proveedor_correo: None,
            proveedor_direccion: None,
            proveedor_contacto: None,
            tipo_producto: req.tipo_producto,
            items,
        })
    }

    pub async fn generar_pdf_devolucion(
        pool: &DbPool,
        id_devolucion: i64,
        tipo: TipoProducto,
    ) -> Result<(Vec<u8>, String), AppError> {
        let devolucion = DevolucionRepo::buscar_devolucion(pool, id_devolucion)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Devolución #{} no encontrada", id_devolucion))
            })?;

        let pdf_bytes = PdfService::generar_pdf_devolucion(&devolucion, tipo);
        let filename = format!(
            "devolucion_{}_{}.pdf",
            id_devolucion,
            tipo.as_str().to_lowercase()
        );

        Ok((pdf_bytes, filename))
    }

    pub async fn consultar_historial_devoluciones(
        pool: &DbPool,
        params: HistorialDevolucionQuery,
    ) -> Result<Vec<Devolucion>, AppError> {
        DevolucionRepo::listar_historial_devoluciones(pool, &params).await
    }

    pub async fn aprobar_devolucion(
        pool: &DbPool,
        actor_id: i64,
        actor_nombre: &str,
        id_devolucion: i64,
        req: AprobarDevolucionRequest,
    ) -> Result<Devolucion, AppError> {
        let actual = DevolucionRepo::buscar_devolucion(pool, id_devolucion)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Devolución #{} no encontrada", id_devolucion))
            })?;

        let nuevo_estado = if req.aprobar {
            "Autorizado"
        } else {
            "Rechazado"
        };
        DevolucionRepo::actualizar_estado_devolucion(pool, id_devolucion, nuevo_estado, actor_nombre)
            .await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "APROBAR DEVOLUCIÓN",
            &format!(
                "Devolución #{} actualizada a estado '{}'",
                id_devolucion, nuevo_estado
            ),
        )
        .await;

        Ok(Devolucion {
            estado: nuevo_estado.to_string(),
            autorizado_por: Some(actor_nombre.to_string()),
            ..actual
        })
    }
}
