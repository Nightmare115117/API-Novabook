-- ============================================================================
-- BASE DE DATOS UNIFICADA: Libreria
-- ----------------------------------------------------------------------------
-- Esquema : diagrama ER (Editorial, Proveedor, Productos, Libros, Revistas,
--           Autores, Genero, Mueble, Existencias, Ventas, Compras,
--           Devoluciones, Movimientos, Usuarios, Bitácora, ...)
-- Datos   : backup_libreria.sql (datos vivos) + seeds.sql (ventas de ejemplo)
--           + migrations_v2.sql (ya estaba contemplado en el backup)
-- Compatible con: MySQL 8.0+ / MariaDB 10.5+
-- Es re-ejecutable: elimina y recrea todas las tablas de Libreria.
-- ============================================================================

CREATE DATABASE IF NOT EXISTS `Libreria` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
USE `Libreria`;
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------------------------------------------------------
-- 0. Limpieza previa (vistas y tablas, esquema viejo y nuevo)
-- ----------------------------------------------------------------------------
DROP VIEW IF EXISTS `vista_cliente_compras`;

-- `proveedor` antes era una vista; ahora es tabla. Se elimina solo si es vista.
SET @drop_view_proveedor = (
  SELECT IF(COUNT(*) > 0, 'DROP VIEW `proveedor`', 'SELECT 1')
  FROM information_schema.VIEWS
  WHERE TABLE_SCHEMA = 'Libreria' AND TABLE_NAME = 'proveedor'
);
PREPARE stmt FROM @drop_view_proveedor;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

DROP TABLE IF EXISTS `detalle_devolucion`;
DROP TABLE IF EXISTS `devolucion`;
DROP TABLE IF EXISTS `motivo`;
DROP TABLE IF EXISTS `motivos`;
DROP TABLE IF EXISTS `movimientos`;
DROP TABLE IF EXISTS `existencias`;
DROP TABLE IF EXISTS `detalle_ventas`;
DROP TABLE IF EXISTS `ventas`;
DROP TABLE IF EXISTS `clientes`;
DROP TABLE IF EXISTS `detalles_compras`;
DROP TABLE IF EXISTS `detalle_compra`;
DROP TABLE IF EXISTS `compras_proveedor`;
DROP TABLE IF EXISTS `bitacora`;
DROP TABLE IF EXISTS `tipo_evento`;
DROP TABLE IF EXISTS `autores_has_libros`;
DROP TABLE IF EXISTS `libro_autor`;
DROP TABLE IF EXISTS `productos_has_genero`;
DROP TABLE IF EXISTS `libro_genero`;
DROP TABLE IF EXISTS `revista_genero`;
DROP TABLE IF EXISTS `revistas`;
DROP TABLE IF EXISTS `libros`;
DROP TABLE IF EXISTS `productos`;
DROP TABLE IF EXISTS `editorial`;
DROP TABLE IF EXISTS `mueble`;
DROP TABLE IF EXISTS `usuarios`;
DROP TABLE IF EXISTS `autores`;
DROP TABLE IF EXISTS `genero`;
DROP TABLE IF EXISTS `proveedores`;
DROP TABLE IF EXISTS `proveedor`;
DROP TABLE IF EXISTS `ubicacion`;
DROP TABLE IF EXISTS `roles`;

-- ============================================================================
-- 1. CATÁLOGOS BASE
-- ============================================================================

