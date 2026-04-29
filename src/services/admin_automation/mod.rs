/* [274A-46] Servicio de automatizacion admin: usa app_config como reemplazo
 * real de las options WP (`*_enabled`) y conserva historial desde lotes. */

use std::collections::BTreeMap;

use chrono::{Duration as ChronoDuration, Utc};
use serde_json::Value;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::repositories::{
    AdminAutomationRepository, AppConfigEntry, AppConfigRepository, AutomationBatchRow,
    AutomationBatchUpdate,
};
use crate::services::AdminProcessService;

const PROCESS_EXTRACCION: &str = "extraccion";
const PROCESS_SCRAPING: &str = "scraping";
const DEFAULT_EXTRACCION_LIMIT: i32 = 20;
const DEFAULT_SCRAPING_LIMIT: i32 = 10;
const DEFAULT_EXTRACCION_INTERVAL_SECONDS: i32 = 60;
const DEFAULT_SCRAPING_INTERVAL_SECONDS: i32 = 900;
const EXTRACTION_FAILURE_THRESHOLD: i32 = 20;
const SCRAPING_IDLE_THRESHOLD: i32 = 5;
const STALE_BATCH_GRACE_SECONDS: i64 = 45;
const AUTOMATION_PROCESS_TYPES: [&str; 2] = [PROCESS_EXTRACCION, PROCESS_SCRAPING];

pub struct AdminAutomationService;

#[derive(Debug, Clone)]
pub struct AutomationTypeStatus {
    pub activo: bool,
    pub limite_por_lote: i32,
    pub intervalo_segundos: i32,
    pub fallos_consecutivos: i32,
    pub ultimo_lote: Option<AutomationBatchRow>,
}

#[derive(Debug, Clone)]
pub struct AutomationStatus {
    pub extraccion: AutomationTypeStatus,
    pub scraping: AutomationTypeStatus,
}

#[derive(Debug, Clone)]
pub struct AutomationHistory {
    pub items: Vec<AutomationBatchRow>,
    pub total: i64,
    pub pagina: i64,
}

pub struct AutomationBatchReport {
    pub batch_id: i64,
    pub exitosos: i64,
    pub fallidos: i64,
    pub recortes: Option<i64>,
    pub samples_publicados: Option<i64>,
    pub canciones_nuevas: Option<i64>,
    pub sampleos_nuevos: Option<i64>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ReactivationOutcome {
    pub mensaje: String,
}

impl AdminAutomationService {
    pub async fn run_due(pool: &PgPool) -> Result<(), AppError> {
        for process_type in AUTOMATION_PROCESS_TYPES {
            Self::run_due_for(pool, process_type).await?;
        }

        Ok(())
    }

    pub async fn status(pool: &PgPool) -> Result<AutomationStatus, AppError> {
        Ok(AutomationStatus {
            extraccion: Self::status_for(pool, PROCESS_EXTRACCION).await?,
            scraping: Self::status_for(pool, PROCESS_SCRAPING).await?,
        })
    }

