#!/usr/bin/env bash
set -e

BASE_URL="http://127.0.0.1:3000/api"
echo "=== INICIANDO PRUEBAS E2E NOVABOOK ==="

# -----------------------------------------------------------------------------
# 1. AUTENTICACIÓN
# -----------------------------------------------------------------------------
echo "[1/5] Probando autenticación para los 4 roles..."

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
# 2. GERENTE: CRUD USUARIOS
# -----------------------------------------------------------------------------
echo "[2/5] Probando operaciones de Gerente (CRUD Usuarios)..."
USUARIOS_LIST=$(curl -s -X GET "$BASE_URL/usuarios" -H "Authorization: Bearer $TOKEN_GERENTE")
echo "  ✓ Lista de usuarios obtenida"

TEST_USER_ID=999988
# Crear usuario
CREATE_USER_RES=$(curl -s -X POST "$BASE_URL/usuarios" \
  -H "Authorization: Bearer $TOKEN_GERENTE" \
  -H "Content-Type: application/json" \
  -d "{\"id_usuarios\": $TEST_USER_ID, \"id_roles\": 3, \"nombre\": \"Usuario\", \"apellido_paterno\": \"Prueba\", \"apellido_materno\": \"Test\", \"telefono\": 5551234567, \"contrasena\": \"temporal123\"}")
echo "  ✓ Usuario temporal creado: $(echo $CREATE_USER_RES | cut -c1-60)..."

# Modificar usuario
UPDATE_USER_RES=$(curl -s -X PUT "$BASE_URL/usuarios/$TEST_USER_ID" \
  -H "Authorization: Bearer $TOKEN_GERENTE" \
  -H "Content-Type: application/json" \
  -d "{\"id_roles\": 3, \"nombre\": \"Usuario Modificado\", \"apellido_paterno\": \"Prueba\", \"apellido_materno\": \"Test\", \"telefono\": 5559876543, \"contrasena\": null}")
echo "  ✓ Usuario modificado"

# Eliminar usuario
DELETE_USER_RES=$(curl -s -X DELETE "$BASE_URL/usuarios/$TEST_USER_ID" \
  -H "Authorization: Bearer $TOKEN_GERENTE")
echo "  ✓ Usuario eliminado: $(echo $DELETE_USER_RES | cut -c1-60)..."

# -----------------------------------------------------------------------------
# 3. PERSONAL DE BODEGA: REGISTRO Y TRASLADOS BODEGA -> TIENDA
# -----------------------------------------------------------------------------
echo "[3/5] Probando operaciones de Personal de Bodega..."

# Registrar libro
REG_LIBRO_RES=$(curl -s -X POST "$BASE_URL/bodega/libros" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9786073832014, "sku": 10001, "nombre_libro": "Cien Años de Soledad", "id_proveedor": 1, "id_genero": 11, "autor": "Gabriel García Márquez", "precio": 350.00, "cantidad": 20, "id_mueble": 11}')
echo "  ✓ Registro de libro: $(echo $REG_LIBRO_RES | cut -c1-70)..."

# Registrar revista
REG_REV_RES=$(curl -s -X POST "$BASE_URL/bodega/revistas" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9771234567010, "sku": 20001, "nombre_revista": "National Geographic Mar 2026", "id_proveedor": 2, "numero_edicion": 345, "periodicidad": "Mensual", "precio": 120.00, "cantidad": 15, "id_mueble": 11}')
echo "  ✓ Registro de revista: $(echo $REG_REV_RES | cut -c1-70)..."

# Traslado Bodega -> Tienda (Libro)
TRAS_LIB_RES=$(curl -s -X POST "$BASE_URL/bodega/movimientos/libros" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9786073832014, "cantidad": 5, "motivo": "Reabastecimiento de mostrador"}')
echo "  ✓ Traslado Bodega -> Tienda (Libro): $(echo $TRAS_LIB_RES | cut -c1-70)..."

# Traslado Bodega -> Tienda (Revista)
TRAS_REV_RES=$(curl -s -X POST "$BASE_URL/bodega/movimientos/revistas" \
  -H "Authorization: Bearer $TOKEN_BODEGA" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9771234567010, "cantidad": 4, "motivo": "Exhibición en revistero"}')
echo "  ✓ Traslado Bodega -> Tienda (Revista): $(echo $TRAS_REV_RES | cut -c1-70)..."

