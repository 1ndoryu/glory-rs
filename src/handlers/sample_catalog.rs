use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::middleware::{CurrentUser, OptionalUser};
use crate::models::{
    DeleteSampleResponse, ListSamplesQuery, ListSamplesResponse, SampleDetailResponse,
    SimilarSamplesQuery, SimilarSamplesResponse, UpdateSampleRequest,
};
use crate::services::SampleCatalogService;
use crate::AppState;

/* [174A-44] Handler público de catálogo de samples.
 * Vive separado de handlers/samples.rs porque ese archivo ya concentra upload,
 * hashing y multipart. El listado requiere otra responsabilidad: query params,
 * documentación OpenAPI y respuesta paginada. */

#[utoipa::path(
    get,
    path = "/api/samples",
    params(
        ("page" = Option<i64>, Query, description = "Página 1-based. Default: 1"),
        ("per_page" = Option<i64>, Query, description = "Tamaño de página. Default: 20, máximo: 100"),
        ("search" = Option<String>, Query, description = "Búsqueda textual. También acepta alias legacy `busqueda` y alias corto `q`"),
        ("search_normalized" = Option<String>, Query, description = "Forma normalizada del término. También acepta alias legacy `busqueda_norm`"),
        ("bpm" = Option<i32>, Query, description = "Filtro exacto por BPM"),
        ("key" = Option<String>, Query, description = "Tónica musical. Acepta C, F#, Bb, Eb, etc."),
        ("type" = Option<String>, Query, description = "Tipo de sample. También acepta alias legado `tipo`"),
        ("tags" = Option<String>, Query, description = "CSV de tags enriquecidos: trap,drill,vocal"),
        ("premium" = Option<bool>, Query, description = "Filtra por premium true/false"),
        ("creator" = Option<String>, Query, description = "Username del creador. También acepta alias legado `creador`")
    ),
    responses(
        (status = 200, description = "Listado público paginado de samples", body = ListSamplesResponse),
        (status = 422, description = "Filtros inválidos", body = ErrorResponse)
    )
)]
pub async fn list_samples(
    State(state): State<AppState>,
    OptionalUser(user): OptionalUser,
    Query(query): Query<ListSamplesQuery>,
) -> Result<Json<ListSamplesResponse>, AppError> {
    let response = SampleCatalogService::list_public_samples(
        &state.pool,
        state.public_base_url.as_deref(),
        user.map(|current| current.user_id),
        query,
    )
    .await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/samples/random",
    responses(
        (status = 200, description = "Sample aleatorio del catálogo público", body = SampleDetailResponse),
        (status = 404, description = "No hay samples públicos disponibles", body = ErrorResponse)
    )
)]
pub async fn random_sample(
    State(state): State<AppState>,
    OptionalUser(user): OptionalUser,
) -> Result<Json<SampleDetailResponse>, AppError> {
    let response = SampleCatalogService::get_random_sample(
        &state.pool,
        state.public_base_url.as_deref(),
        user.map(|current| current.user_id),
    )
    .await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/samples/{id}/similar",
    params(
        ("id" = i32, Path, description = "ID numérico del sample base"),
        ("limit" = Option<i64>, Query, description = "Cantidad máxima de resultados. También acepta alias legacy `limite`. Default: 5, máximo: 50")
    ),
    responses(
        (status = 200, description = "Samples similares usando pgvector con fallback contextual", body = SimilarSamplesResponse),
        (status = 404, description = "Sample no encontrado o no visible públicamente", body = ErrorResponse),
        (status = 422, description = "Query inválida", body = ErrorResponse)
    )
)]
pub async fn similar_samples(
    State(state): State<AppState>,
    OptionalUser(user): OptionalUser,
    Path(id): Path<i32>,
    Query(query): Query<SimilarSamplesQuery>,
) -> Result<Json<SimilarSamplesResponse>, AppError> {
    let response = SampleCatalogService::get_similar_samples(
        &state.pool,
        state.public_base_url.as_deref(),
        user.map(|current| current.user_id),
        id,
        query,
    )
    .await?;

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/samples/{slug}",
    params(("slug" = String, Path, description = "Slug público o id_corto del sample")),
    responses(
        (status = 200, description = "Detalle público del sample", body = SampleDetailResponse),
        (status = 404, description = "Sample no encontrado", body = ErrorResponse)
    )
)]
pub async fn get_sample(
    State(state): State<AppState>,
    OptionalUser(user): OptionalUser,
    Path(slug): Path<String>,
) -> Result<Json<SampleDetailResponse>, AppError> {
    let response = SampleCatalogService::get_sample_detail(
        &state.pool,
        state.public_base_url.as_deref(),
        user.map(|current| current.user_id),
        &slug,
    )
    .await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/samples/{slug}",
    request_body = UpdateSampleRequest,
    params(("slug" = String, Path, description = "Slug público o id_corto del sample")),
    responses(
        (status = 200, description = "Sample actualizado por su creador", body = SampleDetailResponse),
        (status = 401, description = "Autenticación requerida", body = ErrorResponse),
        (status = 403, description = "El sample no pertenece al usuario autenticado", body = ErrorResponse),
        (status = 404, description = "Sample no encontrado", body = ErrorResponse),
        (status = 422, description = "Payload inválido o sin cambios", body = ErrorResponse)
    )
)]
pub async fn update_sample(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(slug): Path<String>,
    Json(request): Json<UpdateSampleRequest>,
) -> Result<Json<SampleDetailResponse>, AppError> {
    let response = SampleCatalogService::update_owned_sample(
        &state.pool,
        state.public_base_url.as_deref(),
        current_user.user_id,
        &slug,
        request,
    )
    .await?;

    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/samples/{slug}",
    params(("slug" = String, Path, description = "Slug público o id_corto del sample")),
    responses(
        (status = 200, description = "Sample enviado a papelera", body = DeleteSampleResponse),
        (status = 401, description = "Autenticación requerida", body = ErrorResponse),
        (status = 403, description = "El sample no pertenece al usuario autenticado", body = ErrorResponse),
        (status = 404, description = "Sample no encontrado", body = ErrorResponse)
    )
)]
pub async fn delete_sample(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(slug): Path<String>,
) -> Result<Json<DeleteSampleResponse>, AppError> {
    let response =
        SampleCatalogService::delete_owned_sample(&state.pool, current_user.user_id, &slug).await?;

    Ok(Json(response))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/samples", get(list_samples))
        .route("/samples/random", get(random_sample))
        .route("/samples/:id/similar", get(similar_samples))
        .route(
            "/samples/:slug",
            get(get_sample).patch(update_sample).delete(delete_sample),
        )
}
