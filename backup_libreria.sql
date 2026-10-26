/*M!999999\- enable the sandbox mode */ 
-- MariaDB dump 10.20-12.3.3-MariaDB, for Linux (x86_64)
--
-- Host: dev-server    Database: Libreria
-- ------------------------------------------------------
-- Server version	10.11.19-MariaDB-ubu2204

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*M!100616 SET @OLD_NOTE_VERBOSITY=@@NOTE_VERBOSITY, NOTE_VERBOSITY=0 */;

--
-- Table structure for table `autores`
--

DROP TABLE IF EXISTS `autores`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `autores` (
  `id_autor` int(11) NOT NULL AUTO_INCREMENT,
  `nombre` varchar(100) NOT NULL,
  `apellidos` varchar(100) NOT NULL,
  `nacionalidad` varchar(100) DEFAULT NULL,
  `biografia` text DEFAULT NULL,
  PRIMARY KEY (`id_autor`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `autores`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `autores` WRITE;
/*!40000 ALTER TABLE `autores` DISABLE KEYS */;
INSERT INTO `autores` VALUES
(1,'Viktor','Frankl','Austriaco','Neurólogo, psiquiatra y fundador de la Logoterapia.'),
(2,'Gabriel','García Márquez','Colombiano','Premio Nobel de Literatura 1982, autor de Cien Años de Soledad.'),
(3,'Gisele','Pelicot','Francesa','Autora y activista francesa.'),
(4,'Michael','Ende','Alemán','Escritor de literatura fantástica y juvenil.'),
(5,'Robert','Fisher','Estadounidense','Escritor y dramaturgo, autor de El Caballero de la Armadura Oxidada.');
/*!40000 ALTER TABLE `autores` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `bitacora`
--

DROP TABLE IF EXISTS `bitacora`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `bitacora` (
  `id_bitacora` bigint(20) NOT NULL AUTO_INCREMENT,
  `fecha_hora` datetime NOT NULL DEFAULT current_timestamp(),
  `accion` varchar(100) NOT NULL,
  `detalle` varchar(255) NOT NULL,
  `id_usuarios` bigint(20) NOT NULL,
  PRIMARY KEY (`id_bitacora`),
  KEY `FK_BITACORA_USUARIO` (`id_usuarios`),
  CONSTRAINT `FK_BITACORA_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `bitacora`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `bitacora` WRITE;
/*!40000 ALTER TABLE `bitacora` DISABLE KEYS */;
INSERT INTO `bitacora` VALUES
(1,'2026-10-07 05:31:47','SISTEMA_INICIALIZACION','Carga de esquema V2 con relaciones N:M, Autores, Proveedores y Compras',1001),
(2,'2026-10-07 05:33:02','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: gerente',1001),
(3,'2026-10-07 05:33:12','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: jefe_departamento',350976899),
(4,'2026-10-07 05:33:49','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: jefe_departamento',350976899),
(5,'2026-10-07 05:33:59','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(6,'2026-10-07 05:34:42','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(7,'2026-10-07 05:34:50','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: vendedor',628777129),
(8,'2026-10-07 05:40:27','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(9,'2026-10-07 06:01:10','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(10,'2026-10-07 06:14:24','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(11,'2026-10-07 06:14:38','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: vendedor',628777129),
(12,'2026-10-07 06:14:54','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: jefe_departamento',350976899),
(13,'2026-10-07 06:22:05','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: jefe_departamento',350976899),
(14,'2026-10-07 06:23:44','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: vendedor',628777129),
(15,'2026-10-07 06:27:23','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(16,'2026-10-07 06:31:53','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: vendedor',628777129),
(17,'2026-10-07 06:33:57','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(18,'2026-10-07 07:12:34','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(19,'2026-10-07 07:13:16','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: vendedor',628777129),
(20,'2026-10-07 07:13:42','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(21,'2026-10-07 07:19:49','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: personal_bodega',628777130),
(22,'2026-10-19 20:40:52','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: jefe_departamento',350976899),
(23,'2026-10-21 06:45:46','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: gerente',1001),
(24,'2026-10-21 06:46:37','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: jefe_departamento',350976899),
(25,'2026-10-23 02:02:06','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: gerente',1001),
(26,'2026-10-23 02:02:46','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: gerente',1001),
(27,'2026-10-23 02:11:08','INICIO DE SESIÓN','Inicio de sesión exitoso con rol: gerente',1001);
/*!40000 ALTER TABLE `bitacora` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `compras_proveedor`
--

DROP TABLE IF EXISTS `compras_proveedor`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `compras_proveedor` (
  `id_compra` bigint(20) NOT NULL AUTO_INCREMENT,
  `id_proveedor` int(11) NOT NULL,
  `id_usuarios` bigint(20) NOT NULL,
  `fecha_hora` datetime NOT NULL DEFAULT current_timestamp(),
  `total` double NOT NULL DEFAULT 0,
  `observaciones` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id_compra`),
  KEY `FK_COMPRAS_PROVEEDOR` (`id_proveedor`),
  KEY `FK_COMPRAS_USUARIO` (`id_usuarios`),
  CONSTRAINT `FK_COMPRAS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedores` (`id_proveedor`),
  CONSTRAINT `FK_COMPRAS_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `compras_proveedor`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `compras_proveedor` WRITE;
/*!40000 ALTER TABLE `compras_proveedor` DISABLE KEYS */;
INSERT INTO `compras_proveedor` VALUES
(1,1,628777130,'2026-10-05 05:31:48',7000,'Factura Planeta México F-4029'),
(2,3,628777130,'2026-10-06 05:31:48',5980,'Recepción novedades Penguin Random House');
/*!40000 ALTER TABLE `compras_proveedor` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `detalle_compra`
--

DROP TABLE IF EXISTS `detalle_compra`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `detalle_compra` (
  `id_detalle_compra` bigint(20) NOT NULL AUTO_INCREMENT,
  `id_compra` bigint(20) NOT NULL,
  `codigo_ean` bigint(20) NOT NULL,
  `tipo_producto` varchar(20) NOT NULL,
  `cantidad` int(11) NOT NULL,
  `costo_unitario` double NOT NULL,
  `subtotal` double NOT NULL,
  PRIMARY KEY (`id_detalle_compra`),
  KEY `FK_DETALLE_COMPRA` (`id_compra`),
  CONSTRAINT `FK_DETALLE_COMPRA` FOREIGN KEY (`id_compra`) REFERENCES `compras_proveedor` (`id_compra`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `detalle_compra`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `detalle_compra` WRITE;
/*!40000 ALTER TABLE `detalle_compra` DISABLE KEYS */;
INSERT INTO `detalle_compra` VALUES
(1,1,9786073832014,'Libro',20,210,4200),
(2,1,9788497776639,'Libro',20,140,2800),
(3,2,9786073868723,'Libro',20,199,3980),
(4,2,9771234567010,'Revista',25,80,2000);
/*!40000 ALTER TABLE `detalle_compra` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `detalle_ventas`
--

DROP TABLE IF EXISTS `detalle_ventas`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `detalle_ventas` (
  `id_detalle` bigint(20) NOT NULL AUTO_INCREMENT,
  `id_venta` bigint(20) NOT NULL,
  `codigo_ean` bigint(20) NOT NULL,
  `tipo_producto` varchar(20) NOT NULL,
  `cantidad` int(11) NOT NULL,
  `precio_unitario` double NOT NULL,
  `subtotal` double NOT NULL,
  PRIMARY KEY (`id_detalle`),
  KEY `FK_DETALLE_VENTA` (`id_venta`),
  CONSTRAINT `FK_DETALLE_VENTA` FOREIGN KEY (`id_venta`) REFERENCES `ventas` (`id_venta`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `detalle_ventas`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `detalle_ventas` WRITE;
/*!40000 ALTER TABLE `detalle_ventas` DISABLE KEYS */;
/*!40000 ALTER TABLE `detalle_ventas` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `devolucion`
--

DROP TABLE IF EXISTS `devolucion`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `devolucion` (
  `id_devolucion` bigint(20) NOT NULL AUTO_INCREMENT,
  `fecha` date NOT NULL,
  `total_piezas` int(11) NOT NULL,
  `estado` varchar(50) NOT NULL DEFAULT 'PENDIENTE',
  `autorizado_por` varchar(100) DEFAULT NULL,
  `id_usuarios` bigint(20) NOT NULL,
  `id_proveedor` int(11) NOT NULL,
  PRIMARY KEY (`id_devolucion`),
  KEY `FK_DEVOLUCION_USUARIO` (`id_usuarios`),
  KEY `FK_DEVOLUCION_PROVEEDOR` (`id_proveedor`),
  CONSTRAINT `FK_DEVOLUCION_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedores` (`id_proveedor`),
  CONSTRAINT `FK_DEVOLUCION_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `devolucion`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `devolucion` WRITE;
/*!40000 ALTER TABLE `devolucion` DISABLE KEYS */;
INSERT INTO `devolucion` VALUES
(1,'2026-10-07',3,'PENDIENTE',NULL,628777129,1),
(2,'2026-10-07',5,'APROBADO','Marco Perez Luevano',628777129,3);
/*!40000 ALTER TABLE `devolucion` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `genero`
--

DROP TABLE IF EXISTS `genero`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `genero` (
  `id_genero` int(11) NOT NULL AUTO_INCREMENT,
  `genero_literario` varchar(50) NOT NULL,
  PRIMARY KEY (`id_genero`)
) ENGINE=InnoDB AUTO_INCREMENT=63 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `genero`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `genero` WRITE;
/*!40000 ALTER TABLE `genero` DISABLE KEYS */;
INSERT INTO `genero` VALUES
(11,'Psicologia'),
(12,'Autoayuda'),
(13,'Metafisica'),
(14,'Ciencia Ficcion'),
(15,'Historia'),
(16,'Novela Contemporanea'),
(17,'Ensayo'),
(18,'Divulgacion Cientifica'),
(61,'Infantiles'),
(62,'Juvenil');
/*!40000 ALTER TABLE `genero` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `libro_autor`
--

DROP TABLE IF EXISTS `libro_autor`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `libro_autor` (
  `codigo_ean` bigint(20) NOT NULL,
  `id_autor` int(11) NOT NULL,
  PRIMARY KEY (`codigo_ean`,`id_autor`),
  KEY `FK_LA_AUTOR` (`id_autor`),
  CONSTRAINT `FK_LA_AUTOR` FOREIGN KEY (`id_autor`) REFERENCES `autores` (`id_autor`) ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `libro_autor`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `libro_autor` WRITE;
/*!40000 ALTER TABLE `libro_autor` DISABLE KEYS */;
INSERT INTO `libro_autor` VALUES
(9786070775390,1),
(9786073832014,2),
(9786073868723,3),
(9786078852895,4),
(9788497776639,5);
/*!40000 ALTER TABLE `libro_autor` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `libro_genero`
--

DROP TABLE IF EXISTS `libro_genero`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `libro_genero` (
  `codigo_ean` bigint(20) NOT NULL,
  `id_genero` int(11) NOT NULL,
  PRIMARY KEY (`codigo_ean`,`id_genero`),
  KEY `FK_LG_GENERO` (`id_genero`),
  CONSTRAINT `FK_LG_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `libro_genero`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `libro_genero` WRITE;
/*!40000 ALTER TABLE `libro_genero` DISABLE KEYS */;
INSERT INTO `libro_genero` VALUES
(9786070775390,11),
(9786070775390,17),
(9786073832014,16),
(9786073868723,12),
(9786078852895,61),
(9786078852895,62),
(9788497776639,12),
(9788497776639,13);
/*!40000 ALTER TABLE `libro_genero` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `libros`
--

DROP TABLE IF EXISTS `libros`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `libros` (
  `codigo_ean` bigint(20) NOT NULL,
  `id_ubicacion` int(11) NOT NULL,
  `id_mueble` int(11) NOT NULL,
  `id_proveedor` int(11) NOT NULL,
  `nombre_libro` varchar(255) NOT NULL,
  `precio` double NOT NULL,
  `cantidad` int(11) NOT NULL,
  `SKU` bigint(20) DEFAULT NULL,
  PRIMARY KEY (`codigo_ean`,`id_ubicacion`),
  KEY `FK_LIBROS_MUEBLE` (`id_mueble`),
  KEY `FK_LIBROS_PROVEEDOR` (`id_proveedor`),
  KEY `FK_LIBROS_UBICACION` (`id_ubicacion`),
  CONSTRAINT `FK_LIBROS_MUEBLE` FOREIGN KEY (`id_mueble`) REFERENCES `mueble` (`id_Mueble`),
  CONSTRAINT `FK_LIBROS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedores` (`id_proveedor`),
  CONSTRAINT `FK_LIBROS_UBICACION` FOREIGN KEY (`id_ubicacion`) REFERENCES `ubicacion` (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `libros`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `libros` WRITE;
/*!40000 ALTER TABLE `libros` DISABLE KEYS */;
INSERT INTO `libros` VALUES
(9786070775390,2,11,1,'El Hombre en Busca de Sentido',249,40,884102),
(9786073832014,1,11,1,'Cien Años de Soledad',350,10,10001),
(9786073832014,2,11,1,'Cien Años de Soledad',350,45,10001),
(9786073868723,1,12,3,'Un Himno a la Vida',299,10,1321371),
(9786073868723,2,12,3,'Un Himno a la Vida',299,50,1321371),
(9786078852895,1,61,2,'Momo',260,5,750912),
(9786078852895,2,61,2,'Momo',260,30,750912),
(9788497776639,1,13,1,'El Regreso del Caballero de la Armadura Oxidada',199,8,9777668),
(9788497776639,2,13,1,'El Regreso del Caballero de la Armadura Oxidada',199,25,9777668);
/*!40000 ALTER TABLE `libros` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `movimientos`
--

DROP TABLE IF EXISTS `movimientos`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `movimientos` (
  `id_movimiento` bigint(20) NOT NULL AUTO_INCREMENT,
  `id_usuarios` bigint(20) NOT NULL,
  `codigo_ean` bigint(20) NOT NULL,
  `tipo_producto` varchar(20) NOT NULL,
  `origen_ubicacion` int(11) NOT NULL,
  `destino_ubicacion` int(11) NOT NULL,
  `cantidad` int(11) NOT NULL,
  `fecha_hora` datetime NOT NULL DEFAULT current_timestamp(),
  `observaciones` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id_movimiento`),
  KEY `FK_MOVIMIENTOS_USUARIO` (`id_usuarios`),
  CONSTRAINT `FK_MOVIMIENTOS_USUARIO` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `movimientos`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `movimientos` WRITE;
/*!40000 ALTER TABLE `movimientos` DISABLE KEYS */;
/*!40000 ALTER TABLE `movimientos` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `mueble`
--

DROP TABLE IF EXISTS `mueble`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `mueble` (
  `id_Mueble` int(11) NOT NULL,
  `id_genero` int(11) NOT NULL,
  PRIMARY KEY (`id_Mueble`),
  KEY `FK_MUEBLE_GENERO` (`id_genero`),
  CONSTRAINT `FK_MUEBLE_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `mueble`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `mueble` WRITE;
/*!40000 ALTER TABLE `mueble` DISABLE KEYS */;
INSERT INTO `mueble` VALUES
(11,11),
(12,12),
(13,13),
(61,61),
(62,62);
/*!40000 ALTER TABLE `mueble` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Temporary table structure for view `proveedor`
--

DROP TABLE IF EXISTS `proveedor`;
/*!50001 DROP VIEW IF EXISTS `proveedor`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8mb4;
/*!50001 CREATE VIEW `proveedor` AS SELECT
 NULL AS `id_proveedor`,
 NULL AS `nombre_proveedor`,
 NULL AS `rfc`,
 NULL AS `telefono`,
 NULL AS `correo`,
 NULL AS `direccion`,
 NULL AS `persona_contacto`,
 NULL AS `estatus` */;
SET character_set_client = @saved_cs_client;

--
-- Table structure for table `proveedores`
--

DROP TABLE IF EXISTS `proveedores`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `proveedores` (
  `id_proveedor` int(11) NOT NULL AUTO_INCREMENT,
  `nombre_proveedor` varchar(150) NOT NULL,
  `rfc` varchar(20) NOT NULL,
  `telefono` varchar(25) DEFAULT NULL,
  `correo` varchar(100) DEFAULT NULL,
  `direccion` varchar(255) DEFAULT NULL,
  `persona_contacto` varchar(100) DEFAULT NULL,
  `estatus` varchar(20) NOT NULL DEFAULT 'ACTIVO',
  PRIMARY KEY (`id_proveedor`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `proveedores`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `proveedores` WRITE;
/*!40000 ALTER TABLE `proveedores` DISABLE KEYS */;
INSERT INTO `proveedores` VALUES
(1,'Planeta Mexico','PME850101AB1','5553331100','contacto@planetamexico.com.mx','Av. Presidente Masaryk 111, Polanco, CDMX','Lic. Carlos Fuentes Rivera','ACTIVO'),
(2,'Planeta Mexico Infantil','PMI920415CD2','5553331122','infantil@planetamexico.com.mx','Av. Presidente Masaryk 115, Polanco, CDMX','Mtra. Elena Poniatowska Gil','ACTIVO'),
(3,'Penguin Random House','PRH010920EF3','5554442200','ventas@penguinrandomhouse.mx','Blvd. Miguel de Cervantes Saavedra 301, Granada, CDMX','Lic. Javier Velasco Ruiz','ACTIVO'),
(4,'Editorial Televisa','ETE730508GH4','5552612000','distribucion@editorialtelevisa.com.mx','Av. Vasco de Quiroga 2000, Santa Fe, CDMX','Lic. Martha Debayle Gallardo','ACTIVO');
/*!40000 ALTER TABLE `proveedores` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `revista_genero`
--

DROP TABLE IF EXISTS `revista_genero`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `revista_genero` (
  `codigo_ean` bigint(20) NOT NULL,
  `id_genero` int(11) NOT NULL,
  PRIMARY KEY (`codigo_ean`,`id_genero`),
  KEY `FK_RG_GENERO` (`id_genero`),
  CONSTRAINT `FK_RG_GENERO` FOREIGN KEY (`id_genero`) REFERENCES `genero` (`id_genero`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `revista_genero`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `revista_genero` WRITE;
/*!40000 ALTER TABLE `revista_genero` DISABLE KEYS */;
INSERT INTO `revista_genero` VALUES
(9771234567010,15),
(9771234567010,18),
(9772007490001,18);
/*!40000 ALTER TABLE `revista_genero` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `revistas`
--

DROP TABLE IF EXISTS `revistas`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `revistas` (
  `codigo_ean` bigint(20) NOT NULL,
  `id_ubicacion` int(11) NOT NULL,
  `id_mueble` int(11) NOT NULL,
  `id_proveedor` int(11) NOT NULL,
  `nombre_revista` varchar(255) NOT NULL,
  `numero_edicion` int(11) DEFAULT NULL,
  `periodicidad` varchar(50) DEFAULT NULL,
  `precio` double NOT NULL,
  `cantidad` int(11) NOT NULL,
  `SKU` bigint(20) DEFAULT NULL,
  PRIMARY KEY (`codigo_ean`,`id_ubicacion`),
  KEY `FK_REVISTAS_MUEBLE` (`id_mueble`),
  KEY `FK_REVISTAS_PROVEEDOR` (`id_proveedor`),
  KEY `FK_REVISTAS_UBICACION` (`id_ubicacion`),
  CONSTRAINT `FK_REVISTAS_MUEBLE` FOREIGN KEY (`id_mueble`) REFERENCES `mueble` (`id_Mueble`),
  CONSTRAINT `FK_REVISTAS_PROVEEDOR` FOREIGN KEY (`id_proveedor`) REFERENCES `proveedores` (`id_proveedor`),
  CONSTRAINT `FK_REVISTAS_UBICACION` FOREIGN KEY (`id_ubicacion`) REFERENCES `ubicacion` (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `revistas`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `revistas` WRITE;
/*!40000 ALTER TABLE `revistas` DISABLE KEYS */;
INSERT INTO `revistas` VALUES
(9771234567010,1,11,2,'National Geographic Mar 2026',345,'Mensual',120,10,20001),
(9771234567010,2,11,2,'National Geographic Mar 2026',345,'Mensual',120,35,20001),
(9772007490001,1,11,4,'Muy Interesante Mexico Ed. 452',452,'Mensual',65,15,901001),
(9772007490001,2,11,4,'Muy Interesante Mexico Ed. 452',452,'Mensual',65,40,901001);
/*!40000 ALTER TABLE `revistas` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `roles`
--

DROP TABLE IF EXISTS `roles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `roles` (
  `id_roles` int(11) NOT NULL,
  `nombre_rol` varchar(50) NOT NULL,
  PRIMARY KEY (`id_roles`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `roles`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `roles` WRITE;
/*!40000 ALTER TABLE `roles` DISABLE KEYS */;
INSERT INTO `roles` VALUES
(1,'Jefe Departamento'),
(2,'Bodega'),
(3,'Vendedor'),
(4,'Gerente');
/*!40000 ALTER TABLE `roles` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `ubicacion`
--

DROP TABLE IF EXISTS `ubicacion`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `ubicacion` (
  `id_ubicacion` int(11) NOT NULL,
  `lugar` varchar(50) NOT NULL,
  PRIMARY KEY (`id_ubicacion`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ubicacion`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `ubicacion` WRITE;
/*!40000 ALTER TABLE `ubicacion` DISABLE KEYS */;
INSERT INTO `ubicacion` VALUES
(1,'Piso'),
(2,'Bodega');
/*!40000 ALTER TABLE `ubicacion` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `usuarios`
--

DROP TABLE IF EXISTS `usuarios`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `usuarios` (
  `id_usuarios` bigint(20) NOT NULL,
  `id_roles` int(11) NOT NULL,
  `nombre` varchar(50) NOT NULL,
  `apellido_paterno` varchar(50) DEFAULT NULL,
  `apellido_materno` varchar(50) DEFAULT NULL,
  `telefono` bigint(20) DEFAULT NULL,
  `contrasena` varchar(255) NOT NULL,
  PRIMARY KEY (`id_usuarios`),
  KEY `FK_USUARIOS_ROLES` (`id_roles`),
  CONSTRAINT `FK_USUARIOS_ROLES` FOREIGN KEY (`id_roles`) REFERENCES `roles` (`id_roles`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `usuarios`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `usuarios` WRITE;
/*!40000 ALTER TABLE `usuarios` DISABLE KEYS */;
INSERT INTO `usuarios` VALUES
(1001,4,'Valeria','Morales','Gomez',8441112233,'$2b$10$GYRtA19hDIFbxL/ktJsMluzBL6RtxZu4KFSIOClBq14aG0Mvyh2zy'),
(350976899,1,'Marco','Perez','Luevano',8444578906,'$2b$10$JEis.kKv578RybJ7CPTKzOnzwJMf7iUbQWZwnpgSPmii0Rv8QLpyC'),
(628777129,3,'Aldo','Soto','Velez',8442836877,'$2b$10$9ZdsD93wmahmAOvWoEJ5SeI3yDWTZMNzlutEVfiiNbj4ZTgLPNUZy'),
(628777130,2,'Ian','Herrera','Ramirez',8449970912,'$2b$10$okBEvf28NTEBI3eA/jrO3euCYnsqKpk4w2T0eOdpNFucX4Y6pSUPq');
/*!40000 ALTER TABLE `usuarios` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Table structure for table `ventas`
--

DROP TABLE IF EXISTS `ventas`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `ventas` (
  `id_venta` bigint(20) NOT NULL AUTO_INCREMENT,
  `id_usuarios` bigint(20) NOT NULL,
  `nombre_cliente` varchar(150) NOT NULL DEFAULT 'Público en General',
  `fecha_hora` datetime NOT NULL DEFAULT current_timestamp(),
  `total` double NOT NULL DEFAULT 0,
  PRIMARY KEY (`id_venta`),
  KEY `FK_VENTAS_USUARIOS` (`id_usuarios`),
  CONSTRAINT `FK_VENTAS_USUARIOS` FOREIGN KEY (`id_usuarios`) REFERENCES `usuarios` (`id_usuarios`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ventas`
--

SET @OLD_AUTOCOMMIT=@@AUTOCOMMIT, @@AUTOCOMMIT=0;
LOCK TABLES `ventas` WRITE;
/*!40000 ALTER TABLE `ventas` DISABLE KEYS */;
/*!40000 ALTER TABLE `ventas` ENABLE KEYS */;
UNLOCK TABLES;
COMMIT;
SET AUTOCOMMIT=@OLD_AUTOCOMMIT;

--
-- Final view structure for view `proveedor`
--

/*!50001 DROP VIEW IF EXISTS `proveedor`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_general_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`root`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `proveedor` AS select `proveedores`.`id_proveedor` AS `id_proveedor`,`proveedores`.`nombre_proveedor` AS `nombre_proveedor`,`proveedores`.`rfc` AS `rfc`,`proveedores`.`telefono` AS `telefono`,`proveedores`.`correo` AS `correo`,`proveedores`.`direccion` AS `direccion`,`proveedores`.`persona_contacto` AS `persona_contacto`,`proveedores`.`estatus` AS `estatus` from `proveedores` */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

-- Dump completed on 2026-10-22 20:17:12
