#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

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

/* [174A-5] Estado compartido de la aplicación.
 * - `pool`: PostgreSQL (siempre presente).
 * - `redis`: pool opcional. Cuando es None, los servicios deben tener fallback
 *   en memoria (DashMap/parking_lot) o degradarse limpiamente.
 * - `jwt_secret`: clave HMAC para firmar/verificar tokens. */
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: Option<deadpool_redis::Pool>,
    pub jwt_secret: String,
}
