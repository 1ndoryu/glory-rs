#![allow(clippy::needless_for_each)] // Generado por utoipa OpenApi derive

mod assignment;
mod auth;
mod health;
mod notes;
mod orders;
mod payments;
mod seo;
mod services;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
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
        notes::create_note,
        notes::get_note,
        notes::list_notes,
        notes::update_note,
        notes::delete_note,
        services::list_services,
        services::get_service,
        orders::create_order,
        orders::list_orders,
        orders::get_order,
        orders::assign_order,
        orders::switch_role,
        orders::cancel_order_handler,
        orders::deliver_phase,
        orders::approve_phase,
        orders::request_revision,
        payments::initiate_payment,
        payments::stripe_webhook,
        payments::list_payments,
        assignment::take_order,
        assignment::list_unassigned,
        assignment::list_employees,
        assignment::create_delegation,
        assignment::create_help_request,
        assignment::respond_delegation,
        assignment::list_delegations,
    ),
    components(schemas(
        health::HealthResponse,
        crate::models::RegisterRequest,
        crate::models::LoginRequest,
        crate::models::AuthResponse,
        crate::models::Note,
        crate::models::CreateNoteRequest,
        crate::models::UpdateNoteRequest,
        crate::models::PaginatedNotes,
        crate::models::UserRole,
        crate::models::ServiceDetailResponse,
        crate::models::ServicePlanResponse,
        crate::models::ServicePlanPhaseResponse,
        crate::models::CreateOrderRequest,
        crate::models::OrderResponse,
        crate::models::OrderPhaseResponse,
        crate::models::SwitchRoleRequest,
        crate::models::PaymentMode,
        crate::models::OrderStatus,
        crate::models::PhaseStatus,
        crate::models::PaymentStatus,
        crate::models::InitiatePaymentRequest,
        crate::models::PaymentIntentResponse,
        crate::models::PaymentResponse,
        crate::models::DelegationStatus,
        crate::models::DelegationResponse,
        crate::models::EmployeeListItem,
        crate::models::CreateDelegationRequest,
        crate::models::RespondDelegationRequest,
        crate::errors::ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    info(
        title = "Glory RS API",
        version = "0.1.0",
        description = "Template API — Rust + Axum + OpenAPI"
    )
)]
#[allow(clippy::needless_for_each)]
pub struct ApiDoc;

/// Crea el router principal con CORS, tracing, Swagger UI y todas las rutas
pub fn create_router(pool: sqlx::PgPool, config: crate::config::AppConfig) -> Router {
    let state = AppState {
        pool,
        jwt_secret: config.jwt_secret,
        http_client: reqwest::Client::new(),
        stripe_secret_key: config.stripe_secret_key,
        stripe_webhook_secret: config.stripe_webhook_secret,
    };

    /* CORS: en desarrollo se permite todo. En producción, restringir orígenes */
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(seo::routes())
        .nest("/api", api_routes())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

/* [044A-9] Monta el frontend React como SPA: archivos estaticos con fallback a index.html.
 * Solo se activa si STATIC_DIR esta configurado (en produccion). En desarrollo, Vite sirve el frontend. */
pub fn create_app(pool: sqlx::PgPool, config: crate::config::AppConfig) -> Router {
    let static_dir = config.static_dir.clone();
    let router = create_router(pool, config);

    if let Some(dir) = static_dir {
        let index_path = format!("{dir}/index.html");
        let serve = ServeDir::new(&dir).not_found_service(ServeFile::new(&index_path));
        router.fallback_service(serve)
    } else {
        router
    }
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(auth::routes())
        .merge(notes::routes())
        .merge(services::routes())
        /* assignment routes ANTES de orders: /orders/unassigned (literal) debe
         * registrarse antes de /orders/:order_id (parámetro) para evitar conflicto */
        .merge(assignment::routes())
        .merge(orders::routes())
        .merge(payments::routes())
}
