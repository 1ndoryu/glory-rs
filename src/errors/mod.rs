use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

/* [174A-4] Error global del backend Kamples.
 * Cada variante mapea a un HTTP status. Los handlers retornan AppError;
 * el IntoResponse convierte a JSON {error, message, request_id?}.
 * Errores 5xx se loggean con tracing::error! antes de responder. */
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("No encontrado: {0}")]
    NotFound(String),

    #[error("Solicitud inválida: {0}")]
    BadRequest(String),

    #[error("No autorizado")]
    Unauthorized,

    #[error("Prohibido: {0}")]
    Forbidden(String),

    #[error("Conflicto: {0}")]
    Conflict(String),

    #[error("Demasiadas solicitudes")]
    RateLimited,

    #[error("Demasiadas solicitudes: {0}")]
    TooManyRequests(String),

    #[error("Carga demasiado grande")]
    PayloadTooLarge,

    #[error("Tipo de medio no soportado: {0}")]
    UnsupportedMediaType(String),

    #[error("Error interno: {0}")]
    Internal(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de servicio externo ({service}): {message}")]
    ExternalService { service: String, message: String },

    #[error("Error de validación: {0}")]
    Validation(String),
}

/// Estructura de respuesta de error expuesta en la API
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Tipo de error (`not_found`, `unauthorized`, etc.)
    pub error: String,
    /// Mensaje legible para el usuario
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Credenciales inválidas o ausentes".to_string(),
            ),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, "forbidden", msg.clone()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone()),
            Self::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limited",
                "Demasiadas solicitudes, espera un momento".to_string(),
            ),
            Self::TooManyRequests(msg) => (
                StatusCode::TOO_MANY_REQUESTS,
                "too_many_requests",
                msg.clone(),
            ),
            Self::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "payload_too_large",
                "El archivo o solicitud excede el límite permitido".to_string(),
            ),
            Self::UnsupportedMediaType(msg) => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "unsupported_media_type",
                msg.clone(),
            ),
            Self::Internal(msg) => {
                tracing::error!(error.kind = "internal", error.detail = %msg, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "Ocurrió un error interno".to_string(),
                )
            }
            Self::Database(err) => {
                tracing::error!(error.kind = "database", error.detail = %err, "database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    "Ocurrió un error de base de datos".to_string(),
                )
            }
            Self::ExternalService { service, message } => {
                tracing::error!(error.kind = "external", error.service = %service, error.detail = %message, "external service error");
                (
                    StatusCode::BAD_GATEWAY,
                    "external_service_error",
                    format!("Error en servicio externo: {service}"),
                )
            }
            Self::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                msg.clone(),
            ),
        };

        let body = ErrorResponse {
            error: error_type.to_string(),
            message,
        };

        (status, Json(body)).into_response()
    }
}

/// Resultado canónico del backend.
pub type AppResult<T> = std::result::Result<T, AppError>;
