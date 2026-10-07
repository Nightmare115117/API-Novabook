use crate::{
    config::Config,
    db::{DbPool, log_bitacora},
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
        if req.contrasena.trim().is_empty() {
            return Err(AppError::BadRequest(
                "La contraseña no puede estar vacía".to_string(),
            ));
        }

        let usuario = UsuarioRepo::buscar_por_id(pool, req.id_usuario)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Credenciales incorrectas".to_string()))?;

        // Verificación de contraseña (admite hash bcrypt o texto plano compatible)
        let pass_valida = if usuario.contrasena.starts_with("$2b$")
            || usuario.contrasena.starts_with("$2a$")
            || usuario.contrasena.starts_with("$2y$")
        {
            bcrypt::verify(&req.contrasena, &usuario.contrasena).unwrap_or(false)
        } else {
            usuario.contrasena == req.contrasena
        };

        if !pass_valida {
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

        let _ = log_bitacora(
            pool,
            usuario.id_usuarios,
            "INICIO DE SESIÓN",
            &format!("Inicio de sesión exitoso con rol: {}", rol_enum.as_str()),
        )
        .await;

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
        if req.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".to_string()));
        }
        if RolEnum::from_id(req.id_roles).is_none() {
            return Err(AppError::BadRequest(format!(
                "Rol inválido: {}",
                req.id_roles
            )));
        }

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

        let mut req = req;
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
