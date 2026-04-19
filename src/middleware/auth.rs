use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::errors::AppError;
use crate::services::{AuthService, TokenStore};
use crate::AppState;

/* [174A-19+174A-20] Extractor obligatorio que ademas verifica blacklist de jti. */
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: i32,
    pub jti: String,
    pub plan: String,
    pub rol: String,
}

pub type AuthUser = CurrentUser;

#[async_trait]
impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;
        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;
        let claims = AuthService::verify_token(token, &state.jwt_secret)?;
        if TokenStore::is_access_revoked(&state.redis, &claims.jti).await? {
            return Err(AppError::Unauthorized);
        }
        Ok(Self {
            user_id: claims.sub,
            jti: claims.jti,
            plan: claims.plan,
            rol: claims.rol,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct OptionalUser(pub Option<CurrentUser>);

#[async_trait]
impl FromRequestParts<AppState> for OptionalUser {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));
        let user = match token {
            Some(t) => match AuthService::verify_token(t, &state.jwt_secret) {
                Ok(c) => {
                    let revoked = TokenStore::is_access_revoked(&state.redis, &c.jti)
                        .await
                        .unwrap_or(false);
                    if revoked {
                        None
                    } else {
                        Some(CurrentUser {
                            user_id: c.sub,
                            jti: c.jti,
                            plan: c.plan,
                            rol: c.rol,
                        })
                    }
                }
                Err(_) => None,
            },
            None => None,
        };
        Ok(OptionalUser(user))
    }
}

impl CurrentUser {
    pub fn require_admin(&self) -> Result<(), AppError> {
        if self.rol == "admin" {
            Ok(())
        } else {
            Err(AppError::Forbidden("Requiere rol admin".into()))
        }
    }
}
