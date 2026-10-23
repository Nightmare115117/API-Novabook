-- ============================================================================
-- SCRIPT DE CREACIÓN Y DATOS SEMILLA (SEEDS V2) - LIBRERÍA NOVABOOK
-- ============================================================================
-- Base de Datos: Libreria
-- Versión: 2.0 (Relación N:M Géneros, CRUD Autores, Proveedores, Ventas, Compras, Muebles y Motivos)
-- Compatible con: MySQL 8.0+ / MariaDB 10.5+
-- ============================================================================

CREATE DATABASE IF NOT EXISTS `Libreria` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
USE `Libreria`;

SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------------------------------------------------------
-- 0. Limpieza previa de tablas y vistas (en orden inverso de dependencia)
-- ----------------------------------------------------------------------------
DROP VIEW IF EXISTS `vista_cliente_compras`;
DROP VIEW IF EXISTS `proveedor`;
DROP TABLE IF EXISTS `detalle_compra`;
DROP TABLE IF EXISTS `compras_proveedor`;
DROP TABLE IF EXISTS `detalle_ventas`;
DROP TABLE IF EXISTS `ventas`;
DROP TABLE IF EXISTS `clientes`;
DROP TABLE IF EXISTS `movimientos`;
DROP TABLE IF EXISTS `bitacora`;
DROP TABLE IF EXISTS `devolucion`;
DROP TABLE IF EXISTS `revista_genero`;
DROP TABLE IF EXISTS `libro_autor`;
DROP TABLE IF EXISTS `libro_genero`;
DROP TABLE IF EXISTS `revistas`;
DROP TABLE IF EXISTS `libros`;
DROP TABLE IF EXISTS `usuarios`;
DROP TABLE IF EXISTS `motivos`;
DROP TABLE IF EXISTS `mueble`;
DROP TABLE IF EXISTS `autores`;
DROP TABLE IF EXISTS `genero`;
DROP TABLE IF EXISTS `proveedores`;
DROP TABLE IF EXISTS `proveedor`;
DROP TABLE IF EXISTS `ubicacion`;
DROP TABLE IF EXISTS `roles`;

