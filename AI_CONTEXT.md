# Contexto técnico del backend NovaBook

Este documento describe el estado actual del backend y está basado en la estructura real del proyecto.

## 1. Resumen del proyecto

La API REST de NovaBook se implementa con:

- Rust 2024
- Axum 0.8
- Tokio
- SQLx con MySQL
- JWT para autenticación
- bcrypt para hashing de contraseñas
- dotenvy para configuración por entorno
- tower-http CORS

El backend gestiona flujo de negocio de librería y papelería con módulos de usuarios, productos, inventario, ventas, devoluciones, compras y bitácora.

## 2. Punto de entrada

La aplicación arranca en `api_migrado/src/main.rs`.

El flujo principal es:

1. Cargar configuración con `Config::from_env()`.
2. Crear el pool de conexiones con `create_pool()`.
3. Construir `AppState { pool, config }`.
4. Montar routers de usuarios y productos.
5. Agregar CORS y middleware de auditoría.
6. Iniciar el servidor HTTP.

## 3. Configuración

Archivo: `api_migrado/src/config.rs`

Variables esperadas:

- `DATABASE_URL`
- `JWT_SECRET`
- `JWT_EXPIRATION_HOURS` (opcional, default 24)
- `SERVER_HOST` (opcional, default `0.0.0.0`)
- `SERVER_PORT` (opcional, default `8080`)

## 4. Arquitectura general

La app sigue una organización modular por dominio:

- `usuarios/`: autenticación, perfil y administración de usuarios
- `productos/`: catálogo, inventario, ventas, compras y devoluciones
- `middleware/`: JWT y validación por permisos
- `services/`: reportes y servicios auxiliares
- `db.rs`: acceso a la BD
- `error.rs`: manejo centralizado de errores

## 5. Módulo de usuarios

Carpeta: `api_migrado/src/usuarios/`

Subestructura real:

- `controller/mod.rs`
- `model/`
- `repo/`
- `service/`

El módulo expone rutas:

- `POST /api/auth/login`
- `GET /api/auth/perfil`
- `GET /api/usuarios/`
- `POST /api/usuarios/`
- `GET /api/usuarios/{id}`
- `PUT /api/usuarios/{id}`
- `DELETE /api/usuarios/{id}`

Los roles esperados son:

- `1`: Jefe de Departamento
- `2`: Personal de Bodega
- `3`: Vendedor
- `4`: Gerente

## 6. Módulo de productos

Carpeta: `api_migrado/src/productos/`

Subestructura real:

- `controller/mod.rs`
- `model/`
- `repo/`
- `service/`

Este módulo incluye operaciones de negocio para:

- catálogo de autores, géneros, motivos, muebles y proveedores
- registro y movimiento de libros y revistas
- compras y stock
- ventas
- devoluciones
- bitácora y auditoría

Rutas relevantes:

### Catálogos

- `GET /api/generos`
- `POST /api/generos`
- `PUT /api/generos/{id}`
- `DELETE /api/generos/{id}`
- `GET /api/autores`
- `POST /api/autores`
- `PUT /api/autores/{id}`
- `DELETE /api/autores/{id}`
- `GET /api/motivos`
- `POST /api/motivos`
- `PUT /api/motivos/{id}`
- `DELETE /api/motivos/{id}`
- `GET /api/muebles`
- `POST /api/muebles`
- `PUT /api/muebles/{id}`
- `DELETE /api/muebles/{id}`
- `GET /api/proveedores`
- `POST /api/proveedores`
- `PUT /api/proveedores/{id}`
- `DELETE /api/proveedores/{id}`

### Bodega

- `POST /api/bodega/libros`
- `POST /api/bodega/revistas`
- `POST /api/bodega/movimientos/libros`
- `POST /api/bodega/movimientos/revistas`
- `POST /api/bodega/compras`
- `GET /api/bodega/compras`

### Vendedor

- `POST /api/vendedor/ventas`
- `GET /api/vendedor/existencias`
- `POST /api/vendedor/movimientos/libros`
- `POST /api/vendedor/movimientos/revistas`
- `POST /api/vendedor/devoluciones`
- `GET /api/vendedor/devoluciones/{id}/pdf/libros`
- `GET /api/vendedor/devoluciones/{id}/pdf/revistas`

### Jefe

- `GET /api/jefe/devoluciones/historial`
- `GET /api/jefe/devoluciones/{id}/pdf/libros`
- `GET /api/jefe/devoluciones/{id}/pdf/revistas`
- `GET /api/jefe/movimientos/diarios`
- `PUT /api/jefe/devoluciones/{id}/aprobar`

## 7. Seguridad y autenticación

La API usa middleware para proteger las rutas y decodificar JWT.

Middlewares presentes en el código actual:

- `auth_middleware`
- `require_gerente_o_jefe`
- `require_bodega_o_gerente`
- `require_jefe`
- `require_bodega`
- `require_vendedor`

Esto define el control de acceso por rol y evita que usuarios de otros perfiles accedan a rutas no autorizadas.

## 8. Base de datos y scripts

Los scripts SQL del repositorio son:

- `Libreriav2.sql`
- `api_migrado/backup_libreria.sql`
- `api_migrado/libreria_unificada-definitivo.sql`
- `api_migrado/migrations_v2.sql`
- `api_migrado/seeds.sql`

El backend espera una base MySQL/MariaDB con esquema compatible con `usuarios`, `roles`, `libros`, `revistas`, `ventas`, `detalle_ventas`, `bitacora`, `devolucion`, `movimientos`, `proveedor`, `mueble`, `genero` y `autor`.

## 9. Nota importante

La documentación previa incluía referencias a una SPA web o archivos inexistentes (`web_frontend`, `middleware.rs` directos, etc.). Ese contenido es histórico y no representa la estructura actual del repositorio. La referencia operativa actual es:

- backend Rust en `api_migrado/`
- frontend Qt 6 en `frontend_migrado/`
- scripts y prototipos Python en `Python/`
