/* [094A-4] Modelos de reseñas.
 * Token único por reseña: el cliente accede a /resena/{token} sin auth.
 * Si puntuación >= 4 y hay google_review_url configurado → redirigir. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct Resena {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reserva_id: Option<Uuid>,
    pub cliente_id: Option<Uuid>,
    pub token: String,
    pub puntuacion: Option<i16>,
    pub comentario: String,
    pub redirigido_google: bool,
    pub created_at: DateTime<Utc>,
    pub respondida_at: Option<DateTime<Utc>>,
}

/* Respuesta pública (sin user_id ni datos internos) */
#[derive(Debug, Serialize, ToSchema)]
pub struct ResenaPublicaResponse {
    pub token: String,
    pub respondida: bool,
    pub nombre_restaurante: Option<String>,
}

/* Body que envía el cliente al responder */
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResponderResenaRequest {
    #[validate(range(min = 1, max = 5, message = "Puntuación debe ser 1-5"))]
    pub puntuacion: i16,
    #[validate(length(max = 2000))]
    pub comentario: Option<String>,
}

/* Respuesta tras responder — incluye redirect_url si aplica */
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponderResenaResponse {
    pub gracias: String,
    pub redirect_url: Option<String>,
}

/* Para panel del propietario: lista de reseñas recibidas */
#[derive(Debug, sqlx::FromRow, Serialize, ToSchema)]
pub struct ResenaAdmin {
    pub id: Uuid,
    pub reserva_id: Option<Uuid>,
    pub cliente_nombre: Option<String>,
    pub puntuacion: Option<i16>,
    pub comentario: String,
    pub redirigido_google: bool,
    pub created_at: DateTime<Utc>,
    pub respondida_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResenasPaginadas {
    pub data: Vec<ResenaAdmin>,
    pub total: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ResenasQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub min_puntuacion: Option<i16>,
    pub max_puntuacion: Option<i16>,
    pub solo_respondidas: Option<bool>,
}
