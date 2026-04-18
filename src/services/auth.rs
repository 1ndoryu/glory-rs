use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use crate::repositories::UserRepository;

/* [174A-18] JWT claims sobre `usuarios_ext` (sub i32). */
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub iat: i64,
    pub plan: String,
    pub rol: String,
}

pub struct AuthService;

impl AuthService {
    pub async fn register(pool: &PgPool, req: RegisterRequest, jwt_secret: &str) -> Result<AuthResponse, AppError> {
        if UserRepository::find_by_email(pool, &req.email).await?.is_some() {
            return Err(AppError::Conflict("Email ya registrado".into()));
        }
        if UserRepository::find_by_username(pool, &req.username).await?.is_some() {
            return Err(AppError::Conflict("Username ya registrado".into()));
        }
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Hash error: {e}")))?
            .to_string();
        let nombre = req.nombre_visible.clone().unwrap_or_else(|| req.username.clone());
        let user = UserRepository::create_native(pool, &req.username, &req.email, &password_hash, &nombre).await?;
        let token = Self::generate_token(&user, jwt_secret)?;
        Ok(AuthResponse { token, user: UserResponse::from(user) })
    }

    pub async fn login(pool: &PgPool, req: LoginRequest, jwt_secret: &str) -> Result<AuthResponse, AppError> {
        let user = UserRepository::find_by_identifier(pool, &req.identifier).await?.ok_or(AppError::Unauthorized)?;
        if user.estado != "activo" {
            return Err(AppError::Forbidden(format!("Cuenta {}", user.estado)));
        }
        let stored = user.password_hash.as_deref().ok_or(AppError::Unauthorized)?;
        let parsed = PasswordHash::new(stored).map_err(|e| AppError::Internal(format!("Hash invalido: {e}")))?;
        Argon2::default().verify_password(req.password.as_bytes(), &parsed).map_err(|_| AppError::Unauthorized)?;
        let token = Self::generate_token(&user, jwt_secret)?;
        Ok(AuthResponse { token, user: UserResponse::from(user) })
    }

    pub fn generate_token(user: &crate::models::User, secret: &str) -> Result<String, AppError> {
        let now = chrono::Utc::now();
        let exp = now.checked_add_signed(chrono::Duration::hours(24))
            .ok_or_else(|| AppError::Internal("exp overflow".into()))?
            .timestamp();
        let claims = Claims { sub: user.id, exp, iat: now.timestamp(), plan: user.plan.clone(), rol: user.rol.clone() };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
            .map_err(|e| AppError::Internal(format!("Token gen: {e}")))
    }

    pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
            .map(|d| d.claims).map_err(|_| AppError::Unauthorized)
    }
}