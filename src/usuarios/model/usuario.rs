use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolEnum {
    Gerente = 4,
    JefeDepartamento = 1,
    PersonalBodega = 2,
    Vendedor = 3,
}

impl RolEnum {
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            4 => Some(Self::Gerente),
            1 => Some(Self::JefeDepartamento),
            2 => Some(Self::PersonalBodega),
            3 => Some(Self::Vendedor),
            _ => None,
        }
    }

    pub fn to_id(&self) -> i32 {
        *self as i32
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gerente => "gerente",
            Self::JefeDepartamento => "jefe_departamento",
            Self::PersonalBodega => "personal_bodega",
            Self::Vendedor => "vendedor",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id_usuarios: i64,
    pub id_roles: i32,
    pub nombre: String,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub telefono: Option<i64>,
    pub contrasena: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsuarioDto {
    pub id_usuarios: i64,
    pub id_roles: i32,
    pub rol_nombre: String,
    pub nombre: String,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub telefono: Option<i64>,
}

impl From<Usuario> for UsuarioDto {
    fn from(u: Usuario) -> Self {
        let rol_nombre = RolEnum::from_id(u.id_roles)
            .map(|r| r.as_str().to_string())
            .unwrap_or_else(|| format!("Rol {}", u.id_roles));

        Self {
            id_usuarios: u.id_usuarios,
            id_roles: u.id_roles,
            rol_nombre,
            nombre: u.nombre,
            apellido_paterno: u.apellido_paterno,
            apellido_materno: u.apellido_materno,
            telefono: u.telefono,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrearUsuarioRequest {
    pub id_usuarios: i64,
    pub id_roles: i32,
    pub nombre: String,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub telefono: Option<i64>,
    pub contrasena: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActualizarUsuarioRequest {
    pub id_roles: Option<i32>,
    pub nombre: Option<String>,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub telefono: Option<i64>,
    pub contrasena: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub id_usuario: i64,
    pub contrasena: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in_hours: i64,
    pub usuario: UsuarioDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub id_usuario: i64,
    pub rol: RolEnum,
    pub nombre: String,
    pub exp: usize,
}
