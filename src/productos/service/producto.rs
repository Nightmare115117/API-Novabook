use crate::{
    db::{DbPool, log_bitacora},
    error::AppError,
    productos::{
        model::{
            Libro, RegistrarLibroRequest, RegistrarRevistaRequest, Revista, TipoProducto,
            validar_patron_ean13,
        },
        repo::{ProductoRepo, UBICACION_BODEGA},
    },
};

pub struct ProductoService;

impl ProductoService {
    // ========================================================================
    // Registro de Libros y Revistas con Validación EAN (Bodega)
    // ========================================================================

    pub async fn registrar_libro(
        pool: &DbPool,
        actor_id: i64,
        req: RegistrarLibroRequest,
    ) -> Result<Libro, AppError> {
        // Validación obligatoria de patrón EAN-13 y dígito verificador
        let tipo_detectado =
            validar_patron_ean13(req.codigo_ean).map_err(AppError::BadRequest)?;

        if tipo_detectado != TipoProducto::Libro {
            return Err(AppError::BadRequest(format!(
                "El código EAN {} corresponde a una Revista (ISSN: prefijo 977), no a un Libro",
                req.codigo_ean
            )));
        }

        // Evitar duplicados cruzados en revistas
        if (ProductoRepo::buscar_revista(pool, req.codigo_ean, UBICACION_BODEGA).await?).is_some() {
            return Err(AppError::BadRequest(format!(
                "El código EAN {} ya está registrado en el sistema como una Revista",
                req.codigo_ean
            )));
        }

        if req.cantidad <= 0 {
            return Err(AppError::BadRequest(
                "La cantidad debe ser mayor a 0".to_string(),
            ));
        }
        if req.precio < 0.0 {
            return Err(AppError::BadRequest(
                "El precio no puede ser negativo".to_string(),
            ));
        }

        let ubicacion = req.id_ubicacion.unwrap_or(UBICACION_BODEGA);
        let libro = ProductoRepo::guardar_o_actualizar_libro(pool, &req, ubicacion).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "REGISTRO LIBRO BODEGA",
            &format!(
                "EAN: {}, Título: '{}', Cant: {}",
                req.codigo_ean, req.nombre_libro, req.cantidad
            ),
        )
        .await;

        Ok(libro)
    }

    pub async fn registrar_revista(
        pool: &DbPool,
        actor_id: i64,
        req: RegistrarRevistaRequest,
    ) -> Result<Revista, AppError> {
        // Validación obligatoria de patrón EAN-13 y dígito verificador
        let tipo_detectado =
            validar_patron_ean13(req.codigo_ean).map_err(AppError::BadRequest)?;

        if tipo_detectado != TipoProducto::Revista {
            return Err(AppError::BadRequest(format!(
                "El código EAN {} corresponde a un Libro (ISBN: prefijo 978/979), no a una Revista",
                req.codigo_ean
            )));
        }

        // Evitar duplicados cruzados en libros
        if (ProductoRepo::buscar_libro(pool, req.codigo_ean, UBICACION_BODEGA).await?).is_some() {
            return Err(AppError::BadRequest(format!(
                "El código EAN {} ya está registrado en el sistema como un Libro",
                req.codigo_ean
            )));
        }

        if req.cantidad <= 0 {
            return Err(AppError::BadRequest(
                "La cantidad debe ser mayor a 0".to_string(),
            ));
        }
        if req.precio < 0.0 {
            return Err(AppError::BadRequest(
                "El precio no puede ser negativo".to_string(),
            ));
        }

        let ubicacion = req.id_ubicacion.unwrap_or(UBICACION_BODEGA);
        let revista = ProductoRepo::guardar_o_actualizar_revista(pool, &req, ubicacion).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "REGISTRO REVISTA BODEGA",
            &format!(
                "EAN: {}, Título: '{}', Edición: {:?}, Cant: {}",
                req.codigo_ean, req.nombre_revista, req.numero_edicion, req.cantidad
            ),
        )
        .await;

        Ok(revista)
    }
}
