# Contexto técnico de la API NovaBook para IA

Este documento resume el estado actual del proyecto para que otra IA pueda entender rápidamente la arquitectura, los patrones de desarrollo, los endpoints y los puntos de entrada sin tener que inspeccionar todo el código a ciegas.

## 1. Resumen del proyecto

Es una API REST en Rust construida con:

- Axum para HTTP
- SQLx para acceso a MySQL
- JWT para autenticación
- bcrypt para hash de contraseñas
- dotenvy para configuración por variables de entorno
- tower-http CORS

El dominio es una librería/papelería llamada NovaBook. La aplicación está orientada a roles y a flujos de negocio de bodega, ventas, devoluciones, inventario y auditoría.

## 2. Punto de entrada

El arranque está en:

- `src/main.rs`

Flujo principal:

1. Carga configuración desde entorno con `Config::from_env()`.
2. Crea el pool de MySQL con `create_pool()`.
3. Construye `AppState { pool, config }`.
4. Monta routers de usuarios y productos.
5. Añade CORS y middleware de auditoría.
6. Inicia el servidor HTTP con `axum::serve()`.

La aplicación se monta así:

- `usuarios::controller::router(state.clone())`
- `productos::controller::router(state.clone())`

## 3. Configuración y entorno

Archivo:

- `src/config.rs`

Variables esperadas:

- `DATABASE_URL` (obligatoria)
- `JWT_SECRET` (obligatoria)
- `JWT_EXPIRATION_HOURS` (opcional, default 24)
- `SERVER_HOST` (opcional, default `0.0.0.0`)
- `SERVER_PORT` (opcional, default `8080`)

La aplicación usa `dotenvy::dotenv()` para intentar cargar `.env` si existe.

## 4. Arquitectura general

La app sigue una estructura por dominio, con el patrón:

- `controller`: HTTP handlers y rutas
- `model`: DTOs, entidades y tipos de dominio
- `repo`: acceso a SQL / queries
- `service`: lógica de negocio y composiciones

Módulos raíz importantes:

- `src/usuarios/`
- `src/productos/`
- `src/middleware/`
- `src/services/`
- `src/db.rs`
- `src/error.rs`

### 4.1 Módulo usuarios

Ruta base: `src/usuarios/`

Subestructura:

- `controller/`
- `model/`
- `repo/`
- `service/`

Roles definidos en `src/usuarios/model/usuario.rs`:

- `Gerente` = 4
- `JefeDepartamento` = 1
- `PersonalBodega` = 2
- `Vendedor` = 3

Modelo principal:

- `Usuario`
- `UsuarioDto`
- `CrearUsuarioRequest`
- `ActualizarUsuarioRequest`
- `LoginRequest`
- `LoginResponse`
- `Claims`

### 4.2 Módulo productos

Ruta base: `src/productos/`

Subestructura:

- `controller/`
- `model/`
- `repo/`
- `service/`

El módulo contiene las entidades del catálogo y flujos operativos del negocio:

- `autor`
- `bitacora`
- `bodega`
- `compra`
- `devolucion`
- `genero`
- `inventario`
- `motivo`
- `movimiento`
- `mueble`
- `producto`
- `proveedor`
- `venta`

## 5. Rutas HTTP principales

La composición real de rutas se encuentra en:

- `src/usuarios/controller/mod.rs`
- `src/productos/controller/mod.rs`

### 5.1 Autenticación y usuarios

`/api/auth`

- `POST /api/auth/login`
- `GET /api/auth/perfil` (requiere JWT)

`/api/usuarios`

- `GET /api/usuarios/`
- `POST /api/usuarios/`
- `GET /api/usuarios/{id}`
- `PUT /api/usuarios/{id}`
- `DELETE /api/usuarios/{id}`

Estas rutas están protegidas con `auth_middleware` y requieren roles específicos para escritura.

### 5.2 Catálogos

Rutas bajo `/api`:

- `GET /api/generos`
- `POST /api/generos`
- `PUT /api/generos/{id}`
- `DELETE /api/generos/{id}`

- `GET /api/motivos`
- `POST /api/motivos`
- `PUT /api/motivos/{id}`
- `DELETE /api/motivos/{id}`

