/* [283A-8] Servicio de digitalización de documentos vía Groq IA.
 * Envía imagen base64 al modelo Llama 4 Scout con JSON mode para extraer datos de facturas/albaranes/tickets.
 * API de Groq es compatible con OpenAI: POST https://api.groq.com/openai/v1/chat/completions */

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::DatosDocumentoExtraidos;
use crate::repositories::ConfiguracionRepository;

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const GROQ_MODEL: &str = "meta-llama/llama-4-scout-17b-16e-instruct";

/* Prompt de sistema para extracción de datos de documentos de gasto */
const SYSTEM_PROMPT: &str = r#"Eres un asistente especializado en extraer datos de facturas, albaranes y tickets de restaurantes y proveedores en España.

Dada una imagen de un documento, extrae los siguientes campos en formato JSON:
- fecha: fecha del documento en formato YYYY-MM-DD
- proveedor: nombre del proveedor o emisor
- numero_documento: número de factura, albarán o ticket
- tipo_documento: uno de "factura", "albaran" o "ticket"
- importe_base: importe base antes de IVA (solo número decimal, sin símbolo €)
- importe_iva: importe del IVA (solo número decimal, sin símbolo €)
- importe_total: importe total con IVA incluido (solo número decimal, sin símbolo €)
- confianza: número entre 0.0 y 1.0 indicando tu confianza en la extracción
- notas: cualquier observación relevante sobre la extracción (texto libre o null)

Si no puedes determinar un campo, devuelve null para ese campo.
Si la imagen no es un documento de gasto, devuelve confianza 0.0 y una nota explicativa.
Responde SOLO con el JSON, sin texto adicional."#;

pub struct DigitalizacionService;

/* Estructuras para la request/response de Groq (compatible OpenAI) */
#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<GroqMessage>,
    temperature: f32,
    max_completion_tokens: u32,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Serialize)]
struct GroqMessage {
    role: String,
    content: serde_json::Value,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<GroqChoice>,
}

#[derive(Deserialize)]
struct GroqChoice {
    message: GroqChoiceMessage,
}

#[derive(Deserialize)]
struct GroqChoiceMessage {
    content: Option<String>,
}

impl DigitalizacionService {
    /// Digitaliza un documento de gasto enviando la imagen a Groq IA.
    /// Requiere que el usuario tenga configurada su API key de Groq.
    pub async fn digitalizar(
        pool: &PgPool,
        user_id: Uuid,
        imagen_base64: &str,
        mime_type: &str,
    ) -> Result<DatosDocumentoExtraidos, AppError> {
        /* Obtener API key de Groq desde la configuración del usuario */
        let config = ConfiguracionRepository::obtener_o_crear(pool, user_id).await?;
        let api_key = config.groq_api_key.ok_or_else(|| {
            AppError::BadRequest(
                "No se ha configurado la API key de Groq. Configúrala en Ajustes.".to_string(),
            )
        })?;

        if api_key.trim().is_empty() {
            return Err(AppError::BadRequest(
                "La API key de Groq está vacía. Configúrala en Ajustes.".to_string(),
            ));
        }

        /* Validar MIME type */
        let mime_validos = ["image/jpeg", "image/png", "image/webp", "image/gif"];
        if !mime_validos.contains(&mime_type) {
            return Err(AppError::BadRequest(format!(
                "Tipo de imagen no soportado: {mime_type}. Usa JPEG, PNG, WebP o GIF."
            )));
        }

        /* Construir la request a Groq */
        let data_url = format!("data:{mime_type};base64,{imagen_base64}");

        let messages = vec![
            GroqMessage {
                role: "system".to_string(),
                content: serde_json::Value::String(SYSTEM_PROMPT.to_string()),
            },
            GroqMessage {
                role: "user".to_string(),
                content: serde_json::json!([
                    {
                        "type": "text",
                        "text": "Extrae los datos de este documento de gasto."
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": data_url
                        }
                    }
                ]),
            },
        ];

        let request_body = GroqRequest {
            model: GROQ_MODEL.to_string(),
            messages,
            temperature: 0.1,
            max_completion_tokens: 1024,
            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
        };

        /* Llamar a Groq API */
        let client = reqwest::Client::new();
        let response = client
            .post(GROQ_API_URL)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Error al llamar a Groq API: {e}");
                AppError::Internal(format!("Error al conectar con el servicio de IA: {e}"))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            tracing::error!("Groq API error {status}: {body}");

            return Err(match status.as_u16() {
                401 => AppError::BadRequest(
                    "API key de Groq inválida. Verifica tu clave en Ajustes.".to_string(),
                ),
                429 => AppError::BadRequest(
                    "Se ha excedido el límite de uso de Groq. Intenta de nuevo en unos minutos."
                        .to_string(),
                ),
                _ => AppError::Internal(format!("Error del servicio de IA (HTTP {status})")),
            });
        }

        let groq_response: GroqResponse = response.json().await.map_err(|e| {
            tracing::error!("Error al parsear respuesta de Groq: {e}");
            AppError::Internal("Error al procesar la respuesta del servicio de IA".to_string())
        })?;

        let content = groq_response
            .choices
            .first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| {
                AppError::Internal("Respuesta vacía del servicio de IA".to_string())
            })?;

        /* Parsear el JSON devuelto por la IA */
        let datos: DatosDocumentoExtraidos =
            serde_json::from_str(content).map_err(|e| {
                tracing::error!("Error al parsear JSON de Groq: {e}, contenido: {content}");
                AppError::Internal(
                    "El servicio de IA devolvió un formato inesperado. Intenta con otra imagen."
                        .to_string(),
                )
            })?;

        Ok(datos)
    }
}
