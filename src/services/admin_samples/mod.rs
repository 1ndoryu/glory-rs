/* [274A-53] Limpieza masiva admin de samples: replica el endpoint legacy
 * borrando assets de storage antes de eliminar los registros en cascada. */

use crate::errors::AppError;
use crate::repositories::{AdminSampleAssetRow, AdminSamplesRepository};
use crate::services::FileStorage;
use sqlx::PgPool;

pub struct AdminSamplesService;

#[derive(Debug, Clone)]
pub struct DeleteAllSamplesOutcome {
    pub eliminados: usize,
    pub errores: usize,
}

impl AdminSamplesService {
    pub async fn delete_all(
        pool: &PgPool,
        storage: &dyn FileStorage,
    ) -> Result<DeleteAllSamplesOutcome, AppError> {
        let samples = AdminSamplesRepository::list_all_for_deletion(pool).await?;
        if samples.is_empty() {
            return Ok(DeleteAllSamplesOutcome {
                eliminados: 0,
                errores: 0,
            });
        }

        let mut deletable_ids = Vec::with_capacity(samples.len());
        let mut errores = 0_usize;

        for sample in &samples {
            match delete_sample_assets(storage, sample).await {
                Ok(()) => deletable_ids.push(sample.id),
                Err(error) => {
                    errores += 1;
                    tracing::warn!(sample.id = sample.id, error = %error, "error borrando assets de sample en limpieza masiva");
                }
            }
        }

        let deleted_rows = AdminSamplesRepository::hard_delete_by_ids(pool, &deletable_ids).await?;
        let eliminados = usize::try_from(deleted_rows).map_err(|_| {
            AppError::Internal("cantidad de samples eliminados excede usize".into())
        })?;
        errores += deletable_ids.len().saturating_sub(eliminados);

        Ok(DeleteAllSamplesOutcome {
            eliminados,
            errores,
        })
    }
}

async fn delete_sample_assets(
    storage: &dyn FileStorage,
    sample: &AdminSampleAssetRow,
) -> Result<(), AppError> {
    for key in sample_asset_keys(sample) {
        storage.delete(&key).await?;
    }
    Ok(())
}

fn sample_asset_keys(sample: &AdminSampleAssetRow) -> Vec<String> {
    let mut keys = Vec::new();
    push_key(&mut keys, sample.ruta_original.as_deref());
    push_key(&mut keys, sample.ruta_optimizada.as_deref());
    push_key(&mut keys, sample.ruta_preview.as_deref());
    push_key(&mut keys, sample.ruta_waveform.as_deref());

    if let Some(original_key) = normalize_storage_key(sample.ruta_original.as_deref()) {
        if let Some(derived_key) = derived_waveform_json_key(&original_key) {
            push_unique(&mut keys, derived_key);
        }
    }

    keys
}

fn push_key(keys: &mut Vec<String>, raw: Option<&str>) {
    if let Some(key) = normalize_storage_key(raw) {
        push_unique(keys, key);
    }
}

fn push_unique(keys: &mut Vec<String>, key: String) {
    if !keys.iter().any(|existing| existing == &key) {
        keys.push(key);
    }
}

fn normalize_storage_key(raw: Option<&str>) -> Option<String> {
    let value = raw?.trim();
    if value.is_empty() {
        return None;
    }

    Some(value.trim_start_matches(['/', '\\']).replace('\\', "/"))
}

fn derived_waveform_json_key(original_key: &str) -> Option<String> {
    let (base, extension) = original_key.rsplit_once('.')?;
    if base.is_empty() || extension.eq_ignore_ascii_case("json") {
        return None;
    }
    Some(format!("{base}.json"))
}
