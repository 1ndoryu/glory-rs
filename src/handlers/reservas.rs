/* 253A-5: Handlers de reservas — CRUD + conteo para Home
   263A-6: Filtros turno/estado para vista día, resumen mensual para vista mes */

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    ActualizarReservaRequest, CrearReservaRequest, NoShowQuery, NoShowStats, Reserva,
    ReservasConteo, ReservasPaginadas, ReservasQuery, ResumenDiario, ResumenMesQuery,
};
use crate::services::ReservaService;
use crate::AppState;

/// Crear una reserva
#[utoipa::path(
    post,
    path = "/api/reservas",
    tag = "Reservas",
    request_body = CrearReservaRequest,
    responses(
        (status = 201, description = "Reserva creada", body = Reserva),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_reserva(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearReservaRequest>,
) -> Result<(StatusCode, Json<Reserva>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let reserva = ReservaService::create(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(reserva)))
}

/// Obtener una reserva por ID
#[utoipa::path(
    get,
    path = "/api/reservas/{id}",
    tag = "Reservas",
    params(("id" = Uuid, Path, description = "ID de la reserva")),
    responses(
        (status = 200, description = "Reserva encontrada", body = Reserva),
        (status = 404, description = "Reserva no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_reserva(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Reserva>, AppError> {
    let reserva = ReservaService::get(&state.pool, id, auth.user_id).await?;
    Ok(Json(reserva))
}

/// Listar reservas con paginación y filtro por fecha
#[utoipa::path(
    get,
    path = "/api/reservas",
    tag = "Reservas",
    params(ReservasQuery),
    responses(
        (status = 200, description = "Lista de reservas", body = ReservasPaginadas),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar_reservas(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ReservasQuery>,
) -> Result<Json<ReservasPaginadas>, AppError> {
    let reservas = ReservaService::list(
        &state.pool,
        auth.user_id,
        params.page,
        params.per_page,
        params.fecha,
        params.estado.as_deref(),
        params.turno.as_deref(),
    )
    .await?;
    Ok(Json(reservas))
}

/// Actualizar una reserva
#[utoipa::path(
    put,
    path = "/api/reservas/{id}",
    tag = "Reservas",
    params(("id" = Uuid, Path, description = "ID de la reserva")),
    request_body = ActualizarReservaRequest,
    responses(
        (status = 200, description = "Reserva actualizada", body = Reserva),
        (status = 404, description = "Reserva no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn actualizar_reserva(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<ActualizarReservaRequest>,
) -> Result<Json<Reserva>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let reserva = ReservaService::update(&state.pool, id, auth.user_id, req).await?;
    Ok(Json(reserva))
}

/// Eliminar una reserva
#[utoipa::path(
    delete,
    path = "/api/reservas/{id}",
    tag = "Reservas",
    params(("id" = Uuid, Path, description = "ID de la reserva")),
    responses(
        (status = 204, description = "Reserva eliminada"),
        (status = 404, description = "Reserva no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar_reserva(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    ReservaService::delete(&state.pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Conteo de reservas del mes y día actual — para el widget de Home
#[utoipa::path(
    get,
    path = "/api/reservas/conteo",
    tag = "Reservas",
    responses(
        (status = 200, description = "Conteo de reservas", body = ReservasConteo),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn conteo_reservas(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<ReservasConteo>, AppError> {
    let conteo = ReservaService::conteo(&state.pool, auth.user_id).await?;
    Ok(Json(conteo))
}

/// Resumen diario de un mes — para la vista calendario
#[utoipa::path(
    get,
    path = "/api/reservas/resumen-mes",
    tag = "Reservas",
    params(ResumenMesQuery),
    responses(
        (status = 200, description = "Resumen mensual", body = Vec<ResumenDiario>),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn resumen_mensual(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ResumenMesQuery>,
) -> Result<Json<Vec<ResumenDiario>>, AppError> {
    let resumen =
        ReservaService::resumen_mensual(&state.pool, auth.user_id, params.anio, params.mes)
            .await?;
    Ok(Json(resumen))
}

/// Estadísticas de no-shows con desglose por canal (263A-8)
#[utoipa::path(
    get,
    path = "/api/reservas/no-shows",
    tag = "Reservas",
    params(NoShowQuery),
    responses(
        (status = 200, description = "Estadísticas de no-shows", body = NoShowStats),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn no_show_stats(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<NoShowQuery>,
) -> Result<Json<NoShowStats>, AppError> {
    let resultado = ReservaService::no_show_stats(
        &state.pool,
        auth.user_id,
        params.fecha_desde,
        params.fecha_hasta,
    )
    .await?;
    Ok(Json(resultado))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reservas/conteo", get(conteo_reservas))
        .route("/reservas/resumen-mes", get(resumen_mensual))
        .route("/reservas/no-shows", get(no_show_stats))
        .route("/reservas", post(crear_reserva).get(listar_reservas))
        .route(
            "/reservas/{id}",
            get(obtener_reserva)
                .put(actualizar_reserva)
                .delete(eliminar_reserva),
        )
}