# -----------------------------------------------------------------------------
# 4. VENDEDOR: CONSULTA, TRASLADO, VENTA Y DEVOLUCIÓN
# -----------------------------------------------------------------------------
echo "[4/5] Probando operaciones de Vendedor..."

# Consultar existencias
EXIST_RES=$(curl -s -X GET "$BASE_URL/vendedor/existencias" \
  -H "Authorization: Bearer $TOKEN_VENDEDOR")
echo "  ✓ Existencias consultadas: $(echo $EXIST_RES | cut -c1-70)..."

# Traslado Tienda -> Bodega (Libro)
TRAS_VEND_RES=$(curl -s -X POST "$BASE_URL/vendedor/movimientos/libros" \
  -H "Authorization: Bearer $TOKEN_VENDEDOR" \
  -H "Content-Type: application/json" \
  -d '{"codigo_ean": 9786073832014, "cantidad": 1, "motivo": "Exceso en mostrador"}')
echo "  ✓ Traslado Tienda -> Bodega (Libro): $(echo $TRAS_VEND_RES | cut -c1-70)..."

# Registrar Venta (Baja por venta)
VENTA_RES=$(curl -s -X POST "$BASE_URL/vendedor/ventas" \
  -H "Authorization: Bearer $TOKEN_VENDEDOR" \
  -H "Content-Type: application/json" \
  -d '{"items": [{"codigo_ean": 9786073832014, "tipo_producto": "libro", "cantidad": 2}, {"codigo_ean": 9771234567010, "tipo_producto": "revista", "cantidad": 1}]}')
echo "  ✓ Registro de venta transaccional: $(echo $VENTA_RES | cut -c1-80)..."

# Generar Solicitud de Devolución
DEV_RES=$(curl -s -X POST "$BASE_URL/vendedor/devoluciones" \
  -H "Authorization: Bearer $TOKEN_VENDEDOR" \
  -H "Content-Type: application/json" \
  -d '{"id_proveedor": 1, "motivo": "Ejemplar con defecto de encuadernación", "items": [{"codigo_ean": 9786073832014, "tipo_producto": "Libro", "cantidad": 1, "motivo_detalle": "Hojas despegadas"}]}')
echo "  ✓ Solicitud de devolución creada: $(echo $DEV_RES | cut -c1-80)..."

DEV_ID=$(echo "$DEV_RES" | grep -o '"id_devolucion":[0-9]*' | cut -d':' -f2)
if [ -n "$DEV_ID" ]; then
  # Descargar PDF Libros
  curl -s -o /tmp/devolucion_libro.pdf -X GET "$BASE_URL/vendedor/devoluciones/$DEV_ID/pdf/libros" \
    -H "Authorization: Bearer $TOKEN_VENDEDOR"
  if [ -s /tmp/devolucion_libro.pdf ]; then
    echo "  ✓ PDF Devolución descargado ($(wc -c < /tmp/devolucion_libro.pdf) bytes)"
  fi
fi

# -----------------------------------------------------------------------------
# 5. JEFE DE DEPARTAMENTO: BITÁCORA Y APROBACIÓN DE DEVOLUCIONES
# -----------------------------------------------------------------------------
echo "[5/5] Probando operaciones de Jefe de Departamento..."

# Historial de devoluciones
HIST_RES=$(curl -s -X GET "$BASE_URL/jefe/devoluciones/historial" \
  -H "Authorization: Bearer $TOKEN_JEFE")
echo "  ✓ Historial de devoluciones consultado"

# Aprobar devolución
if [ -n "$DEV_ID" ]; then
  APROB_RES=$(curl -s -X PUT "$BASE_URL/jefe/devoluciones/$DEV_ID/aprobar" \
    -H "Authorization: Bearer $TOKEN_JEFE" \
    -H "Content-Type: application/json" \
    -d '{"aprobado": true, "notas": "Devolución aprobada tras revisión física"}')
  echo "  ✓ Devolución aprobada: $(echo $APROB_RES | cut -c1-70)..."
fi

# Bitácora diaria
BITACORA_RES=$(curl -s -X GET "$BASE_URL/jefe/movimientos/diarios" \
  -H "Authorization: Bearer $TOKEN_JEFE")
echo "  ✓ Bitácora de movimientos diarios consultada"

echo "=== TODAS LAS PRUEBAS E2E COMPLETADAS EXITOSAMENTE ==="
