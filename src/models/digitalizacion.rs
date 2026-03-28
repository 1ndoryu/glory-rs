/* [283A-8] Modelo para digitalización de documentos (gastos) vía Groq IA.
 * Request: imagen base64 del documento.
 * Response: datos extraídos para pre-rellenar el formulario de gasto. */

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Request para digitalizar un documento (factura, albarán, ticket)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DigitalizarDocumentoRequest {
    /// Imagen codificada en base64 (JPEG, PNG, WebP)
    #[validate(length(min = 100, max = 10_000_000, message = "Imagen base64 inválida o demasiado grande (máx ~7MB)"))]
    pub imagen_base64: String,
    /// MIME type de la imagen (image/jpeg, image/png, image/webp)
    #[validate(length(max = 30))]
    pub mime_type: String,
}

/// Datos extraídos del documento por la IA
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatosDocumentoExtraidos {
    pub fecha: Option<String>,
    pub proveedor: Option<String>,
    pub numero_documento: Option<String>,
    /// "factura", "albaran" o "ticket"
    pub tipo_documento: Option<String>,
    pub importe_base: Option<String>,
    pub importe_iva: Option<String>,
    pub importe_total: Option<String>,
    /// Confianza general de la extracción (0.0 - 1.0)
    pub confianza: f64,
    /// Notas o advertencias sobre la extracción
    pub notas: Option<String>,
}
