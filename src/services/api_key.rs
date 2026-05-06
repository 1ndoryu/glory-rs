/* [283A-2] Servicio de API keys — genera keys criptográficamente seguras,
 * almacena el hash SHA-256, y retorna la key completa solo al crearla. */

use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::api_key_auth::sha256_hex;
use crate::models::{ApiKeyCreatedResponse, ApiKeyResponse, CrearApiKeyRequest};
use crate::repositories::ApiKeyRepository;

pub struct ApiKeyService;

impl ApiKeyService {
    /// Genera una API key segura y la almacena hasheada en BD.
    /// Retorna la key completa (solo visible una vez).
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearApiKeyRequest,
    ) -> Result<ApiKeyCreatedResponse, AppError> {
        let raw_key = Self::generate_key();
        let key_hash = sha256_hex(&raw_key);
        let key_prefix = raw_key.chars().take(12).collect::<String>();
        let id = Uuid::new_v4();

        let api_key =
            ApiKeyRepository::create(pool, id, user_id, &req.nombre, &key_hash, &key_prefix)
                .await?;

        Ok(ApiKeyCreatedResponse {
            id: api_key.id,
            nombre: api_key.nombre,
            key: raw_key,
            key_prefix: api_key.key_prefix,
            permisos: api_key.permisos,
            created_at: api_key.created_at,
        })
    }

    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<ApiKeyResponse>, AppError> {
        let keys = ApiKeyRepository::list_by_user(pool, user_id).await?;
        Ok(keys.into_iter().map(ApiKeyResponse::from).collect())
    }

    pub async fn revoke(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let deleted = ApiKeyRepository::revoke(pool, id, user_id).await?;
        if !deleted {
            return Err(AppError::NotFound("API key no encontrada".into()));
        }
        Ok(())
    }

    /// Genera una key con formato `grst_XXXX`  (grst = glory restaurant, 48 hex chars)
    fn generate_key() -> String {
        use std::fmt::Write;
        let mut rng = rand::thread_rng();
        let bytes: [u8; 24] = rng.gen();
        let hex = bytes.iter().fold(String::with_capacity(48), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        });
        format!("grst_{hex}")
    }
}
