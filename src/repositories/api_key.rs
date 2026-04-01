/* [283A-2] Repositorio de API keys.
 * [014A-11] Convertido de query_as runtime a query_as! macro para
 * verificación SQL en tiempo de compilación. */

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
        sqlx::query_as!(
            ApiKey,
            "INSERT INTO api_keys (id, user_id, nombre, key_hash, key_prefix) \
             VALUES ($1, $2, $3, $4, $5) \
             RETURNING id, user_id, nombre, key_hash, key_prefix, permisos, activa, last_used_at, created_at",
            id,
            user_id,
            nombre,
            key_hash,
            key_prefix
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_hash(pool: &PgPool, key_hash: &str) -> Result<Option<ApiKey>, sqlx::Error> {
        sqlx::query_as!(
            ApiKey,
            "SELECT id, user_id, nombre, key_hash, key_prefix, permisos, activa, last_used_at, created_at \
             FROM api_keys WHERE key_hash = $1",
            key_hash
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<ApiKey>, sqlx::Error> {
        sqlx::query_as!(
            ApiKey,
            "SELECT id, user_id, nombre, key_hash, key_prefix, permisos, activa, last_used_at, created_at \
             FROM api_keys WHERE user_id = $1 ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn revoke(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM api_keys WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_last_used(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE api_keys SET last_used_at = NOW() WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
