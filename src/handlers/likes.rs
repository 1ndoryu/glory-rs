use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};

use crate::algorithm::InteractionKind;
use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::repositories::{LikeKind, LikeRepository, Reaction};
use crate::AppState;

/* [174A-59] Likes polimórficos. Port directo de
 * `SocialController::darLike/quitarLike`. Endpoints:
 *   POST   /api/like — crea o actualiza la reacción del usuario al target.
 *   DELETE /api/like — elimina la reacción.
 *
 * Notas vs legado:
 * - Rate limit (30/min) NO portado: pendiente cuando exista RateLimiter global.
 * - Notificaciones al creador NO portadas: llegan en Fase 11.
 * - Verificación de ban/suspensión activa NO portada (depende de QQ71).
 * - Trigger de AlgoPlanner: Like/Encanta → InteractionKind::Like;
 *   Dislike → InteractionKind::Dislike (mismo path en el legado).
 */

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LikeRequest {
    /// Tipo de target. Valores: `sample`, `publicacion`, `comentario`, `cancion`, `relacion`.
    pub tipo: String,
    pub target_id: i32,
    /// Reacción opcional. Valores: `like`, `dislike`, `encanta`. Default `like`.
    #[serde(default)]
    pub reaccion: Option<String>,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct UnlikeQuery {
    pub tipo: String,
    pub target_id: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LikeResponse {
    pub ok: bool,
    pub liked: bool,
    pub reaccion: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/like",
    tag = "social",
    request_body = LikeRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Like creado o actualizado", body = LikeResponse),
        (status = 400, description = "Tipo o reacción inválidos"),
        (status = 401, description = "No autenticado"),
        (status = 404, description = "Target no encontrado"),
    )
)]
pub async fn create_like(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(body): Json<LikeRequest>,
) -> Result<Json<LikeResponse>, AppError> {
    let kind = LikeKind::from_str(&body.tipo)?;
    let reaction = match body.reaccion.as_deref() {
        Some(value) => Reaction::from_str(value)?,
        None => Reaction::Like,
    };

    if !LikeRepository::target_exists(&state.pool, kind, body.target_id).await? {
        return Err(AppError::NotFound(format!(
            "{} {} no existe",
            kind.as_db_str(),
            body.target_id
        )));
    }

    LikeRepository::upsert_reaction(&state.pool, user.user_id, kind, body.target_id, reaction)
        .await?;
    LikeRepository::recount_target(&state.pool, kind, body.target_id).await?;

    let interaction = if reaction.is_positive() {
        InteractionKind::Like
    } else {
        /* Legado: dislike no tiene contador propio en algoritmo_estado;
         * se cuenta como interacción de tipo `like` para refrescar el bucket. */
        InteractionKind::Like
    };
    state
        .algo_planner
        .register_interaction(&state.pool, &state.redis, user.user_id, interaction)
        .await?;

    Ok(Json(LikeResponse {
        ok: true,
        liked: true,
        reaccion: Some(reaction.as_db_str().to_string()),
    }))
}

#[utoipa::path(
    delete,
    path = "/api/like",
    tag = "social",
    params(UnlikeQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Reacción eliminada", body = LikeResponse),
        (status = 400, description = "Tipo inválido"),
        (status = 401, description = "No autenticado"),
    )
)]
pub async fn delete_like(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(q): Query<UnlikeQuery>,
) -> Result<(StatusCode, Json<LikeResponse>), AppError> {
    let kind = LikeKind::from_str(&q.tipo)?;
    LikeRepository::delete_reaction(&state.pool, user.user_id, kind, q.target_id).await?;
    LikeRepository::recount_target(&state.pool, kind, q.target_id).await?;

    Ok((
        StatusCode::OK,
        Json(LikeResponse {
            ok: true,
            liked: false,
            reaccion: None,
        }),
    ))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/like", post(create_like))
        .route("/like", delete(delete_like))
}