-- ---- roles -----------------------------------------------------------------
CREATE TABLE `roles` (
  `id_roles`   INT         NOT NULL,
  `nombre_rol` VARCHAR(50) NOT NULL,
  PRIMARY KEY (`id_roles`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `roles` (`id_roles`, `nombre_rol`) VALUES
  (1, 'Jefe Departamento'),
  (2, 'Bodega'),
  (3, 'Vendedor'),
  (4, 'Gerente');

-- ---- ubicacion -------------------------------------------------------------
CREATE TABLE `ubicacion` (
  `id_ubicacion` INT         NOT NULL,
  `lugar`        VARCHAR(20) NOT NULL,
  PRIMARY KEY (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `ubicacion` (`id_ubicacion`, `lugar`) VALUES
  (1, 'Piso'),
  (2, 'Bodega');

-- ---- proveedor -------------------------------------------------------------
CREATE TABLE `proveedor` (
  `id_proveedor`     INT          NOT NULL AUTO_INCREMENT,
  `nombre_proveedor` VARCHAR(50)  NOT NULL,
  `rfc`              VARCHAR(13)  NOT NULL,
  `telefono`         VARCHAR(15)  DEFAULT NULL,
  `correo`           VARCHAR(100) DEFAULT NULL,
  `direccion`        VARCHAR(200) DEFAULT NULL,
  `persona_contacto` VARCHAR(150) DEFAULT NULL,
  `estatus`          VARCHAR(100) NOT NULL DEFAULT 'ACTIVO',
  PRIMARY KEY (`id_proveedor`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `proveedor` (`id_proveedor`, `nombre_proveedor`, `rfc`, `telefono`, `correo`, `direccion`, `persona_contacto`, `estatus`) VALUES
  (1, 'Planeta Mexico', 'PME850101AB1', '5553331100', 'contacto@planetamexico.com.mx', 'Av. Presidente Masaryk 111, Polanco, CDMX', 'Lic. Carlos Fuentes Rivera', 'ACTIVO'),
  (2, 'Planeta Mexico Infantil', 'PMI920415CD2', '5553331122', 'infantil@planetamexico.com.mx', 'Av. Presidente Masaryk 115, Polanco, CDMX', 'Mtra. Elena Poniatowska Gil', 'ACTIVO'),
  (3, 'Penguin Random House', 'PRH010920EF3', '5554442200', 'ventas@penguinrandomhouse.mx', 'Blvd. Miguel de Cervantes Saavedra 301, Granada, CDMX', 'Lic. Javier Velasco Ruiz', 'ACTIVO'),
  (4, 'Editorial Televisa', 'ETE730508GH4', '5552612000', 'distribucion@editorialtelevisa.com.mx', 'Av. Vasco de Quiroga 2000, Santa Fe, CDMX', 'Lic. Martha Debayle Gallardo', 'ACTIVO');

-- ---- editorial -------------------------------------------------------------
-- Las revistas antes solo apuntaban a un proveedor; aquí se crea la editorial
-- a partir de los datos de ese proveedor (ajústala si la editorial real es otra).
CREATE TABLE `editorial` (
  `id_editorial`     INT          NOT NULL AUTO_INCREMENT,
  `nombre_editorial` VARCHAR(100) NOT NULL,
  `rfc`              VARCHAR(13)  DEFAULT NULL,
  PRIMARY KEY (`id_editorial`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `editorial` (`id_editorial`, `nombre_editorial`, `rfc`) VALUES
  (1, 'Planeta Mexico Infantil', 'PMI920415CD2'),
  (2, 'Editorial Televisa', 'ETE730508GH4');

-- ---- genero ----------------------------------------------------------------
CREATE TABLE `genero` (
  `id_genero`        INT         NOT NULL AUTO_INCREMENT,
  `genero_literario` VARCHAR(50) NOT NULL,
  PRIMARY KEY (`id_genero`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `genero` (`id_genero`, `genero_literario`) VALUES
  (11, 'Psicologia'),
  (12, 'Autoayuda'),
  (13, 'Metafisica'),
  (14, 'Ciencia Ficcion'),
  (15, 'Historia'),
  (16, 'Novela Contemporanea'),
  (17, 'Ensayo'),
  (18, 'Divulgacion Cientifica'),
  (61, 'Infantiles'),
  (62, 'Juvenil');

-- ---- autores ---------------------------------------------------------------
CREATE TABLE `autores` (
  `id_autores`   INT          NOT NULL AUTO_INCREMENT,
  `nombre`       VARCHAR(50)  NOT NULL,
  `apellidos`    VARCHAR(100) NOT NULL,
  `nacionalidad` VARCHAR(100) DEFAULT NULL,
  `estado`       BOOLEAN      NOT NULL DEFAULT 1,
  PRIMARY KEY (`id_autores`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `autores` (`id_autores`, `nombre`, `apellidos`, `nacionalidad`, `estado`) VALUES
  (1, 'Viktor', 'Frankl', 'Austriaco', 1),
  (2, 'Gabriel', 'García Márquez', 'Colombiano', 1),
  (3, 'Gisele', 'Pelicot', 'Francesa', 1),
  (4, 'Michael', 'Ende', 'Alemán', 1),
  (5, 'Robert', 'Fisher', 'Estadounidense', 1);

-- ---- mueble ----------------------------------------------------------------
-- En el diagrama el mueble pertenece a UNA ubicación (Piso o Bodega), y el
-- inventario vive en `existencias`. Antes un mismo mueble (11, 12, 13, 61, 62)
-- se usaba en Piso y en Bodega, así que:
--   * Piso   : conserva los ids originales (11, 12, 13, 61, 62)
--   * Bodega : se crean muebles equivalentes con id = id original + 100
CREATE TABLE `mueble` (
  `id_mueble`     INT          NOT NULL AUTO_INCREMENT,
  `id_genero`     INT          NOT NULL,
  `id_ubicacion`  INT          NOT NULL,
  `codigo_mueble` VARCHAR(10)  NOT NULL,
  `nombre_mueble` VARCHAR(50)  NOT NULL,
  `capacidad`     INT          DEFAULT NULL,
  `descripcion`   VARCHAR(200) DEFAULT NULL,
  PRIMARY KEY (`id_mueble`),
  UNIQUE KEY `codigo_mueble_UNIQUE` (`codigo_mueble`),
  KEY `FK_MUEBLE_GENERO` (`id_genero`),
  KEY `FK_MUEBLE_UBICACION` (`id_ubicacion`),
  CONSTRAINT `FK_MUEBLE_GENERO`    FOREIGN KEY (`id_genero`)    REFERENCES `genero` (`id_genero`),
  CONSTRAINT `FK_MUEBLE_UBICACION` FOREIGN KEY (`id_ubicacion`) REFERENCES `ubicacion` (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `mueble` (`id_mueble`, `id_genero`, `id_ubicacion`, `codigo_mueble`, `nombre_mueble`, `capacidad`, `descripcion`) VALUES
  -- Piso (capacidad inicial estimada: 150 piezas)
  (11,  11, 1, 'P-011', 'Psicologia - Piso',  150, 'Mueble de Psicologia en piso de venta'),
  (12,  12, 1, 'P-012', 'Autoayuda - Piso',   150, 'Mueble de Autoayuda en piso de venta'),
  (13,  13, 1, 'P-013', 'Metafisica - Piso',  150, 'Mueble de Metafisica en piso de venta'),
  (61,  61, 1, 'P-061', 'Infantiles - Piso',  150, 'Mueble de Infantiles en piso de venta'),
  (62,  62, 1, 'P-062', 'Juvenil - Piso',     150, 'Mueble de Juvenil en piso de venta'),
  -- Bodega (capacidad inicial estimada: 500 piezas)
  (111, 11, 2, 'B-011', 'Psicologia - Bodega', 500, 'Mueble de Psicologia en bodega'),
  (112, 12, 2, 'B-012', 'Autoayuda - Bodega',  500, 'Mueble de Autoayuda en bodega'),
  (113, 13, 2, 'B-013', 'Metafisica - Bodega', 500, 'Mueble de Metafisica en bodega'),
  (161, 61, 2, 'B-061', 'Infantiles - Bodega', 500, 'Mueble de Infantiles en bodega'),
  (162, 62, 2, 'B-062', 'Juvenil - Bodega',    500, 'Mueble de Juvenil en bodega');

-- ---- motivo ----------------------------------------------------------------
CREATE TABLE `motivo` (
  `id_motivo`     INT          NOT NULL AUTO_INCREMENT,
  `nombre_motivo` VARCHAR(50)  NOT NULL,
  `descripcion`   VARCHAR(200) DEFAULT NULL,
  PRIMARY KEY (`id_motivo`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `motivo` (`id_motivo`, `nombre_motivo`, `descripcion`) VALUES
  (1, 'Reabastecimiento', 'Devolución para reponer o cambiar existencias con el proveedor'),
  (2, 'Defecto de imprenta', 'Producto con fallas de impresión, encuadernación o páginas faltantes'),
  (3, 'Sobrestock', 'Exceso de inventario respecto a la demanda'),
  (4, 'Error de pedido', 'Producto o cantidad distinta a la solicitada al proveedor');

-- ---- tipo_evento -----------------------------------------------------------
CREATE TABLE `tipo_evento` (
  `id_tipo_evento` INT          NOT NULL AUTO_INCREMENT,
  `tipo_evento`    VARCHAR(200) NOT NULL,
  PRIMARY KEY (`id_tipo_evento`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `tipo_evento` (`id_tipo_evento`, `tipo_evento`) VALUES
  (1, 'SISTEMA'),
  (2, 'AUTENTICACION');

-- ============================================================================
-- 2. USUARIOS
-- ============================================================================
CREATE TABLE `usuarios` (
  `id_usuarios`      BIGINT       NOT NULL,
  `id_roles`         INT          NOT NULL,
  `nombre`           VARCHAR(50)  NOT NULL,
  `apellido_paterno` VARCHAR(30)  DEFAULT NULL,
  `apellido_materno` VARCHAR(30)  DEFAULT NULL,
  `telefono`         BIGINT       DEFAULT NULL,
  `contrasena`       VARCHAR(255) NOT NULL,
  `estado`           BOOLEAN      NOT NULL DEFAULT 1,
  PRIMARY KEY (`id_usuarios`),
  KEY `FK_USUARIOS_ROLES` (`id_roles`),
  CONSTRAINT `FK_USUARIOS_ROLES` FOREIGN KEY (`id_roles`) REFERENCES `roles` (`id_roles`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `usuarios` (`id_usuarios`, `id_roles`, `nombre`, `apellido_paterno`, `apellido_materno`, `telefono`, `contrasena`, `estado`) VALUES
  (1001,      4, 'Valeria', 'Morales', 'Gomez',   8441112233, '$2b$10$GYRtA19hDIFbxL/ktJsMluzBL6RtxZu4KFSIOClBq14aG0Mvyh2zy', 1),
  (350976899, 1, 'Marco',   'Perez',   'Luevano', 8444578906, '$2b$10$JEis.kKv578RybJ7CPTKzOnzwJMf7iUbQWZwnpgSPmii0Rv8QLpyC', 1),
  (628777129, 3, 'Aldo',    'Soto',    'Velez',   8442836877, '$2b$10$9ZdsD93wmahmAOvWoEJ5SeI3yDWTZMNzlutEVfiiNbj4ZTgLPNUZy', 1),
  (628777130, 2, 'Ian',     'Herrera', 'Ramirez', 8449970912, '$2b$10$okBEvf28NTEBI3eA/jrO3euCYnsqKpk4w2T0eOdpNFucX4Y6pSUPq', 1);

-- ============================================================================
-- 3. PRODUCTOS (supertipo) -> LIBROS / REVISTAS (subtipos)
-- ============================================================================
CREATE TABLE `productos` (
  `codigo_ean`    BIGINT        NOT NULL,
  `id_proveedor`  INT           NOT NULL,
  `tipo_producto` VARCHAR(10)   NOT NULL,           -- 'Libro' | 'Revista'
  `SKU`           BIGINT        DEFAULT NULL,
  `precio`        DECIMAL(10,2) NOT NULL,
  PRIMARY KEY (`codigo_ean`),
  UNIQUE KEY `SKU_UNIQUE` (`SKU`),
  KEY `FK_PRODUCTOS_PROVEEDOR` (`id_proveedor`),
  CONSTRAINT `FK_PRODUCTOS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedor` (`id_proveedor`) ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `productos` (`codigo_ean`, `id_proveedor`, `tipo_producto`, `SKU`, `precio`) VALUES
  (9786070775390, 1, 'Libro',   884102,  249.00),
  (9786073832014, 1, 'Libro',   10001,   350.00),
  (9786073868723, 3, 'Libro',   1321371, 299.00),
  (9786078852895, 2, 'Libro',   750912,  260.00),
  (9788497776639, 1, 'Libro',   9777668, 199.00),
  (9771234567010, 2, 'Revista', 20001,   120.00),
  (9772007490001, 4, 'Revista', 901001,  65.00);

CREATE TABLE `libros` (
  `codigo_ean`   BIGINT       NOT NULL,
  `nombre_libro` VARCHAR(150) NOT NULL,
  PRIMARY KEY (`codigo_ean`),
  CONSTRAINT `FK_LIBROS_PRODUCTO` FOREIGN KEY (`codigo_ean`) REFERENCES `productos` (`codigo_ean`) ON UPDATE CASCADE ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `libros` (`codigo_ean`, `nombre_libro`) VALUES
  (9786070775390, 'El Hombre en Busca de Sentido'),
  (9786073832014, 'Cien Años de Soledad'),
  (9786073868723, 'Un Himno a la Vida'),
  (9786078852895, 'Momo'),
  (9788497776639, 'El Regreso del Caballero de la Armadura Oxidada');

CREATE TABLE `revistas` (
  `codigo_ean`     BIGINT       NOT NULL,
  `id_editorial`   INT          NOT NULL,
  `nombre_revista` VARCHAR(200) NOT NULL,
  `numero_edicion` INT          DEFAULT NULL,
  `periodicidad`   VARCHAR(100) DEFAULT NULL,
  PRIMARY KEY (`codigo_ean`),
  KEY `FK_REVISTAS_EDITORIAL` (`id_editorial`),
  CONSTRAINT `FK_REVISTAS_PRODUCTO`  FOREIGN KEY (`codigo_ean`)   REFERENCES `productos` (`codigo_ean`)   ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT `FK_REVISTAS_EDITORIAL` FOREIGN KEY (`id_editorial`) REFERENCES `editorial` (`id_editorial`) ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `revistas` (`codigo_ean`, `id_editorial`, `nombre_revista`, `numero_edicion`, `periodicidad`) VALUES
  (9771234567010, 1, 'National Geographic Mar 2026',       345, 'Mensual'),
  (9772007490001, 2, 'Muy Interesante Mexico Ed. 452',     452, 'Mensual');

-- ---- Autores_has_Libros (N:M) ----------------------------------------------
CREATE TABLE `autores_has_libros` (
  `codigo_ean` BIGINT NOT NULL,
  `id_autores` INT    NOT NULL,
  PRIMARY KEY (`codigo_ean`, `id_autores`),
  KEY `FK_AHL_AUTOR` (`id_autores`),
  CONSTRAINT `FK_AHL_LIBRO` FOREIGN KEY (`codigo_ean`) REFERENCES `libros` (`codigo_ean`)   ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT `FK_AHL_AUTOR` FOREIGN KEY (`id_autores`) REFERENCES `autores` (`id_autores`) ON UPDATE CASCADE ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `autores_has_libros` (`codigo_ean`, `id_autores`) VALUES
  (9786070775390, 1), -- El Hombre en Busca de Sentido  - Viktor Frankl
  (9786073832014, 2), -- Cien Años de Soledad           - Gabriel García Márquez
  (9786073868723, 3), -- Un Himno a la Vida             - Gisele Pelicot
  (9786078852895, 4), -- Momo                           - Michael Ende
  (9788497776639, 5); -- El Regreso del Caballero...    - Robert Fisher

-- ---- Productos_has_Genero (N:M, libros y revistas) -------------------------
CREATE TABLE `productos_has_genero` (
  `codigo_ean` BIGINT NOT NULL,
  `id_genero`  INT    NOT NULL,
  PRIMARY KEY (`codigo_ean`, `id_genero`),
  KEY `FK_PHG_GENERO` (`id_genero`),
  CONSTRAINT `FK_PHG_PRODUCTO` FOREIGN KEY (`codigo_ean`) REFERENCES `productos` (`codigo_ean`) ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT `FK_PHG_GENERO`   FOREIGN KEY (`id_genero`)  REFERENCES `genero` (`id_genero`)     ON UPDATE CASCADE ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `productos_has_genero` (`codigo_ean`, `id_genero`) VALUES
  -- Libros
  (9786070775390, 11), -- Psicologia
  (9786070775390, 17), -- Ensayo
  (9786073832014, 16), -- Novela Contemporanea
  (9786073868723, 12), -- Autoayuda
  (9786078852895, 61), -- Infantiles
  (9786078852895, 62), -- Juvenil
  (9788497776639, 12), -- Autoayuda
  (9788497776639, 13), -- Metafisica
  -- Revistas
  (9771234567010, 15), -- Historia
  (9771234567010, 18), -- Divulgacion Cientifica
  (9772007490001, 18); -- Divulgacion Cientifica

-- ============================================================================
-- 4. EXISTENCIAS (inventario por mueble)  y MOVIMIENTOS
-- ============================================================================
CREATE TABLE `existencias` (
  `codigo_ean` BIGINT NOT NULL,
  `id_mueble`  INT    NOT NULL,
  `cantidad`   INT    NOT NULL DEFAULT 0,
  PRIMARY KEY (`codigo_ean`, `id_mueble`),
  KEY `FK_EXISTENCIAS_MUEBLE` (`id_mueble`),
  CONSTRAINT `FK_EXISTENCIAS_PRODUCTO` FOREIGN KEY (`codigo_ean`) REFERENCES `productos` (`codigo_ean`) ON UPDATE CASCADE,
  CONSTRAINT `FK_EXISTENCIAS_MUEBLE`   FOREIGN KEY (`id_mueble`)  REFERENCES `mueble` (`id_mueble`)     ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `existencias` (`codigo_ean`, `id_mueble`, `cantidad`) VALUES
  -- El Hombre en Busca de Sentido (solo bodega)
  (9786070775390, 111, 40),
  -- Cien Años de Soledad
  (9786073832014, 11,  10),
  (9786073832014, 111, 45),
  -- Un Himno a la Vida
  (9786073868723, 12,  10),
  (9786073868723, 112, 50),
  -- Momo
  (9786078852895, 61,   5),
  (9786078852895, 161, 30),
  -- El Regreso del Caballero de la Armadura Oxidada
  (9788497776639, 13,   8),
  (9788497776639, 113, 25),
  -- National Geographic Mar 2026
  (9771234567010, 11,  10),
  (9771234567010, 111, 35),
  -- Muy Interesante Mexico Ed. 452
  (9772007490001, 11,  15),
  (9772007490001, 111, 40);

CREATE TABLE `movimientos` (
  `id_movimientos` BIGINT       NOT NULL AUTO_INCREMENT,
  `id_usuarios`    BIGINT       NOT NULL,
  `codigo_ean`     BIGINT       NOT NULL,
  `mueble_origen`  INT          NOT NULL,
  `mueble_destino` INT          NOT NULL,
  `cantidad`       INT          NOT NULL,
  `fecha_hora`     DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `observaciones`  VARCHAR(200) DEFAULT NULL,
  PRIMARY KEY (`id_movimientos`),
  KEY `FK_MOV_USUARIO` (`id_usuarios`),
  KEY `FK_MOV_PRODUCTO` (`codigo_ean`),
  KEY `FK_MOV_ORIGEN` (`mueble_origen`),
  KEY `FK_MOV_DESTINO` (`mueble_destino`),
  CONSTRAINT `FK_MOV_USUARIO`  FOREIGN KEY (`id_usuarios`)    REFERENCES `usuarios` (`id_usuarios`),
  CONSTRAINT `FK_MOV_PRODUCTO` FOREIGN KEY (`codigo_ean`)     REFERENCES `productos` (`codigo_ean`),
  CONSTRAINT `FK_MOV_ORIGEN`   FOREIGN KEY (`mueble_origen`)  REFERENCES `mueble` (`id_mueble`),
  CONSTRAINT `FK_MOV_DESTINO`  FOREIGN KEY (`mueble_destino`) REFERENCES `mueble` (`id_mueble`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- (sin datos: el backup y los seeds no traían movimientos)

-- ============================================================================
-- 5. BITÁCORA
-- ============================================================================
CREATE TABLE `bitacora` (
  `id_bitacora`    BIGINT       NOT NULL AUTO_INCREMENT,
  `id_usuarios`    BIGINT       NOT NULL,
  `id_tipo_evento` INT          NOT NULL,
  `fecha_hora`     DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `accion`         VARCHAR(100) NOT NULL,
  `detalle`        VARCHAR(255) NOT NULL,
  PRIMARY KEY (`id_bitacora`),
  KEY `FK_BITACORA_USUARIO` (`id_usuarios`),
  KEY `FK_BITACORA_TIPO` (`id_tipo_evento`),
  CONSTRAINT `FK_BITACORA_USUARIO` FOREIGN KEY (`id_usuarios`)    REFERENCES `usuarios` (`id_usuarios`),
  CONSTRAINT `FK_BITACORA_TIPO`    FOREIGN KEY (`id_tipo_evento`) REFERENCES `tipo_evento` (`id_tipo_evento`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `bitacora` (`id_bitacora`, `id_usuarios`, `id_tipo_evento`, `fecha_hora`, `accion`, `detalle`) VALUES
  (1, 1001, 1, '2026-10-07 05:31:47', 'SISTEMA_INICIALIZACION', 'Carga de esquema V2 con relaciones N:M, Autores, Proveedores y Compras'),
  (2, 1001, 2, '2026-10-07 05:33:02', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: gerente'),
  (3, 350976899, 2, '2026-10-07 05:33:12', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: jefe_departamento'),
  (4, 350976899, 2, '2026-10-07 05:33:49', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: jefe_departamento'),
  (5, 628777130, 2, '2026-10-07 05:33:59', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (6, 628777130, 2, '2026-10-07 05:34:42', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (7, 628777129, 2, '2026-10-07 05:34:50', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: vendedor'),
  (8, 628777130, 2, '2026-10-07 05:40:27', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (9, 628777130, 2, '2026-10-07 06:01:10', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (10, 628777130, 2, '2026-10-07 06:14:24', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (11, 628777129, 2, '2026-10-07 06:14:38', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: vendedor'),
  (12, 350976899, 2, '2026-10-07 06:14:54', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: jefe_departamento'),
  (13, 350976899, 2, '2026-10-07 06:22:05', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: jefe_departamento'),
  (14, 628777129, 2, '2026-10-07 06:23:44', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: vendedor'),
  (15, 628777130, 2, '2026-10-07 06:27:23', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (16, 628777129, 2, '2026-10-07 06:31:53', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: vendedor'),
  (17, 628777130, 2, '2026-10-07 06:33:57', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (18, 628777130, 2, '2026-10-07 07:12:34', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (19, 628777129, 2, '2026-10-07 07:13:16', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: vendedor'),
  (20, 628777130, 2, '2026-10-07 07:13:42', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (21, 628777130, 2, '2026-10-07 07:19:49', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: personal_bodega'),
  (22, 350976899, 2, '2026-10-19 20:40:52', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: jefe_departamento'),
  (23, 1001, 2, '2026-10-21 06:45:46', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: gerente'),
  (24, 350976899, 2, '2026-10-21 06:46:37', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: jefe_departamento'),
  (25, 1001, 2, '2026-10-23 02:02:06', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: gerente'),
  (26, 1001, 2, '2026-10-23 02:02:46', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: gerente'),
  (27, 1001, 2, '2026-10-23 02:11:08', 'INICIO DE SESIÓN', 'Inicio de sesión exitoso con rol: gerente');

-- ============================================================================
-- 6. COMPRAS A PROVEEDOR
-- ============================================================================
CREATE TABLE `compras_proveedor` (
  `id_compras_proveedor` BIGINT        NOT NULL AUTO_INCREMENT,
  `id_proveedor`         INT           NOT NULL,
  `id_usuarios`          BIGINT        NOT NULL,
  `fecha_hora`           DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `total`                DECIMAL(10,2) NOT NULL DEFAULT 0.00,
  `factura`              VARCHAR(30)   DEFAULT NULL,
  PRIMARY KEY (`id_compras_proveedor`),
  KEY `FK_COMPRAS_PROVEEDOR` (`id_proveedor`),
  KEY `FK_COMPRAS_USUARIO` (`id_usuarios`),
  CONSTRAINT `FK_COMPRAS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedor` (`id_proveedor`),
  CONSTRAINT `FK_COMPRAS_USUARIO`   FOREIGN KEY (`id_usuarios`)  REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `compras_proveedor` (`id_compras_proveedor`, `id_proveedor`, `id_usuarios`, `fecha_hora`, `total`, `factura`) VALUES
  (1, 1, 628777130, '2026-10-05 05:31:48', 7000.00, 'F-4029'),
  (2, 3, 628777130, '2026-10-06 05:31:48', 5980.00, NULL);

CREATE TABLE `detalles_compras` (
  `id_detalles_c`        BIGINT        NOT NULL AUTO_INCREMENT,
  `id_compras_proveedor` BIGINT        NOT NULL,
  `codigo_ean`           BIGINT        NOT NULL,
  `cantidad`             INT           NOT NULL,
  `subtotal`             DECIMAL(10,2) NOT NULL,
  PRIMARY KEY (`id_detalles_c`),
  KEY `FK_DC_COMPRA` (`id_compras_proveedor`),
  KEY `FK_DC_PRODUCTO` (`codigo_ean`),
  CONSTRAINT `FK_DC_COMPRA`   FOREIGN KEY (`id_compras_proveedor`) REFERENCES `compras_proveedor` (`id_compras_proveedor`) ON DELETE CASCADE,
  CONSTRAINT `FK_DC_PRODUCTO` FOREIGN KEY (`codigo_ean`)           REFERENCES `productos` (`codigo_ean`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `detalles_compras` (`id_detalles_c`, `id_compras_proveedor`, `codigo_ean`, `cantidad`, `subtotal`) VALUES
  (1, 1, 9786073832014, 20, 4200.00),  -- costo unitario 210.00
  (2, 1, 9788497776639, 20, 2800.00),  -- costo unitario 140.00
  (3, 2, 9786073868723, 20, 3980.00),  -- costo unitario 199.00
  (4, 2, 9771234567010, 25, 2000.00);  -- costo unitario  80.00

-- ============================================================================
-- 7. VENTAS
-- ============================================================================
CREATE TABLE `ventas` (
  `id_ventas`      BIGINT        NOT NULL AUTO_INCREMENT,
  `id_usuarios`    BIGINT        NOT NULL,
  `fecha_hora`     DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `total`          DECIMAL(10,2) NOT NULL DEFAULT 0.00,
  `nombre_cliente` VARCHAR(200)  NOT NULL DEFAULT 'Público en General',
  `factura`        VARCHAR(30)   DEFAULT NULL,
  `metodo_pago`    VARCHAR(20)   NOT NULL DEFAULT 'EFECTIVO',
  PRIMARY KEY (`id_ventas`),
  KEY `FK_VENTAS_USUARIOS` (`id_usuarios`),
  CONSTRAINT `FK_VENTAS_USUARIOS` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `detalle_ventas` (
  `id_detalle`      BIGINT        NOT NULL AUTO_INCREMENT,
  `id_ventas`       BIGINT        NOT NULL,
  `codigo_ean`      BIGINT        NOT NULL,
  `cantidad`        INT           NOT NULL,
  `precio_unitario` DECIMAL(10,2) NOT NULL,
  `subtotal`        DECIMAL(10,2) NOT NULL,
  PRIMARY KEY (`id_detalle`),
  KEY `FK_DV_VENTA` (`id_ventas`),
  KEY `FK_DV_PRODUCTO` (`codigo_ean`),
  CONSTRAINT `FK_DV_VENTA`    FOREIGN KEY (`id_ventas`)  REFERENCES `ventas` (`id_ventas`) ON DELETE CASCADE,
  CONSTRAINT `FK_DV_PRODUCTO` FOREIGN KEY (`codigo_ean`) REFERENCES `productos` (`codigo_ean`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- Ventas de ejemplo (seeds.sql): clientes con N libros y N revistas
INSERT INTO `ventas` (`id_ventas`, `id_usuarios`, `fecha_hora`, `total`, `nombre_cliente`) VALUES
  (1, 628777129, DATE_SUB(NOW(), INTERVAL 1 DAY), 1060.00, 'Ing. Fernando Peña'),
  (2, 628777129, NOW(),                           484.00,  'Dra. Carmen Salinas');

INSERT INTO `detalle_ventas` (`id_detalle`, `id_ventas`, `codigo_ean`, `cantidad`, `precio_unitario`, `subtotal`) VALUES
  -- Venta #1: 2 libros (Cien Años de Soledad) + 3 revistas (National Geographic)
  (1, 1, 9786073832014, 2, 350.00, 700.00),
  (2, 1, 9771234567010, 3, 120.00, 360.00),
  -- Venta #2: 1 libro (Un Himno a la Vida) + 1 NatGeo + 1 Muy Interesante
  (3, 2, 9786073868723, 1, 299.00, 299.00),
  (4, 2, 9771234567010, 1, 120.00, 120.00),
  (5, 2, 9772007490001, 1,  65.00,  65.00);

-- ============================================================================
-- 8. DEVOLUCIONES
-- ============================================================================
CREATE TABLE `devolucion` (
  `id_devolucion`      BIGINT      NOT NULL AUTO_INCREMENT,
  `id_proveedor`       INT         NOT NULL,
  `id_usuarios`        BIGINT      NOT NULL,
  `id_usuario_aprueba` BIGINT      DEFAULT NULL,
  `fecha_solicitud`    DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `total_piezas`       INT         NOT NULL,
  `estado`             VARCHAR(20) NOT NULL DEFAULT 'PENDIENTE',
  `fecha_aprobacion`   DATETIME    DEFAULT NULL,
  PRIMARY KEY (`id_devolucion`),
  KEY `FK_DEVOLUCION_PROVEEDOR` (`id_proveedor`),
  KEY `FK_DEVOLUCION_USUARIO` (`id_usuarios`),
  KEY `FK_DEVOLUCION_APRUEBA` (`id_usuario_aprueba`),
  CONSTRAINT `FK_DEVOLUCION_PROVEEDOR` FOREIGN KEY (`id_proveedor`)       REFERENCES `proveedor` (`id_proveedor`),
  CONSTRAINT `FK_DEVOLUCION_USUARIO`   FOREIGN KEY (`id_usuarios`)        REFERENCES `usuarios` (`id_usuarios`),
  CONSTRAINT `FK_DEVOLUCION_APRUEBA`   FOREIGN KEY (`id_usuario_aprueba`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- `autorizado_por` ('Marco Perez Luevano') pasa a ser el usuario 350976899.
INSERT INTO `devolucion` (`id_devolucion`, `id_proveedor`, `id_usuarios`, `id_usuario_aprueba`, `fecha_solicitud`, `total_piezas`, `estado`, `fecha_aprobacion`) VALUES
  (1, 1, 628777129, NULL,       '2026-10-07 00:00:00', 3, 'PENDIENTE', NULL),
  (2, 3, 628777129, 350976899,  '2026-10-07 00:00:00', 5, 'APROBADO',  '2026-10-07 00:00:00');

CREATE TABLE `detalle_devolucion` (
  `id_detalle_dev` BIGINT NOT NULL AUTO_INCREMENT,
  `id_devolucion`  BIGINT NOT NULL,
  `codigo_ean`     BIGINT NOT NULL,
  `id_motivo`      INT    NOT NULL,
  `cantidad`       INT    NOT NULL,
  PRIMARY KEY (`id_detalle_dev`),
  KEY `FK_DD_DEVOLUCION` (`id_devolucion`),
  KEY `FK_DD_PRODUCTO` (`codigo_ean`),
  KEY `FK_DD_MOTIVO` (`id_motivo`),
  CONSTRAINT `FK_DD_DEVOLUCION` FOREIGN KEY (`id_devolucion`) REFERENCES `devolucion` (`id_devolucion`) ON DELETE CASCADE,
  CONSTRAINT `FK_DD_PRODUCTO`   FOREIGN KEY (`codigo_ean`)    REFERENCES `productos` (`codigo_ean`),
  CONSTRAINT `FK_DD_MOTIVO`     FOREIGN KEY (`id_motivo`)     REFERENCES `motivo` (`id_motivo`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- (sin datos: el backup y los seeds no traían detalle de las devoluciones 1 y 2)

-- ============================================================================
-- 9. VISTA: productos comprados por cliente (libros y revistas)
-- ============================================================================
CREATE OR REPLACE VIEW `vista_cliente_compras` AS
SELECT
  v.nombre_cliente,
  v.id_ventas,
  v.fecha_hora AS fecha_compra,
  v.total      AS total_venta,
  v.metodo_pago,
  dv.id_detalle,
  p.tipo_producto,
  dv.codigo_ean,
  COALESCE(l.nombre_libro, r.nombre_revista, 'Producto Desconocido') AS nombre_producto,
  dv.cantidad,
  dv.precio_unitario,
  dv.subtotal
FROM ventas v
JOIN detalle_ventas dv ON dv.id_ventas  = v.id_ventas
JOIN productos p       ON p.codigo_ean  = dv.codigo_ean
LEFT JOIN libros l     ON l.codigo_ean  = dv.codigo_ean
LEFT JOIN revistas r   ON r.codigo_ean  = dv.codigo_ean;

SET FOREIGN_KEY_CHECKS = 1;

-- ============================================================================
-- Verificación rápida (opcional)
-- ============================================================================
-- SELECT 'productos' AS tabla, COUNT(*) AS filas FROM productos
-- UNION ALL SELECT 'libros', COUNT(*) FROM libros
-- UNION ALL SELECT 'revistas', COUNT(*) FROM revistas
-- UNION ALL SELECT 'existencias', COUNT(*) FROM existencias
-- UNION ALL SELECT 'bitacora', COUNT(*) FROM bitacora;