- `GET /api/muebles`
- `POST /api/muebles`
- `PUT /api/muebles/{id}`
- `DELETE /api/muebles/{id}`

- `GET /api/autores`
- `POST /api/autores`
- `PUT /api/autores/{id}`
- `DELETE /api/autores/{id}`

- `GET /api/proveedores`
- `POST /api/proveedores`
- `PUT /api/proveedores/{id}`
- `DELETE /api/proveedores/{id}`

Estas rutas requieren autenticación y permisos de bodega/gerencia según el caso.

### 5.3 Bodega

Base: `/api/bodega`

- `POST /api/bodega/libros`
- `POST /api/bodega/revistas`
- `POST /api/bodega/movimientos/libros`
- `POST /api/bodega/movimientos/revistas`
- `POST /api/bodega/compras`
- `GET /api/bodega/compras`

Requiere rol: `PersonalBodega`.

### 5.4 Vendedor

Base: `/api/vendedor`

- `POST /api/vendedor/ventas`
- `GET /api/vendedor/existencias`
- `POST /api/vendedor/movimientos/libros`
- `POST /api/vendedor/movimientos/revistas`
- `POST /api/vendedor/devoluciones`
- `GET /api/vendedor/devoluciones/{id}/pdf/libros`
- `GET /api/vendedor/devoluciones/{id}/pdf/revistas`

Requiere rol: `Vendedor`.

### 5.5 Jefe de departamento

Base: `/api/jefe`

- `GET /api/jefe/devoluciones/historial`
- `GET /api/jefe/devoluciones/{id}/pdf/libros`
- `GET /api/jefe/devoluciones/{id}/pdf/revistas`
- `GET /api/jefe/movimientos/diarios`
- `PUT /api/jefe/devoluciones/{id}/aprobar`

Requiere rol: `JefeDepartamento`.

## 6. Autenticación y autorización

Archivo principal:

- `src/middleware/auth.rs`

### JWT

Se genera con `generate_jwt()` usando:

- `usuario_id`
- `rol`
- `nombre`
- `secret`
- `expires_in_hours`

La firma usa HMAC por defecto con `jsonwebtoken`.

### Middleware

- `auth_middleware`: valida bearer token y mete `Claims` en `Request` extensions.
- `require_gerente_o_jefe`
- `require_bodega_o_gerente`
- `require_jefe`
- `require_bodega`
- `require_vendedor`

Esto hace que la autorización sea basada en roles, no en permisos granulares.

## 7. Base de datos y capa de acceso

Archivo:

- `src/db.rs`

Patrón:

- `AppState` contiene `pool` y `config`
- `create_pool()` crea un `MySqlPool`
- `log_bitacora()` y `log_bitacora_tipo()` insertan eventos de auditoría

Funciones helper útiles:

- `row_get_i64`
- `row_get_opt_i64`
- `row_get_i32`
- `row_get_opt_i32`
- `row_get_f64`

Estas funciones convierten tipos ambiguos de MySQL (DECIMAL, BIGINT, INT, VARCHAR) para que el código no falle por tipos heterogéneos.

## 8. Patrón de repositorio y acceso a SQL

Un ejemplo claro es:

- `src/usuarios/repo/usuario.rs`

Se usa `sqlx::query()` + `fetch_optional`, `fetch_all`, `execute` sobre `DbPool`.

La convención es:

- `buscar_por_id()`
- `listar_todos()`
- `existe()`
- `crear()`
- `actualizar()`
- `eliminar()`

Esto hace que los handlers controladores sean delgados y que la lógica de acceso a base de datos esté separada en repositorios.

## 9. Manejo de errores

Archivo:

- `src/error.rs`

El proyecto define `AppError` con variantes:

- `Unauthorized`
- `Forbidden`
- `NotFound`
- `BadRequest`
- `Conflict`
- `Database`
- `Jwt`
- `Bcrypt`
- `Internal`

`AppError` implementa `IntoResponse`, por lo que cada handler devuelve respuestas JSON estructuradas con formato consistente.

## 10. Auditoría y bitácora

El proyecto registra eventos a través de la tabla `bitacora` en la base de datos.

Puntos de integración:

- `src/db.rs` -> `log_bitacora()` / `log_bitacora_tipo()`
- `src/middleware/audit.rs` -> `audit_middleware`

