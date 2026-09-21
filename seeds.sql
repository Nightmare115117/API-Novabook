-- ============================================================================
-- SCRIPT DE MIGRACIÓN Y DATOS SEMILLA (SEEDS) - LIBRERÍA NOVABOOK
-- ============================================================================
-- Base de Datos: Libreria
-- Fecha: 2026-09-19
-- Compatible con: MySQL 8.0+ / MariaDB 10.5+
-- ============================================================================

CREATE DATABASE IF NOT EXISTS `Libreria` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
USE `Libreria`;

-- Desactivar verificación de llaves foráneas para permitir reemplazo limpio
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------------------------------------------------------
-- 0. Limpieza previa de tablas (en orden inverso de dependencia)
-- ----------------------------------------------------------------------------
DROP TABLE IF EXISTS `detalle_ventas`;
DROP TABLE IF EXISTS `ventas`;
DROP TABLE IF EXISTS `movimientos`;
DROP TABLE IF EXISTS `bitacora`;
DROP TABLE IF EXISTS `devolucion`;
DROP TABLE IF EXISTS `libros`;
DROP TABLE IF EXISTS `revistas`;
DROP TABLE IF EXISTS `usuarios`;
DROP TABLE IF EXISTS `mueble`;
DROP TABLE IF EXISTS `genero`;
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
-- 3. Tabla: proveedor
-- ----------------------------------------------------------------------------
CREATE TABLE `proveedor` (
  `id_proveedor` INT NOT NULL,
  `nombre_proveedor` VARCHAR(100) NOT NULL,
  PRIMARY KEY (`id_proveedor`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `proveedor` (`id_proveedor`, `nombre_proveedor`) VALUES
  (1, 'Planeta Mexico'),
  (2, 'Planeta Mexico Infantil'),
  (3, 'Penguin Random House'),
  (4, 'Editorial Televisa');

-- ----------------------------------------------------------------------------
-- 4. Tabla: genero
-- ----------------------------------------------------------------------------
CREATE TABLE `genero` (
  `id_genero` INT NOT NULL,
  `genero_literario` VARCHAR(50) NOT NULL,
  PRIMARY KEY (`id_genero`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `genero` (`id_genero`, `genero_literario`) VALUES
  (11, 'Psicologia'),
  (12, 'Autoayuda'),
  (13, 'Metafisica'),
  (61, 'Infantiles'),
  (62, 'Juvenil');

-- ----------------------------------------------------------------------------
-- 5. Tabla: mueble
-- ----------------------------------------------------------------------------
CREATE TABLE `mueble` (
  `id_Mueble` INT NOT NULL,
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
-- 6. Tabla: usuarios
-- NOTA: `contrasena` es VARCHAR(255) para admitir hashes Bcrypt (60 caracteres)
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

-- Usuarios semilla con contraseñas Bcrypt:
-- 1. Gerente:       ID 1001      | Pass: admin123
-- 2. Jefe Depto:    ID 350976899 | Pass: 2501
-- 3. Bodega:        ID 628777130 | Pass: 7777
-- 4. Vendedor:      ID 628777129 | Pass: 8888
INSERT INTO `usuarios` (`id_usuarios`, `id_roles`, `nombre`, `apellido_paterno`, `apellido_materno`, `telefono`, `contrasena`) VALUES
  (1001, 4, 'Valeria', 'Morales', 'Gomez', 8441112233, '$2b$10$GYRtA19hDIFbxL/ktJsMluzBL6RtxZu4KFSIOClBq14aG0Mvyh2zy'),
  (350976899, 1, 'Marco', 'Perez', 'Luevano', 8444578906, '$2b$10$JEis.kKv578RybJ7CPTKzOnzwJMf7iUbQWZwnpgSPmii0Rv8QLpyC'),
  (628777130, 2, 'Ian', 'Herrera', 'Ramirez', 8449970912, '$2b$10$okBEvf28NTEBI3eA/jrO3euCYnsqKpk4w2T0eOdpNFucX4Y6pSUPq'),
  (628777129, 3, 'Aldo', 'Soto', 'Velez', 8442836877, '$2b$10$9ZdsD93wmahmAOvWoEJ5SeI3yDWTZMNzlutEVfiiNbj4ZTgLPNUZy');

-- ----------------------------------------------------------------------------
-- 7. Tabla: libros (Clave compuesta: codigo_ean + id_ubicacion)
-- ----------------------------------------------------------------------------
CREATE TABLE `libros` (
  `codigo_ean` BIGINT NOT NULL,
  `id_ubicacion` INT NOT NULL,
  `id_genero` INT NOT NULL,
  `id_mueble` INT NOT NULL,
  `id_proveedor` INT NOT NULL,
  `nombre_libro` VARCHAR(255) NOT NULL,
  `precio` DOUBLE NOT NULL,
  `cantidad` INT NOT NULL,
  `autor` VARCHAR(100) DEFAULT NULL,
  `SKU` BIGINT DEFAULT NULL,
  PRIMARY KEY (`codigo_ean`, `id_ubicacion`),
  KEY `FK_LIBROS_GENERO` (`id_genero`),
  KEY `FK_LIBROS_MUEBLE` (`id_mueble`),
  KEY `FK_LIBROS_PROVEEDOR` (`id_proveedor`),
  KEY `FK_LIBROS_UBICACION` (`id_ubicacion`),
  CONSTRAINT `FK_LIBROS_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`),
  CONSTRAINT `FK_LIBROS_MUEBLE` FOREIGN KEY (`id_mueble`) REFERENCES `mueble` (`id_Mueble`),
  CONSTRAINT `FK_LIBROS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedor` (`id_proveedor`),
  CONSTRAINT `FK_LIBROS_UBICACION` FOREIGN KEY (`id_ubicacion`) REFERENCES `ubicacion` (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- Semillas de libros en Tienda (1) y Bodega (2)
INSERT INTO `libros` (`codigo_ean`, `id_ubicacion`, `id_genero`, `id_mueble`, `id_proveedor`, `nombre_libro`, `precio`, `cantidad`, `autor`, `SKU`) VALUES
  -- Piso de Venta (id_ubicacion = 1)
  (9786073868723, 1, 12, 12, 3, 'Un Himno a la Vida', 299.00, 10, 'Gisele Pelicot', 1321371),
  (9786078852895, 1, 61, 61, 2, 'Momo', 260.00, 5, 'Michael Ende', 750912),
  (9788497776639, 1, 13, 13, 1, 'El Regreso del Caballero de la Armadura Oxidada', 199.00, 8, 'Robert Fisher', 9777668),
  -- Bodega (id_ubicacion = 2) listos para traslados
  (9786073868723, 2, 12, 12, 3, 'Un Himno a la Vida', 299.00, 50, 'Gisele Pelicot', 1321371),
  (9786078852895, 2, 61, 61, 2, 'Momo', 260.00, 30, 'Michael Ende', 750912),
  (9788497776639, 2, 13, 13, 1, 'El Regreso del Caballero de la Armadura Oxidada', 199.00, 25, 'Robert Fisher', 9777668),
  (9786070775390, 2, 11, 11, 1, 'El Hombre en Busca de Sentido', 249.00, 40, 'Viktor Frankl', 884102);

-- ----------------------------------------------------------------------------
-- 8. Tabla: revistas (Entidad análoga con edición y periodicidad)
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
  CONSTRAINT `FK_REVISTAS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedor` (`id_proveedor`),
  CONSTRAINT `FK_REVISTAS_UBICACION` FOREIGN KEY (`id_ubicacion`) REFERENCES `ubicacion` (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `revistas` (`codigo_ean`, `id_ubicacion`, `id_mueble`, `id_proveedor`, `nombre_revista`, `numero_edicion`, `periodicidad`, `precio`, `cantidad`, `SKU`) VALUES
  (7501000001010, 1, 11, 4, 'Muy Interesante Mexico', 452, 'Mensual', 65.00, 12, 901001),
  (7501000001010, 2, 11, 4, 'Muy Interesante Mexico', 452, 'Mensual', 65.00, 40, 901001),
  (7501000002020, 1, 11, 4, 'National Geographic en Espanol', 315, 'Mensual', 89.00, 6, 902002),
  (7501000002020, 2, 11, 4, 'National Geographic en Espanol', 315, 'Mensual', 89.00, 25, 902002);

-- ----------------------------------------------------------------------------
-- 9. Tabla: bitacora (Auditoría obligatoria de eventos)
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
  (NOW(), 'SISTEMA_INICIALIZACION', 'Carga inicial de semillas y configuración de la base de datos', 1001);

-- ----------------------------------------------------------------------------
-- 10. Tabla: devolucion
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
  CONSTRAINT `FK_DEVOLUCION_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedor` (`id_proveedor`),
  CONSTRAINT `FK_DEVOLUCION_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `devolucion` (`id_devolucion`, `fecha`, `total_piezas`, `estado`, `autorizado_por`, `id_usuarios`, `id_proveedor`) VALUES
  (1, CURDATE(), 3, 'PENDIENTE', NULL, 628777129, 1),
  (2, CURDATE(), 5, 'APROBADO', 'Marco Perez Luevano', 628777129, 3);

-- ----------------------------------------------------------------------------
-- 11. Tablas operacionales: ventas y detalle_ventas
-- ----------------------------------------------------------------------------
CREATE TABLE `ventas` (
  `id_venta` BIGINT NOT NULL AUTO_INCREMENT,
  `id_usuarios` BIGINT NOT NULL,
  `fecha_hora` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `total` DOUBLE NOT NULL DEFAULT 0.0,
  PRIMARY KEY (`id_venta`),
  KEY `FK_VENTAS_USUARIOS` (`id_usuarios`),
  CONSTRAINT `FK_VENTAS_USUARIOS` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
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

-- ----------------------------------------------------------------------------
-- 12. Tabla: movimientos (Histórico de traslados Bodega <-> Tienda)
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

-- Reactivar verificación de llaves foráneas
SET FOREIGN_KEY_CHECKS = 1;
