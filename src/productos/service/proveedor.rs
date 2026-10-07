use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{ActualizarProveedorRequest, CrearProveedorRequest, Proveedor},
        repo::ProveedorRepo,
    },
};

pub struct ProveedorService;

impl ProveedorService {
    pub async fn listar_proveedores(pool: &DbPool) -> Result<Vec<Proveedor>, AppError> {
        ProveedorRepo::listar_proveedores(pool).await
    }

    pub async fn obtener_proveedor(pool: &DbPool, id: i32) -> Result<Proveedor, AppError> {
        ProveedorRepo::buscar_proveedor(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Proveedor con ID {} no encontrado", id)))
    }

    pub async fn crear_proveedor(
        pool: &DbPool,
        actor_id: i64,
        req: CrearProveedorRequest,
    ) -> Result<Proveedor, AppError> {
        if req.nombre_proveedor.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El nombre o razón social del proveedor es obligatorio".to_string(),
            ));
        }
        if req.rfc.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El RFC o identificador fiscal es obligatorio".to_string(),
            ));
        }

        let proveedor = ProveedorRepo::crear_proveedor(pool, &req).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR PROVEEDOR",
            &format!(
                "Proveedor registrado: '{}' (ID: {}, RFC: {})",
                proveedor.nombre_proveedor, proveedor.id_proveedor, proveedor.rfc
            ),
        )
        .await;

        Ok(proveedor)
    }

    pub async fn actualizar_proveedor(
        pool: &DbPool,
        actor_id: i64,
        id: i32,
        req: ActualizarProveedorRequest,
    ) -> Result<Proveedor, AppError> {
        if req.nombre_proveedor.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El nombre o razón social del proveedor es obligatorio".to_string(),
            ));
        }
        if req.rfc.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El RFC o identificador fiscal es obligatorio".to_string(),
            ));
        }

        let proveedor = ProveedorRepo::actualizar_proveedor(pool, id, &req).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "ACTUALIZAR PROVEEDOR",
            &format!(
                "Proveedor actualizado: '{}' (ID: {})",
                proveedor.nombre_proveedor, id
            ),
        )
        .await;

        Ok(proveedor)
    }

    pub async fn eliminar_proveedor(pool: &DbPool, actor_id: i64, id: i32) -> Result<(), AppError> {
        ProveedorRepo::eliminar_proveedor(pool, id).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "ELIMINAR/INACTIVAR PROVEEDOR",
            &format!("Proveedor procesado para baja con ID: {}", id),
        )
        .await;

        Ok(())
    }
}