    pub async fn history(
        pool: &PgPool,
        process_type: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> Result<AutomationHistory, AppError> {
        if let Some(value) = process_type {
            validate_process_type(value)?;
        }

        let safe_page = page.max(1);
        let offset = (safe_page - 1) * page_size;
        let items =
            AdminAutomationRepository::list_history(pool, process_type, page_size, offset).await?;
        let total = AdminAutomationRepository::count_history(pool, process_type).await?;

        Ok(AutomationHistory {
            items,
            total,
            pagina: safe_page,
        })
    }

    pub async fn reactivate(
        pool: &PgPool,
        process_type: &str,
    ) -> Result<ReactivationOutcome, AppError> {
        validate_process_type(process_type)?;

        AppConfigRepository::upsert(pool, enabled_config_key(process_type), &Value::from(true))
            .await?;

        if process_type == PROCESS_SCRAPING {
            AppConfigRepository::upsert(pool, "scraping_fallos_consecutivos", &Value::from(0))
                .await?;
        }

        let label = process_label(process_type);
        tracing::info!(tipo = process_type, "proceso automatizado reactivado");

        Ok(ReactivationOutcome {
            mensaje: format!("{label} reactivado correctamente."),
        })
    }

    pub async fn record_batch_report(
        pool: &PgPool,
        report: AutomationBatchReport,
    ) -> Result<(), AppError> {
        let Some(batch) = AdminAutomationRepository::find_batch(pool, report.batch_id)
            .await
            .map_err(|error| {
                AppError::Internal(format!("No se pudo leer lote {}: {error}", report.batch_id))
            })?
        else {
            return Err(AppError::BadRequest(format!(
                "Lote de automatizacion inexistente: {}",
                report.batch_id
            )));
        };

        let update = AutomationBatchUpdate {
            estado: final_batch_status(report.exitosos, report.fallidos).to_string(),
            exitosos: clamp_i64(report.exitosos),
            fallidos: clamp_i64(report.fallidos),
            recortes: clamp_i64(report.recortes.unwrap_or(report.exitosos)),
            samples_publicados: clamp_i64(report.samples_publicados.unwrap_or(0)),
            canciones_nuevas: clamp_i64(report.canciones_nuevas.unwrap_or(0)),
            sampleos_nuevos: clamp_i64(report.sampleos_nuevos.unwrap_or(0)),
            error_mensaje: build_batch_error_message(
                report.exitosos,
                report.fallidos,
                &report.metadata,
            ),
            metadata: report.metadata,
        };

        AdminAutomationRepository::complete_batch(pool, report.batch_id, &update)
            .await
            .map_err(|error| {
                AppError::Internal(format!(
                    "No se pudo completar lote {}: {error}",
                    report.batch_id
                ))
            })?;

        match batch.tipo.as_str() {
            PROCESS_EXTRACCION => {
                Self::evaluate_extraction_auto_stop(pool, update.exitosos, update.fallidos).await?;
            }
            PROCESS_SCRAPING => {
                Self::evaluate_scraping_auto_stop(pool, update.exitosos, update.fallidos).await?;
            }
            _ => {}
        }

        Ok(())
    }

    async fn status_for(
        pool: &PgPool,
        process_type: &str,
    ) -> Result<AutomationTypeStatus, AppError> {
        let entries =
            AppConfigRepository::list_by_prefix(pool, config_prefix(process_type)).await?;
        let values = config_values(&entries);
        let latest_batch = Self::reconcile_latest_batch(pool, process_type).await?;
        let fallback_failures =
            AdminAutomationRepository::consecutive_failures(pool, process_type).await?;

        Ok(AutomationTypeStatus {
            activo: bool_value(&values, enabled_config_key(process_type), true),
            limite_por_lote: i32_value(
                &values,
                limit_config_key(process_type),
                default_limit(process_type),
            ),
            intervalo_segundos: i32_value(
                &values,
                interval_config_key(process_type),
                default_interval_seconds(process_type),
            ),
            fallos_consecutivos: failure_count(&values, process_type).unwrap_or(fallback_failures),
            ultimo_lote: latest_batch,
        })
    }

    async fn run_due_for(pool: &PgPool, process_type: &str) -> Result<(), AppError> {
        let status = Self::status_for(pool, process_type).await?;
        if !status.activo {
            return Ok(());
        }

        if AdminProcessService::state(process_type)?.estado == "running" {
            return Ok(());
        }

        if !is_due(&status) {
            return Ok(());
        }

        let limit = u32::try_from(status.limite_por_lote.max(1)).ok();
        let response = AdminProcessService::start_automated(process_type, limit, pool).await?;
        tracing::info!(tipo = process_type, ?response, "lote automatico iniciado");
        Ok(())
    }

    async fn reconcile_latest_batch(
        pool: &PgPool,
        process_type: &str,
    ) -> Result<Option<AutomationBatchRow>, AppError> {
        let latest = AdminAutomationRepository::latest_batch(pool, process_type)
            .await
            .map_err(|error| {
                AppError::Internal(format!(
                    "No se pudo leer el ultimo lote de {process_type}: {error}"
                ))
            })?;

        let Some(batch) = latest else {
            return Ok(None);
        };

        if batch.estado != "ejecutando" {
            return Ok(Some(batch));
        }

        let process_state = AdminProcessService::state(process_type)?;
        let stale = process_state.estado != "running"
            && Utc::now()
                .signed_duration_since(batch.iniciado_at)
                .num_seconds()
                > STALE_BATCH_GRACE_SECONDS;

        if !stale {
            return Ok(Some(batch));
        }

        let report_status = if process_state.error.is_some() {
            "error"
        } else {
            "detenido"
        };
        let report_message = process_state.error.unwrap_or_else(|| {
            "El proceso termino sin reportar cierre del lote; no se contabilizo como fallo del scraper/extractor.".to_string()
        });

        let update = AutomationBatchUpdate {
            estado: report_status.to_string(),
            exitosos: batch.exitosos,
            fallidos: batch.fallidos,
            recortes: batch.recortes,
            samples_publicados: batch.samples_publicados,
            canciones_nuevas: batch.canciones_nuevas,
            sampleos_nuevos: batch.sampleos_nuevos,
            error_mensaje: Some(report_message),
            metadata: batch.metadata.clone(),
        };

        AdminAutomationRepository::complete_batch(pool, i64::from(batch.id), &update)
            .await
            .map_err(|error| {
                AppError::Internal(format!(
                    "No se pudo reconciliar lote colgado {}: {error}",
                    batch.id
                ))
            })?;

        AdminAutomationRepository::latest_batch(pool, process_type)
            .await
            .map_err(|error| {
                AppError::Internal(format!(
                    "No se pudo releer el ultimo lote de {process_type}: {error}"
                ))
            })
    }

    async fn evaluate_extraction_auto_stop(
        pool: &PgPool,
        exitosos: i32,
        fallidos: i32,
    ) -> Result<(), AppError> {
        if exitosos > 0 || fallidos < EXTRACTION_FAILURE_THRESHOLD {
            return Ok(());
        }

        AppConfigRepository::upsert(
            pool,
            enabled_config_key(PROCESS_EXTRACCION),
            &Value::from(false),
        )
        .await?;
        tracing::warn!(
            fallidos,
            "extraccion auto-detenida por fallos consecutivos del lote"
        );
        Ok(())
    }

    async fn evaluate_scraping_auto_stop(
        pool: &PgPool,
        exitosos: i32,
        fallidos: i32,
    ) -> Result<(), AppError> {
        if exitosos > 0 {
            AppConfigRepository::upsert(pool, "scraping_fallos_consecutivos", &Value::from(0))
                .await?;
            return Ok(());
        }

        let increment = fallidos.max(1);
        let current = AppConfigRepository::get(pool, "scraping_fallos_consecutivos")
            .await?
            .and_then(|entry| entry.valor.as_i64())
            .and_then(|value| i32::try_from(value).ok())
            .unwrap_or(0);
        let next = current + increment;

        AppConfigRepository::upsert(pool, "scraping_fallos_consecutivos", &Value::from(next))
            .await?;

        if next < SCRAPING_IDLE_THRESHOLD {
            return Ok(());
        }

        AppConfigRepository::upsert(
            pool,
            enabled_config_key(PROCESS_SCRAPING),
            &Value::from(false),
        )
        .await?;
        AppConfigRepository::upsert(pool, "scraping_fallos_consecutivos", &Value::from(0)).await?;
        tracing::warn!(
            fallos_acumulados = next,
            "scraping auto-detenido para evitar gasto de proxy"
        );
        Ok(())
    }
}

fn validate_process_type(process_type: &str) -> Result<(), AppError> {
    if matches!(process_type, PROCESS_EXTRACCION | PROCESS_SCRAPING) {
        Ok(())
    } else {
        Err(AppError::BadRequest(
            "tipo invalido: debe ser extraccion o scraping".into(),
        ))
    }
}

fn config_prefix(process_type: &str) -> &'static str {
    match process_type {
        PROCESS_EXTRACCION => "extraccion_",
        PROCESS_SCRAPING => "scraping_",
        _ => "",
    }
}

fn enabled_config_key(process_type: &str) -> &'static str {
    match process_type {
        PROCESS_EXTRACCION => "extraccion_enabled",
        PROCESS_SCRAPING => "scraping_enabled",
        _ => "",
    }
}

