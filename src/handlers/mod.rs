#![allow(clippy::needless_for_each)] // Generado por utoipa OpenApi derive

mod admin;
mod auth;
mod colecciones;
mod downloads;
mod feed;
mod health;
mod likes;
mod plays;
mod social;
mod sample_catalog;
mod samples;
mod users;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

/// Define el esquema de seguridad Bearer para Swagger UI
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        /* components existe porque el derive ya registra schemas */
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        auth::register,
        auth::login,
        auth::refresh,
        auth::logout,
        auth::google_login,
        auth::google_pkce,
        feed::get_feed,
        feed::get_me_feed,
        sample_catalog::delete_sample,
        sample_catalog::get_sample,
        sample_catalog::list_samples,
        sample_catalog::random_sample,
        sample_catalog::similar_samples,
        sample_catalog::update_sample,
        samples::check_duplicate,
        samples::upload,
        plays::register_play,
        likes::create_like,
        likes::delete_like,
        social::follow_user,
        social::unfollow_user,
        social::block_user,
        social::unblock_user,
        social::my_blocks,
        downloads::register_download,
        downloads::download_limits,
        downloads::stream_download,
        colecciones::create_coleccion,
        colecciones::list_my_colecciones,
        colecciones::get_coleccion,
        colecciones::update_coleccion,
        colecciones::delete_coleccion,
        colecciones::add_sample,
        colecciones::remove_sample,
        colecciones::list_samples,
        colecciones::merge_coleccion,
        users::me,
        users::update_me,
        users::public_profile,
        users::block,
        users::unblock,
        users::list_blocked,
        admin::suspend,
        admin::activate,
        admin::mark_delete,
        admin::algo_timing_history,
    ),
    components(schemas(
        health::HealthResponse,
        crate::models::RegisterRequest,
        crate::models::LoginRequest,
        crate::models::RefreshRequest,
        crate::models::LogoutRequest,
        crate::models::GoogleAuthRequest,
        crate::models::GooglePkceRequest,
        crate::models::AuthResponse,
        crate::models::UserResponse,
        crate::algorithm::recommender::RankedSample,
        feed::FeedResponse,
        crate::models::SampleCreatorSummary,
        crate::models::CheckDuplicateRequest,
        crate::models::CheckDuplicateResponse,
        plays::RegisterPlayRequest,
        plays::RegisterPlayResponse,
        plays::PlayTriggered,
        likes::LikeRequest,
        likes::LikeResponse,
        social::OkResponse,
        social::BlockRequest,
        social::BlockedListResponse,
        crate::repositories::BlockedUser,
        downloads::DownloadResponse,
        downloads::DownloadLimitsResponse,
        colecciones::CreateColeccionRequest,
        colecciones::UpdateColeccionRequest,
        colecciones::AddSampleRequest,
        colecciones::OkResponse,
        colecciones::ColeccionListResponse,
        colecciones::ColeccionSamplesResponse,
        colecciones::MergeColeccionRequest,
        colecciones::MergeColeccionResponse,
        crate::repositories::Coleccion,
        crate::repositories::ColeccionSample,
        crate::models::DeleteSampleResponse,
        crate::models::ListSamplesQuery,
        crate::models::ListSamplesResponse,
        crate::models::SampleDetailResponse,
        crate::models::SampleSummary,
        crate::models::SamplesPagination,
        crate::models::SimilarSamplesQuery,
        crate::models::SimilarSamplesResponse,
        crate::models::UpdateSampleRequest,
        crate::models::UploadSampleRequestDoc,
        crate::models::UploadSampleResponse,
        crate::models::UpdateProfileRequest,
        crate::models::PublicProfileResponse,
        crate::models::PrivateProfileResponse,
        crate::models::BlockUserRequest,
        crate::models::SuspendUserRequest,
        crate::models::DeleteUserRequest,
        crate::services::algo_timing::TimingEntry,
        crate::services::algo_timing::TimingStage,
        crate::errors::ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    info(
        title = "Glory Kamples API",
        version = "0.1.0",
        description = "API Kamples — Rust + Axum + OpenAPI"
    )
)]
#[allow(clippy::needless_for_each)]
pub struct ApiDoc;

/// Crea el router principal con CORS, tracing, Swagger UI y todas las rutas.
pub fn create_router(
    pool: sqlx::PgPool,
    redis: Option<deadpool_redis::Pool>,
    config: crate::config::AppConfig,
    storage: std::sync::Arc<dyn crate::services::FileStorage>,
) -> Router {
    let public_base_url = config.public_base_url.clone();
    let storage_root = config.storage_root.clone();
    let algo_planner =
        crate::algorithm::AlgoPlanner::new(crate::algorithm::AlgoPlannerConfig::legacy_defaults());
    let state = AppState {
        pool,
        redis,
        jwt_secret: config.jwt_secret,
        google: std::sync::Arc::new(crate::services::GoogleVerifier::new(
            config.google_client_ids,
        )),
        storage,
        public_base_url,
        algo_planner,
    };

    /* CORS: en desarrollo se permite todo. En producción, restringir orígenes */
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        /* [174A-6] Alias /docs → /swagger-ui/ para acceso más natural. */
        .route(
            "/docs",
            axum::routing::get(|| async { axum::response::Redirect::permanent("/swagger-ui/") }),
        )
        .nest_service("/uploads", ServeDir::new(storage_root))
        .nest("/api", api_routes())
        .layer(axum::middleware::from_fn(
            crate::middleware::request_id_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(auth::routes())
        .merge(feed::routes())
        .merge(sample_catalog::routes())
        .merge(samples::routes())
        .merge(plays::routes())
        .merge(likes::routes())
        .merge(social::routes())
        .merge(downloads::routes())
        .merge(colecciones::routes())
        .merge(users::routes())
        .merge(admin::routes())
}
