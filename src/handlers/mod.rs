#![allow(clippy::needless_for_each)] // Generado por utoipa OpenApi derive

mod admin;
mod auth;
mod health;
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
        sample_catalog::get_sample,
        sample_catalog::list_samples,
        sample_catalog::random_sample,
        samples::check_duplicate,
        samples::upload,
        users::me,
        users::update_me,
        users::public_profile,
        users::block,
        users::unblock,
        users::list_blocked,
        admin::suspend,
        admin::activate,
        admin::mark_delete,
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
        crate::models::SampleCreatorSummary,
        crate::models::CheckDuplicateRequest,
        crate::models::CheckDuplicateResponse,
        crate::models::ListSamplesQuery,
        crate::models::ListSamplesResponse,
        crate::models::SampleDetailResponse,
        crate::models::SampleSummary,
        crate::models::SamplesPagination,
        crate::models::UploadSampleRequestDoc,
        crate::models::UploadSampleResponse,
        crate::models::UpdateProfileRequest,
        crate::models::PublicProfileResponse,
        crate::models::PrivateProfileResponse,
        crate::models::BlockUserRequest,
        crate::models::SuspendUserRequest,
        crate::models::DeleteUserRequest,
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
    let state = AppState {
        pool,
        redis,
        jwt_secret: config.jwt_secret,
        google: std::sync::Arc::new(crate::services::GoogleVerifier::new(
            config.google_client_ids,
        )),
        storage,
        public_base_url,
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
        .merge(sample_catalog::routes())
        .merge(samples::routes())
        .merge(users::routes())
        .merge(admin::routes())
}
