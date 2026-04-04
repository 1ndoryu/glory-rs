use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, UserRole};
use crate::repositories::UserRepository;

/* [044A-38] Claims extendidos con role y effective_role.
 * El effective_role es el rol con el que el usuario opera: para admins
 * puede ser diferente de role si tienen active_role configurado. */

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub effective_role: UserRole,
    pub exp: usize,
}

pub struct AuthService;

impl AuthService {
    /// Registra un nuevo usuario: valida unicidad, hashea contraseña, genera JWT
    pub async fn register(
        pool: &PgPool,
        req: RegisterRequest,
        jwt_secret: &str,
    ) -> Result<AuthResponse, AppError> {
        if UserRepository::find_by_email(pool, &req.email)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict("Email ya registrado".into()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Error al hashear contraseña: {e}")))?
            .to_string();

        let user = UserRepository::create(pool, &req.email, &password_hash).await?;
        let effective = user.effective_role();
        let token = Self::generate_token(user.id, user.role, effective, jwt_secret)?;

        Ok(AuthResponse {
            token,
            user_id: user.id,
            role: user.role,
            effective_role: effective,
        })
    }

    /// Inicia sesión: verifica credenciales y genera JWT
    pub async fn login(
        pool: &PgPool,
        req: LoginRequest,
        jwt_secret: &str,
    ) -> Result<AuthResponse, AppError> {
        let user = UserRepository::find_by_email(pool, &req.email)
            .await?
            .ok_or(AppError::Unauthorized)?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::Internal(format!("Hash almacenado inválido: {e}")))?;

        Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized)?;

        let effective = user.effective_role();
        let token = Self::generate_token(user.id, user.role, effective, jwt_secret)?;

        Ok(AuthResponse {
            token,
            user_id: user.id,
            role: user.role,
            effective_role: effective,
        })
    }

    /// Genera un JWT con expiración de 24 horas, incluye role y `effective_role`
    pub fn generate_token(
        user_id: Uuid,
        role: UserRole,
        effective_role: UserRole,
        secret: &str,
    ) -> Result<String, AppError> {
        let timestamp = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .ok_or_else(|| AppError::Internal("Error calculando expiración del token".into()))?
            .timestamp();
        let exp = usize::try_from(timestamp)
            .map_err(|_| AppError::Internal("Timestamp fuera de rango".into()))?;

        let claims = Claims {
            sub: user_id,
            role,
            effective_role,
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("Error generando token: {e}")))
    }

    /// Verifica un JWT y retorna los claims
    pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
    }
}
