/* 263A-1: Modelo de etiquetas para clientes y reservas.
   Basado en Data II (Videos 10-11): gestor de etiquetas con categorías,
   preestablecidas del sistema + custom del dueño. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

/// Categoría de etiqueta (agrupa etiquetas por tipo)
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct CategoriaEtiqueta {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub nombre: String,
    pub aplica_a: String,
    pub es_sistema: bool,
    pub created_at: DateTime<Utc>,
}

/// Etiqueta individual
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct Etiqueta {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub categoria_id: Uuid,
    pub nombre: String,
    pub color: String,
    pub es_sistema: bool,
    pub created_at: DateTime<Utc>,
}

/// Etiqueta con nombre de categoría incluido (para listados)
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct EtiquetaConCategoria {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub nombre: String,
    pub color: String,
    pub categoria_id: Uuid,
    pub es_sistema: bool,
    pub created_at: DateTime<Utc>,
    pub categoria_nombre: String,
}

/// Request para crear una etiqueta
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearEtiquetaRequest {
    pub categoria_id: Uuid,
    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre es obligatorio y no debe exceder 100 caracteres"
    ))]
    pub nombre: String,
    #[validate(length(max = 7))]
    pub color: Option<String>,
}

/// Request para crear una categoría de etiquetas
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearCategoriaEtiquetaRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre es obligatorio y no debe exceder 100 caracteres"
    ))]
    pub nombre: String,
    /// "cliente" o "reserva"
    pub aplica_a: String,
}

/// Query para listar etiquetas
#[derive(Debug, Deserialize, IntoParams)]
pub struct EtiquetasQuery {
    /// Filtrar por categoría
    pub categoria_id: Option<Uuid>,
}
