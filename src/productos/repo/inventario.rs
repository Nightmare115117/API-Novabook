use sqlx::Row;

use crate::{
    db::{DbPool, row_get_f64, row_get_i64, row_get_opt_i64},
    error::AppError,
    productos::model::{ExistenciaInventario, FiltroExistencias},
};

pub struct InventarioRepo;

impl InventarioRepo {
    pub async fn consultar_existencias(
        pool: &DbPool,
        params: &FiltroExistencias,
    ) -> Result<Vec<ExistenciaInventario>, AppError> {
        let mut sql = String::from(
            r#"
            SELECT l.codigo_ean, l.SKU as sku, l.nombre_libro as titulo, 'Libro' as tipo_producto,
                   l.precio, p.nombre_proveedor as proveedor,
                   COALESCE(GROUP_CONCAT(DISTINCT CONCAT(a.nombre, ' ', a.apellidos) SEPARATOR ', '), 'Sin Autor') as autor_o_editorial,
                   COALESCE(GROUP_CONCAT(DISTINCT g.genero_literario SEPARATOR ', '), 'General') as generos,
                   COALESCE(SUM(CASE WHEN l.id_ubicacion = 1 THEN l.cantidad ELSE 0 END), 0) as stock_tienda,
                   COALESCE(SUM(CASE WHEN l.id_ubicacion = 2 THEN l.cantidad ELSE 0 END), 0) as stock_bodega
            FROM libros l
            LEFT JOIN proveedores p ON l.id_proveedor = p.id_proveedor
            LEFT JOIN libro_autor la ON l.codigo_ean = la.codigo_ean
            LEFT JOIN autores a ON la.id_autor = a.id_autor
            LEFT JOIN libro_genero lg ON l.codigo_ean = lg.codigo_ean
            LEFT JOIN genero g ON lg.id_genero = g.id_genero
            WHERE 1 = 1
            "#,
        );

        if let Some(ref q) = params.q {
            let q_clean = q.trim();
            if !q_clean.is_empty() {
                sql.push_str(&format!(
                    " AND (l.nombre_libro LIKE '%{0}%' OR a.nombre LIKE '%{0}%' OR a.apellidos LIKE '%{0}%' OR CAST(l.codigo_ean AS CHAR) LIKE '%{0}%')",
                    q_clean.replace('\'', "''")
                ));
            }
        }
        sql.push_str(" GROUP BY l.codigo_ean, l.SKU, l.nombre_libro, l.precio, p.nombre_proveedor");

        sql.push_str(
            r#"
            UNION ALL
            SELECT r.codigo_ean, r.SKU as sku, r.nombre_revista as titulo, 'Revista' as tipo_producto,
                   r.precio, p.nombre_proveedor as proveedor,
                   COALESCE(r.periodicidad, 'Mensual') as autor_o_editorial,
                   COALESCE(GROUP_CONCAT(DISTINCT g.genero_literario SEPARATOR ', '), 'General') as generos,
                   COALESCE(SUM(CASE WHEN r.id_ubicacion = 1 THEN r.cantidad ELSE 0 END), 0) as stock_tienda,
                   COALESCE(SUM(CASE WHEN r.id_ubicacion = 2 THEN r.cantidad ELSE 0 END), 0) as stock_bodega
            FROM revistas r
            LEFT JOIN proveedores p ON r.id_proveedor = p.id_proveedor
            LEFT JOIN revista_genero rg ON r.codigo_ean = rg.codigo_ean
            LEFT JOIN genero g ON rg.id_genero = g.id_genero
            WHERE 1 = 1
            "#,
        );

        if let Some(ref q) = params.q {
            let q_clean = q.trim();
            if !q_clean.is_empty() {
                sql.push_str(&format!(
                    " AND (r.nombre_revista LIKE '%{0}%' OR r.periodicidad LIKE '%{0}%' OR CAST(r.codigo_ean AS CHAR) LIKE '%{0}%')",
                    q_clean.replace('\'', "''")
                ));
            }
        }
        sql.push_str(" GROUP BY r.codigo_ean, r.SKU, r.nombre_revista, r.precio, p.nombre_proveedor, r.periodicidad");
        sql.push_str(" ORDER BY titulo ASC");

        let rows = sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;

        let list: Vec<ExistenciaInventario> = rows
            .into_iter()
            .filter_map(|r| {
                let tipo: String = r.get("tipo_producto");
                if let Some(ref filtro_tipo) = params.tipo {
                    let ft = filtro_tipo.trim().to_lowercase();
                    if !ft.is_empty() && ft != "todos" && tipo.to_lowercase() != ft {
                        return None;
                    }
                }

                let stock_tienda = row_get_i64(&r, "stock_tienda");
                let stock_bodega = row_get_i64(&r, "stock_bodega");

                if let Some(ub) = params.ubicacion
                    && ((ub == 1 && stock_tienda <= 0) || (ub == 2 && stock_bodega <= 0))
                {
                    return None;
                }

                let stock_total = (stock_tienda + stock_bodega) as i32;

                Some(ExistenciaInventario {
                    codigo_ean: row_get_i64(&r, "codigo_ean"),
                    sku: row_get_opt_i64(&r, "sku"),
                    titulo: r.get("titulo"),
                    tipo_producto: tipo,
                    stock_tienda: stock_tienda as i32,
                    stock_bodega: stock_bodega as i32,
                    stock_total,
                    precio: row_get_f64(&r, "precio"),
                    proveedor: r.get("proveedor"),
                    autor_o_editorial: r.get("autor_o_editorial"),
                    generos: r.get("generos"),
                })
            })
            .collect();

        Ok(list)
    }
}
