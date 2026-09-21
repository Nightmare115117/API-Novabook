# CHANGELOG - API REST Librería NovaBook

Todos los cambios notables, decisiones de arquitectura, especificaciones técnicas y consideraciones para la detección de errores (bugs) se documentan en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/) y este proyecto se adhiere a [Semantic Versioning](https://semver.org/).

---

## [0.1.0] - 2026-09-18

### 🚀 Implementación Inicial

#### 1. Arquitectura y Módulos
- **Framework base:** Migración a **Axum 0.8** sobre runtime asíncrono **Tokio**.
- **Acceso a datos:** Pool de conexiones asíncrono con **SQLx 0.9** para MySQL.
- **Estructura modular:**
  - `usuarios/`: Controladores, servicios, modelos y repositorios para autenticación y gestión de usuarios.
  - `productos/`: Controladores, servicios, modelos y repositorios para inventario, bodega, piso de venta, devoluciones y movimientos.
  - `middleware/`: Autenticación por token Bearer JWT y autorización jerárquica basada en roles (RBAC).
  - `services/`: Generador binario de reportes en PDF para devoluciones a proveedor.
  - `db/`: Inicialización del pool de base de datos MySQL y función de registro automático en `bitacora`.
  - `error/`: Enumeración centralizada `AppError` con respuestas JSON uniformes (`ApiResponse<T>`).

---

### 🛡️ Seguridad y Autenticación
- **Contraseñas:** Hasheadas con algoritmo **Bcrypt** (costo 10). Las contraseñas en texto plano nunca se almacenan ni se serializan en los DTOs de salida (`UsuarioDto`).
- **Autenticación:** Tokens **JWT (HMAC-SHA256)** con expiración configurable (por defecto 24 horas).
- **Control de Acceso Basado en Roles (RBAC):**
  - `require_gerente_o_jefe`: Permite acceso a Gerente (`id_roles = 4`) y Jefe de Departamento (`id_roles = 1`), modelando la herencia de permisos.
  - `require_jefe`: Exclusivo para Jefe de Departamento (`id_roles = 1`).
  - `require_bodega`: Exclusivo para Personal de Bodega (`id_roles = 2`).
  - `require_vendedor`: Exclusivo para Vendedor (`id_roles = 3`).

---

### 📦 Supuestos del Modelo de Datos Implementados

1. **Clave primaria de `libros` y `revistas`:**
   - Se maneja clave primaria compuesta `(codigo_ean, id_ubicacion)`.
   - Permite coexistencia del mismo producto físico en múltiples ubicaciones (`id_ubicacion = 1` para Tienda/Piso de venta, `id_ubicacion = 2` para Bodega General) con stock independiente.
2. **Entidad `revistas`:**
   - Creada como tabla espejo a `libros`, incorporando campos editoriales: `numero_edicion`, `periodicidad`, `id_mueble`, `id_proveedor`, `precio`, `cantidad`, `SKU`.
3. **Entidades operacionales `ventas` y `detalle_ventas`:**
   - Cabecera y detalle de ventas para permitir transacciones con múltiples artículos (libros y revistas), cálculo de subtotales y total acumulado.
4. **Tabla `bitacora`:**
   - Inserción automática de eventos (`LOGIN`, `REGISTRO_LIBRO_BODEGA`, `REGISTRO_REVISTA_BODEGA`, `TRASLADO_*`, `REGISTRO_VENTA`, `SOLICITUD_DEVOLUCION`, `EVALUAR_DEVOLUCION`, `CREAR_USUARIO`, `ACTUALIZAR_USUARIO`, `ELIMINAR_USUARIO`).

---

### 📋 Mapeo de Endpoints REST por Actor

| Actor | Método | Endpoint | Middleware / Rol | Descripción |
| :--- | :---: | :--- | :--- | :--- |
| **Todos** | `POST` | `/api/auth/login` | Público | Autenticación y generación de JWT |
| **Autenticados** | `GET` | `/api/auth/perfil` | JWT Válido | Consulta de datos del usuario autenticado |
| **Gerente / Jefe** | `GET` | `/api/usuarios` | `require_gerente_o_jefe` | Listar todos los usuarios |
| **Gerente / Jefe** | `POST` | `/api/usuarios` | `require_gerente_o_jefe` | Crear usuario con contraseña hasheada |
| **Gerente / Jefe** | `GET` | `/api/usuarios/{id}` | `require_gerente_o_jefe` | Obtener usuario por ID |
| **Gerente / Jefe** | `PUT` | `/api/usuarios/{id}` | `require_gerente_o_jefe` | Actualizar rol, nombre o contraseña |
| **Gerente / Jefe** | `DELETE` | `/api/usuarios/{id}` | `require_gerente_o_jefe` | Eliminar usuario |
| **Bodega** | `POST` | `/api/bodega/libros` | `require_bodega` | Registrar libros en bodega |
| **Bodega** | `POST` | `/api/bodega/revistas` | `require_bodega` | Registrar revistas en bodega |
| **Bodega** | `POST` | `/api/bodega/movimientos/libros` | `require_bodega` | Traslado Bodega $\to$ Tienda (Libros) |
| **Bodega** | `POST` | `/api/bodega/movimientos/revistas` | `require_bodega` | Traslado Bodega $\to$ Tienda (Revistas) |
| **Vendedor** | `POST` | `/api/vendedor/ventas` | `require_vendedor` | Registrar venta y descontar stock de tienda |
| **Vendedor** | `GET` | `/api/vendedor/existencias` | `require_vendedor` | Consultar inventario (Tienda vs Bodega) |
| **Vendedor** | `POST` | `/api/vendedor/movimientos/libros` | `require_vendedor` | Traslado Tienda $\to$ Bodega (Libros) |
| **Vendedor** | `POST` | `/api/vendedor/movimientos/revistas` | `require_vendedor` | Traslado Tienda $\to$ Bodega (Revistas) |
| **Vendedor** | `POST` | `/api/vendedor/devoluciones` | `require_vendedor` | Solicitar devolución a proveedor |
| **Vendedor** | `GET` | `/api/vendedor/devoluciones/{id}/pdf/libros` | `require_vendedor` | Descargar PDF de devolución (Libros) |
| **Vendedor** | `GET` | `/api/vendedor/devoluciones/{id}/pdf/revistas` | `require_vendedor` | Descargar PDF de devolución (Revistas) |
| **Jefe de Depto.** | `GET` | `/api/jefe/devoluciones/historial` | `require_jefe` | Historial completo de devoluciones |
| **Jefe de Depto.** | `GET` | `/api/jefe/movimientos/diarios` | `require_jefe` | Eventos del día en bitácora |
| **Jefe de Depto.** | `PUT` | `/api/jefe/devoluciones/{id}/aprobar` | `require_jefe` | Aprobar o rechazar devolución |

---

### 🔍 Guía de Verificación y Detección de Posibles Bugs

Si experimentas algún fallo durante las pruebas o despliegue, verifica los siguientes puntos clave:

#### 1. Concurrencia y Bloqueo de Stock en Traslados / Ventas
- **Mecanismo:** Los traslados de inventario y las ventas utilizan transacciones SQL con `SELECT ... FOR UPDATE` para evitar condiciones de carrera (*race conditions*).
- **Punto de verificación:** Si dos vendedores intentan vender la última pieza simultáneamente, una transacción esperará y la segunda recibirá el error `BAD_REQUEST: Stock insuficiente en piso de venta`.

#### 2. Ausencia de Tablas Adicionales en la Base de Datos
- Si tu instancia de MySQL solo tiene las tablas del modelo original sin las tablas adicionales propuestas (`revistas`, `ventas`, `detalle_ventas`), las consultas correspondientes arrojarán error `sqlx::Error::Database` con código `Table doesn't exist`.
- **Solución:** Crear las tablas auxiliares `revistas`, `ventas` y `detalle_ventas`.

#### 3. Llave Compuesta en Registro de Libros / Revistas
- En `POST /api/bodega/libros`, se utiliza `INSERT ... ON DUPLICATE KEY UPDATE cantidad = cantidad + VALUES(cantidad)`.
- Si la tabla `libros` en tu base de datos tiene únicamente `codigo_ean` como PK (en lugar de `(codigo_ean, id_ubicacion)`), un libro registrado en Bodega que luego intente registrarse en Tienda colisionará como duplicado si no se definió la PK compuesta.

#### 4. Formato del Token JWT en Cabeceras
- Las peticiones protegidas requieren la cabecera:
  ```http
  Authorization: Bearer <token_jwt>
  ```
- Si se omite el prefijo `Bearer ` o el espacio, el middleware devolverá código HTTP `401 UNAUTHORIZED` con el mensaje descriptivo correspondiente.

#### 5. Errores de Conexión a Base de Datos
- Si las credenciales de MySQL cambian y no se configura un archivo `.env`, la aplicación fallará al iniciar. Verificar que el usuario tenga permisos `SELECT`, `INSERT`, `UPDATE`, `DELETE` en la base de datos `Libreria`.

---

### 📄 Servicio de Generación de PDF
- **Implementación:** Generación binaria nativa conforme al estándar **PDF-1.4** (sin requerir compilación de librerías nativas C externas).
- **Descarga:** Retorna cabecera `Content-Type: application/pdf` y `Content-Disposition: attachment; filename="..."`.
- **Recomendación para futuras versiones:** Si se requiere un maquetado gráfico avanzado con imágenes incrustadas o fuentes personalizadas, se sugiere integrar el crate `printpdf` o `genpdf`.
