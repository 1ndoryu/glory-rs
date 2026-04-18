use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::errors::AppError;
use crate::services::AuthService;
use crate::AppState;

/* [174A-18] Extractor de usuario autenticado. user_id es i32 (PK usuarios_ext). */
pub struct AuthUser {
    pub user_id: i32,
    pub plan: String,
    pub rol: String,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let claims = AuthService::verify_token(token, &state.jwt_secret)?;

        Ok(Self {
            user_id: claims.sub,
            plan: claims.plan,
            rol: claims.rol,
        })
    }
}
