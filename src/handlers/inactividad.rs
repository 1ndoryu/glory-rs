/* [094A-5] Handlers de reglas de inactividad.
 * CRUD para que el propietario configure reglas de mensajes automáticos
 * a clientes que no visitan el restaurante en N días. */

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    ActualizarReglaInactividadRequest, CrearReglaInactividadRequest, ReglaInactividad,
};
use crate::repositories::InactividadRepository;
use crate::AppState;

#[utoipa::path(
    get,
    path = "/api/inactividad",
    tag = "Inactividad",
    responses((status = 200, body = Vec<ReglaInactividad>)),
    security(("bearer_auth" = []))
)]
pub async fn listar(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<ReglaInactividad>>, AppError> {
    let reglas = InactividadRepository::list(&state.pool, auth.user_id).await?;
    Ok(Json(reglas))
}

#[utoipa::path(
    post,
    path = "/api/inactividad",
    tag = "Inactividad",
    request_body = CrearReglaInactividadRequest,
    responses((status = 201, body = ReglaInactividad)),
    security(("bearer_auth" = []))
)]
pub async fn crear(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearReglaInactividadRequest>,
) -> Result<(StatusCode, Json<ReglaInactividad>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let regla = InactividadRepository::create(
        &state.pool,
        auth.user_id,
        &req.nombre,
        req.dias_inactividad,
        &req.canal,
        &req.mensaje_plantilla,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(regla)))
}

#[utoipa::path(
    patch,
    path = "/api/inactividad/{id}",
    tag = "Inactividad",
    request_body = ActualizarReglaInactividadRequest,
    params(("id" = Uuid, Path, description = "ID de la regla")),
    responses((status = 200, body = ReglaInactividad)),
    security(("bearer_auth" = []))
)]
pub async fn actualizar(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<ActualizarReglaInactividadRequest>,
) -> Result<Json<ReglaInactividad>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let regla = InactividadRepository::update(
        &state.pool,
        id,
        auth.user_id,
        req.nombre.as_deref(),
        req.dias_inactividad,
        req.canal.as_deref(),
        req.mensaje_plantilla.as_deref(),
        req.activa,
    )
    .await?
    .ok_or_else(|| AppError::NotFound("Regla no encontrada".into()))?;
    Ok(Json(regla))
}

#[utoipa::path(
    delete,
    path = "/api/inactividad/{id}",
    tag = "Inactividad",
    params(("id" = Uuid, Path, description = "ID de la regla")),
    responses((status = 204)),
    security(("bearer_auth" = []))
)]
pub async fn eliminar(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if InactividadRepository::delete(&state.pool, id, auth.user_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("Regla no encontrada".into()))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/inactividad", get(listar).post(crear))
        .route(
            "/inactividad/:id",
            axum::routing::patch(actualizar).delete(eliminar),
        )
}
