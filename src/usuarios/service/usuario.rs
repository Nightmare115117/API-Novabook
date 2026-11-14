use crate::{
    config::Config,
    db::{DbPool, log_bitacora, log_bitacora_tipo},
    error::AppError,
    middleware::generate_jwt,
    usuarios::{
        model::{
            ActualizarUsuarioRequest, CrearUsuarioRequest, LoginRequest, LoginResponse, RolEnum,
            UsuarioDto,
        },
        repo::UsuarioRepo,
    },
};

const MAX_PASSWORD_LENGTH: usize = 16;
const MAX_NAME_LENGTH: usize = 50;
const MAX_SURNAME_LENGTH: usize = 30;

fn validate_password(password: &str, required: bool) -> Result<(), AppError> {
    if required && password.trim().is_empty() {
        return Err(AppError::BadRequest(
            "La contraseña no puede estar vacía".to_string(),
        ));
    }
    if password.chars().count() > MAX_PASSWORD_LENGTH {
        return Err(AppError::BadRequest(
            "La contraseña no puede exceder 16 caracteres".to_string(),
        ));
    }
    if !password.is_empty() && password.trim().is_empty() {
        return Err(AppError::BadRequest(
            "La contraseña no puede contener solo espacios".to_string(),
        ));
    }
    Ok(())
}

fn validate_required_text(value: &str, field: &str, max_length: usize) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::BadRequest(format!("{field} es obligatorio")));
    }
    if value.chars().count() > max_length {
        return Err(AppError::BadRequest(format!(
            "{field} no puede exceder {max_length} caracteres"
        )));
    }
    Ok(())
}

fn validate_optional_text(
    value: Option<&str>,
    field: &str,
    max_length: usize,
) -> Result<(), AppError> {
    if value.is_some_and(|text| text.chars().count() > max_length) {
        return Err(AppError::BadRequest(format!(
            "{field} no puede exceder {max_length} caracteres"
        )));
    }
    Ok(())
}

fn validate_phone(phone: Option<i64>) -> Result<(), AppError> {
    if phone.is_some_and(|value| !(1_000_000_000..=9_999_999_999).contains(&value)) {
        return Err(AppError::BadRequest(
            "El teléfono debe contener exactamente 10 dígitos".to_string(),
        ));
    }
    Ok(())
}

pub struct UsuarioService;

impl UsuarioService {
    /// Autentica al usuario verificando credenciales y emite un JWT
    pub async fn login(
        pool: &DbPool,
        config: &Config,
        req: LoginRequest,
    ) -> Result<LoginResponse, AppError> {
        if req.id_usuario <= 0 {
            return Err(AppError::BadRequest("ID de usuario inválido".to_string()));
        }
        validate_password(&req.contrasena, true)?;

        let usuario = UsuarioRepo::buscar_por_id(pool, req.id_usuario)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Credenciales incorrectas".to_string()))?;

        // Verificación de contraseña (admite hash bcrypt o texto plano compatible)
        let pass_valida = if usuario.contrasena.starts_with("$2b$")
            || usuario.contrasena.starts_with("$2a$")
            || usuario.contrasena.starts_with("$2y$")
        {
            let plano = req.contrasena.clone();
            let hash = usuario.contrasena.clone();
            tokio::task::spawn_blocking(move ||
                bcrypt::verify(&plano, &hash).unwrap_or(false)
            )
            .await
            .map_err(|e| AppError::Internal(format!("Error verificando contraseña: {}", e)))?
        } else {
            usuario.contrasena == req.contrasena
        };

        let activo = usuario.estado;

        if !pass_valida {
            return Err(AppError::Unauthorized(
                "Credenciales incorrectas".to_string(),
            ));
        }

        if !activo {
            return Err(AppError::Unauthorized(
                "Credenciales incorrectas".to_string(),
            ));
        }

        let rol_enum = RolEnum::from_id(usuario.id_roles).ok_or_else(|| {
            AppError::Internal(format!("Rol no válido en BD: {}", usuario.id_roles))
        })?;

        let token = generate_jwt(
            usuario.id_usuarios,
            rol_enum,
            &usuario.nombre,
            &config.jwt_secret,
            config.jwt_expiration_hours,
        )?;

        if let Err(error) = log_bitacora_tipo(
            pool,
            usuario.id_usuarios,
            2,
            "INICIO DE SESIÓN",
            &format!("Inicio de sesión exitoso con rol: {}", rol_enum.as_str()),
        )
        .await
        {
            eprintln!(
                "[AUDIT ERROR] No se pudo registrar inicio de sesión para usuario {}: {error}",
                usuario.id_usuarios
            );
        }

        let dto: UsuarioDto = usuario.into();

        Ok(LoginResponse {
            token,
            token_type: "Bearer".to_string(),
            expires_in_hours: config.jwt_expiration_hours,
            usuario: dto,
        })
    }

