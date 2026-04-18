use sqlx::PgPool;

use crate::models::User;

/* [174A-18] Repositorio sobre `usuarios_ext`. */
pub struct UserRepository;

const USER_COLS: &str = "id, username, email, nombre_visible, password_hash, plan, rol, estado, created_at";

impl UserRepository {
    pub async fn create_native(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
        nombre_visible: &str,
    ) -> Result<User, sqlx::Error> {
        let sql = format!(
            "INSERT INTO usuarios_ext (username, email, password_hash, nombre_visible) \
             VALUES ($1, $2, $3, $4) \
             RETURNING {USER_COLS}"
        );
        sqlx::query_as::<_, User>(&sql)
            .bind(username).bind(email).bind(password_hash).bind(nombre_visible)
            .fetch_one(pool).await
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let sql = format!("SELECT {USER_COLS} FROM usuarios_ext WHERE LOWER(email) = LOWER($1) LIMIT 1");
        sqlx::query_as::<_, User>(&sql).bind(email).fetch_optional(pool).await
    }

    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, sqlx::Error> {
        let sql = format!("SELECT {USER_COLS} FROM usuarios_ext WHERE username = $1 LIMIT 1");
        sqlx::query_as::<_, User>(&sql).bind(username).fetch_optional(pool).await
    }

    pub async fn find_by_identifier(pool: &PgPool, identifier: &str) -> Result<Option<User>, sqlx::Error> {
        if identifier.contains('@') { Self::find_by_email(pool, identifier).await }
        else { Self::find_by_username(pool, identifier).await }
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
        let sql = format!("SELECT {USER_COLS} FROM usuarios_ext WHERE id = $1");
        sqlx::query_as::<_, User>(&sql).bind(id).fetch_optional(pool).await
    }
}