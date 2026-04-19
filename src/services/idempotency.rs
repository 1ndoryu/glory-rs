/* [174A-29] Store genérico de idempotencia.
 * Redis cuando está disponible; fallback en memoria para desarrollo local.
 * Guarda payload JSON serializado por namespace + key, con expiración lazy.
 */

use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

use dashmap::DashMap;
use deadpool_redis::Pool as RedisPool;
use redis::AsyncCommands;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::AppError;

static MEMORY_IDEMPOTENCY: LazyLock<Arc<DashMap<String, (String, Instant)>>> =
    LazyLock::new(|| Arc::new(DashMap::new()));

pub struct IdempotencyStore;

impl IdempotencyStore {
    pub fn sanitize_key(raw: &str) -> Result<String, AppError> {
        let sanitized: String = raw
            .chars()
            .filter(|char| char.is_ascii_alphanumeric() || *char == '-')
            .take(64)
            .collect();
        if sanitized.is_empty() {
            return Err(AppError::BadRequest("X-Idempotency-Key invalido".into()));
        }
        Ok(sanitized)
    }

    pub async fn get_json<T: DeserializeOwned>(
        redis: &Option<RedisPool>,
        namespace: &str,
        key: &str,
    ) -> Result<Option<T>, AppError> {
        let namespaced = format!("idem:{namespace}:{key}");
        if let Some(pool) = redis {
            let mut conn = pool
                .get()
                .await
                .map_err(|error| AppError::Internal(format!("Redis pool: {error}")))?;
            let payload: Option<String> = conn
                .get(&namespaced)
                .await
                .map_err(|error| AppError::Internal(format!("Redis GET: {error}")))?;
            payload
                .map(|json| {
                    serde_json::from_str::<T>(&json).map_err(|error| {
                        AppError::Internal(format!("deserializar idempotencia: {error}"))
                    })
                })
                .transpose()
        } else {
            cleanup_memory();
            MEMORY_IDEMPOTENCY
                .get(&namespaced)
                .filter(|entry| Instant::now() < entry.value().1)
                .map(|entry| {
                    serde_json::from_str::<T>(&entry.value().0).map_err(|error| {
                        AppError::Internal(format!("deserializar idempotencia: {error}"))
                    })
                })
                .transpose()
        }
    }

    pub async fn set_json<T: Serialize>(
        redis: &Option<RedisPool>,
        namespace: &str,
        key: &str,
        ttl_secs: u64,
        value: &T,
    ) -> Result<(), AppError> {
        let namespaced = format!("idem:{namespace}:{key}");
        let payload = serde_json::to_string(value)
            .map_err(|error| AppError::Internal(format!("serializar idempotencia: {error}")))?;
        if let Some(pool) = redis {
            let mut conn = pool
                .get()
                .await
                .map_err(|error| AppError::Internal(format!("Redis pool: {error}")))?;
            let _: () = conn
                .set_ex(&namespaced, payload, ttl_secs)
                .await
                .map_err(|error| AppError::Internal(format!("Redis SETEX: {error}")))?;
        } else {
            MEMORY_IDEMPOTENCY.insert(
                namespaced,
                (payload, Instant::now() + Duration::from_secs(ttl_secs)),
            );
            cleanup_memory();
        }
        Ok(())
    }
}

fn cleanup_memory() {
    let now = Instant::now();
    MEMORY_IDEMPOTENCY.retain(|_, (_, expires_at)| *expires_at > now);
}
