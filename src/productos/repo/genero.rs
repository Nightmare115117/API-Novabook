use sqlx::Row;

use crate::{
    db::{DbPool, row_get_i32, row_get_i64},
    error::AppError,
    productos::model::{CrearGeneroRequest, Genero},
};

pub struct GeneroRepo;

impl GeneroRepo {
    pub async fn listar_generos(pool: &DbPool) -> Result<Vec<Genero>, AppError> {
        let rows = sqlx::query(
            "SELECT id_genero, genero_literario FROM genero ORDER BY genero_literario ASC",
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(rows
            .into_iter()
            .map(|r| Genero {
                id_genero: row_get_i32(&r, "id_genero"),
                genero_literario: r.get("genero_literario"),
            })
            .collect())
    }

    pub async fn crear_genero(pool: &DbPool, req: &CrearGeneroRequest) -> Result<Genero, AppError> {
        let res = sqlx::query("INSERT INTO genero (genero_literario) VALUES (?)")
            .bind(&req.genero_literario)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(Genero {
            id_genero: res.last_insert_id() as i32,
            genero_literario: req.genero_literario.clone(),
        })
    }

    pub async fn actualizar_genero(
        pool: &DbPool,
        id: i32,
        req: &CrearGeneroRequest,
    ) -> Result<Genero, AppError> {
        let res = sqlx::query("UPDATE genero SET genero_literario = ? WHERE id_genero = ?")
            .bind(&req.genero_literario)
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Género con ID {} no encontrado", id)));
        }

        Ok(Genero {
            id_genero: id,
            genero_literario: req.genero_literario.clone(),
        })
    }

    pub async fn eliminar_genero(pool: &DbPool, id: i32) -> Result<(), AppError> {
        // libro_genero/revista_genero borran en cascada: sin este chequeo se perderían géneros de productos.
        let row = sqlx::query(
            "SELECT (SELECT COUNT(*) FROM libro_genero WHERE id_genero = ?)
                  + (SELECT COUNT(*) FROM revista_genero WHERE id_genero = ?)
                  + (SELECT COUNT(*) FROM mueble WHERE id_genero = ?) AS cant",
        )
        .bind(id)
        .bind(id)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        let cant = row_get_i64(&row, "cant");
        if cant > 0 {
            return Err(AppError::BadRequest(format!(
                "No es posible eliminar el género (ID {}): está en uso por {} libro(s), revista(s) o mueble(s).",
                id, cant
            )));
        }

        let res = sqlx::query("DELETE FROM genero WHERE id_genero = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Género con ID {} no encontrado", id)));
        }
        Ok(())
    }

    pub async fn asociar_generos_libro(
        pool: &DbPool,
        codigo_ean: i64,
        generos: &[i32],
    ) -> Result<(), AppError> {
        // Limpiar géneros existentes para este libro
        sqlx::query("DELETE FROM libro_genero WHERE codigo_ean = ?")
            .bind(codigo_ean)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        for &id_gen in generos {
            sqlx::query("INSERT IGNORE INTO libro_genero (codigo_ean, id_genero) VALUES (?, ?)")
                .bind(codigo_ean)
                .bind(id_gen)
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
        }
        Ok(())
    }

    pub async fn asociar_generos_revista(
        pool: &DbPool,
        codigo_ean: i64,
        generos: &[i32],
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM revista_genero WHERE codigo_ean = ?")
            .bind(codigo_ean)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        for &id_gen in generos {
            sqlx::query("INSERT IGNORE INTO revista_genero (codigo_ean, id_genero) VALUES (?, ?)")
                .bind(codigo_ean)
                .bind(id_gen)
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
        }
        Ok(())
    }
}
