use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{
            AprobarDevolucionRequest, CrearDevolucionRequest, Devolucion, HistorialDevolucionQuery,
            TipoProducto,
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

        for item in &req.items {
            if item.codigo_ean <= 0 || item.cantidad <= 0 || item.id_motivo <= 0 {
                return Err(AppError::BadRequest(
                    "Cada artículo debe tener EAN, cantidad e id_motivo válidos".to_string(),
                ));
            }
        }

        let id_devolucion = DevolucionRepo::registrar_devolucion(
            pool,
            actor_id,
            req.id_proveedor,
            req.tipo_producto,
            &req.items,
        )
        .await?;
        let devolucion = DevolucionRepo::buscar_devolucion(pool, id_devolucion)
            .await?
            .ok_or_else(|| {
                AppError::Internal(format!(
                    "Devolución #{} no encontrada después de crearla",
                    id_devolucion
                ))
            })?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR DEVOLUCIÓN",
            &format!(
                "Devolución #{} creada. Proveedor ID: {}, Piezas: {}",
                id_devolucion, req.id_proveedor, devolucion.total_piezas
            ),
        )
        .await;

        Ok(Devolucion {
            vendedor_nombre: Some(actor_nombre.to_string()),
            ..devolucion
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
            "APROBADO"
        } else {
            "RECHAZADO"
        };
        if !actual.estado.eq_ignore_ascii_case("PENDIENTE") {
            return Err(AppError::Conflict(format!(
                "La devolución #{} ya fue evaluada (estado: {})",
                id_devolucion, actual.estado
            )));
        }
        DevolucionRepo::actualizar_estado_devolucion(
            pool,
            id_devolucion,
            actor_id,
            nuevo_estado,
            req.aprobar,
        )
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
