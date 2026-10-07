-- ============================================================================
-- SCRIPT DE MIGRACIÓN NOVABOOK V2
-- Actualización de esquema: Géneros (N:M), Autores, Proveedores, Ventas y Compras
-- ============================================================================

USE `Libreria`;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------------------------------------------------------
-- 1. Actualización de tabla Género y tablas intermedias libro_genero / revista_genero
-- ----------------------------------------------------------------------------
ALTER TABLE `genero` MODIFY COLUMN `id_genero` INT NOT NULL AUTO_INCREMENT;

INSERT IGNORE INTO `genero` (`id_genero`, `genero_literario`) VALUES
  (14, 'Ciencia Ficción'),
  (15, 'Historia'),
  (16, 'Novela Contemporánea'),
  (17, 'Ensayo'),
  (18, 'Divulgación Científica');

CREATE TABLE IF NOT EXISTS `libro_genero` (
  `codigo_ean` BIGINT NOT NULL,
  `id_genero` INT NOT NULL,
  PRIMARY KEY (`codigo_ean`, `id_genero`),
  KEY `FK_LG_GENERO` (`id_genero`),
  CONSTRAINT `FK_LG_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`) ON UPDATE CASCADE ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `revista_genero` (
  `codigo_ean` BIGINT NOT NULL,
  `id_genero` INT NOT NULL,
  PRIMARY KEY (`codigo_ean`, `id_genero`),
  KEY `FK_RG_GENERO` (`id_genero`),
  CONSTRAINT `FK_RG_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`) ON UPDATE CASCADE ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- Migrar géneros actuales de libros antes de eliminar la columna
INSERT IGNORE INTO `libro_genero` (`codigo_ean`, `id_genero`)
SELECT DISTINCT `codigo_ean`, `id_genero` FROM `libros` WHERE `id_genero` IS NOT NULL;

-- Asignar géneros semilla a revistas
INSERT IGNORE INTO `revista_genero` (`codigo_ean`, `id_genero`) VALUES
  (9771234567010, 18), -- Divulgación Científica
  (9771234567010, 15); -- Historia

