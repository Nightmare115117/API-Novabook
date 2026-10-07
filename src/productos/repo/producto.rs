use sqlx::Row;

use crate::{
    db::{DbPool, row_get_f64, row_get_i32, row_get_i64, row_get_opt_i32, row_get_opt_i64},
    error::AppError,
    productos::{
        model::{
            ActualizarAutorRequest, ActualizarProveedorRequest, Autor, CompraResponse,
            CrearAutorRequest, CrearProveedorRequest, Devolucion, ExistenciaInventario,
            FiltroExistencias, Genero, HistorialDevolucionQuery, Libro, MovimientoDiarioItem,
            Proveedor, RegistrarCompraRequest, RegistrarLibroRequest, RegistrarRevistaRequest,
            RegistrarVentaRequest, Revista, VentaResponse,
        },
        repo::{
            AutorRepo, BitacoraRepo, CompraRepo, DevolucionRepo, GeneroRepo, InventarioRepo,
            MovimientoRepo, ProveedorRepo, VentaRepo,
        },
    },
};

pub struct ProductoRepo;

impl ProductoRepo {
    // ========================================================================
    // Operaciones de Libros y Revistas (Producto)
    // ========================================================================

    pub async fn buscar_libro(
        pool: &DbPool,
        codigo_ean: i64,
        id_ubicacion: i32,
    ) -> Result<Option<Libro>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT codigo_ean, id_mueble, id_proveedor, id_ubicacion,
                   nombre_libro, precio, cantidad, SKU
            FROM libros
            WHERE codigo_ean = ? AND id_ubicacion = ?
            "#,
        )
        .bind(codigo_ean)
        .bind(id_ubicacion)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        if let Some(r) = row {
            let gen_rows = sqlx::query(
                "SELECT lg.id_genero, g.genero_literario FROM libro_genero lg JOIN genero g ON lg.id_genero = g.id_genero WHERE lg.codigo_ean = ?"
            )
            .bind(codigo_ean)
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;

            let generos: Vec<i32> = gen_rows
                .iter()
                .map(|gr| row_get_i32(gr, "id_genero"))
                .collect();
            let generos_nombres: Vec<String> = gen_rows
                .iter()
                .map(|gr| gr.get("genero_literario"))
                .collect();

            let aut_rows = sqlx::query(
                "SELECT la.id_autor, CONCAT(a.nombre, ' ', a.apellidos) as autor_nom FROM libro_autor la JOIN autores a ON la.id_autor = a.id_autor WHERE la.codigo_ean = ?"
            )
            .bind(codigo_ean)
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;

            let autores: Vec<i32> = aut_rows
                .iter()
                .map(|ar| row_get_i32(ar, "id_autor"))
                .collect();
            let autores_nombres: Vec<String> =
                aut_rows.iter().map(|ar| ar.get("autor_nom")).collect();

            Ok(Some(Libro {
                codigo_ean: row_get_i64(&r, "codigo_ean"),
                id_mueble: row_get_i32(&r, "id_mueble"),
                id_proveedor: row_get_i32(&r, "id_proveedor"),
                id_ubicacion: row_get_i32(&r, "id_ubicacion"),
                nombre_libro: r.get("nombre_libro"),
                precio: row_get_f64(&r, "precio"),
                cantidad: row_get_i32(&r, "cantidad"),
                sku: row_get_opt_i64(&r, "SKU"),
                generos,
                autores,
                generos_nombres: if generos_nombres.is_empty() {
                    None
                } else {
                    Some(generos_nombres.join(", "))
                },
                autores_nombres: if autores_nombres.is_empty() {
                    None
                } else {
                    Some(autores_nombres.join(", "))
                },
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn guardar_o_actualizar_libro(
        pool: &DbPool,
        req: &RegistrarLibroRequest,
        ubicacion: i32,
    ) -> Result<Libro, AppError> {
        let existing = Self::buscar_libro(pool, req.codigo_ean, ubicacion).await?;

        let mut final_generos = req.generos.clone();
        if final_generos.is_empty()
            && let Some(ig) = req.id_genero {
                final_generos.push(ig);
            }

        let mut final_autores = req.autores.clone();
        if final_autores.is_empty()
            && let Some(ref nombre_autor) = req.autor {
                let clean = nombre_autor.trim();
                if !clean.is_empty() {
                    let row_aut = sqlx::query("SELECT id_autor FROM autores WHERE CONCAT(nombre, ' ', apellidos) LIKE ? OR nombre LIKE ?")
                        .bind(clean)
                        .bind(clean)
                        .fetch_optional(pool)
                        .await
                        .map_err(AppError::Database)?;

                    if let Some(ra) = row_aut {
                        final_autores.push(row_get_i32(&ra, "id_autor"));
                    } else {
                        let parts: Vec<&str> = clean.split_whitespace().collect();
                        let (nom, ape) = if parts.len() > 1 {
                            (parts[0], parts[1..].join(" "))
                        } else {
                            (clean, ".".to_string())
                        };
                        let ins_aut =
                            sqlx::query("INSERT INTO autores (nombre, apellidos) VALUES (?, ?)")
                                .bind(nom)
                                .bind(ape)
                                .execute(pool)
                                .await
                                .map_err(AppError::Database)?;
                        final_autores.push(ins_aut.last_insert_id() as i32);
                    }
                }
            }

        let libro_resultado = if let Some(libro_actual) = existing {
            let nueva_cantidad = libro_actual.cantidad + req.cantidad;
            sqlx::query(
                "UPDATE libros SET cantidad = ?, precio = ? WHERE codigo_ean = ? AND id_ubicacion = ?"
            )
            .bind(nueva_cantidad)
            .bind(req.precio)
            .bind(req.codigo_ean)
            .bind(ubicacion)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

            Libro {
                cantidad: nueva_cantidad,
                precio: req.precio,
                generos: final_generos.clone(),
                autores: final_autores.clone(),
                ..libro_actual
            }
        } else {
            sqlx::query(
                r#"
                INSERT INTO libros (codigo_ean, id_mueble, id_proveedor, id_ubicacion,
                                   nombre_libro, precio, cantidad, SKU)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(req.codigo_ean)
            .bind(req.id_mueble)
            .bind(req.id_proveedor)
            .bind(ubicacion)
            .bind(&req.nombre_libro)
            .bind(req.precio)
            .bind(req.cantidad)
            .bind(req.sku)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

            Libro {
                codigo_ean: req.codigo_ean,
                id_mueble: req.id_mueble,
                id_proveedor: req.id_proveedor,
                id_ubicacion: ubicacion,
                nombre_libro: req.nombre_libro.clone(),
                precio: req.precio,
                cantidad: req.cantidad,
                sku: req.sku,
                generos: final_generos.clone(),
                autores: final_autores.clone(),
                generos_nombres: None,
                autores_nombres: None,
            }
        };

        if !final_generos.is_empty() {
            GeneroRepo::asociar_generos_libro(pool, req.codigo_ean, &final_generos).await?;
        }
        if !final_autores.is_empty() {
            AutorRepo::asociar_autores_libro(pool, req.codigo_ean, &final_autores).await?;
        }

        Ok(libro_resultado)
    }

    pub async fn buscar_revista(
        pool: &DbPool,
        codigo_ean: i64,
        id_ubicacion: i32,
    ) -> Result<Option<Revista>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT codigo_ean, id_mueble, id_proveedor, id_ubicacion,
                   nombre_revista, numero_edicion, periodicidad, precio, cantidad, SKU
            FROM revistas
            WHERE codigo_ean = ? AND id_ubicacion = ?
            "#,
        )
        .bind(codigo_ean)
        .bind(id_ubicacion)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        if let Some(r) = row {
            let gen_rows = sqlx::query(
                "SELECT rg.id_genero, g.genero_literario FROM revista_genero rg JOIN genero g ON rg.id_genero = g.id_genero WHERE rg.codigo_ean = ?"
            )
            .bind(codigo_ean)
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)?;

            let generos: Vec<i32> = gen_rows
                .iter()
                .map(|gr| row_get_i32(gr, "id_genero"))
                .collect();
            let generos_nombres: Vec<String> = gen_rows
                .iter()
                .map(|gr| gr.get("genero_literario"))
                .collect();

            Ok(Some(Revista {
                codigo_ean: row_get_i64(&r, "codigo_ean"),
                id_ubicacion: row_get_i32(&r, "id_ubicacion"),
                id_mueble: row_get_i32(&r, "id_mueble"),
                id_proveedor: row_get_i32(&r, "id_proveedor"),
                nombre_revista: r.get("nombre_revista"),
                numero_edicion: row_get_opt_i32(&r, "numero_edicion"),
                periodicidad: r
                    .try_get::<Option<String>, _>("periodicidad")
                    .ok()
                    .flatten(),
                precio: row_get_f64(&r, "precio"),
                cantidad: row_get_i32(&r, "cantidad"),
                sku: row_get_opt_i64(&r, "SKU"),
                generos,
                generos_nombres: if generos_nombres.is_empty() {
                    None
                } else {
                    Some(generos_nombres.join(", "))
                },
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn guardar_o_actualizar_revista(
        pool: &DbPool,
        req: &RegistrarRevistaRequest,
        ubicacion: i32,
    ) -> Result<Revista, AppError> {
        let existing = Self::buscar_revista(pool, req.codigo_ean, ubicacion).await?;

        let revista_resultado = if let Some(revista_actual) = existing {
            let nueva_cantidad = revista_actual.cantidad + req.cantidad;
            sqlx::query(
                "UPDATE revistas SET cantidad = ?, precio = ? WHERE codigo_ean = ? AND id_ubicacion = ?"
            )
            .bind(nueva_cantidad)
            .bind(req.precio)
            .bind(req.codigo_ean)
            .bind(ubicacion)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

            Revista {
                cantidad: nueva_cantidad,
                precio: req.precio,
                generos: req.generos.clone(),
                ..revista_actual
            }
        } else {
            sqlx::query(
                r#"
                INSERT INTO revistas (codigo_ean, id_ubicacion, id_mueble, id_proveedor,
                                     nombre_revista, numero_edicion, periodicidad, precio, cantidad, SKU)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(req.codigo_ean)
            .bind(ubicacion)
            .bind(req.id_mueble)
            .bind(req.id_proveedor)
            .bind(&req.nombre_revista)
            .bind(req.numero_edicion)
            .bind(&req.periodicidad)
            .bind(req.precio)
            .bind(req.cantidad)
            .bind(req.sku)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

            Revista {
                codigo_ean: req.codigo_ean,
                id_ubicacion: ubicacion,
                id_mueble: req.id_mueble,
                id_proveedor: req.id_proveedor,
                nombre_revista: req.nombre_revista.clone(),
                numero_edicion: req.numero_edicion,
                periodicidad: req.periodicidad.clone(),
                precio: req.precio,
                cantidad: req.cantidad,
                sku: req.sku,
                generos: req.generos.clone(),
                generos_nombres: None,
            }
        };

        if !req.generos.is_empty() {
            GeneroRepo::asociar_generos_revista(pool, req.codigo_ean, &req.generos).await?;
        }

        Ok(revista_resultado)
    }

    // ========================================================================
    // Delegaciones a repositorios especializados
    // ========================================================================

    pub async fn listar_generos(pool: &DbPool) -> Result<Vec<Genero>, AppError> {
        GeneroRepo::listar_generos(pool).await
    }

    pub async fn listar_autores(pool: &DbPool) -> Result<Vec<Autor>, AppError> {
        AutorRepo::listar_autores(pool).await
    }

    pub async fn buscar_autor(pool: &DbPool, id: i32) -> Result<Option<Autor>, AppError> {
        AutorRepo::buscar_autor(pool, id).await
    }

    pub async fn crear_autor(pool: &DbPool, req: &CrearAutorRequest) -> Result<Autor, AppError> {
        AutorRepo::crear_autor(pool, req).await
    }

    pub async fn actualizar_autor(
        pool: &DbPool,
        id: i32,
        req: &ActualizarAutorRequest,
    ) -> Result<Autor, AppError> {
        AutorRepo::actualizar_autor(pool, id, req).await
    }

    pub async fn eliminar_autor(pool: &DbPool, id: i32) -> Result<(), AppError> {
        AutorRepo::eliminar_autor(pool, id).await
    }

    pub async fn listar_proveedores(pool: &DbPool) -> Result<Vec<Proveedor>, AppError> {
        ProveedorRepo::listar_proveedores(pool).await
    }

    pub async fn buscar_proveedor(pool: &DbPool, id: i32) -> Result<Option<Proveedor>, AppError> {
        ProveedorRepo::buscar_proveedor(pool, id).await
    }

    pub async fn crear_proveedor(
        pool: &DbPool,
        req: &CrearProveedorRequest,
    ) -> Result<Proveedor, AppError> {
        ProveedorRepo::crear_proveedor(pool, req).await
    }

    pub async fn actualizar_proveedor(
        pool: &DbPool,
        id: i32,
        req: &ActualizarProveedorRequest,
    ) -> Result<Proveedor, AppError> {
        ProveedorRepo::actualizar_proveedor(pool, id, req).await
    }

    pub async fn eliminar_proveedor(pool: &DbPool, id: i32) -> Result<(), AppError> {
        ProveedorRepo::eliminar_proveedor(pool, id).await
    }

    pub async fn trasladar_stock_libro(
        pool: &DbPool,
        codigo_ean: i64,
        cantidad: i32,
        origen: i32,
        destino: i32,
        actor_id: i64,
        observaciones: Option<&str>,
    ) -> Result<(i32, i32), AppError> {
        MovimientoRepo::trasladar_stock_libro(
            pool,
            codigo_ean,
            cantidad,
            origen,
            destino,
            actor_id,
            observaciones,
        )
        .await
    }

    pub async fn trasladar_stock_revista(
        pool: &DbPool,
        codigo_ean: i64,
        cantidad: i32,
        origen: i32,
        destino: i32,
        actor_id: i64,
        observaciones: Option<&str>,
    ) -> Result<(i32, i32), AppError> {
        MovimientoRepo::trasladar_stock_revista(
            pool,
            codigo_ean,
            cantidad,
            origen,
            destino,
            actor_id,
            observaciones,
        )
        .await
    }

    pub async fn registrar_venta_transaccional(
        pool: &DbPool,
        actor_id: i64,
        req: &RegistrarVentaRequest,
    ) -> Result<VentaResponse, AppError> {
        VentaRepo::registrar_venta_transaccional(pool, actor_id, req).await
    }

    pub async fn registrar_compra_transaccional(
        pool: &DbPool,
        actor_id: i64,
        req: &RegistrarCompraRequest,
    ) -> Result<CompraResponse, AppError> {
        CompraRepo::registrar_compra_transaccional(pool, actor_id, req).await
    }

    pub async fn listar_compras(pool: &DbPool) -> Result<Vec<CompraResponse>, AppError> {
        CompraRepo::listar_compras(pool).await
    }

    pub async fn consultar_existencias(
        pool: &DbPool,
        params: &FiltroExistencias,
    ) -> Result<Vec<ExistenciaInventario>, AppError> {
        InventarioRepo::consultar_existencias(pool, params).await
    }

    pub async fn registrar_devolucion(
        pool: &DbPool,
        id_devolucion: i64,
        total_piezas: i32,
        id_usuario: i64,
        id_proveedor: i32,
    ) -> Result<(), AppError> {
        DevolucionRepo::registrar_devolucion(pool, id_devolucion, total_piezas, id_usuario, id_proveedor).await
    }

    pub async fn buscar_devolucion(
        pool: &DbPool,
        id: i64,
    ) -> Result<Option<Devolucion>, AppError> {
        DevolucionRepo::buscar_devolucion(pool, id).await
    }

    pub async fn listar_historial_devoluciones(
        pool: &DbPool,
        params: &HistorialDevolucionQuery,
    ) -> Result<Vec<Devolucion>, AppError> {
        DevolucionRepo::listar_historial_devoluciones(pool, params).await
    }

    pub async fn actualizar_estado_devolucion(
        pool: &DbPool,
        id: i64,
        nuevo_estado: &str,
        autorizado_por: &str,
    ) -> Result<(), AppError> {
        DevolucionRepo::actualizar_estado_devolucion(pool, id, nuevo_estado, autorizado_por).await
    }

    pub async fn consultar_movimiento_diario(
        pool: &DbPool,
        fecha: Option<&str>,
    ) -> Result<Vec<MovimientoDiarioItem>, AppError> {
        BitacoraRepo::consultar_movimiento_diario(pool, fecha).await
    }
}
