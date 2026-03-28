/* [283A-2] Extractor de autenticación por API key para endpoints de chatbot.
 * Lee el header X-API-Key, hashea con SHA-256, busca en BD, verifica estado activo.
 * Actualiza last_used_at en background para no bloquear la respuesta. */

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::ApiKeyRepository;
use crate::AppState;

pub struct ApiKeyAuth {
    pub user_id: Uuid,
    pub api_key_id: Uuid,
}

/// Convierte una API key en su hash SHA-256 hexadecimal
#[must_use]
pub fn sha256_hex(input: &str) -> String {
    use std::fmt::Write;
    let hash = Sha256::digest(input.as_bytes());
    hash.iter().fold(String::with_capacity(64), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

#[async_trait]
impl FromRequestParts<AppState> for ApiKeyAuth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let key = parts
            .headers
            .get("X-API-Key")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let key_hash = sha256_hex(key);

        let api_key = ApiKeyRepository::find_by_hash(&state.pool, &key_hash)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or(AppError::Unauthorized)?;

        if !api_key.activa {
            return Err(AppError::Unauthorized);
        }

        /* Actualizar last_used_at en background — no bloquea la respuesta */
        let pool = state.pool.clone();
        let key_id = api_key.id;
        tokio::spawn(async move {
            let _ = ApiKeyRepository::update_last_used(&pool, key_id).await;
        });

        Ok(Self {
            user_id: api_key.user_id,
            api_key_id: api_key.id,
        })
    }
}
