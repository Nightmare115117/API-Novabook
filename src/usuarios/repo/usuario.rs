use crate::{
    db::{DbPool, row_get_i32, row_get_i64, row_get_opt_i64},
    error::AppError,
    usuarios::model::{ActualizarUsuarioRequest, CrearUsuarioRequest, Usuario},
};
use sqlx::Row;

pub struct UsuarioRepo;

impl UsuarioRepo {
    pub async fn buscar_por_id(pool: &DbPool, id: i64) -> Result<Option<Usuario>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id_usuarios, id_roles, nombre, apellido_paterno, apellido_materno, telefono, contrasena
            FROM usuarios
            WHERE id_usuarios = ?
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row.map(|r| Usuario {
            id_usuarios: row_get_i64(&r, "id_usuarios"),
            id_roles: row_get_i32(&r, "id_roles"),
            nombre: r.get("nombre"),
            apellido_paterno: r.get("apellido_paterno"),
            apellido_materno: r.get("apellido_materno"),
            telefono: row_get_opt_i64(&r, "telefono"),
            contrasena: r.get("contrasena"),
        }))
    }

    pub async fn listar_todos(pool: &DbPool) -> Result<Vec<Usuario>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id_usuarios, id_roles, nombre, apellido_paterno, apellido_materno, telefono, contrasena
            FROM usuarios
            ORDER BY id_usuarios ASC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

        let usuarios = rows
            .into_iter()
            .map(|r| Usuario {
                id_usuarios: row_get_i64(&r, "id_usuarios"),
                id_roles: row_get_i32(&r, "id_roles"),
                nombre: r.get("nombre"),
                apellido_paterno: r.get("apellido_paterno"),
                apellido_materno: r.get("apellido_materno"),
                telefono: row_get_opt_i64(&r, "telefono"),
                contrasena: r.get("contrasena"),
            })
            .collect();

        Ok(usuarios)
    }

    pub async fn existe(pool: &DbPool, id: i64) -> Result<bool, AppError> {
        let row = sqlx::query("SELECT id_usuarios FROM usuarios WHERE id_usuarios = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(row.is_some())
    }

    pub async fn crear(pool: &DbPool, req: &CrearUsuarioRequest) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO usuarios (id_usuarios, id_roles, nombre, apellido_paterno, apellido_materno, telefono, contrasena)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(req.id_usuarios)
        .bind(req.id_roles)
        .bind(&req.nombre)
        .bind(&req.apellido_paterno)
        .bind(&req.apellido_materno)
        .bind(req.telefono)
        .bind(&req.contrasena)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn actualizar(
        pool: &DbPool,
        id: i64,
        actual: &Usuario,
        req: &ActualizarUsuarioRequest,
    ) -> Result<Usuario, AppError> {
        let nuevo_rol = req.id_roles.unwrap_or(actual.id_roles);
        let nuevo_nombre = req.nombre.clone().unwrap_or_else(|| actual.nombre.clone());
        let nuevo_paterno = req
            .apellido_paterno
            .clone()
            .or_else(|| actual.apellido_paterno.clone());
        let nuevo_materno = req
            .apellido_materno
            .clone()
            .or_else(|| actual.apellido_materno.clone());
        let nuevo_tel = req.telefono.or(actual.telefono);
        let nueva_pass = req
            .contrasena
            .clone()
            .unwrap_or_else(|| actual.contrasena.clone());

        sqlx::query(
            r#"
            UPDATE usuarios
            SET id_roles = ?, nombre = ?, apellido_paterno = ?, apellido_materno = ?, telefono = ?, contrasena = ?
            WHERE id_usuarios = ?
            "#,
        )
        .bind(nuevo_rol)
        .bind(&nuevo_nombre)
        .bind(&nuevo_paterno)
        .bind(&nuevo_materno)
        .bind(nuevo_tel)
        .bind(&nueva_pass)
        .bind(id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(Usuario {
            id_usuarios: id,
            id_roles: nuevo_rol,
            nombre: nuevo_nombre,
            apellido_paterno: nuevo_paterno,
            apellido_materno: nuevo_materno,
            telefono: nuevo_tel,
            contrasena: nueva_pass,
        })
    }

    pub async fn eliminar(pool: &DbPool, id: i64) -> Result<bool, AppError> {
        let res = sqlx::query("DELETE FROM usuarios WHERE id_usuarios = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(res.rows_affected() > 0)
    }
}
