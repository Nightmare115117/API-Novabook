use sqlx::Row;

use crate::{
    db::{DbPool, row_get_i32, row_get_i64},
    error::AppError,
    productos::model::{ActualizarProveedorRequest, CrearProveedorRequest, Proveedor},
};

pub struct ProveedorRepo;

impl ProveedorRepo {
    pub async fn listar_proveedores(pool: &DbPool) -> Result<Vec<Proveedor>, AppError> {
        let rows = sqlx::query(
            "SELECT id_proveedor, nombre_proveedor, rfc, telefono, correo, direccion, persona_contacto, estatus FROM proveedores ORDER BY nombre_proveedor ASC"
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(rows
            .into_iter()
            .map(|r| Proveedor {
                id_proveedor: row_get_i32(&r, "id_proveedor"),
                nombre_proveedor: r.get("nombre_proveedor"),
                rfc: r.get("rfc"),
                telefono: r.get("telefono"),
                correo: r.get("correo"),
                direccion: r.get("direccion"),
                persona_contacto: r.get("persona_contacto"),
                estatus: r.get("estatus"),
            })
            .collect())
    }

    pub async fn buscar_proveedor(pool: &DbPool, id: i32) -> Result<Option<Proveedor>, AppError> {
        let row = sqlx::query(
            "SELECT id_proveedor, nombre_proveedor, rfc, telefono, correo, direccion, persona_contacto, estatus FROM proveedores WHERE id_proveedor = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row.map(|r| Proveedor {
            id_proveedor: row_get_i32(&r, "id_proveedor"),
            nombre_proveedor: r.get("nombre_proveedor"),
            rfc: r.get("rfc"),
            telefono: r.get("telefono"),
            correo: r.get("correo"),
            direccion: r.get("direccion"),
            persona_contacto: r.get("persona_contacto"),
            estatus: r.get("estatus"),
        }))
    }

    pub async fn crear_proveedor(
        pool: &DbPool,
        req: &CrearProveedorRequest,
    ) -> Result<Proveedor, AppError> {
        let res = sqlx::query(
            r#"
            INSERT INTO proveedores (nombre_proveedor, rfc, telefono, correo, direccion, persona_contacto, estatus)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&req.nombre_proveedor)
        .bind(&req.rfc)
        .bind(&req.telefono)
        .bind(&req.correo)
        .bind(&req.direccion)
        .bind(&req.persona_contacto)
        .bind(&req.estatus)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        let id = res.last_insert_id() as i32;
        Ok(Proveedor {
            id_proveedor: id,
            nombre_proveedor: req.nombre_proveedor.clone(),
            rfc: req.rfc.clone(),
            telefono: req.telefono.clone(),
            correo: req.correo.clone(),
            direccion: req.direccion.clone(),
            persona_contacto: req.persona_contacto.clone(),
            estatus: req.estatus.clone(),
        })
    }

    pub async fn actualizar_proveedor(
        pool: &DbPool,
        id: i32,
        req: &ActualizarProveedorRequest,
    ) -> Result<Proveedor, AppError> {
        let res = sqlx::query(
            r#"
            UPDATE proveedores
            SET nombre_proveedor = ?, rfc = ?, telefono = ?, correo = ?, direccion = ?, persona_contacto = ?, estatus = ?
            WHERE id_proveedor = ?
            "#,
        )
        .bind(&req.nombre_proveedor)
        .bind(&req.rfc)
        .bind(&req.telefono)
        .bind(&req.correo)
        .bind(&req.direccion)
        .bind(&req.persona_contacto)
        .bind(&req.estatus)
        .bind(id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Proveedor con ID {} no encontrado",
                id
            )));
        }

        Ok(Proveedor {
            id_proveedor: id,
            nombre_proveedor: req.nombre_proveedor.clone(),
            rfc: req.rfc.clone(),
            telefono: req.telefono.clone(),
            correo: req.correo.clone(),
            direccion: req.direccion.clone(),
            persona_contacto: req.persona_contacto.clone(),
            estatus: req.estatus.clone(),
        })
    }

    pub async fn eliminar_proveedor(pool: &DbPool, id: i32) -> Result<(), AppError> {
        let row_ref = sqlx::query(
            r#"
            SELECT
                (SELECT COUNT(*) FROM libros WHERE id_proveedor = ?) +
                (SELECT COUNT(*) FROM revistas WHERE id_proveedor = ?) +
                (SELECT COUNT(*) FROM devolucion WHERE id_proveedor = ?) +
                (SELECT COUNT(*) FROM compras_proveedor WHERE id_proveedor = ?) as total_ref
            "#,
        )
        .bind(id)
        .bind(id)
        .bind(id)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        let total_ref: i64 = row_get_i64(&row_ref, "total_ref");
        if total_ref > 0 {
            sqlx::query("UPDATE proveedores SET estatus = 'INACTIVO' WHERE id_proveedor = ?")
                .bind(id)
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
            return Ok(());
        }

        let res = sqlx::query("DELETE FROM proveedores WHERE id_proveedor = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Proveedor con ID {} no encontrado",
                id
            )));
        }

        Ok(())
    }
}
