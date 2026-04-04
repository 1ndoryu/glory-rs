use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{User, UserRole};

pub struct UserRepository;

impl UserRepository {
    /// Crea un usuario y retorna el registro completo
    pub async fn create(
        pool: &PgPool,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, email, password_hash) \
             VALUES ($1, $2, $3) \
             RETURNING id, email, password_hash, role, active_role, email_verified, status, created_at",
        )
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    /// Busca un usuario por email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, role, active_role, email_verified, status, created_at \
             FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await
    }

    /// Busca un usuario por ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, role, active_role, email_verified, status, created_at \
             FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /* [044A-38] Actualiza el active_role de un admin para cambiar genuinamente de vista */
    pub async fn update_active_role(
        pool: &PgPool,
        user_id: Uuid,
        active_role: Option<UserRole>,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET active_role = $2 \
             WHERE id = $1 \
             RETURNING id, email, password_hash, role, active_role, email_verified, status, created_at",
        )
        .bind(user_id)
        .bind(active_role)
        .fetch_one(pool)
        .await
    }
}
