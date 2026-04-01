/* [263A-14] Handlers del plano de sala — CRUD zonas/mesas/combinaciones + export/import.
 * Tag OpenAPI: "PlanoSala" */

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    ActualizarMesaRequest, ActualizarPosicionesRequest, ActualizarZonaRequest,
    CombinacionMesas, CrearCombinacionRequest, CrearMesaRequest, CrearZonaRequest,
    Mesa, PlanoExport, PlanoOcupacion, PlanoOcupacionQuery, PlanoSala, ZonaSala,
};
use crate::services::PlanoSalaService;
use crate::AppState;

/* ========== Plano completo ========== */

#[utoipa::path(
    get,
    path = "/api/plano-sala",
    tag = "PlanoSala",
    responses(
        (status = 200, description = "Plano completo", body = PlanoSala),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_plano(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<PlanoSala>, AppError> {
    let plano = PlanoSalaService::plano_completo(&state.pool, auth.user_id).await?;
    Ok(Json(plano))
}

/* ========== Zonas ========== */

#[utoipa::path(
    post,
    path = "/api/plano-sala/zonas",
    tag = "PlanoSala",
    request_body = CrearZonaRequest,
    responses(
        (status = 201, description = "Zona creada", body = ZonaSala),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_zona(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearZonaRequest>,
) -> Result<(StatusCode, Json<ZonaSala>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let zona = PlanoSalaService::crear_zona(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(zona)))
}

#[utoipa::path(
    patch,
    path = "/api/plano-sala/zonas/{id}",
    tag = "PlanoSala",
    params(("id" = Uuid, Path, description = "ID de la zona")),
    request_body = ActualizarZonaRequest,
    responses(
        (status = 200, description = "Zona actualizada", body = ZonaSala),
        (status = 404, description = "Zona no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn actualizar_zona(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<ActualizarZonaRequest>,
) -> Result<Json<ZonaSala>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let zona =
        PlanoSalaService::actualizar_zona(&state.pool, id, auth.user_id, req).await?;
    Ok(Json(zona))
}

#[utoipa::path(
    delete,
    path = "/api/plano-sala/zonas/{id}",
    tag = "PlanoSala",
    params(("id" = Uuid, Path, description = "ID de la zona")),
    responses(
        (status = 204, description = "Zona eliminada"),
        (status = 404, description = "Zona no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar_zona(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    PlanoSalaService::eliminar_zona(&state.pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/* ========== Mesas ========== */

#[utoipa::path(
    post,
    path = "/api/plano-sala/mesas",
    tag = "PlanoSala",
    request_body = CrearMesaRequest,
    responses(
        (status = 201, description = "Mesa creada", body = Mesa),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 409, description = "Número de mesa duplicado en la zona", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_mesa(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(req): Json<CrearMesaRequest>,
) -> Result<(StatusCode, Json<Mesa>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let mesa = PlanoSalaService::crear_mesa(&state.pool, req).await?;
    Ok((StatusCode::CREATED, Json(mesa)))
}

#[utoipa::path(
    patch,
    path = "/api/plano-sala/mesas/{id}",
    tag = "PlanoSala",
    params(("id" = Uuid, Path, description = "ID de la mesa")),
    request_body = ActualizarMesaRequest,
    responses(
        (status = 200, description = "Mesa actualizada", body = Mesa),
        (status = 404, description = "Mesa no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn actualizar_mesa(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<ActualizarMesaRequest>,
) -> Result<Json<Mesa>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let mesa = PlanoSalaService::actualizar_mesa(&state.pool, id, req).await?;
    Ok(Json(mesa))
}

#[utoipa::path(
    delete,
    path = "/api/plano-sala/mesas/{id}",
    tag = "PlanoSala",
    params(("id" = Uuid, Path, description = "ID de la mesa")),
    responses(
        (status = 204, description = "Mesa eliminada"),
        (status = 404, description = "Mesa no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar_mesa(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    PlanoSalaService::eliminar_mesa(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    patch,
    path = "/api/plano-sala/mesas/posiciones",
    tag = "PlanoSala",
    request_body = ActualizarPosicionesRequest,
    responses(
        (status = 200, description = "Posiciones actualizadas", body = u64),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn actualizar_posiciones(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(req): Json<ActualizarPosicionesRequest>,
) -> Result<Json<u64>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let posiciones: Vec<(Uuid, i32, i32)> = req
        .posiciones
        .into_iter()
        .map(|p| (p.id, p.pos_x, p.pos_y))
        .collect();
    let total = PlanoSalaService::actualizar_posiciones(&state.pool, &posiciones).await?;
    Ok(Json(total))
}

/* ========== Combinaciones ========== */

#[utoipa::path(
    post,
    path = "/api/plano-sala/combinaciones",
    tag = "PlanoSala",
    request_body = CrearCombinacionRequest,
    responses(
        (status = 201, description = "Combinación creada", body = CombinacionMesas),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_combinacion(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearCombinacionRequest>,
) -> Result<(StatusCode, Json<CombinacionMesas>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let combo =
        PlanoSalaService::crear_combinacion(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(combo)))
}

#[utoipa::path(
    delete,
    path = "/api/plano-sala/combinaciones/{id}",
    tag = "PlanoSala",
    params(("id" = Uuid, Path, description = "ID de la combinación")),
    responses(
        (status = 204, description = "Combinación eliminada"),
        (status = 404, description = "Combinación no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar_combinacion(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    PlanoSalaService::eliminar_combinacion(&state.pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/* ========== Export / Import ========== */

#[utoipa::path(
    get,
    path = "/api/plano-sala/export",
    tag = "PlanoSala",
    responses(
        (status = 200, description = "Plano exportado", body = PlanoExport),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn exportar_plano(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<PlanoExport>, AppError> {
    let export = PlanoSalaService::exportar(&state.pool, auth.user_id).await?;
    Ok(Json(export))
}

#[utoipa::path(
    post,
    path = "/api/plano-sala/import",
    tag = "PlanoSala",
    request_body = PlanoExport,
    responses(
        (status = 200, description = "Plano importado", body = PlanoSala),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn importar_plano(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(data): Json<PlanoExport>,
) -> Result<Json<PlanoSala>, AppError> {
    let plano =
        PlanoSalaService::importar(&state.pool, auth.user_id, data).await?;
    Ok(Json(plano))
}

/* ========== Ocupación (263A-16) ========== */

/* [014A-4] turno_a_rango ahora acepta la config de turnos.
 * Si no se tiene config disponible, usa defaults. */
fn turno_a_rango_config(
    turno: &str,
    config: Option<&crate::models::ConfiguracionRestaurante>,
) -> (Option<chrono::NaiveTime>, Option<chrono::NaiveTime>) {
    use chrono::NaiveTime;
    match (turno, config) {
        ("desayuno", Some(c)) => (Some(c.hora_desayuno_inicio), Some(c.hora_desayuno_fin)),
        ("comida", Some(c)) => (Some(c.hora_comida_inicio), Some(c.hora_comida_fin)),
        ("cena", Some(c)) => (Some(c.hora_cena_inicio), Some(c.hora_cena_fin)),
        ("desayuno", None) => (
            Some(NaiveTime::from_hms_opt(0, 0, 0).expect("hora literal válida")),
            Some(NaiveTime::from_hms_opt(12, 0, 0).expect("hora literal válida")),
        ),
        ("comida", None) => (
            Some(NaiveTime::from_hms_opt(12, 0, 0).expect("hora literal válida")),
            Some(NaiveTime::from_hms_opt(18, 0, 0).expect("hora literal válida")),
        ),
        ("cena", None) => (
            Some(NaiveTime::from_hms_opt(18, 0, 0).expect("hora literal válida")),
            Some(NaiveTime::from_hms_opt(23, 59, 59).expect("hora literal válida")),
        ),
        _ => (None, None),
    }
}

#[utoipa::path(
    get,
    path = "/api/plano-sala/ocupacion",
    tag = "PlanoSala",
    params(PlanoOcupacionQuery),
    responses(
        (status = 200, description = "Plano con ocupación de mesas", body = PlanoOcupacion),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_ocupacion(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<PlanoOcupacionQuery>,
) -> Result<Json<PlanoOcupacion>, AppError> {
    /* [014A-4] Obtener config de turnos para rangos horarios configurables */
    let config = crate::repositories::ConfiguracionRepository::obtener_o_crear(
        &state.pool,
        auth.user_id,
    )
    .await
    .ok();

    let (hora_desde, hora_hasta) = query
        .turno
        .as_deref()
        .map_or((None, None), |t| turno_a_rango_config(t, config.as_ref()));

    let plano = PlanoSalaService::plano_ocupacion(
        &state.pool,
        auth.user_id,
        query.fecha,
        hora_desde,
        hora_hasta,
    )
    .await?;
    Ok(Json(plano))
}

/* ========== Router ========== */

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/plano-sala", get(obtener_plano))
        .route("/plano-sala/ocupacion", get(obtener_ocupacion))
        .route("/plano-sala/zonas", post(crear_zona))
        .route(
            "/plano-sala/zonas/:id",
            patch(actualizar_zona).delete(eliminar_zona),
        )
        .route("/plano-sala/mesas", post(crear_mesa))
        .route("/plano-sala/mesas/posiciones", patch(actualizar_posiciones))
        .route(
            "/plano-sala/mesas/:id",
            patch(actualizar_mesa).delete(eliminar_mesa),
        )
        .route("/plano-sala/combinaciones", post(crear_combinacion))
        .route(
            "/plano-sala/combinaciones/:id",
            delete(eliminar_combinacion),
        )
        .route("/plano-sala/export", get(exportar_plano))
        .route("/plano-sala/import", post(importar_plano))
}