-- ----------------------------------------------------------------------------
-- 2. Creación de tabla autores y tabla intermedia libro_autor
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS `autores` (
  `id_autor` INT NOT NULL AUTO_INCREMENT,
  `nombre` VARCHAR(100) NOT NULL,
  `apellidos` VARCHAR(100) NOT NULL,
  `nacionalidad` VARCHAR(100) DEFAULT NULL,
  `biografia` TEXT DEFAULT NULL,
  PRIMARY KEY (`id_autor`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `libro_autor` (
  `codigo_ean` BIGINT NOT NULL,
  `id_autor` INT NOT NULL,
  PRIMARY KEY (`codigo_ean`, `id_autor`),
  KEY `FK_LA_AUTOR` (`id_autor`),
  CONSTRAINT `FK_LA_AUTOR` FOREIGN KEY (`id_autor`) REFERENCES `autores` (`id_autor`) ON UPDATE CASCADE ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- Insertar autores iniciales
INSERT INTO `autores` (`id_autor`, `nombre`, `apellidos`, `nacionalidad`, `biografia`) VALUES
  (1, 'Viktor', 'Frankl', 'Austriaco', 'Neurólogo, psiquiatra y fundador de la Logoterapia.'),
  (2, 'Gabriel', 'García Márquez', 'Colombiano', 'Premio Nobel de Literatura 1982, autor de Cien Años de Soledad.'),
  (3, 'Gisele', 'Pelicot', 'Francesa', 'Autora y activista francesa.'),
  (4, 'Michael', 'Ende', 'Alemán', 'Escritor de literatura fantástica y juvenil.'),
  (5, 'Robert', 'Fisher', 'Estadounidense', 'Escritor y dramaturgo, autor de El Caballero de la Armadura Oxidada.')
ON DUPLICATE KEY UPDATE `nombre` = VALUES(`nombre`);

-- Asociar libros existentes con sus autores
INSERT IGNORE INTO `libro_autor` (`codigo_ean`, `id_autor`) VALUES
  (9786070775390, 1), -- Viktor Frankl
  (9786073832014, 2), -- Gabriel García Márquez
  (9786073868723, 3), -- Gisele Pelicot
  (9786078852895, 4), -- Michael Ende
  (9788497776639, 5); -- Robert Fisher

-- ----------------------------------------------------------------------------
-- 3. Actualización de estructura de tabla libros (eliminar autor e id_genero)
-- ----------------------------------------------------------------------------
-- Eliminar FK si existe
SET @drop_fk_genero = (
  SELECT IF(COUNT(*) > 0, 'ALTER TABLE libros DROP FOREIGN KEY FK_LIBROS_GENERO;', 'SELECT 1;')
  FROM information_schema.TABLE_CONSTRAINTS
  WHERE CONSTRAINT_SCHEMA = 'Libreria' AND TABLE_NAME = 'libros' AND CONSTRAINT_NAME = 'FK_LIBROS_GENERO'
);
PREPARE stmt FROM @drop_fk_genero;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Eliminar columnas de libros si existen
SET @drop_col_genero = (
  SELECT IF(COUNT(*) > 0, 'ALTER TABLE libros DROP COLUMN id_genero;', 'SELECT 1;')
  FROM information_schema.COLUMNS
  WHERE TABLE_SCHEMA = 'Libreria' AND TABLE_NAME = 'libros' AND COLUMN_NAME = 'id_genero'
);
PREPARE stmt FROM @drop_col_genero;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @drop_col_autor = (
  SELECT IF(COUNT(*) > 0, 'ALTER TABLE libros DROP COLUMN autor;', 'SELECT 1;')
  FROM information_schema.COLUMNS
  WHERE TABLE_SCHEMA = 'Libreria' AND TABLE_NAME = 'libros' AND COLUMN_NAME = 'autor'
);
PREPARE stmt FROM @drop_col_autor;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- ----------------------------------------------------------------------------
-- 4. Actualización / Creación de tabla proveedores
-- ----------------------------------------------------------------------------
-- Adaptar o renombrar tabla proveedor
CREATE TABLE IF NOT EXISTS `proveedores` (
  `id_proveedor` INT NOT NULL AUTO_INCREMENT,
  `nombre_proveedor` VARCHAR(150) NOT NULL,
  `rfc` VARCHAR(20) NOT NULL,
  `telefono` VARCHAR(25) DEFAULT NULL,
  `correo` VARCHAR(100) DEFAULT NULL,
  `direccion` VARCHAR(255) DEFAULT NULL,
  `persona_contacto` VARCHAR(100) DEFAULT NULL,
  `estatus` VARCHAR(20) NOT NULL DEFAULT 'ACTIVO',
  PRIMARY KEY (`id_proveedor`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- Si existe la tabla anterior proveedor, migrar sus datos a proveedores
INSERT INTO `proveedores` (`id_proveedor`, `nombre_proveedor`, `rfc`, `telefono`, `correo`, `direccion`, `persona_contacto`, `estatus`)
SELECT `id_proveedor`, `nombre_proveedor`, 'PME850101XYZ', '5551234567', 'contacto@proveedor.com', 'Av. Insurgentes Sur 1200, CDMX', 'Lic. Roberto Morales', 'ACTIVO'
FROM `proveedor`
ON DUPLICATE KEY UPDATE `nombre_proveedor` = VALUES(`nombre_proveedor`);

-- Actualizar datos reales y completos para cada proveedor inicial
UPDATE `proveedores` SET
  `rfc` = 'PME850101AB1',
  `telefono` = '5553331100',
  `correo` = 'contacto@planetamexico.com.mx',
  `direccion` = 'Av. Presidente Masaryk 111, Polanco, CDMX',
  `persona_contacto` = 'Lic. Carlos Fuentes Rivera',
  `estatus` = 'ACTIVO'
WHERE `id_proveedor` = 1;

UPDATE `proveedores` SET
  `rfc` = 'PMI920415CD2',
  `telefono` = '5553331122',
  `correo` = 'infantil@planetamexico.com.mx',
  `direccion` = 'Av. Presidente Masaryk 115, Polanco, CDMX',
  `persona_contacto` = 'Mtra. Elena Poniatowska Gil',
  `estatus` = 'ACTIVO'
WHERE `id_proveedor` = 2;

UPDATE `proveedores` SET
  `rfc` = 'PRH010920EF3',
  `telefono` = '5554442200',
  `correo` = 'ventas@penguinrandomhouse.mx',
  `direccion` = 'Blvd. Miguel de Cervantes Saavedra 301, Granada, CDMX',
  `persona_contacto` = 'Lic. Javier Velasco Ruiz',
  `estatus` = 'ACTIVO'
WHERE `id_proveedor` = 3;

UPDATE `proveedores` SET
  `rfc` = 'ETE730508GH4',
  `telefono` = '5552612000',
  `correo` = 'distribucion@editorialtelevisa.com.mx',
  `direccion` = 'Av. Vasco de Quiroga 2000, Santa Fe, CDMX',
  `persona_contacto` = 'Lic. Martha Debayle Gallardo',
  `estatus` = 'ACTIVO'
WHERE `id_proveedor` = 4;

-- Asegurar vista `proveedor` para retrocompatibilidad
DROP TABLE IF EXISTS `proveedor`;
CREATE OR REPLACE VIEW `proveedor` AS SELECT * FROM `proveedores`;

-- ----------------------------------------------------------------------------
-- 5. Agregar nombre_cliente a la tabla ventas
-- ----------------------------------------------------------------------------
SET @add_col_cliente = (
  SELECT IF(COUNT(*) = 0, 'ALTER TABLE ventas ADD COLUMN nombre_cliente VARCHAR(150) NOT NULL DEFAULT \'Público en General\' AFTER id_usuarios;', 'SELECT 1;')
  FROM information_schema.COLUMNS
  WHERE TABLE_SCHEMA = 'Libreria' AND TABLE_NAME = 'ventas' AND COLUMN_NAME = 'nombre_cliente'
);
PREPARE stmt FROM @add_col_cliente;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- ----------------------------------------------------------------------------
-- 6. Crear tablas para registro de Compras a Proveedor
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS `compras_proveedor` (
  `id_compra` BIGINT NOT NULL AUTO_INCREMENT,
  `id_proveedor` INT NOT NULL,
  `id_usuarios` BIGINT NOT NULL,
  `fecha_hora` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `total` DOUBLE NOT NULL DEFAULT 0.0,
  `observaciones` VARCHAR(255) DEFAULT NULL,
  PRIMARY KEY (`id_compra`),
  KEY `FK_COMPRAS_PROVEEDOR` (`id_proveedor`),
  KEY `FK_COMPRAS_USUARIO` (`id_usuarios`),
  CONSTRAINT `FK_COMPRAS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedores` (`id_proveedor`),
  CONSTRAINT `FK_COMPRAS_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `detalle_compra` (
  `id_detalle_compra` BIGINT NOT NULL AUTO_INCREMENT,
  `id_compra` BIGINT NOT NULL,
  `codigo_ean` BIGINT NOT NULL,
  `tipo_producto` VARCHAR(20) NOT NULL,
  `cantidad` INT NOT NULL,
  `costo_unitario` DOUBLE NOT NULL,
  `subtotal` DOUBLE NOT NULL,
  PRIMARY KEY (`id_detalle_compra`),
  KEY `FK_DETALLE_COMPRA` (`id_compra`),
  CONSTRAINT `FK_DETALLE_COMPRA` FOREIGN KEY (`id_compra`) REFERENCES `compras_proveedor` (`id_compra`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- Datos semilla para compras a proveedor
INSERT INTO `compras_proveedor` (`id_compra`, `id_proveedor`, `id_usuarios`, `fecha_hora`, `total`, `observaciones`) VALUES
  (1, 1, 628777130, DATE_SUB(NOW(), INTERVAL 2 DAY), 7000.00, 'Recepción factura F-4029'),
  (2, 3, 628777130, DATE_SUB(NOW(), INTERVAL 1 DAY), 5980.00, 'Pedido quincenal editorial Penguin')
ON DUPLICATE KEY UPDATE `total` = VALUES(`total`);

INSERT INTO `detalle_compra` (`id_detalle_compra`, `id_compra`, `codigo_ean`, `tipo_producto`, `cantidad`, `costo_unitario`, `subtotal`) VALUES
  (1, 1, 9786073832014, 'Libro', 20, 210.00, 4200.00),
  (2, 1, 9788497776639, 'Libro', 20, 140.00, 2800.00),
  (3, 2, 9786073868723, 'Libro', 20, 199.00, 3980.00),
  (4, 2, 9771234567010, 'Revista', 25, 80.00, 2000.00)
ON DUPLICATE KEY UPDATE `subtotal` = VALUES(`subtotal`);

SET FOREIGN_KEY_CHECKS = 1;
