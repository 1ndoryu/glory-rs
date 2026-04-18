#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::ref_option)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]

pub mod algorithm;
pub mod audio;
pub mod config;
pub mod domain;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod workers;
pub mod ws;

use sqlx::PgPool;
use std::sync::Arc;

/* [174A-5+174A-21+174A-26] Estado compartido de la aplicación.
 * - `pool`: PostgreSQL (siempre presente).
 * - `redis`: pool opcional. Cuando es None, los servicios deben tener fallback
 *   en memoria (DashMap/parking_lot) o degradarse limpiamente.
 * - `jwt_secret`: clave HMAC para firmar/verificar tokens.
 * - `google`: verificador OAuth Google ID-token (compartido).
 * - `storage`: backend de almacenamiento (LocalFs/S3) detrás de trait.
 * - `public_base_url`: prefijo opcional para construir URLs absolutas. */
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: Option<deadpool_redis::Pool>,
    pub jwt_secret: String,
    pub google: Arc<services::GoogleVerifier>,
    pub storage: Arc<dyn services::FileStorage>,
    pub public_base_url: Option<String>,
}
