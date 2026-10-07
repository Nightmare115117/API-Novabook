use sqlx::Row;

use crate::{
    db::{DbPool, row_get_i32, row_get_i64},
    error::AppError,
    productos::model::{Devolucion, HistorialDevolucionQuery, TipoProducto},
};

pub struct DevolucionRepo;

impl DevolucionRepo {
    pub async fn registrar_devolucion(
        pool: &DbPool,
        id_devolucion: i64,
        total_piezas: i32,
        id_usuario: i64,
        id_proveedor: i32,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO devolucion (id_devolucion, fecha, total_piezas, estado, autorizado_por, id_usuarios, id_proveedor)
            VALUES (?, CURDATE(), ?, 'Pendiente', NULL, ?, ?)
            "#,
        )
        .bind(id_devolucion)
        .bind(total_piezas)
        .bind(id_usuario)
        .bind(id_proveedor)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn buscar_devolucion(pool: &DbPool, id: i64) -> Result<Option<Devolucion>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT d.id_devolucion, DATE_FORMAT(d.fecha, '%Y-%m-%d') as fecha,
                   d.total_piezas, d.estado, d.autorizado_por, d.id_usuarios,
                   CONCAT(u.nombre, ' ', COALESCE(u.apellido_paterno, '')) as vendedor_nombre,
                   d.id_proveedor, p.nombre_proveedor, p.rfc as proveedor_rfc,
                   p.telefono as proveedor_telefono, p.correo as proveedor_correo,
                   p.direccion as proveedor_direccion, p.persona_contacto as proveedor_contacto
            FROM devolucion d
            JOIN usuarios u ON d.id_usuarios = u.id_usuarios
            JOIN proveedores p ON d.id_proveedor = p.id_proveedor
            WHERE d.id_devolucion = ?
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row.map(|r| Devolucion {
            id_devolucion: row_get_i64(&r, "id_devolucion"),
            fecha: r.get("fecha"),
            total_piezas: row_get_i32(&r, "total_piezas"),
            estado: r
                .get::<Option<String>, _>("estado")
                .unwrap_or_else(|| "Pendiente".to_string()),
            autorizado_por: r.get("autorizado_por"),
            id_usuarios: row_get_i64(&r, "id_usuarios"),
            vendedor_nombre: r.get("vendedor_nombre"),
            id_proveedor: row_get_i32(&r, "id_proveedor"),
            nombre_proveedor: r.get("nombre_proveedor"),
            proveedor_rfc: r.get("proveedor_rfc"),
            proveedor_telefono: r.get("proveedor_telefono"),
            proveedor_correo: r.get("proveedor_correo"),
            proveedor_direccion: r.get("proveedor_direccion"),
            proveedor_contacto: r.get("proveedor_contacto"),
            tipo_producto: TipoProducto::Libro,
            items: Vec::new(),
        }))
    }

    pub async fn listar_historial_devoluciones(
        pool: &DbPool,
        params: &HistorialDevolucionQuery,
    ) -> Result<Vec<Devolucion>, AppError> {
        let mut builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
            r#"
            SELECT d.id_devolucion, DATE_FORMAT(d.fecha, '%Y-%m-%d') as fecha,
                   d.total_piezas, d.estado, d.autorizado_por, d.id_usuarios,
                   CONCAT(u.nombre, ' ', COALESCE(u.apellido_paterno, '')) AS vendedor_nombre,
                   d.id_proveedor, p.nombre_proveedor, p.rfc as proveedor_rfc,
                   p.telefono as proveedor_telefono, p.correo as proveedor_correo,
                   p.direccion as proveedor_direccion, p.persona_contacto as proveedor_contacto
            FROM devolucion d
            JOIN usuarios u ON d.id_usuarios = u.id_usuarios
            JOIN proveedores p ON d.id_proveedor = p.id_proveedor
            WHERE 1 = 1
            "#,
        );

        if let Some(ref fecha) = params.fecha {
            let f = fecha.trim();
            if !f.is_empty() {
                builder.push(" AND d.fecha = ");
                builder.push_bind(f);
            }
        }
        if let Some(ref estado) = params.estado {
            let e = estado.trim();
            if !e.is_empty() && e != "todos" && e != "Todos" {
                builder.push(" AND d.estado = ");
                builder.push_bind(e);
            }
        }
        if let Some(prov_id) = params.id_proveedor
            && prov_id > 0 {
                builder.push(" AND d.id_proveedor = ");
                builder.push_bind(prov_id);
            }

        builder.push(" ORDER BY d.fecha DESC, d.id_devolucion DESC");

        let rows = builder
            .build()
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;

        let list = rows
            .into_iter()
            .map(|r| Devolucion {
                id_devolucion: row_get_i64(&r, "id_devolucion"),
                fecha: r.get("fecha"),
                total_piezas: row_get_i32(&r, "total_piezas"),
                estado: r
                    .get::<Option<String>, _>("estado")
                    .unwrap_or_else(|| "Pendiente".to_string()),
                autorizado_por: r.get("autorizado_por"),
                id_usuarios: row_get_i64(&r, "id_usuarios"),
                vendedor_nombre: r.get("vendedor_nombre"),
                id_proveedor: row_get_i32(&r, "id_proveedor"),
                nombre_proveedor: r.get("nombre_proveedor"),
                proveedor_rfc: r.get("proveedor_rfc"),
                proveedor_telefono: r.get("proveedor_telefono"),
                proveedor_correo: r.get("proveedor_correo"),
                proveedor_direccion: r.get("proveedor_direccion"),
                proveedor_contacto: r.get("proveedor_contacto"),
                tipo_producto: TipoProducto::Libro,
                items: Vec::new(),
            })
            .collect();

        Ok(list)
    }

    pub async fn actualizar_estado_devolucion(
        pool: &DbPool,
        id: i64,
        nuevo_estado: &str,
        autorizado_por: &str,
    ) -> Result<(), AppError> {
        sqlx::query("UPDATE devolucion SET estado = ?, autorizado_por = ? WHERE id_devolucion = ?")
            .bind(nuevo_estado)
            .bind(autorizado_por)
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
