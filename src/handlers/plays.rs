use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::algorithm::InteractionKind;
use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::repositories::PlayRepository;
use crate::AppState;

/* [174A-58] Tracking de reproducciones (`POST /api/samples/{id}/play`).
 *
 * Port directo de `ReproduccionesController::registrar`. Diferencias:
 * - Rate limit (60/min) NO portado: pendiente cuando llegue el RateLimiter
 *   global. Ver tarea futura — el endpoint queda anotado con TODO.
 * - Debounce 3s manejado por `PlayRepository::register_with_debounce`.
 * - Se invoca `AlgoPlanner::register_interaction(Reproduccion)` siempre y
 *   `Completa` adicionalmente cuando el cliente reporta `completada=true`.
 */

const PLAY_DEBOUNCE_SECONDS: i64 = 3;

#[derive(Debug, Clone, Deserialize, ToSchema, Validate, Default)]
pub struct RegisterPlayRequest {
    /// Segundos de audio efectivamente escuchados antes del POST.
    #[serde(default)]
    #[validate(range(min = 0.0, max = 86_400.0))]
    pub duracion_escuchada: f32,
    /// `true` si el sample se reprodujo hasta el final (>=80% típico).
    #[serde(default)]
    pub completada: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RegisterPlayResponse {
    pub ok: bool,
    /// `true` cuando el play se fusionó con uno reciente en lugar de crear fila.
    pub debounce: bool,
    /// Detalle de los recálculos disparados por el planificador.
    pub triggered: PlayTriggered,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PlayTriggered {
    /// `true` cuando el planificador disparó el recálculo rápido.
    pub fast: bool,
    /// `true` cuando el planificador disparó el recálculo preciso.
    pub precise: bool,
}

#[utoipa::path(
    post,
    path = "/api/samples/{id}/play",
    tag = "samples",
    params(("id" = i32, Path, description = "ID del sample reproducido")),
    request_body = RegisterPlayRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Play registrado", body = RegisterPlayResponse),
        (status = 200, description = "Play debounced (fusionado con uno reciente)", body = RegisterPlayResponse),
        (status = 400, description = "Body inválido", body = ErrorResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
    )
)]
pub async fn register_play(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(sample_id): Path<i32>,
    Json(body): Json<RegisterPlayRequest>,
) -> Result<(StatusCode, Json<RegisterPlayResponse>), AppError> {
    body.validate()
        .map_err(|err| AppError::Validation(err.to_string()))?;

    /* TODO(174A-58 follow-up): rate limit 60/min por usuario cuando exista
     * RateLimiter global. Mientras tanto el debounce de 3s mitiga abuso. */

    let outcome = PlayRepository::register_with_debounce(
        &state.pool,
        user.user_id,
        sample_id,
        body.duracion_escuchada,
        body.completada,
        PLAY_DEBOUNCE_SECONDS,
    )
    .await?;

    let mut triggered_fast = false;
    let mut triggered_precise = false;

    if !outcome.debounced {
        PlayRepository::increment_sample_counter(&state.pool, sample_id).await?;

        let triggered = state
            .algo_planner
            .register_interaction(
                &state.pool,
                &state.redis,
                user.user_id,
                InteractionKind::Reproduccion,
            )
            .await?;
        triggered_fast |= triggered.fast;
        triggered_precise |= triggered.precise;

        if outcome.completed_now {
            let extra = state
                .algo_planner
                .register_interaction(
                    &state.pool,
                    &state.redis,
                    user.user_id,
                    InteractionKind::Completa,
                )
                .await?;
            triggered_fast |= extra.fast;
            triggered_precise |= extra.precise;
        }
    }

    let status = if outcome.debounced {
        StatusCode::OK
    } else {
        StatusCode::CREATED
    };

    Ok((
        status,
        Json(RegisterPlayResponse {
            ok: true,
            debounce: outcome.debounced,
            triggered: PlayTriggered {
                fast: triggered_fast,
                precise: triggered_precise,
            },
        }),
    ))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/samples/:id/play", post(register_play))
}
