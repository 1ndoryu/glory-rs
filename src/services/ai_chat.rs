/* [044A-38 Fase 5] AI Chat Service: integración con API de IA (Gemini/OpenAI).
 * System prompt dinámico: pre-venta (servicios, precios) vs soporte de orden
 * (contexto de orden, fase actual, historial). Usa reqwest HTTP client. */

use std::fmt::Write;

use sqlx::PgPool;
use uuid::Uuid;

use crate::repositories::{ChatRepository, OrderRepository};

/// Configuración del servicio de IA
#[derive(Clone)]
pub struct AiChatConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub api_url: String,
}

impl AiChatConfig {
    /// Intenta cargar config desde variables de entorno
    #[must_use]
    pub fn from_env() -> Self {
        let api_key = std::env::var("AI_API_KEY").ok()
            .or_else(|| std::env::var("GEMINI_API_KEY").ok())
            .or_else(|| std::env::var("OPENAI_API_KEY").ok());

        let model = std::env::var("AI_MODEL")
            .unwrap_or_else(|_| "gemini-2.0-flash".to_string());

        let api_url = std::env::var("AI_API_URL").unwrap_or_else(|_| {
            "https://generativelanguage.googleapis.com/v1beta/models".to_string()
        });

        Self {
            api_key,
            model,
            api_url,
        }
    }

    #[must_use]
    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
    }
}

pub struct AiChatService;

impl AiChatService {
    /// Genera respuesta de IA para un mensaje en una sesión
    pub async fn generate_response(
        pool: &PgPool,
        config: &AiChatConfig,
        session_id: Uuid,
        user_message: &str,
    ) -> Result<String, String> {
        let Some(ref api_key) = config.api_key else {
            return Ok(
                "Un miembro del equipo se conectará pronto para ayudarte. \
                 Mientras tanto, ¿en qué puedo orientarte?"
                    .to_string(),
            );
        };

        /* Construir system prompt dinámico */
        let system_prompt = build_system_prompt(pool, session_id).await;

        /* Obtener historial reciente para contexto */
        let history = ChatRepository::list_messages(pool, session_id, 20, 0)
            .await
            .unwrap_or_default();

        /* Armar messages para la API */
        let mut contents = Vec::new();

        for msg in &history {
            let role = if msg.sender_type == "ai" {
                "model"
            } else {
                "user"
            };
            contents.push(serde_json::json!({
                "role": role,
                "parts": [{"text": msg.content}]
            }));
        }

        /* Agregar el mensaje actual */
        contents.push(serde_json::json!({
            "role": "user",
            "parts": [{"text": user_message}]
        }));

        /* Llamar a Gemini API */
        let url = format!(
            "{}/{}:generateContent?key={}",
            config.api_url, config.model, api_key
        );

        let body = serde_json::json!({
            "system_instruction": {
                "parts": [{"text": system_prompt}]
            },
            "contents": contents,
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": 800,
                "topP": 0.9
            }
        });

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Error llamando a IA: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            tracing::error!("AI API error {status}: {text}");
            return Ok(
                "Disculpa, en este momento no puedo procesar tu consulta. \
                 Un miembro del equipo te atenderá pronto."
                    .to_string(),
            );
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Error parseando respuesta IA: {e}"))?;

        /* Extraer texto de la respuesta Gemini */
        let text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("No pude generar una respuesta. Un miembro del equipo te ayudará.");

        Ok(text.to_string())
    }
}

/// Construye system prompt dinámico según contexto de la sesión
async fn build_system_prompt(pool: &PgPool, session_id: Uuid) -> String {
    let mut prompt = String::from(
        "Eres el asistente virtual de Nakomi Studio, una agencia de desarrollo web y diseño. \
         Responde de forma concisa, amable y profesional en el mismo idioma que el usuario. \
         Si la pregunta requiere atención humana (presupuesto específico, problema técnico, \
         reunión), indica que un miembro del equipo se conectará pronto.\n\n"
    );

    /* Agregar contexto de servicios */
    if let Ok(services) = OrderRepository::list_services(pool).await {
        prompt.push_str("Servicios disponibles:\n");
        for svc in &services {
            let _ = writeln!(
                prompt,
                "- {}: desde ${:.2} USD",
                svc.title,
                f64::from(svc.base_price_cents) / 100.0
            );
        }
        prompt.push('\n');
    }

    /* Contexto de orden si la sesión está vinculada */
    if let Ok(Some(session)) = ChatRepository::find_session_by_id(pool, session_id).await {
        if let Some(order_id) = session.order_id {
            if let Ok(Some(order)) = OrderRepository::find_order_by_id(pool, order_id).await {
                if let Ok((svc_title, plan_name)) =
                    OrderRepository::get_order_display_info(pool, order.service_id, order.plan_id)
                        .await
                {
                    let _ = write!(
                        prompt,
                        "CONTEXTO DE ORDEN ACTIVA:\n\
                         - Orden #{}: {} ({})\n\
                         - Estado: {:?}\n\
                         - Fase actual: {}/{}\n\
                         El usuario tiene una orden activa. Responde preguntas sobre su \
                         progreso y ofrece ayuda específica.\n",
                        order.order_number,
                        svc_title,
                        plan_name,
                        order.status,
                        order.current_phase,
                        0 /* total phases retrieved if needed */
                    );
                }
            }
        }
    }

    prompt
}
