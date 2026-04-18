#![allow(clippy::needless_for_each)] // Generado por utoipa OpenApi derive

mod auth;
mod health;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
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
    ),
    components(schemas(
        health::HealthResponse,
        crate::models::RegisterRequest,
        crate::models::LoginRequest,
        crate::models::AuthResponse,
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
pub struct ApiDoc;/// Crea el router principal con CORS, tracing, Swagger UI y todas las rutas
pub fn create_router(
    pool: sqlx::PgPool,
    redis: Option<deadpool_redis::Pool>,
    config: crate::config::AppConfig,
) -> Router {
    let state = AppState {
        pool,
        redis,
        jwt_secret: config.jwt_secret,
    };

    /* CORS: en desarrollo se permite todo. En producción, restringir orígenes */
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        /* [174A-6] Alias /docs → /swagger-ui/ para acceso más natural. */
        .route("/docs", axum::routing::get(|| async {
            axum::response::Redirect::permanent("/swagger-ui/")
        }))
        .nest("/api", api_routes())
        .layer(axum::middleware::from_fn(crate::middleware::request_id_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(auth::routes())
}
