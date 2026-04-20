/* [174A-108b] Endpoints para el scraper Python (clients/kamples-scraper/).
 *
 * Reemplaza a los endpoints WordPress legados:
 *   - POST /wp-json/kamples/v1/dev/extraccion/publicar-auto
 *   - POST /wp-json/kamples/v1/admin/automatizacion/reporte-lote
 *
 * Auth: header `X-Kamples-Secret` comparado constant-time contra
 * `AppState.scraper_secret` (env var `SCRAPER_SECRET`).
 *
 * Si `scraper_secret` no está configurado, los endpoints responden
 * 403 Forbidden — protege contra exposición accidental.
 */

use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::errors::AppError;
use crate::AppState;

const HEADER_NAME: &str = "x-kamples-secret";

/// Resultado de la operación `publicar-auto`.
#[derive(Debug, Serialize, ToSchema)]
pub struct PublicarAutoResponse {
    /// Cantidad de items de `cola_extraccion_samples` que pasaron de
    /// `extraido` → `completado` en esta llamada.
    pub publicados: i64,
}

/// Payload del reporte de un lote de scraping.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ReporteLoteRequest {
    pub batch_id: i64,
    pub exitosos: i64,
    pub fallidos: i64,
    #[serde(default)]
    pub recortes: Option<i64>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Resultado del reporte de lote.
#[derive(Debug, Serialize, ToSchema)]
pub struct ReporteLoteResponse {
    pub ok: bool,
    pub batch_id: i64,
}

/* Validación constant-time del header `X-Kamples-Secret`. */
fn require_scraper_secret(state: &AppState, headers: &HeaderMap) -> Result<(), AppError> {
    let Some(expected) = state.scraper_secret.as_deref() else {
        return Err(AppError::Forbidden(
            "scraper endpoints disabled: SCRAPER_SECRET not configured".into(),
        ));
    };
    let provided = headers
        .get(HEADER_NAME)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    /* Comparación constant-time para evitar timing attacks. */
    let provided_bytes = provided.as_bytes();
    let expected_bytes = expected.as_bytes();
    if provided_bytes.len() != expected_bytes.len() {
        return Err(AppError::Unauthorized);
    }
    let mut diff: u8 = 0;
    for (a, b) in provided_bytes.iter().zip(expected_bytes.iter()) {
        diff |= a ^ b;
    }
    if diff == 0 {
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}

/// Marca como `completado` los items en estado `extraido` con `sample_id`
/// asignado. Equivale a la "publicación automática" del legado WordPress.
#[utoipa::path(
    post,
    path = "/api/admin/scraper/publicar-auto",
    tag = "scraper",
    responses(
        (status = 200, description = "Items publicados", body = PublicarAutoResponse),
        (status = 401, description = "Secret inválido"),
        (status = 403, description = "SCRAPER_SECRET no configurado")
    )
)]
pub async fn publicar_auto(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<PublicarAutoResponse>, AppError> {
    require_scraper_secret(&state, &headers)?;

    let result = sqlx::query(
        "UPDATE cola_extraccion_samples \
         SET estado = 'completado', procesado_at = NOW() \
         WHERE estado = 'extraido' AND sample_id IS NOT NULL",
    )
    .execute(&state.pool)
    .await?;

    let publicados = result.rows_affected().cast_signed();
    tracing::info!(publicados, "scraper publicar-auto ejecutado");
    Ok(Json(PublicarAutoResponse { publicados }))
}

/// Recibe el reporte resumen de un lote del scraper. Por ahora solo
/// loggea estructuradamente — la persistencia en `automatizacion_lotes`
/// se implementará cuando esa tabla exista en este repo.
#[utoipa::path(
    post,
    path = "/api/admin/scraper/reporte-lote",
    tag = "scraper",
    request_body = ReporteLoteRequest,
    responses(
        (status = 200, description = "Reporte aceptado", body = ReporteLoteResponse),
        (status = 401, description = "Secret inválido"),
        (status = 403, description = "SCRAPER_SECRET no configurado")
    )
)]
pub async fn reporte_lote(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ReporteLoteRequest>,
) -> Result<Json<ReporteLoteResponse>, AppError> {
    require_scraper_secret(&state, &headers)?;

    let motivos: HashMap<String, serde_json::Value> = payload
        .metadata
        .as_ref()
        .and_then(|m| m.get("motivos_fallo"))
        .and_then(|m| m.as_object())
        .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default();

    tracing::info!(
        batch_id = payload.batch_id,
        exitosos = payload.exitosos,
        fallidos = payload.fallidos,
        recortes = payload.recortes.unwrap_or(payload.exitosos),
        motivos_fallo = ?motivos,
        "scraper reporte-lote recibido",
    );

    Ok(Json(ReporteLoteResponse {
        ok: true,
        batch_id: payload.batch_id,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin/scraper/publicar-auto", post(publicar_auto))
        .route("/admin/scraper/reporte-lote", post(reporte_lote))
}
