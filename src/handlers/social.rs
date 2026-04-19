use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::algorithm::InteractionKind;
use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::repositories::{BlockRepository, BlockedUser, FollowRepository};
use crate::services::NotificationFanoutService;
use crate::AppState;
use tracing::warn;

/* [174A-60] Follows + Blocks. Port de:
 * - SocialController::seguir/dejarDeSeguir → POST/DELETE /api/follow/:userId.
 * - ModeracionController::bloquearUsuario/desbloquearUsuario/misBloqueados →
 *   POST/DELETE /api/block/:userId, GET /api/me/bloqueados.
 *
 * Reglas portadas:
 * - Self-follow / self-block prohibido (400).
 * - Verificar que target existe (404).
 * - Bloquear → unfollow mutuo + recount.
 * - Trigger AlgoPlanner Follow al seguir.
 *
 * NO portado:
 * - Rate limit 20 follows/min, 10 bloqueos/min (sin RateLimiter global).
 * - Notificación al target (Fase 11).
 * - Verificación de ban activo (depende de QQ71).
 */

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct OkResponse {
    pub ok: bool,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BlockRequest {
    #[serde(default)]
    pub razon: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BlockedListResponse {
    pub data: Vec<BlockedUser>,
}

#[utoipa::path(
    post,
    path = "/api/follow/{userId}",
    tag = "social",
    params(("userId" = i32, Path, description = "ID del usuario a seguir")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Follow registrado", body = OkResponse),
        (status = 400, description = "Auto-follow prohibido"),
        (status = 401, description = "No autenticado"),
        (status = 404, description = "Usuario no encontrado"),
    )
)]
pub async fn follow_user(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(target_id): Path<i32>,
) -> Result<Json<OkResponse>, AppError> {
    if user.user_id == target_id {
        return Err(AppError::Validation("No puedes seguirte a ti mismo".into()));
    }
    if !FollowRepository::user_exists(&state.pool, target_id).await? {
        return Err(AppError::NotFound(format!("usuario {target_id} no existe")));
    }

    FollowRepository::follow(&state.pool, user.user_id, target_id).await?;
    FollowRepository::recount(&state.pool, user.user_id, target_id).await?;

    state
        .algo_planner
        .register_interaction(
            &state.pool,
            &state.redis,
            user.user_id,
            InteractionKind::Follow,
        )
        .await?;

    if let Err(error) =
        NotificationFanoutService::dispatch_follow(&state, target_id, user.user_id).await
    {
        warn!(
            actor_id = user.user_id,
            recipient_id = target_id,
            error = %error,
            "falló fanout de follow"
        );
    }

    Ok(Json(OkResponse { ok: true }))
}

#[utoipa::path(
    delete,
    path = "/api/follow/{userId}",
    tag = "social",
    params(("userId" = i32, Path, description = "ID del usuario a dejar de seguir")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Unfollow registrado", body = OkResponse),
        (status = 401, description = "No autenticado"),
    )
)]
pub async fn unfollow_user(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(target_id): Path<i32>,
) -> Result<Json<OkResponse>, AppError> {
    FollowRepository::unfollow(&state.pool, user.user_id, target_id).await?;
    FollowRepository::recount(&state.pool, user.user_id, target_id).await?;
    Ok(Json(OkResponse { ok: true }))
}

#[utoipa::path(
    post,
    path = "/api/block/{userId}",
    tag = "social",
    params(("userId" = i32, Path, description = "ID del usuario a bloquear")),
    request_body = BlockRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Bloqueo registrado", body = OkResponse),
        (status = 400, description = "Auto-bloqueo prohibido"),
        (status = 401, description = "No autenticado"),
        (status = 404, description = "Usuario no encontrado"),
    )
)]
pub async fn block_user(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(target_id): Path<i32>,
    Json(body): Json<BlockRequest>,
) -> Result<Json<OkResponse>, AppError> {
    if user.user_id == target_id {
        return Err(AppError::Validation(
            "No puedes bloquearte a ti mismo".into(),
        ));
    }
    if !FollowRepository::user_exists(&state.pool, target_id).await? {
        return Err(AppError::NotFound(format!("usuario {target_id} no existe")));
    }

    let razon = body.razon.unwrap_or_default();
    BlockRepository::block(&state.pool, user.user_id, target_id, &razon).await?;

    /* Bloquear implica unfollow mutuo. */
    FollowRepository::unfollow(&state.pool, user.user_id, target_id).await?;
    FollowRepository::unfollow(&state.pool, target_id, user.user_id).await?;
    FollowRepository::recount(&state.pool, user.user_id, target_id).await?;
    FollowRepository::recount(&state.pool, target_id, user.user_id).await?;

    Ok(Json(OkResponse { ok: true }))
}

#[utoipa::path(
    delete,
    path = "/api/block/{userId}",
    tag = "social",
    params(("userId" = i32, Path, description = "ID del usuario a desbloquear")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Desbloqueo registrado", body = OkResponse),
        (status = 401, description = "No autenticado"),
    )
)]
pub async fn unblock_user(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(target_id): Path<i32>,
) -> Result<Json<OkResponse>, AppError> {
    BlockRepository::unblock(&state.pool, user.user_id, target_id).await?;
    Ok(Json(OkResponse { ok: true }))
}

#[utoipa::path(
    get,
    path = "/api/me/bloqueados",
    tag = "social",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de usuarios bloqueados", body = BlockedListResponse),
        (status = 401, description = "No autenticado"),
    )
)]
pub async fn my_blocks(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<(StatusCode, Json<BlockedListResponse>), AppError> {
    let data = BlockRepository::list(&state.pool, user.user_id).await?;
    Ok((StatusCode::OK, Json(BlockedListResponse { data })))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/follow/:userId", post(follow_user))
        .route("/follow/:userId", delete(unfollow_user))
        .route("/block/:userId", post(block_user))
        .route("/block/:userId", delete(unblock_user))
        .route("/me/bloqueados", get(my_blocks))
}
