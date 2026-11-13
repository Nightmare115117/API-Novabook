use sqlx::Row;

use crate::{
    db::{DbPool, row_get_i32, row_get_i64},
    error::AppError,
    productos::model::{CrearMuebleRequest, Mueble},
};

pub struct MuebleRepo;

fn mapear(r: &sqlx::mysql::MySqlRow) -> Mueble {
    Mueble {
        id_mueble: row_get_i32(r, "id_mueble"),
        id_genero: row_get_i32(r, "id_genero"),
        id_ubicacion: row_get_i32(r, "id_ubicacion"),
        codigo_mueble: r.get("codigo_mueble"),
        nombre_mueble: r.get("nombre_mueble"),
        capacidad: r.get("capacidad"),
        capacidad_actual: row_get_i32(r, "capacidad_actual"),
        descripcion: r.get("descripcion"),
        genero_literario: r.get("genero_literario"),
    }
}

impl MuebleRepo {
    pub async fn listar_muebles(pool: &DbPool) -> Result<Vec<Mueble>, AppError> {
        let rows = sqlx::query(
            "SELECT m.id_mueble, m.id_genero, m.id_ubicacion, m.codigo_mueble, \
             m.nombre_mueble, m.capacidad, \
             COALESCE((SELECT SUM(e.cantidad) FROM existencias e WHERE e.id_mueble = m.id_mueble), 0) AS capacidad_actual, \
             m.descripcion, g.genero_literario \
             FROM mueble m JOIN genero g ON g.id_genero = m.id_genero ORDER BY m.id_Mueble ASC",
        )
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;
        Ok(rows.iter().map(mapear).collect())
    }

    async fn buscar_mueble(pool: &DbPool, id: i32) -> Result<Option<Mueble>, AppError> {
        let row = sqlx::query(
            "SELECT m.id_mueble, m.id_genero, m.id_ubicacion, m.codigo_mueble, \
             m.nombre_mueble, m.capacidad, \
             COALESCE((SELECT SUM(e.cantidad) FROM existencias e WHERE e.id_mueble = m.id_mueble), 0) AS capacidad_actual, \
             m.descripcion, g.genero_literario \
             FROM mueble m JOIN genero g ON g.id_genero = m.id_genero WHERE m.id_Mueble = ?",
        )
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?;
        Ok(row.as_ref().map(mapear))
    }

    async fn validar_genero(pool: &DbPool, id_genero: i32) -> Result<(), AppError> {
        let existe = sqlx::query("SELECT 1 FROM genero WHERE id_genero = ?")
            .bind(id_genero)
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?;
        if existe.is_none() {
            return Err(AppError::BadRequest(format!("El género con ID {} no existe", id_genero)));
        }
        Ok(())
    }

    pub async fn crear_mueble(pool: &DbPool, req: &CrearMuebleRequest) -> Result<Mueble, AppError> {
        Self::validar_genero(pool, req.id_genero).await?;
        let res = sqlx::query(
            "INSERT INTO mueble (id_genero, id_ubicacion, codigo_mueble, nombre_mueble, capacidad, descripcion) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
            .bind(req.id_genero)
            .bind(req.id_ubicacion)
            .bind(&req.codigo_mueble)
            .bind(&req.nombre_mueble)
            .bind(req.capacidad)
            .bind(&req.descripcion)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        let id = res.last_insert_id() as i32;
        Self::buscar_mueble(pool, id)
            .await?
            .ok_or_else(|| AppError::Internal(format!("Mueble {} no encontrado tras crearlo", id)))
    }

    pub async fn actualizar_mueble(
        pool: &DbPool,
        id: i32,
        req: &CrearMuebleRequest,
    ) -> Result<Mueble, AppError> {
        Self::validar_genero(pool, req.id_genero).await?;
        let res = sqlx::query(
            "UPDATE mueble SET id_genero = ?, id_ubicacion = ?, codigo_mueble = ?, \
             nombre_mueble = ?, capacidad = ?, descripcion = ? WHERE id_mueble = ?",
        )
            .bind(req.id_genero)
            .bind(req.id_ubicacion)
            .bind(&req.codigo_mueble)
            .bind(&req.nombre_mueble)
            .bind(req.capacidad)
            .bind(&req.descripcion)
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Mueble con ID {} no encontrado", id)));
        }
        Self::buscar_mueble(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Mueble con ID {} no encontrado", id)))
    }

    pub async fn eliminar_mueble(pool: &DbPool, id: i32) -> Result<(), AppError> {
        let row = sqlx::query(
            "SELECT COUNT(*) AS cant FROM existencias WHERE id_mueble = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        let cant = row_get_i64(&row, "cant");
        if cant > 0 {
            return Err(AppError::BadRequest(format!(
                "No es posible eliminar el mueble (ID {}): tiene {} registro(s) de libros/revistas asignados.",
                id, cant
            )));
        }

        let res = sqlx::query("DELETE FROM mueble WHERE id_mueble = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Mueble con ID {} no encontrado", id)));
        }
        Ok(())
    }
}
