use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Variable de entorno requerida no encontrada: {0}")]
    MissingEnvVar(String),
    #[error("Puerto inválido: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
}

/// Configuración de la aplicación cargada desde variables de entorno
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
    pub smtp: Option<SmtpConfig>,
    pub app_url: String,
    /// Email de destino para reportes de errores (opcional)
    pub error_report_email: Option<String>,
}

/* [263A-15] Configuración SMTP para envío de emails (recuperación de contraseña).
 * Todas las variables son opcionales: si faltan, el sistema loguea el enlace
 * en vez de enviarlo por email, lo que facilita desarrollo local. */
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
}

impl AppConfig {
    /// Carga la configuración desde variables de entorno.
    /// Requiere `DATABASE_URL` y `JWT_SECRET`. `HOST` y `PORT` son opcionales.
    pub fn from_env() -> Result<Self, ConfigError> {
        let smtp = Self::load_smtp();
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".into()))?,
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| ConfigError::MissingEnvVar("JWT_SECRET".into()))?,
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            smtp,
            app_url: std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:5173".to_string()),
            error_report_email: std::env::var("ERROR_REPORT_EMAIL").ok(),
        })
    }

    fn load_smtp() -> Option<SmtpConfig> {
        let host = std::env::var("SMTP_HOST").ok()?;
        let port = std::env::var("SMTP_PORT").ok()?.parse().ok()?;
        let user = std::env::var("SMTP_USER").ok()?;
        let password = std::env::var("SMTP_PASSWORD").ok()?;
        Some(SmtpConfig {
            host,
            port,
            user,
            password,
            from_email: std::env::var("SMTP_FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@restaurante.com".to_string()),
            from_name: std::env::var("SMTP_FROM_NAME")
                .unwrap_or_else(|_| "Restaurante".to_string()),
        })
    }
}
