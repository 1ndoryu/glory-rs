use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/* [174A-25] DTOs admin: suspension/reactivacion/eliminacion + bloqueo entre usuarios. */

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SuspendUserRequest {
    #[validate(length(min = 3, max = 500))]
    pub razon: String,
    /// Si se omite, suspension indefinida.
    pub hasta: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DeleteUserRequest {
    /// Dias hasta eliminacion definitiva (default 30).
    pub dias_gracia: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Default)]
pub struct BlockUserRequest {
    #[validate(length(max = 255))]
    pub razon: Option<String>,
}
