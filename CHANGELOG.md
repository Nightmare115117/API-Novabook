# CHANGELOG — API REST NovaBook

Todos los cambios relevantes del backend se registran aquí siguiendo Keep a Changelog.

## [Unreleased]

### Changed
- Sincronizada la documentación del backend con la estructura real de `src/usuarios`, `src/productos`, `src/middleware` y `src/services`.
- Eliminadas referencias a artefactos no presentes en el repositorio actual, como una SPA web o archivos de módulo inexistentes.
- Aclarado el conjunto real de rutas HTTP y permisos por perfil.

### Added
- Resumen actualizado del punto de entrada, módulos, rutas y configuración del backend.

## [0.1.0] - 2026-09-18

### Added
- Backend en Rust con Axum y SQLx.
- Módulos de usuarios y productos.
- Autenticación con JWT y bcrypt.
- Capa de permisos por roles.
- CRUD de usuarios y flujo de negocio para catálogo, inventario, compras, ventas y devoluciones.
- Generación de PDF para devoluciones.

### Changed
- Ajuste modular para separar rutas de catálogo, bodega, vendedor y jefe.
- Fortalecimiento del manejo de errores y auditoría.
