/* [274A-43] Servicio real de backfill de duplicados: calcula SHA-256 desde
 * ruta_original y registra candidatos sin romper la unicidad de audio_hash. */

use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tokio::io::AsyncReadExt;
use utoipa::ToSchema;

use crate::errors::AppError;
use crate::repositories::AdminDuplicatesRepository;

pub struct AdminDuplicatesService;

#[derive(Debug, Clone, Default, Serialize, ToSchema)]
pub struct BackfillStats {
    pub procesados: i64,
    pub hasheados: i64,
    pub duplicados: i64,
    pub sin_archivo: i64,
}

impl AdminDuplicatesService {
    pub async fn run_hash_backfill(pool: &PgPool, batch: i64) -> Result<BackfillStats, AppError> {
        let limit = batch.clamp(10, 500);
        let pending = AdminDuplicatesRepository::list_samples_without_hash(pool, limit).await?;
        let mut stats = BackfillStats::default();

        for sample in pending {
            stats.procesados += 1;
            let Some(path) = sample
                .ruta_original
                .as_deref()
                .filter(|value| !value.trim().is_empty())
            else {
                stats.sin_archivo += 1;
                tracing::warn!(sample.id = sample.id, "backfill hash sin ruta_original");
                continue;
            };

            if !audio_file_exists(path).await {
                stats.sin_archivo += 1;
                tracing::warn!(sample.id = sample.id, path = %path, "backfill hash archivo inexistente");
                continue;
            }

            let audio_hash = match hash_file_sha256(path).await {
                Ok(hash) => hash,
                Err(error) => {
                    stats.sin_archivo += 1;
                    tracing::warn!(sample.id = sample.id, path = %path, error = %error, "backfill hash no pudo leer archivo");
                    continue;
                }
            };

            if let Some(existing) =
                AdminDuplicatesRepository::find_existing_hash_owner(pool, &audio_hash, sample.id)
                    .await?
            {
                let duplicate_kind = if existing.creador_id == sample.creador_id {
                    "mismo_usuario"
                } else {
                    "cross_usuario"
                };
                let inserted = AdminDuplicatesRepository::record_duplicate_from_backfill(
                    pool,
                    existing.id,
                    sample.id,
                    duplicate_kind,
                )
                .await?;
                if inserted {
                    stats.duplicados += 1;
                }
                continue;
            }

            let stored =
                AdminDuplicatesRepository::store_unique_hash(pool, sample.id, &audio_hash).await?;
            if stored {
                stats.hasheados += 1;
            } else if let Some(existing) =
                AdminDuplicatesRepository::find_existing_hash_owner(pool, &audio_hash, sample.id)
                    .await?
            {
                let duplicate_kind = if existing.creador_id == sample.creador_id {
                    "mismo_usuario"
                } else {
                    "cross_usuario"
                };
                let inserted = AdminDuplicatesRepository::record_duplicate_from_backfill(
                    pool,
                    existing.id,
                    sample.id,
                    duplicate_kind,
                )
                .await?;
                if inserted {
                    stats.duplicados += 1;
                }
            }
        }

        tracing::info!(
            procesados = stats.procesados,
            hasheados = stats.hasheados,
            duplicados = stats.duplicados,
            sin_archivo = stats.sin_archivo,
            "backfill hash completado"
        );

        Ok(stats)
    }
}

async fn audio_file_exists(path: &str) -> bool {
    tokio::fs::metadata(path)
        .await
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

async fn hash_file_sha256(path: &str) -> Result<String, std::io::Error> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let read_bytes = file.read(&mut buffer).await?;
        if read_bytes == 0 {
            break;
        }
        hasher.update(&buffer[..read_bytes]);
    }

    Ok(hex::encode(hasher.finalize()))
}
