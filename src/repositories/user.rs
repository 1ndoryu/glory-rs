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
        sqlx::query_as!(
            User,
            r#"INSERT INTO users (id, email, password_hash)
             VALUES ($1, $2, $3)
             RETURNING id, email, password_hash,
                       role as "role: UserRole", active_role as "active_role: UserRole",
                       email_verified, status, avatar_url, display_name, created_at"#,
            id,
            email,
            password_hash,
        )
        .fetch_one(pool)
        .await
    }

    /// Busca un usuario por email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash,
                      role as "role: UserRole", active_role as "active_role: UserRole",
                      email_verified, status, avatar_url, display_name, created_at
             FROM users WHERE email = $1"#,
            email,
        )
        .fetch_optional(pool)
        .await
    }

    /// Busca un usuario por ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash,
                      role as "role: UserRole", active_role as "active_role: UserRole",
                      email_verified, status, avatar_url, display_name, created_at
             FROM users WHERE id = $1"#,
            id,
        )
        .fetch_optional(pool)
        .await
    }

    /* [044A-38] Actualiza el active_role de un admin para cambiar genuinamente de vista */
    pub async fn update_active_role(
        pool: &PgPool,
        user_id: Uuid,
        active_role: Option<UserRole>,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"UPDATE users SET active_role = $2
             WHERE id = $1
             RETURNING id, email, password_hash,
                       role as "role: UserRole", active_role as "active_role: UserRole",
                       email_verified, status, avatar_url, display_name, created_at"#,
            user_id,
            active_role as Option<UserRole>,
        )
        .fetch_one(pool)
        .await
    }

    /* [044A-43] Actualiza la URL del avatar del usuario */
    pub async fn update_avatar(
        pool: &PgPool,
        user_id: Uuid,
        avatar_url: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE users SET avatar_url = $2 WHERE id = $1", user_id, avatar_url)
            .execute(pool)
            .await?;
        Ok(())
    }
}
