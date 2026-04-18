use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Variable de entorno requerida no encontrada: {0}")]
    MissingEnvVar(String),
    #[error("Puerto inválido: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
}

/* [174A-5] Configuración de la app cargada desde variables de entorno.
 * REDIS_URL es opcional para permitir desarrollo sin Redis (cache en memoria fallback). */
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: Option<String>,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
}

impl AppConfig {
    /// Carga la configuración desde variables de entorno.
    /// Requiere `DATABASE_URL` y `JWT_SECRET`. `REDIS_URL`, `HOST`, `PORT`,
    /// `DB_MAX_CONNECTIONS` y `DB_MIN_CONNECTIONS` son opcionales.
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".into()))?,
            redis_url: std::env::var("REDIS_URL").ok(),
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| ConfigError::MissingEnvVar("JWT_SECRET".into()))?,
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            db_max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            db_min_connections: std::env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()?,
        })
    }
}
