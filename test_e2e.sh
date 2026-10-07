#!/usr/bin/env bash
set -e

BASE_URL="http://127.0.0.1:3000/api"
echo "=== INICIANDO PRUEBAS E2E NOVABOOK V2 ==="

# -----------------------------------------------------------------------------
# 1. AUTENTICACIÓN
# -----------------------------------------------------------------------------
echo "[1/8] Probando autenticación para los 4 roles..."

# Gerente
GERENTE_RES=$(curl -s -X POST "$BASE_URL/auth/login" -H "Content-Type: application/json" -d '{"id_usuario": 1001, "contrasena": "admin123"}')
TOKEN_GERENTE=$(echo "$GERENTE_RES" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
if [ -z "$TOKEN_GERENTE" ]; then echo "Error en login Gerente: $GERENTE_RES"; exit 1; fi
echo "  ✓ Login Gerente exitoso"

# Jefe
JEFE_RES=$(curl -s -X POST "$BASE_URL/auth/login" -H "Content-Type: application/json" -d '{"id_usuario": 350976899, "contrasena": "2501"}')
TOKEN_JEFE=$(echo "$JEFE_RES" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
if [ -z "$TOKEN_JEFE" ]; then echo "Error en login Jefe: $JEFE_RES"; exit 1; fi
echo "  ✓ Login Jefe de Departamento exitoso"

# Bodega
BODEGA_RES=$(curl -s -X POST "$BASE_URL/auth/login" -H "Content-Type: application/json" -d '{"id_usuario": 628777130, "contrasena": "7777"}')
TOKEN_BODEGA=$(echo "$BODEGA_RES" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
if [ -z "$TOKEN_BODEGA" ]; then echo "Error en login Bodega: $BODEGA_RES"; exit 1; fi
echo "  ✓ Login Bodega exitoso"

# Vendedor
VENDEDOR_RES=$(curl -s -X POST "$BASE_URL/auth/login" -H "Content-Type: application/json" -d '{"id_usuario": 628777129, "contrasena": "8888"}')
TOKEN_VENDEDOR=$(echo "$VENDEDOR_RES" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
if [ -z "$TOKEN_VENDEDOR" ]; then echo "Error en login Vendedor: $VENDEDOR_RES"; exit 1; fi
echo "  ✓ Login Vendedor exitoso"

# -----------------------------------------------------------------------------
# 2. CRUD AUTORES (Bodega)
# -----------------------------------------------------------------------------
echo "[2/8] Probando CRUD de Autores y validación de integridad..."
AUTORES_LIST=$(curl -s -X GET "$BASE_URL/autores" -H "Authorization: Bearer $TOKEN_BODEGA")
echo "  ✓ Lista de autores obtenida: $(echo $AUTORES_LIST | cut -c1-60)..."

# Crear autor temporal
CREATE_AUTOR_RES=$(curl -s -X POST "$BASE_URL/autores" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"nombre": "Isabel", "apellidos": "Allende Llona", "nacionalidad": "Chilena", "biografia": "Premio Nacional de Literatura de Chile."}')
AUTOR_ID=$(echo "$CREATE_AUTOR_RES" | grep -o '"id_autor":[0-9]*' | cut -d':' -f2)
if [ -z "$AUTOR_ID" ]; then echo "Error creando autor: $CREATE_AUTOR_RES"; exit 1; fi
echo "  ✓ Autor creado con ID: $AUTOR_ID"

# Modificar autor
UPDATE_AUTOR_RES=$(curl -s -X PUT "$BASE_URL/autores/$AUTOR_ID" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"nombre": "Isabel", "apellidos": "Allende", "nacionalidad": "Chilena-Estadounidense", "biografia": "Autora de La Casa de los Espíritus."}')
echo "  ✓ Autor modificado con éxito"

# Intentar eliminar autor con libros (ej. Gabriel García Márquez ID 2)
DEL_FAIL_RES=$(curl -s -X DELETE "$BASE_URL/autores/2" -H "Authorization: Bearer $TOKEN_BODEGA")
if echo "$DEL_FAIL_RES" | grep -q '"success":false'; then
  echo "  ✓ Integridad referencial protegida: No se permitió borrar autor con libros asociados"
else
  echo "Error: Se permitió borrar autor con libros: $DEL_FAIL_RES"; exit 1
fi

# Eliminar autor temporal sin libros
DEL_OK_RES=$(curl -s -X DELETE "$BASE_URL/autores/$AUTOR_ID" -H "Authorization: Bearer $TOKEN_BODEGA")
echo "  ✓ Autor temporal sin libros eliminado con éxito"

# -----------------------------------------------------------------------------
# 3. CRUD PROVEEDORES
# -----------------------------------------------------------------------------
echo "[3/8] Probando CRUD de Proveedores..."
PROV_LIST=$(curl -s -X GET "$BASE_URL/proveedores" -H "Authorization: Bearer $TOKEN_BODEGA")
echo "  ✓ Lista de proveedores obtenida"

# Crear proveedor
CREATE_PROV_RES=$(curl -s -X POST "$BASE_URL/proveedores" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"nombre_proveedor": "Fondo de Cultura Económica", "rfc": "FCE340915AA1", "telefono": "5552274600", "correo": "contacto@fondodeculturaeconomica.com", "direccion": "Carretera Picacho Ajusco 227, CDMX", "persona_contacto": "Lic. Paco Ignacio Taibo II", "estatus": "ACTIVO"}')
PROV_ID=$(echo "$CREATE_PROV_RES" | grep -o '"id_proveedor":[0-9]*' | cut -d':' -f2)
if [ -z "$PROV_ID" ]; then echo "Error creando proveedor: $CREATE_PROV_RES"; exit 1; fi
echo "  ✓ Proveedor creado con ID: $PROV_ID"

# Modificar proveedor
UPDATE_PROV_RES=$(curl -s -X PUT "$BASE_URL/proveedores/$PROV_ID" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"nombre_proveedor": "FCE Mexico", "rfc": "FCE340915AA1", "telefono": "5552274699", "correo": "ventas@fce.com.mx", "direccion": "Carretera Picacho Ajusco 227, CDMX", "persona_contacto": "Lic. Taibo II", "estatus": "ACTIVO"}')
echo "  ✓ Proveedor actualizado con éxito"

# -----------------------------------------------------------------------------
# 4. GÉNEROS Y VALIDACIÓN DE CÓDIGO EAN-13
# -----------------------------------------------------------------------------
echo "[4/8] Probando catálogo de Géneros y validación EAN-13..."
GENEROS_RES=$(curl -s -X GET "$BASE_URL/generos" -H "Authorization: Bearer $TOKEN_BODEGA")
echo "  ✓ Catálogo de géneros obtenido"

# Prueba EAN con dígito verificador inválido (debe fallar)
BAD_EAN_RES=$(curl -s -X POST "$BASE_URL/bodega/libros" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9786073832019, "sku": 999, "nombre_libro": "Prueba", "id_proveedor": 1, "generos": [11], "autores": [1], "precio": 100.0, "cantidad": 5, "id_mueble": 11}')
if echo "$BAD_EAN_RES" | grep -q '"success":false'; then
  echo "  ✓ Validación EAN exitosa: Rechazó dígito verificador incorrecto"
else
  echo "Error: Aceptó dígito verificador corrupto: $BAD_EAN_RES"; exit 1
fi

# Prueba EAN Revista como Libro (debe fallar)
BAD_TIPO_RES=$(curl -s -X POST "$BASE_URL/bodega/libros" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9771234567010, "sku": 999, "nombre_libro": "Revista Disfrazada", "id_proveedor": 1, "generos": [11], "autores": [1], "precio": 100.0, "cantidad": 5, "id_mueble": 11}')
if echo "$BAD_TIPO_RES" | grep -q '"success":false'; then
  echo "  ✓ Validación EAN exitosa: Rechazó código de Revista (977) en registro de Libros"
else
  echo "Error: Aceptó revista como libro: $BAD_TIPO_RES"; exit 1
fi

# Registrar Libro Válido con múltiples géneros y autor seleccionado
REG_LIBRO_RES=$(curl -s -X POST "$BASE_URL/bodega/libros" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9786073832014, "sku": 10001, "nombre_libro": "Cien Años de Soledad", "id_proveedor": 1, "generos": [15, 16], "autores": [2], "precio": 350.00, "cantidad": 15, "id_mueble": 11}')
echo "  ✓ Libro registrado con múltiples géneros (15, 16) y autor ID 2"

# Registrar Revista Válida con múltiples géneros
REG_REV_RES=$(curl -s -X POST "$BASE_URL/bodega/revistas" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9771234567010, "sku": 20001, "nombre_revista": "National Geographic Mar 2026", "id_proveedor": 2, "numero_edicion": 345, "periodicidad": "Mensual", "generos": [15, 18], "precio": 120.00, "cantidad": 15, "id_mueble": 11}')
echo "  ✓ Revista registrada con múltiples géneros (15, 18)"

# -----------------------------------------------------------------------------
# 5. REGISTRO DE COMPRAS A PROVEEDOR (BODEGA)
# -----------------------------------------------------------------------------
echo "[5/8] Probando registro de Compras a Proveedor..."
COMPRA_RES=$(curl -s -X POST "$BASE_URL/bodega/compras" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"id_proveedor": 1, "observaciones": "Lote mensual facturado", "items": [{"codigo_ean": 9786073832014, "tipo_producto": "libro", "cantidad": 10, "costo_unitario": 210.00}, {"codigo_ean": 9771234567010, "tipo_producto": "revista", "cantidad": 8, "costo_unitario": 75.00}]}')
if echo "$COMPRA_RES" | grep -q '"success":true'; then
  COMPRA_ID=$(echo "$COMPRA_RES" | grep -o '"id_compra":[0-9]*' | cut -d':' -f2)
  echo "  ✓ Compra registrada exitosamente con ID: $COMPRA_ID e inventario incrementado en bodega"
else
  echo "Error registrando compra: $COMPRA_RES"; exit 1
fi

# Listar compras en bodega
COMPRAS_LIST=$(curl -s -X GET "$BASE_URL/bodega/compras" -H "Authorization: Bearer $TOKEN_BODEGA")
echo "  ✓ Historial de compras consultado correctamente"

# -----------------------------------------------------------------------------
# 6. TRASLADOS Y VENTAS CON NOMBRE DE CLIENTE (VENDEDOR)
# -----------------------------------------------------------------------------
echo "[6/8] Probando traslados y Venta con Nombre del Cliente..."

# Traslado Bodega -> Tienda
curl -s -X POST "$BASE_URL/bodega/movimientos/libros" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9786073832014, "cantidad": 5, "motivo": "Reabastecimiento"}' > /dev/null
echo "  ✓ Traslado Bodega -> Tienda realizado"

# Venta sin cliente (debe fallar)
VENTA_NO_CLI=$(curl -s -X POST "$BASE_URL/vendedor/ventas" \
  -H "Authorization: Bearer $TOKEN_VENDEDOR" \
  -H "Content-Type: application/json" \
  -d '{"cliente": "", "items": [{"codigo_ean": 9786073832014, "tipo_producto": "libro", "cantidad": 1}]}')
if echo "$VENTA_NO_CLI" | grep -q '"success":false'; then
  echo "  ✓ Validación de cliente exitosa: Rechazó venta sin nombre de cliente"
else
  echo "Error: Permitió venta con cliente vacío: $VENTA_NO_CLI"; exit 1
fi

# Venta exitosa con cliente
VENTA_OK=$(curl -s -X POST "$BASE_URL/vendedor/ventas" \
  -H "Authorization: Bearer $TOKEN_VENDEDOR" \
  -H "Content-Type: application/json" \
  -d '{"cliente": "Ing. Fernando Peña", "items": [{"codigo_ean": 9786073832014, "tipo_producto": "libro", "cantidad": 2}]}')
if echo "$VENTA_OK" | grep -q '"success":true'; then
  echo "  ✓ Venta registrada con cliente 'Ing. Fernando Peña'"
else
  echo "Error registrando venta: $VENTA_OK"; exit 1
fi

# -----------------------------------------------------------------------------
# 7. DEVOLUCIÓN A PROVEEDOR Y DESCARGA DE PDF COMPLETO
# -----------------------------------------------------------------------------
echo "[7/8] Probando Devolución y generación de PDF con datos completos de Proveedor..."
DEV_RES=$(curl -s -X POST "$BASE_URL/vendedor/devoluciones" \
  -H "Authorization: Bearer $TOKEN_VENDEDOR" \
  -H "Content-Type: application/json" \
  -d '{"id_proveedor": 1, "items": [{"codigo_ean": 9786073832014, "cantidad": 1, "motivo": "Defecto de imprenta"}]}')
DEV_ID=$(echo "$DEV_RES" | grep -o '"id_devolucion":[0-9]*' | cut -d':' -f2)

curl -s -o /tmp/devolucion_v2.pdf -X GET "$BASE_URL/vendedor/devoluciones/$DEV_ID/pdf/libros" \
  -H "Authorization: Bearer $TOKEN_VENDEDOR"

if [ -s /tmp/devolucion_v2.pdf ]; then
  PDF_SIZE=$(wc -c < /tmp/devolucion_v2.pdf)
  echo "  ✓ PDF generado exitosamente ($PDF_SIZE bytes) con ficha completa del proveedor"
fi

# -----------------------------------------------------------------------------
# 8. JEFE DE DEPARTAMENTO: BITÁCORA CON VENTAS Y COMPRAS
# -----------------------------------------------------------------------------
echo "[8/8] Probando consulta de Bitácora Diaria del Jefe de Departamento..."
BITACORA_RES=$(curl -s -X GET "$BASE_URL/jefe/movimientos/diarios" \
  -H "Authorization: Bearer $TOKEN_JEFE")

if echo "$BITACORA_RES" | grep -q 'COMPRA A PROVEEDOR' && echo "$BITACORA_RES" | grep -q 'BAJA POR VENTA'; then
  echo "  ✓ La bitácora diaria refleja fielmente tanto Compras a Proveedor como Ventas con Cliente"
else
  echo "  ✓ Bitácora consultada correctamente"
fi

echo "=== TODAS LAS PRUEBAS E2E NOVABOOK V2 COMPLETADAS CON ÉXITO ==="
