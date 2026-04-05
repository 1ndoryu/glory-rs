/* [044A-38 Fase 8] Modelo de reviews de órdenes.
 * Rating 1-5 + comentario del cliente, respuesta del empleado.
 * order_reviews tiene UNIQUE en order_id → una review por orden. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, FromRow, Serialize)]
pub struct OrderReview {
    pub id: Uuid,
    pub order_id: Uuid,
    pub client_id: Uuid,
    pub employee_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub employee_response: Option<String>,
    pub employee_responded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateReviewBody {
    #[validate(range(min = 1, max = 5))]
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RespondReviewBody {
    #[validate(length(min = 1, max = 2000))]
    pub response: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReviewResponse {
    pub id: String,
    pub order_id: String,
    pub client_id: String,
    pub employee_id: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub employee_response: Option<String>,
    pub employee_responded_at: Option<String>,
    pub created_at: String,
}

impl From<OrderReview> for ReviewResponse {
    fn from(r: OrderReview) -> Self {
        Self {
            id: r.id.to_string(),
            order_id: r.order_id.to_string(),
            client_id: r.client_id.to_string(),
            employee_id: r.employee_id.to_string(),
            rating: r.rating,
            comment: r.comment,
            employee_response: r.employee_response,
            employee_responded_at: r.employee_responded_at.map(|d| d.to_rfc3339()),
            created_at: r.created_at.to_rfc3339(),
        }
    }
}
