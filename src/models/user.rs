use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/* [044A-38] Roles de usuario: admin puede cambiar genuinamente entre roles vía active_role.
 * El effective_role se calcula: si admin tiene active_role, usa ese; si no, usa role real. */

/// Roles del sistema — mapea al enum `user_role` de `PostgreSQL`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Employee,
    Client,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "admin"),
            Self::Employee => write!(f, "employee"),
            Self::Client => write!(f, "client"),
        }
    }
}

/// Modelo de usuario almacenado en base de datos
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub active_role: Option<UserRole>,
    pub email_verified: bool,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Retorna el rol efectivo: si admin tiene `active_role`, usa ese; si no, role real
    #[must_use]
    pub fn effective_role(&self) -> UserRole {
        if self.role == UserRole::Admin {
            self.active_role.unwrap_or(self.role)
        } else {
            self.role
        }
    }
}

/// Response público de usuario — sin datos sensibles
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub effective_role: UserRole,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        let effective_role = user.effective_role();
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            effective_role,
            created_at: user.created_at,
        }
    }
}

/// Request body para registrar un nuevo usuario
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "Formato de email inválido"))]
    pub email: String,
    #[validate(length(min = 8, message = "La contraseña debe tener al menos 8 caracteres"))]
    pub password: String,
}

/// Request body para iniciar sesión
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

/// Response con token JWT después de autenticarse
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub role: UserRole,
    pub effective_role: UserRole,
}