fn limit_config_key(process_type: &str) -> &'static str {
    match process_type {
        PROCESS_EXTRACCION => "extraccion_lote_size",
        PROCESS_SCRAPING => "scraping_lote_size",
        _ => "",
    }
}

fn interval_config_key(process_type: &str) -> &'static str {
    match process_type {
        PROCESS_EXTRACCION => "extraccion_intervalo_seg",
        PROCESS_SCRAPING => "scraping_intervalo_seg",
        _ => "",
    }
}

fn default_limit(process_type: &str) -> i32 {
    match process_type {
        PROCESS_EXTRACCION => DEFAULT_EXTRACCION_LIMIT,
        PROCESS_SCRAPING => DEFAULT_SCRAPING_LIMIT,
        _ => 0,
    }
}

fn default_interval_seconds(process_type: &str) -> i32 {
    match process_type {
        PROCESS_EXTRACCION => DEFAULT_EXTRACCION_INTERVAL_SECONDS,
        PROCESS_SCRAPING => DEFAULT_SCRAPING_INTERVAL_SECONDS,
        _ => 0,
    }
}

fn process_label(process_type: &str) -> &'static str {
    match process_type {
        PROCESS_EXTRACCION => "Extractor de Audio",
        PROCESS_SCRAPING => "Scraper WhoSampled",
        _ => "Proceso",
    }
}

