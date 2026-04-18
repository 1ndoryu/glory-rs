use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::models::{ListSamplesQuery, ListSamplesResponse};
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
    Query(query): Query<ListSamplesQuery>,
) -> Result<Json<ListSamplesResponse>, AppError> {
    let response = SampleCatalogService::list_public_samples(&state.pool, query).await?;
    Ok(Json(response))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/samples", get(list_samples))
}
