/* [283A-2] Repositorio de API keys.
 * Usa queries runtime (no macros) porque la tabla api_keys no está en el
 * cache offline .sqlx/ hasta que se ejecute la migración y cargo sqlx prepare. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::ApiKey;

pub struct ApiKeyRepository;

impl ApiKeyRepository {
    pub async fn create(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        nombre: &str,
        key_hash: &str,
        key_prefix: &str,
    ) -> Result<ApiKey, sqlx::Error> {
        sqlx::query_as::<_, ApiKey>(
            "INSERT INTO api_keys (id, user_id, nombre, key_hash, key_prefix) \
             VALUES ($1, $2, $3, $4, $5) \
             RETURNING id, user_id, nombre, key_hash, key_prefix, permisos, activa, last_used_at, created_at",
        )
        .bind(id)
        .bind(user_id)
        .bind(nombre)
        .bind(key_hash)
        .bind(key_prefix)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_hash(pool: &PgPool, key_hash: &str) -> Result<Option<ApiKey>, sqlx::Error> {
        sqlx::query_as::<_, ApiKey>(
            "SELECT id, user_id, nombre, key_hash, key_prefix, permisos, activa, last_used_at, created_at \
             FROM api_keys WHERE key_hash = $1",
        )
        .bind(key_hash)
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<ApiKey>, sqlx::Error> {
        sqlx::query_as::<_, ApiKey>(
            "SELECT id, user_id, nombre, key_hash, key_prefix, permisos, activa, last_used_at, created_at \
             FROM api_keys WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn revoke(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM api_keys WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_last_used(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE api_keys SET last_used_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
