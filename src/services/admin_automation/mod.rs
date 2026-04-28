/* [274A-46] Servicio de automatizacion admin: usa app_config como reemplazo
 * real de las options WP (`*_enabled`) y conserva historial desde lotes. */

use std::collections::BTreeMap;

use sqlx::PgPool;

use crate::errors::AppError;
use crate::repositories::{
    AdminAutomationRepository, AppConfigEntry, AppConfigRepository, AutomationBatchRow,
};

const PROCESS_EXTRACCION: &str = "extraccion";
const PROCESS_SCRAPING: &str = "scraping";
const DEFAULT_EXTRACCION_LIMIT: i32 = 20;
const DEFAULT_SCRAPING_LIMIT: i32 = 10;
const DEFAULT_EXTRACCION_INTERVAL_SECONDS: i32 = 60;
const DEFAULT_SCRAPING_INTERVAL_SECONDS: i32 = 900;

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

#[derive(Debug, Clone)]
pub struct ReactivationOutcome {
    pub mensaje: String,
}

impl AdminAutomationService {
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

        let enabled_key = enabled_config_key(process_type);
        AppConfigRepository::upsert(pool, enabled_key, &serde_json::Value::from(true)).await?;

        if process_type == PROCESS_SCRAPING {
            AppConfigRepository::upsert(
                pool,
                "scraping_fallos_consecutivos",
                &serde_json::Value::from(0),
            )
            .await?;
        }

        let label = process_label(process_type);
        tracing::info!(tipo = process_type, "proceso automatizado reactivado");

        Ok(ReactivationOutcome {
            mensaje: format!("{label} reactivado correctamente."),
        })
    }

    async fn status_for(
        pool: &PgPool,
        process_type: &str,
    ) -> Result<AutomationTypeStatus, AppError> {
        let entries =
            AppConfigRepository::list_by_prefix(pool, config_prefix(process_type)).await?;
        let values = config_values(&entries);
        let latest_batch = AdminAutomationRepository::latest_batch(pool, process_type).await?;
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
