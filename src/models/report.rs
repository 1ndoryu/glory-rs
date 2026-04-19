use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum GenericReportType {
    Usuario,
    Publicacion,
    Comentario,
    Sample,
    ErrorPlataforma,
}

impl GenericReportType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Usuario => "usuario",
            Self::Publicacion => "publicacion",
            Self::Comentario => "comentario",
            Self::Sample => "sample",
            Self::ErrorPlataforma => "error_plataforma",
        }
    }

    pub const fn checks_duplicate(self) -> bool {
        matches!(self, Self::Publicacion | Self::Comentario | Self::Sample)
    }

    pub const fn requires_target(self) -> bool {
        !matches!(self, Self::ErrorPlataforma)
    }

    pub const fn duplicate_message(self) -> &'static str {
        match self {
            Self::Publicacion => "Ya reportaste esta publicacion",
            Self::Comentario => "Ya reportaste este comentario",
            Self::Sample => "Ya reportaste este sample",
            Self::Usuario | Self::ErrorPlataforma => "Ya existe un reporte previo",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum LegalReportType {
    LegalSample,
    LegalRelacion,
}

impl LegalReportType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LegalSample => "legal_sample",
            Self::LegalRelacion => "legal_relacion",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "legal_sample" => Some(Self::LegalSample),
            "legal_relacion" => Some(Self::LegalRelacion),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum LegalRightType {
    Copyright,
    Trademark,
    Otro,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateGenericReportRequest {
    #[serde(rename = "tipo")]
    pub tipo: GenericReportType,
    #[serde(rename = "targetId", alias = "target_id")]
    pub target_id: i32,
    pub razon: String,
    #[serde(default)]
    pub detalles: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateReportReasonRequest {
    pub razon: String,
    #[serde(default)]
    pub detalles: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, ToSchema)]
pub struct CreateScopedReportRequest {
    #[serde(default)]
    pub razon: Option<String>,
    #[serde(default)]
    pub detalles: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePlatformErrorReportRequest {
    pub razon: String,
    pub detalles: String,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateLegalReportRequest {
    pub tipo: LegalReportType,
    pub target_id: i32,
    pub razon: String,
    pub nombre: String,
    pub email: String,
    pub tipo_derecho: LegalRightType,
    pub obra_protegida: String,
    pub declaracion: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ReportResponse {
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ErrorReportResponse {
    pub ok: bool,
    pub message: String,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LegalReportResponse {
    pub ok: bool,
    pub reporte_id: i32,
    pub mensaje: String,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct AdminLegalReportsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LegalReportDetails {
    pub nombre: String,
    pub email: String,
    pub tipo_derecho: LegalRightType,
    pub obra_protegida: String,
    pub declaracion_bf: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_origen: Option<String>,
    pub fecha_envio: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminLegalReportItem {
    pub id: i32,
    pub tipo: LegalReportType,
    pub target_id: i32,
    pub razon: String,
    pub estado: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_origen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detalles: Option<LegalReportDetails>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminLegalReportsResponse {
    pub ok: bool,
    pub reportes: Vec<AdminLegalReportItem>,
    pub total: i64,
}