use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::errors::AppError;
use crate::services::AuthService;
use crate::AppState;

/* [174A-19] Extractor obligatorio: rechaza la peticion con 401 si no hay JWT valido. */
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: i32,
    pub plan: String,
    pub rol: String,
}

/* [174A-18] Alias historico  preferir CurrentUser en codigo nuevo. */
pub type AuthUser = CurrentUser;

#[async_trait]
impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts.headers.get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;
        let token = header.strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?;
        let claims = AuthService::verify_token(token, &state.jwt_secret)?;
        Ok(Self { user_id: claims.sub, plan: claims.plan, rol: claims.rol })
    }
}

/* [174A-19] Extractor opcional: nunca falla; None si no hay token o es invalido.
 * Util para endpoints publicos que personalizan respuesta si el usuario esta logueado. */
#[derive(Debug, Clone, Default)]
pub struct OptionalUser(pub Option<CurrentUser>);

#[async_trait]
impl FromRequestParts<AppState> for OptionalUser {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let token = parts.headers.get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));
        let user = token
            .and_then(|t| AuthService::verify_token(t, &state.jwt_secret).ok())
            .map(|claims| CurrentUser { user_id: claims.sub, plan: claims.plan, rol: claims.rol });
        Ok(OptionalUser(user))
    }
}

/* [174A-19] Helper para handlers admin: 403 si rol != "admin". */
impl CurrentUser {
    pub fn require_admin(&self) -> Result<(), AppError> {
        if self.rol == "admin" { Ok(()) } else { Err(AppError::Forbidden("Requiere rol admin".into())) }
    }
}