-- ----------------------------------------------------------------------------
-- 1. Tabla: roles
-- ----------------------------------------------------------------------------
CREATE TABLE `roles` (
  `id_roles` INT NOT NULL,
  `nombre_rol` VARCHAR(50) NOT NULL,
  PRIMARY KEY (`id_roles`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `roles` (`id_roles`, `nombre_rol`) VALUES
  (1, 'Jefe Departamento'),
  (2, 'Bodega'),
  (3, 'Vendedor'),
  (4, 'Gerente');

-- ----------------------------------------------------------------------------
-- 2. Tabla: ubicacion
-- ----------------------------------------------------------------------------
CREATE TABLE `ubicacion` (
  `id_ubicacion` INT NOT NULL,
  `lugar` VARCHAR(50) NOT NULL,
  PRIMARY KEY (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `ubicacion` (`id_ubicacion`, `lugar`) VALUES
  (1, 'Piso'),
  (2, 'Bodega');

-- ----------------------------------------------------------------------------
-- 3. Tabla: proveedores
-- ----------------------------------------------------------------------------
CREATE TABLE `proveedores` (
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

INSERT INTO `proveedores` (`id_proveedor`, `nombre_proveedor`, `rfc`, `telefono`, `correo`, `direccion`, `persona_contacto`, `estatus`) VALUES
  (1, 'Planeta Mexico', 'PME850101AB1', '5553331100', 'contacto@planetamexico.com.mx', 'Av. Presidente Masaryk 111, Polanco, CDMX', 'Lic. Carlos Fuentes Rivera', 'ACTIVO'),
  (2, 'Planeta Mexico Infantil', 'PMI920415CD2', '5553331122', 'infantil@planetamexico.com.mx', 'Av. Presidente Masaryk 115, Polanco, CDMX', 'Mtra. Elena Poniatowska Gil', 'ACTIVO'),
  (3, 'Penguin Random House', 'PRH010920EF3', '5554442200', 'ventas@penguinrandomhouse.mx', 'Blvd. Miguel de Cervantes Saavedra 301, Granada, CDMX', 'Lic. Javier Velasco Ruiz', 'ACTIVO'),
  (4, 'Editorial Televisa', 'ETE730508GH4', '5552612000', 'distribucion@editorialtelevisa.com.mx', 'Av. Vasco de Quiroga 2000, Santa Fe, CDMX', 'Lic. Martha Debayle Gallardo', 'ACTIVO');

-- Vista de compatibilidad para consultas heredadas
CREATE OR REPLACE VIEW `proveedor` AS SELECT * FROM `proveedores`;

-- ----------------------------------------------------------------------------
-- 4. Tabla: genero (Catálogo de Géneros)
-- ----------------------------------------------------------------------------
CREATE TABLE `genero` (
  `id_genero` INT NOT NULL AUTO_INCREMENT,
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

-- ----------------------------------------------------------------------------
-- 5. Tabla: autores (Catálogo de Autores)
-- ----------------------------------------------------------------------------
CREATE TABLE `autores` (
  `id_autor` INT NOT NULL AUTO_INCREMENT,
  `nombre` VARCHAR(100) NOT NULL,
  `apellidos` VARCHAR(100) NOT NULL,
  `nacionalidad` VARCHAR(100) DEFAULT NULL,
  `biografia` TEXT DEFAULT NULL,
  PRIMARY KEY (`id_autor`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `autores` (`id_autor`, `nombre`, `apellidos`, `nacionalidad`, `biografia`) VALUES
  (1, 'Viktor', 'Frankl', 'Austriaco', 'Neurologo, psiquiatra y fundador de la Logoterapia.'),
  (2, 'Gabriel', 'Garcia Marquez', 'Colombiano', 'Premio Nobel de Literatura 1982, autor cumbre del Realismo Magico.'),
  (3, 'Gisele', 'Pelicot', 'Francesa', 'Autora y activista francesa por la dignidad humana.'),
  (4, 'Michael', 'Ende', 'Aleman', 'Escritor de literatura fantastica, autor de Momo y La Historia Interminable.'),
  (5, 'Robert', 'Fisher', 'Estadounidense', 'Escritor y guionista, autor de El Caballero de la Armadura Oxidada.');

-- ----------------------------------------------------------------------------
-- 6. Tabla: mueble
-- ----------------------------------------------------------------------------
CREATE TABLE `mueble` (
  `id_Mueble` INT NOT NULL AUTO_INCREMENT,
  `id_genero` INT NOT NULL,
  PRIMARY KEY (`id_Mueble`),
  KEY `FK_MUEBLE_GENERO` (`id_genero`),
  CONSTRAINT `FK_MUEBLE_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `mueble` (`id_Mueble`, `id_genero`) VALUES
  (11, 11),
  (12, 12),
  (13, 13),
  (61, 61),
  (62, 62);

-- ----------------------------------------------------------------------------
-- 6b. Tabla: motivos (Catálogo de motivos)
-- ----------------------------------------------------------------------------
CREATE TABLE `motivos` (
  `id_motivo` INT NOT NULL AUTO_INCREMENT,
  `motivo` VARCHAR(100) NOT NULL,
  PRIMARY KEY (`id_motivo`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `motivos` (`id_motivo`, `motivo`) VALUES
  (1, 'Reabastecimiento'),
  (2, 'Defecto de imprenta'),
  (3, 'Sobrestock'),
  (4, 'Error de pedido');

-- ----------------------------------------------------------------------------
-- 7. Tabla: usuarios
-- ----------------------------------------------------------------------------
CREATE TABLE `usuarios` (
  `id_usuarios` BIGINT NOT NULL,
  `id_roles` INT NOT NULL,
  `nombre` VARCHAR(50) NOT NULL,
  `apellido_paterno` VARCHAR(50) DEFAULT NULL,
  `apellido_materno` VARCHAR(50) DEFAULT NULL,
  `telefono` BIGINT DEFAULT NULL,
  `contrasena` VARCHAR(255) NOT NULL,
  PRIMARY KEY (`id_usuarios`),
  KEY `FK_USUARIOS_ROLES` (`id_roles`),
  CONSTRAINT `FK_USUARIOS_ROLES` FOREIGN KEY (`id_roles`) REFERENCES `roles` (`id_roles`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `usuarios` (`id_usuarios`, `id_roles`, `nombre`, `apellido_paterno`, `apellido_materno`, `telefono`, `contrasena`) VALUES
  (1001, 4, 'Valeria', 'Morales', 'Gomez', 8441112233, '$2b$10$GYRtA19hDIFbxL/ktJsMluzBL6RtxZu4KFSIOClBq14aG0Mvyh2zy'),
  (350976899, 1, 'Marco', 'Perez', 'Luevano', 8444578906, '$2b$10$JEis.kKv578RybJ7CPTKzOnzwJMf7iUbQWZwnpgSPmii0Rv8QLpyC'),
  (628777130, 2, 'Ian', 'Herrera', 'Ramirez', 8449970912, '$2b$10$okBEvf28NTEBI3eA/jrO3euCYnsqKpk4w2T0eOdpNFucX4Y6pSUPq'),
  (628777129, 3, 'Aldo', 'Soto', 'Velez', 8442836877, '$2b$10$9ZdsD93wmahmAOvWoEJ5SeI3yDWTZMNzlutEVfiiNbj4ZTgLPNUZy');

-- ----------------------------------------------------------------------------
-- 8. Tabla: libros (Sin autor unico ni id_genero unico)
-- ----------------------------------------------------------------------------
CREATE TABLE `libros` (
  `codigo_ean` BIGINT NOT NULL,
  `id_ubicacion` INT NOT NULL,
  `id_mueble` INT NOT NULL,
  `id_proveedor` INT NOT NULL,
  `nombre_libro` VARCHAR(255) NOT NULL,
  `precio` DOUBLE NOT NULL,
  `cantidad` INT NOT NULL,
  `SKU` BIGINT DEFAULT NULL,
  PRIMARY KEY (`codigo_ean`, `id_ubicacion`),
  KEY `FK_LIBROS_MUEBLE` (`id_mueble`),
  KEY `FK_LIBROS_PROVEEDOR` (`id_proveedor`),
  KEY `FK_LIBROS_UBICACION` (`id_ubicacion`),
  CONSTRAINT `FK_LIBROS_MUEBLE` FOREIGN KEY (`id_mueble`) REFERENCES `mueble` (`id_Mueble`),
  CONSTRAINT `FK_LIBROS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedores` (`id_proveedor`),
  CONSTRAINT `FK_LIBROS_UBICACION` FOREIGN KEY (`id_ubicacion`) REFERENCES `ubicacion` (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `libros` (`codigo_ean`, `id_ubicacion`, `id_mueble`, `id_proveedor`, `nombre_libro`, `precio`, `cantidad`, `SKU`) VALUES
  -- Piso (1) y Bodega (2)
  (9786070775390, 2, 11, 1, 'El Hombre en Busca de Sentido', 249.00, 40, 884102),
  (9786073832014, 1, 11, 1, 'Cien Años de Soledad', 350.00, 10, 10001),
  (9786073832014, 2, 11, 1, 'Cien Años de Soledad', 350.00, 45, 10001),
  (9786073868723, 1, 12, 3, 'Un Himno a la Vida', 299.00, 10, 1321371),
  (9786073868723, 2, 12, 3, 'Un Himno a la Vida', 299.00, 50, 1321371),
  (9786078852895, 1, 61, 2, 'Momo', 260.00, 5, 750912),
  (9786078852895, 2, 61, 2, 'Momo', 260.00, 30, 750912),
  (9788497776639, 1, 13, 1, 'El Regreso del Caballero de la Armadura Oxidada', 199.00, 8, 9777668),
  (9788497776639, 2, 13, 1, 'El Regreso del Caballero de la Armadura Oxidada', 199.00, 25, 9777668);

-- ----------------------------------------------------------------------------
-- 9. Tablas Intermedias: libro_genero y libro_autor
-- ----------------------------------------------------------------------------
CREATE TABLE `libro_genero` (
  `codigo_ean` BIGINT NOT NULL,
  `id_genero` INT NOT NULL,
  PRIMARY KEY (`codigo_ean`, `id_genero`),
  KEY `FK_LG_GENERO` (`id_genero`),
  CONSTRAINT `FK_LG_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`) ON UPDATE CASCADE ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `libro_genero` (`codigo_ean`, `id_genero`) VALUES
  (9786070775390, 11), -- Psicología
  (9786070775390, 17), -- Ensayo
  (9786073832014, 16), -- Novela Contemporánea
  (9786073868723, 12), -- Autoayuda
  (9786078852895, 61), -- Infantiles
  (9786078852895, 62), -- Juvenil
  (9788497776639, 12), -- Autoayuda
  (9788497776639, 13); -- Metafísica

CREATE TABLE `libro_autor` (
  `codigo_ean` BIGINT NOT NULL,
  `id_autor` INT NOT NULL,
  PRIMARY KEY (`codigo_ean`, `id_autor`),
  KEY `FK_LA_AUTOR` (`id_autor`),
  CONSTRAINT `FK_LA_AUTOR` FOREIGN KEY (`id_autor`) REFERENCES `autores` (`id_autor`) ON UPDATE CASCADE ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `libro_autor` (`codigo_ean`, `id_autor`) VALUES
  (9786070775390, 1), -- Viktor Frankl
  (9786073832014, 2), -- Gabriel García Márquez
  (9786073868723, 3), -- Gisele Pelicot
  (9786078852895, 4), -- Michael Ende
  (9788497776639, 5); -- Robert Fisher

-- ----------------------------------------------------------------------------
-- 10. Tabla: revistas
-- ----------------------------------------------------------------------------
CREATE TABLE `revistas` (
  `codigo_ean` BIGINT NOT NULL,
  `id_ubicacion` INT NOT NULL,
  `id_mueble` INT NOT NULL,
  `id_proveedor` INT NOT NULL,
  `nombre_revista` VARCHAR(255) NOT NULL,
  `numero_edicion` INT DEFAULT NULL,
  `periodicidad` VARCHAR(50) DEFAULT NULL,
  `precio` DOUBLE NOT NULL,
  `cantidad` INT NOT NULL,
  `SKU` BIGINT DEFAULT NULL,
  PRIMARY KEY (`codigo_ean`, `id_ubicacion`),
  KEY `FK_REVISTAS_MUEBLE` (`id_mueble`),
  KEY `FK_REVISTAS_PROVEEDOR` (`id_proveedor`),
  KEY `FK_REVISTAS_UBICACION` (`id_ubicacion`),
  CONSTRAINT `FK_REVISTAS_MUEBLE` FOREIGN KEY (`id_mueble`) REFERENCES `mueble` (`id_Mueble`),
  CONSTRAINT `FK_REVISTAS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedores` (`id_proveedor`),
  CONSTRAINT `FK_REVISTAS_UBICACION` FOREIGN KEY (`id_ubicacion`) REFERENCES `ubicacion` (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `revistas` (`codigo_ean`, `id_ubicacion`, `id_mueble`, `id_proveedor`, `nombre_revista`, `numero_edicion`, `periodicidad`, `precio`, `cantidad`, `SKU`) VALUES
  (9771234567010, 1, 11, 2, 'National Geographic Mar 2026', 345, 'Mensual', 120.00, 10, 20001),
  (9771234567010, 2, 11, 2, 'National Geographic Mar 2026', 345, 'Mensual', 120.00, 35, 20001),
  (9772007490001, 1, 11, 4, 'Muy Interesante Mexico Ed. 452', 452, 'Mensual', 65.00, 15, 901001),
  (9772007490001, 2, 11, 4, 'Muy Interesante Mexico Ed. 452', 452, 'Mensual', 65.00, 40, 901001);

-- ----------------------------------------------------------------------------
-- 11. Tabla Intermedia: revista_genero
-- ----------------------------------------------------------------------------
CREATE TABLE `revista_genero` (
  `codigo_ean` BIGINT NOT NULL,
  `id_genero` INT NOT NULL,
  PRIMARY KEY (`codigo_ean`, `id_genero`),
  KEY `FK_RG_GENERO` (`id_genero`),
  CONSTRAINT `FK_RG_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`) ON UPDATE CASCADE ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `revista_genero` (`codigo_ean`, `id_genero`) VALUES
  (9771234567010, 15), -- Historia
  (9771234567010, 18), -- Divulgación Científica
  (9772007490001, 18); -- Divulgación Científica

-- ----------------------------------------------------------------------------
-- 12. Tabla: bitacora
-- ----------------------------------------------------------------------------
CREATE TABLE `bitacora` (
  `id_bitacora` BIGINT NOT NULL AUTO_INCREMENT,
  `fecha_hora` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `accion` VARCHAR(100) NOT NULL,
  `detalle` VARCHAR(255) NOT NULL,
  `id_usuarios` BIGINT NOT NULL,
  PRIMARY KEY (`id_bitacora`),
  KEY `FK_BITACORA_USUARIO` (`id_usuarios`),
  CONSTRAINT `FK_BITACORA_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `bitacora` (`fecha_hora`, `accion`, `detalle`, `id_usuarios`) VALUES
  (NOW(), 'SISTEMA_INICIALIZACION', 'Carga de esquema V2 con relaciones N:M, Autores, Proveedores y Compras', 1001);

-- ----------------------------------------------------------------------------
-- 13. Tabla: devolucion
-- ----------------------------------------------------------------------------
CREATE TABLE `devolucion` (
  `id_devolucion` BIGINT NOT NULL AUTO_INCREMENT,
  `fecha` DATE NOT NULL,
  `total_piezas` INT NOT NULL,
  `estado` VARCHAR(50) NOT NULL DEFAULT 'PENDIENTE',
  `autorizado_por` VARCHAR(100) DEFAULT NULL,
  `id_usuarios` BIGINT NOT NULL,
  `id_proveedor` INT NOT NULL,
  PRIMARY KEY (`id_devolucion`),
  KEY `FK_DEVOLUCION_USUARIO` (`id_usuarios`),
  KEY `FK_DEVOLUCION_PROVEEDOR` (`id_proveedor`),
  CONSTRAINT `FK_DEVOLUCION_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedores` (`id_proveedor`),
  CONSTRAINT `FK_DEVOLUCION_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `devolucion` (`id_devolucion`, `fecha`, `total_piezas`, `estado`, `autorizado_por`, `id_usuarios`, `id_proveedor`) VALUES
  (1, CURDATE(), 3, 'PENDIENTE', NULL, 628777129, 1),
  (2, CURDATE(), 5, 'APROBADO', 'Marco Perez Luevano', 628777129, 3);

-- ----------------------------------------------------------------------------
-- 14. Tablas: clientes, ventas y detalle_ventas (Soporte N libros y N revistas por cliente)
-- ----------------------------------------------------------------------------
CREATE TABLE `clientes` (
  `id_cliente` INT NOT NULL AUTO_INCREMENT,
  `nombre` VARCHAR(150) NOT NULL,
  `rfc` VARCHAR(20) DEFAULT NULL,
  `telefono` VARCHAR(25) DEFAULT NULL,
  `correo` VARCHAR(100) DEFAULT NULL,
  `direccion` VARCHAR(255) DEFAULT NULL,
  `fecha_registro` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id_cliente`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `clientes` (`id_cliente`, `nombre`, `rfc`, `telefono`, `correo`, `direccion`) VALUES
  (1, 'Público en General', 'XAXX010101000', '0000000000', 'general@novabook.com', 'Venta mostrador'),
  (2, 'Ing. Fernando Peña', 'PEEF850315AA1', '8441234567', 'fernando.pena@email.com', 'Blvd. Venustiano Carranza 2340, Saltillo, Coah.'),
  (3, 'Dra. Carmen Salinas', 'SAGC781012BB2', '8449876543', 'carmen.salinas@email.com', 'Calle Hidalgo 450, Centro, Saltillo, Coah.'),
  (4, 'Lic. Roberto Gómez', 'GORL900220CC3', '8445558899', 'roberto.gomez@email.com', 'Av. Universidad 890, Saltillo, Coah.');

CREATE TABLE `ventas` (
  `id_venta` BIGINT NOT NULL AUTO_INCREMENT,
  `id_usuarios` BIGINT NOT NULL,
  `id_cliente` INT NOT NULL DEFAULT 1,
  `nombre_cliente` VARCHAR(150) NOT NULL DEFAULT 'Público en General',
  `fecha_hora` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `total` DOUBLE NOT NULL DEFAULT 0.0,
  PRIMARY KEY (`id_venta`),
  KEY `FK_VENTAS_USUARIOS` (`id_usuarios`),
  KEY `FK_VENTAS_CLIENTE` (`id_cliente`),
  CONSTRAINT `FK_VENTAS_USUARIOS` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`),
  CONSTRAINT `FK_VENTAS_CLIENTE` FOREIGN KEY (`id_cliente`) REFERENCES `clientes` (`id_cliente`) ON UPDATE CASCADE ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `detalle_ventas` (
  `id_detalle` BIGINT NOT NULL AUTO_INCREMENT,
  `id_venta` BIGINT NOT NULL,
  `codigo_ean` BIGINT NOT NULL,
  `tipo_producto` VARCHAR(20) NOT NULL,
  `cantidad` INT NOT NULL,
  `precio_unitario` DOUBLE NOT NULL,
  `subtotal` DOUBLE NOT NULL,
  PRIMARY KEY (`id_detalle`),
  KEY `FK_DETALLE_VENTA` (`id_venta`),
  CONSTRAINT `FK_DETALLE_VENTA` FOREIGN KEY (`id_venta`) REFERENCES `ventas` (`id_venta`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- Datos semilla: Cliente adquiriendo N cantidad de libros y N cantidad de revistas
INSERT INTO `ventas` (`id_venta`, `id_usuarios`, `id_cliente`, `nombre_cliente`, `fecha_hora`, `total`) VALUES
  (1, 628777129, 2, 'Ing. Fernando Peña', DATE_SUB(NOW(), INTERVAL 1 DAY), 1060.00),
  (2, 628777129, 3, 'Dra. Carmen Salinas', NOW(), 484.00);

INSERT INTO `detalle_ventas` (`id_detalle`, `id_venta`, `codigo_ean`, `tipo_producto`, `cantidad`, `precio_unitario`, `subtotal`) VALUES
  -- Venta #1: Cliente adquiere 2 libros (Cien Años de Soledad) y 3 revistas (National Geographic)
  (1, 1, 9786073832014, 'Libro', 2, 350.00, 700.00),
  (2, 1, 9771234567010, 'Revista', 3, 120.00, 360.00),
  -- Venta #2: Cliente adquiere 1 libro (Un Himno a la Vida), 1 revista NatGeo y 1 revista Muy Interesante
  (3, 2, 9786073868723, 'Libro', 1, 299.00, 299.00),
  (4, 2, 9771234567010, 'Revista', 1, 120.00, 120.00),
  (5, 2, 9772007490001, 'Revista', 1, 65.00, 65.00);

-- Vista Relacional: Consulta completa de clientes y productos comprados (libros y revistas)
CREATE OR REPLACE VIEW `vista_cliente_compras` AS
SELECT 
  c.id_cliente,
  c.nombre AS nombre_cliente,
  c.telefono AS telefono_cliente,
  c.correo AS correo_cliente,
  v.id_venta,
  v.fecha_hora AS fecha_compra,
  v.total AS total_venta,
  dv.id_detalle,
  dv.tipo_producto,
  dv.codigo_ean,
  COALESCE(l.nombre_libro, r.nombre_revista, 'Producto Desconocido') AS nombre_producto,
  dv.cantidad,
  dv.precio_unitario,
  dv.subtotal
FROM clientes c
JOIN ventas v ON c.id_cliente = v.id_cliente
JOIN detalle_ventas dv ON v.id_venta = dv.id_venta
LEFT JOIN libros l ON dv.codigo_ean = l.codigo_ean AND dv.tipo_producto = 'Libro' AND l.id_ubicacion = 1
LEFT JOIN revistas r ON dv.codigo_ean = r.codigo_ean AND dv.tipo_producto = 'Revista' AND r.id_ubicacion = 1;


-- ----------------------------------------------------------------------------
-- 15. Tablas: compras_proveedor y detalle_compra
-- ----------------------------------------------------------------------------
CREATE TABLE `compras_proveedor` (
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

CREATE TABLE `detalle_compra` (
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

INSERT INTO `compras_proveedor` (`id_compra`, `id_proveedor`, `id_usuarios`, `fecha_hora`, `total`, `observaciones`) VALUES
  (1, 1, 628777130, DATE_SUB(NOW(), INTERVAL 2 DAY), 7000.00, 'Factura Planeta México F-4029'),
  (2, 3, 628777130, DATE_SUB(NOW(), INTERVAL 1 DAY), 5980.00, 'Recepción novedades Penguin Random House');

INSERT INTO `detalle_compra` (`id_detalle_compra`, `id_compra`, `codigo_ean`, `tipo_producto`, `cantidad`, `costo_unitario`, `subtotal`) VALUES
  (1, 1, 9786073832014, 'Libro', 20, 210.00, 4200.00),
  (2, 1, 9788497776639, 'Libro', 20, 140.00, 2800.00),
  (3, 2, 9786073868723, 'Libro', 20, 199.00, 3980.00),
  (4, 2, 9771234567010, 'Revista', 25, 80.00, 2000.00);

-- ----------------------------------------------------------------------------
-- 16. Tabla: movimientos (Histórico de traslados Bodega <-> Tienda)
-- ----------------------------------------------------------------------------
CREATE TABLE `movimientos` (
  `id_movimiento` BIGINT NOT NULL AUTO_INCREMENT,
  `id_usuarios` BIGINT NOT NULL,
  `codigo_ean` BIGINT NOT NULL,
  `tipo_producto` VARCHAR(20) NOT NULL,
  `origen_ubicacion` INT NOT NULL,
  `destino_ubicacion` INT NOT NULL,
  `cantidad` INT NOT NULL,
  `fecha_hora` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `observaciones` VARCHAR(255) DEFAULT NULL,
  PRIMARY KEY (`id_movimiento`),
  KEY `FK_MOVIMIENTOS_USUARIO` (`id_usuarios`),
  CONSTRAINT `FK_MOVIMIENTOS_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

SET FOREIGN_KEY_CHECKS = 1;