    pub async fn obtener_perfil(pool: &DbPool, id_usuario: i64) -> Result<UsuarioDto, AppError> {
        let usuario = UsuarioRepo::buscar_por_id(pool, id_usuario)
            .await?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        Ok(usuario.into())
    }

    pub async fn listar_usuarios(pool: &DbPool) -> Result<Vec<UsuarioDto>, AppError> {
        let usuarios = UsuarioRepo::listar_todos(pool).await?;
        Ok(usuarios.into_iter().map(Into::into).collect())
    }

    pub async fn obtener_usuario(pool: &DbPool, id: i64) -> Result<UsuarioDto, AppError> {
        let usuario = UsuarioRepo::buscar_por_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Usuario #{} no encontrado", id)))?;

        Ok(usuario.into())
    }

    pub async fn crear_usuario(
        pool: &DbPool,
        actor_id: i64,
        req: CrearUsuarioRequest,
    ) -> Result<UsuarioDto, AppError> {
        if req.id_usuarios <= 0 {
            return Err(AppError::BadRequest(
                "ID de usuario debe ser mayor a 0".to_string(),
            ));
        }
        validate_required_text(&req.nombre, "El nombre", MAX_NAME_LENGTH)?;
        validate_optional_text(
            req.apellido_paterno.as_deref(),
            "El apellido paterno",
            MAX_SURNAME_LENGTH,
        )?;
        validate_optional_text(
            req.apellido_materno.as_deref(),
            "El apellido materno",
            MAX_SURNAME_LENGTH,
        )?;
        validate_phone(req.telefono)?;
        if RolEnum::from_id(req.id_roles).is_none() {
            return Err(AppError::BadRequest(format!(
                "Rol inválido: {}",
                req.id_roles
            )));
        }
        validate_password(&req.contrasena, true)?;

        if UsuarioRepo::existe(pool, req.id_usuarios).await? {
            return Err(AppError::Conflict(format!(
                "El usuario con ID {} ya se encuentra registrado",
                req.id_usuarios
            )));
        }

        let mut req = req;
        if !req.contrasena.starts_with("$2b$") && !req.contrasena.starts_with("$2a$") {
            req.contrasena = bcrypt::hash(&req.contrasena, 10)
                .map_err(|e| AppError::Internal(format!("Error al hashear contraseña: {}", e)))?;
        }

        UsuarioRepo::crear(pool, &req).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "CREAR USUARIO",
            &format!("Usuario creado: {} (ID: {})", req.nombre, req.id_usuarios),
        )
        .await;

        let usuario = UsuarioRepo::buscar_por_id(pool, req.id_usuarios)
            .await?
            .ok_or_else(|| {
                AppError::Internal("Error al recuperar usuario recién creado".to_string())
            })?;

