/* [174A-91] Handler GET /api/sync/changelog para sync delta de desktop/mobile.
 * Acepta `cursor` (contrato legacy) o `since` (alias del roadmap). Auth requerida:
 * el usuario_id viene del bearer, no de query, para evitar leaks entre cuentas. */

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::models::{SyncChangelogDelta, SyncChangelogQuery};
use crate::repositories::SyncChangelogRepository;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/sync/changelog", get(get_changelog))
}

#[utoipa::path(
    get,
    path = "/api/sync/changelog",
    tag = "sync",
    params(SyncChangelogQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Delta de cambios", body = SyncChangelogDelta),
        (status = 401, description = "No autenticado"),
    )
)]
pub async fn get_changelog(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(params): Query<SyncChangelogQuery>,
) -> Result<Json<SyncChangelogDelta>, AppError> {
    let cursor = params.cursor.or(params.since).unwrap_or(0);
    let limite = params.limite.unwrap_or(100).clamp(1, 500);

    let delta = SyncChangelogRepository::delta(&state.pool, user.user_id, cursor, limite).await?;
    Ok(Json(delta))
}