fn config_values(entries: &[AppConfigEntry]) -> BTreeMap<&str, &serde_json::Value> {
    entries
        .iter()
        .map(|entry| (entry.clave.as_str(), &entry.valor))
        .collect()
}

fn bool_value(values: &BTreeMap<&str, &serde_json::Value>, key: &str, default: bool) -> bool {
    values
        .get(key)
        .and_then(|value| value.as_bool())
        .unwrap_or(default)
}

fn i32_value(values: &BTreeMap<&str, &serde_json::Value>, key: &str, default: i32) -> i32 {
    values
        .get(key)
        .and_then(|value| value.as_i64())
        .and_then(|value| i32::try_from(value).ok())
        .unwrap_or(default)
}

fn failure_count(values: &BTreeMap<&str, &serde_json::Value>, process_type: &str) -> Option<i32> {
    if process_type != PROCESS_SCRAPING {
        return None;
    }

    values
        .get("scraping_fallos_consecutivos")
        .and_then(|value| value.as_i64())
        .and_then(|value| i32::try_from(value).ok())
}

fn is_due(status: &AutomationTypeStatus) -> bool {
    if !status.activo {
        return false;
    }

    let Some(latest_batch) = status.ultimo_lote.as_ref() else {
        return true;
    };

    let base = latest_batch
        .completado_at
        .unwrap_or(latest_batch.iniciado_at);
    let next_run = base + ChronoDuration::seconds(i64::from(status.intervalo_segundos.max(1)));
    Utc::now() >= next_run
}

fn final_batch_status(exitosos: i64, fallidos: i64) -> &'static str {
    if exitosos > 0 || fallidos == 0 {
        "completado"
    } else {
        "error"
    }
}

fn clamp_i64(value: i64) -> i32 {
    i32::try_from(value).unwrap_or_else(|_| if value.is_negative() { 0 } else { i32::MAX })
}

fn build_batch_error_message(
    exitosos: i64,
    fallidos: i64,
    metadata: &Option<Value>,
) -> Option<String> {
    if final_batch_status(exitosos, fallidos) != "error" {
        return None;
    }

    metadata
        .as_ref()
        .and_then(|value| value.get("error_mensaje"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| Some("El lote termino sin exitos y con fallos reportados.".to_string()))
}
