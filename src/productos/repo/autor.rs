use sqlx::Row;

use crate::{
    db::{DbPool, row_get_i32, row_get_i64},
    error::AppError,
    productos::model::{ActualizarAutorRequest, Autor, CrearAutorRequest},
};

pub struct AutorRepo;

impl AutorRepo {
    pub async fn listar_autores(pool: &DbPool) -> Result<Vec<Autor>, AppError> {
        let rows = sqlx::query("SELECT id_autor, nombre, apellidos, nacionalidad, biografia FROM autores ORDER BY apellidos ASC, nombre ASC")
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(rows
            .into_iter()
            .map(|r| Autor {
                id_autor: row_get_i32(&r, "id_autor"),
                nombre: r.get("nombre"),
                apellidos: r.get("apellidos"),
                nacionalidad: r.get("nacionalidad"),
                biografia: r.get("biografia"),
            })
            .collect())
    }

    pub async fn buscar_autor(pool: &DbPool, id: i32) -> Result<Option<Autor>, AppError> {
        let row = sqlx::query("SELECT id_autor, nombre, apellidos, nacionalidad, biografia FROM autores WHERE id_autor = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(row.map(|r| Autor {
            id_autor: row_get_i32(&r, "id_autor"),
            nombre: r.get("nombre"),
            apellidos: r.get("apellidos"),
            nacionalidad: r.get("nacionalidad"),
            biografia: r.get("biografia"),
        }))
    }

    pub async fn crear_autor(pool: &DbPool, req: &CrearAutorRequest) -> Result<Autor, AppError> {
        let res = sqlx::query(
            "INSERT INTO autores (nombre, apellidos, nacionalidad, biografia) VALUES (?, ?, ?, ?)",
        )
        .bind(&req.nombre)
        .bind(&req.apellidos)
        .bind(&req.nacionalidad)
        .bind(&req.biografia)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        let id = res.last_insert_id() as i32;
        Ok(Autor {
            id_autor: id,
            nombre: req.nombre.clone(),
            apellidos: req.apellidos.clone(),
            nacionalidad: req.nacionalidad.clone(),
            biografia: req.biografia.clone(),
        })
    }

    pub async fn actualizar_autor(
        pool: &DbPool,
        id: i32,
        req: &ActualizarAutorRequest,
    ) -> Result<Autor, AppError> {
        let res = sqlx::query(
            "UPDATE autores SET nombre = ?, apellidos = ?, nacionalidad = ?, biografia = ? WHERE id_autor = ?"
        )
        .bind(&req.nombre)
        .bind(&req.apellidos)
        .bind(&req.nacionalidad)
        .bind(&req.biografia)
        .bind(id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Autor con ID {} no encontrado",
                id
            )));
        }

        Ok(Autor {
            id_autor: id,
            nombre: req.nombre.clone(),
            apellidos: req.apellidos.clone(),
            nacionalidad: req.nacionalidad.clone(),
            biografia: req.biografia.clone(),
        })
    }

    pub async fn eliminar_autor(pool: &DbPool, id: i32) -> Result<(), AppError> {
        let count_row = sqlx::query("SELECT COUNT(*) as cant FROM libro_autor WHERE id_autor = ?")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::Database)?;

        let cant: i64 = row_get_i64(&count_row, "cant");
        if cant > 0 {
            return Err(AppError::BadRequest(format!(
                "No es posible eliminar al autor (ID {}) porque tiene {} libro(s) asociado(s). Elimine o desvincule los libros primero.",
                id, cant
            )));
        }

        let res = sqlx::query("DELETE FROM autores WHERE id_autor = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Autor con ID {} no encontrado",
                id
            )));
        }

        Ok(())
    }

    pub async fn asociar_autores_libro(
        pool: &DbPool,
        codigo_ean: i64,
        autores: &[i32],
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM libro_autor WHERE codigo_ean = ?")
            .bind(codigo_ean)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        for &id_aut in autores {
            sqlx::query("INSERT IGNORE INTO libro_autor (codigo_ean, id_autor) VALUES (?, ?)")
                .bind(codigo_ean)
                .bind(id_aut)
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
        }
        Ok(())
    }
}
