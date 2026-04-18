/* [174A-4] Middleware que asigna un request_id único por request.
 * Reusa X-Request-Id entrante si existe; si no, genera UUIDv7 (ordenable por tiempo).
 * El id se inyecta en el span de tracing y se devuelve en el header de respuesta
 * para correlación cliente↔backend↔logs. */

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let request_id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map_or_else(|| Uuid::now_v7().to_string(), str::to_owned);

    /* Inyectar en extensiones para que handlers puedan leerlo. */
    req.extensions_mut().insert(RequestId(request_id.clone()));

    let span = tracing::info_span!(
        "request",
        request_id = %request_id,
        method = %req.method(),
        uri = %req.uri(),
    );
    let _enter = span.enter();

    let mut res = next.run(req).await;

    if let Ok(value) = HeaderValue::from_str(&request_id) {
        res.headers_mut().insert(REQUEST_ID_HEADER, value);
    }

    res
}

/// Wrapper para extraer el `request_id` desde extensiones de la request.
#[derive(Debug, Clone)]
pub struct RequestId(pub String);