        Ok(usuario.into())
    }

    pub async fn actualizar_usuario(
        pool: &DbPool,
        actor_id: i64,
        id: i64,
        req: ActualizarUsuarioRequest,
    ) -> Result<UsuarioDto, AppError> {
        let actual = UsuarioRepo::buscar_por_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Usuario #{} no encontrado", id)))?;

        if let Some(r) = req.id_roles
            && RolEnum::from_id(r).is_none() {
                return Err(AppError::BadRequest(format!("Rol inválido: {}", r)));
            }

        if let Some(nombre) = req.nombre.as_deref() {
            validate_required_text(nombre, "El nombre", MAX_NAME_LENGTH)?;
        }
        validate_optional_text(
            req.apellido_paterno.as_deref(),
            "El apellido paterno",
            MAX_SURNAME_LENGTH,
        )?;
        validate_optional_text(
            req.apellido_materno.as_deref(),
            "El apellido materno",
            MAX_SURNAME_LENGTH,
        )?;
        validate_phone(req.telefono)?;

        let mut req = req;
        if req.contrasena.as_deref() == Some("") {
            req.contrasena = None;
        }
        if let Some(pass) = req.contrasena.as_deref() {
            validate_password(pass, true)?;
        }
        if let Some(ref pass) = req.contrasena
            && !pass.starts_with("$2b$") && !pass.starts_with("$2a$") {
                req.contrasena = Some(bcrypt::hash(pass, 10).map_err(|e| {
                    AppError::Internal(format!("Error al hashear contraseña: {}", e))
                })?);
            }

        let actualizado = UsuarioRepo::actualizar(pool, id, &actual, &req).await?;

        let _ = log_bitacora(
            pool,
            actor_id,
            "ACTUALIZAR USUARIO",
            &format!("Usuario #{} actualizado", id),
        )
        .await;

        Ok(actualizado.into())
    }

    pub async fn eliminar_usuario(pool: &DbPool, actor_id: i64, id: i64) -> Result<(), AppError> {
        if id == actor_id {
            return Err(AppError::BadRequest(
                "No puedes eliminar tu propio usuario".to_string(),
            ));
        }

        let eliminado = UsuarioRepo::eliminar(pool, id).await?;
        if !eliminado {
            return Err(AppError::NotFound(format!("Usuario #{} no encontrado", id)));
        }

        let _ = log_bitacora(
            pool,
            actor_id,
            "ELIMINAR USUARIO",
            &format!("Usuario #{} eliminado", id),
        )
        .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        validate_optional_text, validate_password, validate_phone, validate_required_text,
    };

    #[test]
    fn password_accepts_maximum_length() {
        assert!(validate_password("abcdefghijklmnop", true).is_ok());
    }

    #[test]
    fn password_rejects_more_than_maximum_length() {
        assert!(validate_password("abcdefghijklmnopq", true).is_err());
    }

    #[test]
    fn required_password_rejects_blank_values() {
        assert!(validate_password("", true).is_err());
        assert!(validate_password("   ", true).is_err());
    }

    #[test]
    fn optional_password_accepts_empty_value() {
        assert!(validate_password("", false).is_ok());
    }

    #[test]
    fn names_enforce_required_and_schema_length_limits() {
        assert!(validate_required_text("Ana", "El nombre", 50).is_ok());
        assert!(validate_required_text(" ", "El nombre", 50).is_err());
        assert!(validate_required_text(&"a".repeat(51), "El nombre", 50).is_err());
        assert!(validate_optional_text(Some(&"a".repeat(30)), "Apellido", 30).is_ok());
        assert!(validate_optional_text(Some(&"a".repeat(31)), "Apellido", 30).is_err());
        assert!(validate_optional_text(None, "Apellido", 30).is_ok());
    }

    #[test]
    fn phone_must_have_exactly_ten_digits_when_provided() {
        assert!(validate_phone(None).is_ok());
        assert!(validate_phone(Some(8443509259)).is_ok());
        assert!(validate_phone(Some(844350925)).is_err());
        assert!(validate_phone(Some(10_000_000_000)).is_err());
    }
}