La auditoría se aplica mediante middleware en `main.rs` y solo registra acciones cuando la petición trae un JWT válido en el header `Authorization`. En ese caso, se guarda el usuario autenticado, el método HTTP, la ruta y el código de estado resultante.

## 11. PDF

Archivo relevante:

- `src/services/pdf.rs`

La generación de PDF para devoluciones no usa una librería de render pesado sino que crea un PDF manual de tipo 1.4 con objetos y stream de texto. Está pensado para actas de devolución a proveedor.

Función central:

- `PdfService::generar_pdf_devolucion(devolucion, tipo_producto)`

Se usa típicamente para exportar devoluciones a proveedor en formato PDF.

## 12. Observaciones de diseño importantes

### 12.1 El proyecto no usa `actix-web`, sino `axum`

Importante para añadir middlewares, State, routers y extractors.

### 12.2 El acceso a DB es directo y explícito

No hay ORM central ni abstraction de entidad por repositorio genérico. La lógica está muy cerca de SQL.

### 12.3 Hay control por roles, no por permisos individuales

La lógica principal de autorización está en `middleware/auth.rs` y se basa en enumeraciones de rol.

### 12.4 CORS está abierto

En `main.rs` se usa:

- `.allow_origin(Any)`
- `.allow_methods(Any)`
- `.allow_headers(Any)`

Esto es útil para desarrollo, pero no es un entorno de producción con restricciones.

## 13. Archivos SQL y base de datos de referencia

En la raíz del proyecto hay scripts SQL relevantes:

- `libreria_unificada-definitivo.sql`
- `migrations_v2.sql`
- `seeds.sql`
- `backup_libreria.sql`

Estos archivos son una pieza importante para reconstruir el esquema real de la base de datos y entender restricciones y tablas del negocio.

## 14. Cómo continuar el trabajo eficientemente

Cuando otra IA necesite implementar cambios, la ruta más segura es:

1. Revisar `src/main.rs` para ver el ensamblaje de rutas.
2. Entrar al módulo del dominio en cuestión (`usuarios` o `productos`).
3. Leer `controller/mod.rs` para conocer endpoints exactos.
4. Revisar `model/*.rs` para DTOs y entidades.
5. Revisar `repo/*.rs` para SQL.
6. Revisar `service/*.rs` para la lógica de negocio.
7. Mantener el patrón `AppError` para respuestas JSON.
8. No romper el esquema JWT/roles.

## 15. Sugerencias de mantenimiento

- Si se añade un nuevo dominio, seguir el patrón `mod.rs` + `controller` + `model` + `repo` + `service`.
- Mantener `AppState` como único estado global compartido por todos los handlers.
- Cuando se agreguen endpoints nuevos, revisar también el middleware de roles y la auditoría.
- Si se cambia el modelo de usuario o los roles, revisar `Claims` y `RolEnum` juntos.
- Para cambios de DB, comparar primero con los scripts SQL y no asumir que el estado del código coincide con el schema real.

## 16. Detecciones de contexto útiles para IA

- El proyecto parece estar en una etapa funcional de desarrollo, no en una versión de `cargo` completamente “limpia” ni con suite de tests centralizada visible en raíz.
- La lógica de negocio está fuerte en consultas SQL y `axum` handlers, no en una capa de abstracción de servicios vasta.
- La mayoría de las rutas cuentan con roles y autenticación JWT por capa.
- La estructura del código facilita añadir nuevos dominios y endpoints usando el mismo patrón.

## 17. Resumen corto para otra IA

Este repositorio es una API REST de Rust usando Axum + SQLx + MySQL. Tiene dos grandes dominios: `usuarios` y `productos`. La aplicación usa JWT para autenticación y `RolEnum` para autorización por roles. El flujo típico es `controller -> repo -> DB` con modelos compartidos y `AppError` estructurado. También incluye middleware de auditoría y generación de PDFs para devoluciones.

## 18. Archivos clave para empezar

- `src/main.rs`
- `src/config.rs`
- `src/db.rs`
- `src/error.rs`
- `src/middleware/auth.rs`
- `src/usuarios/controller/mod.rs`
- `src/productos/controller/mod.rs`
- `src/services/pdf.rs`
- `libreria_unificada-definitivo.sql`

Fin del documento.
