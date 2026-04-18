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
    /// Lista CSV de Google OAuth client_ids aceptados (web/android/ios).
    pub google_client_ids: Vec<String>,
    /// Directorio raiz para storage local (LocalFs). Default: "./uploads"
    pub storage_root: String,
    /// Backend de storage: "local" (default) o "s3" (requiere feature `s3`).
    pub storage_backend: String,
    /// Bucket S3 (requerido si storage_backend == "s3").
    pub s3_bucket: Option<String>,
    /// Endpoint URL custom para S3-compatibles (R2/MinIO). Opcional.
    pub s3_endpoint_url: Option<String>,
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
            google_client_ids: std::env::var("GOOGLE_CLIENT_IDS")
                .unwrap_or_default()
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect(),
            storage_root: std::env::var("STORAGE_ROOT")
                .unwrap_or_else(|_| "./uploads".to_string()),
            storage_backend: std::env::var("STORAGE_BACKEND")
                .unwrap_or_else(|_| "local".to_string()),
            s3_bucket: std::env::var("S3_BUCKET").ok(),
            s3_endpoint_url: std::env::var("S3_ENDPOINT_URL").ok(),
        })
    }
}
