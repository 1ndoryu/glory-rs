#![allow(clippy::needless_for_each)] // Generado por utoipa OpenApi derive

mod auth;
mod canales_reserva;
mod clientes;
mod dashboard;
mod etiquetas;
mod gastos;
mod health;
mod reservas;
mod ventas;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::errors::ErrorResponse;

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
        ventas::crear_venta,
        ventas::obtener_venta,
        ventas::listar_ventas,
        ventas::eliminar_venta,
        gastos::crear_gasto,
        gastos::obtener_gasto,
        gastos::listar_gastos,
        gastos::eliminar_gasto,
        gastos::listar_categorias,
        reservas::crear_reserva,
        reservas::obtener_reserva,
        reservas::listar_reservas,
        reservas::actualizar_reserva,
        reservas::eliminar_reserva,
        reservas::conteo_reservas,
        reservas::resumen_mensual,
        reservas::no_show_stats,
        clientes::crear_cliente,
        clientes::obtener_cliente,
        clientes::listar_clientes,
        clientes::actualizar_cliente,
        clientes::eliminar_cliente,
        etiquetas::listar_categorias,
        etiquetas::crear_categoria,
        etiquetas::listar_etiquetas,
        etiquetas::crear_etiqueta,
        etiquetas::eliminar_etiqueta,
        etiquetas::asignar_etiqueta_cliente,
        etiquetas::desasignar_etiqueta_cliente,
        etiquetas::obtener_etiquetas_cliente,
        etiquetas::asignar_etiqueta_reserva,
        etiquetas::desasignar_etiqueta_reserva,
        etiquetas::obtener_etiquetas_reserva,
        canales_reserva::listar_canales,
        canales_reserva::crear_canal,
        canales_reserva::eliminar_canal,
        dashboard::resumen,
    ),
    components(schemas(
        health::HealthResponse,
        crate::models::RegisterRequest,
        crate::models::LoginRequest,
        crate::models::AuthResponse,
        crate::models::Venta,
        crate::models::CrearVentaRequest,
        crate::models::VentasPaginadas,
        crate::models::Gasto,
        crate::models::CrearGastoRequest,
        crate::models::GastosPaginados,
        crate::models::CategoriaGasto,
        crate::models::Reserva,
        crate::models::CrearReservaRequest,
        crate::models::ActualizarReservaRequest,
        crate::models::ReservasPaginadas,
        crate::models::ReservasConteo,
        crate::models::ResumenDiario,
        crate::models::NoShowStats,
        crate::models::NoShowPorCanal,
        crate::models::CanalReserva,
        crate::models::CrearCanalReservaRequest,
        crate::models::Cliente,
        crate::models::CrearClienteRequest,
        crate::models::ActualizarClienteRequest,
        crate::models::ClientesPaginados,
        crate::models::CategoriaEtiqueta,
        crate::models::Etiqueta,
        crate::models::EtiquetaConCategoria,
        crate::models::CrearEtiquetaRequest,
        crate::models::CrearCategoriaEtiquetaRequest,
        etiquetas::TagAssignBody,
        crate::models::ResumenEconomico,
        crate::models::Turno,
        crate::models::CanalVenta,
        crate::models::MetodoPago,
        crate::models::TipoDocumento,
        crate::models::EstadoReserva,
        ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    info(
        title = "Gestión Restaurante API",
        version = "0.1.0",
        description = "API para gestión de restaurantes — Ventas, Gastos, Reservas, Dashboard"
    )
)]
#[allow(clippy::needless_for_each)]
pub struct ApiDoc;

/// Crea el router principal con CORS, tracing, Swagger UI y todas las rutas
pub fn create_router(pool: sqlx::PgPool, config: crate::config::AppConfig) -> Router {
    let state = AppState {
        pool,
        jwt_secret: config.jwt_secret,
    };

    /* CORS: en desarrollo se permite todo. En producción, restringir orígenes */
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(auth::routes())
        .merge(ventas::routes())
        .merge(gastos::routes())
        .merge(reservas::routes())
        .merge(clientes::routes())
        .merge(etiquetas::routes())
        .merge(canales_reserva::routes())
        .merge(dashboard::routes())
}
