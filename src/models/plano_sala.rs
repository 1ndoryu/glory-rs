/* [263A-14] Modelos del plano de sala: zonas, mesas, combinaciones.
 * El dueño construye el plano arrastrando mesas en un canvas por zona. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/* ========== Zona de sala ========== */

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct ZonaSala {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: String,
    pub orden: i32,
    pub ancho: i32,
    pub alto: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearZonaRequest {
    #[validate(length(min = 1, max = 100, message = "El nombre de la zona es obligatorio (máx 100)"))]
    pub nombre: String,
    pub orden: Option<i32>,
    #[validate(range(min = 200, max = 3000, message = "Ancho entre 200 y 3000 px"))]
    pub ancho: Option<i32>,
    #[validate(range(min = 200, max = 3000, message = "Alto entre 200 y 3000 px"))]
    pub alto: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarZonaRequest {
    #[validate(length(min = 1, max = 100))]
    pub nombre: Option<String>,
    pub orden: Option<i32>,
    #[validate(range(min = 200, max = 3000))]
    pub ancho: Option<i32>,
    #[validate(range(min = 200, max = 3000))]
    pub alto: Option<i32>,
}

/* ========== Mesa ========== */

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct Mesa {
    pub id: Uuid,
    pub zona_id: Uuid,
    pub numero: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub ancho: i32,
    pub alto: i32,
    pub forma: String,
    pub min_personas: i32,
    pub max_personas: i32,
    pub activa: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearMesaRequest {
    pub zona_id: Uuid,
    #[validate(range(min = 1, message = "El número de mesa debe ser >= 1"))]
    pub numero: i32,
    pub pos_x: Option<i32>,
    pub pos_y: Option<i32>,
    #[validate(range(min = 30, max = 400))]
    pub ancho: Option<i32>,
    #[validate(range(min = 30, max = 400))]
    pub alto: Option<i32>,
    #[validate(length(min = 1, max = 20))]
    pub forma: Option<String>,
    #[validate(range(min = 1, max = 50))]
    pub min_personas: Option<i32>,
    #[validate(range(min = 1, max = 50))]
    pub max_personas: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarMesaRequest {
    #[validate(range(min = 1))]
    pub numero: Option<i32>,
    pub pos_x: Option<i32>,
    pub pos_y: Option<i32>,
    #[validate(range(min = 30, max = 400))]
    pub ancho: Option<i32>,
    #[validate(range(min = 30, max = 400))]
    pub alto: Option<i32>,
    #[validate(length(min = 1, max = 20))]
    pub forma: Option<String>,
    #[validate(range(min = 1, max = 50))]
    pub min_personas: Option<i32>,
    #[validate(range(min = 1, max = 50))]
    pub max_personas: Option<i32>,
    pub activa: Option<bool>,
}

/* Batch update de posiciones — para guardar drag-and-drop */
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarPosicionesRequest {
    pub posiciones: Vec<PosicionMesa>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PosicionMesa {
    pub id: Uuid,
    pub pos_x: i32,
    pub pos_y: i32,
}

/* ========== Combinación de mesas ========== */

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct CombinacionMesas {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: String,
    pub min_personas: i32,
    pub max_personas: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearCombinacionRequest {
    #[validate(length(min = 1, max = 100, message = "El nombre es obligatorio (máx 100)"))]
    pub nombre: String,
    #[validate(range(min = 1, max = 50))]
    pub min_personas: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    pub max_personas: i32,
    #[validate(length(min = 2, message = "Una combinación necesita al menos 2 mesas"))]
    pub mesa_ids: Vec<Uuid>,
}

/* ========== Response compuesto: plano completo ========== */

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanoSala {
    pub zonas: Vec<ZonaConMesas>,
    pub combinaciones: Vec<CombinacionConMesas>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ZonaConMesas {
    #[serde(flatten)]
    pub zona: ZonaSala,
    pub mesas: Vec<Mesa>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CombinacionConMesas {
    #[serde(flatten)]
    pub combinacion: CombinacionMesas,
    pub mesas: Vec<Mesa>,
}

/* ========== Export/Import ========== */

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanoExport {
    pub version: String,
    pub zonas: Vec<ZonaExport>,
    pub combinaciones: Vec<CombinacionExport>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ZonaExport {
    pub nombre: String,
    pub orden: i32,
    pub ancho: i32,
    pub alto: i32,
    pub mesas: Vec<MesaExport>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MesaExport {
    pub numero: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub ancho: i32,
    pub alto: i32,
    pub forma: String,
    pub min_personas: i32,
    pub max_personas: i32,
    pub activa: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CombinacionExport {
    pub nombre: String,
    pub min_personas: i32,
    pub max_personas: i32,
    /* Números de mesa referenciados (zona_nombre:numero) para remap al importar */
    pub mesas_ref: Vec<String>,
}
