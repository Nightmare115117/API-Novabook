use sqlx::Row;

use crate::{
    db::{DbPool, row_get_i32},
    error::AppError,
    productos::model::Genero,
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
