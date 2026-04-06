/* [064A-29] AI Chat Service: integración con Groq API (OpenAI-compatible).
 * Rotacion de 3 API keys por mensaje (round-robin via AtomicUsize).
 * System prompt dinámico: pre-venta (servicios, precios) vs soporte de orden
 * (contexto de orden, fase actual, historial). Usa reqwest HTTP client. */

use std::fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

use sqlx::PgPool;
use uuid::Uuid;

use crate::repositories::{ChatRepository, OrderRepository};

/* [064A-29] Contador global para rotacion round-robin de API keys.
 * Cada llamada a generate_response incrementa y usa mod num_keys. */
static KEY_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Configuración del servicio de IA (Groq con rotación de keys)
#[derive(Clone)]
pub struct AiChatConfig {
    pub api_keys: Vec<String>,
    pub model: String,
    pub api_url: String,
}

impl AiChatConfig {
    /// Carga config desde variables de entorno. Soporta `GROQ_API_1`, `GROQ_API_2`, `GROQ_API_3`.
    /// Fallback a `AI_API_KEY`/`GEMINI_API_KEY`/`OPENAI_API_KEY` si no hay keys Groq.
    #[must_use]
    pub fn from_env() -> Self {
        let mut keys = Vec::new();
        for var in ["GROQ_API_1", "GROQ_API_2", "GROQ_API_3", "GROQ_API"] {
            if let Ok(k) = std::env::var(var) {
                if !k.is_empty() && !keys.contains(&k) {
                    keys.push(k);
                }
            }
        }
        /* Fallback a keys genéricas si no hay Groq */
        if keys.is_empty() {
            for var in ["AI_API_KEY", "GEMINI_API_KEY", "OPENAI_API_KEY"] {
                if let Ok(k) = std::env::var(var) {
                    if !k.is_empty() {
                        keys.push(k);
                        break;
                    }
                }
            }
        }

        let model = std::env::var("AI_MODEL")
            .unwrap_or_else(|_| "llama-3.3-70b-versatile".to_string());

        let api_url = std::env::var("AI_API_URL").unwrap_or_else(|_| {
            "https://api.groq.com/openai/v1/chat/completions".to_string()
        });

        Self {
            api_keys: keys,
            model,
            api_url,
        }
    }

    #[must_use]
    pub fn is_configured(&self) -> bool {
        !self.api_keys.is_empty()
    }

    /* [064A-29] Selecciona la siguiente API key en rotacion round-robin */
    fn next_key(&self) -> Option<&str> {
        if self.api_keys.is_empty() {
            return None;
        }
        let idx = KEY_COUNTER.fetch_add(1, Ordering::Relaxed) % self.api_keys.len();
        Some(&self.api_keys[idx])
    }
}

pub struct AiChatService;

impl AiChatService {
    /// Genera respuesta de IA para un mensaje en una sesión.
    /// Usa Groq API (OpenAI-compatible) con rotacion de keys.
    pub async fn generate_response(
        pool: &PgPool,
        config: &AiChatConfig,
        session_id: Uuid,
        user_message: &str,
    ) -> Result<String, String> {
        let Some(api_key) = config.next_key() else {
            return Ok(
                "Un miembro del equipo se conectará pronto para ayudarte. \
                 Mientras tanto, ¿en qué puedo orientarte?"
                    .to_string(),
            );
        };

        let system_prompt = build_system_prompt(pool, session_id).await;

        let history = ChatRepository::list_messages(pool, session_id, 20, 0)
            .await
            .unwrap_or_default();

        /* [064A-29] Formato OpenAI-compatible para Groq API */
        let mut messages = vec![serde_json::json!({
            "role": "system",
            "content": system_prompt
        })];

        for msg in &history {
            let role = if msg.sender_type == "ai" {
                "assistant"
            } else {
                "user"
            };
            messages.push(serde_json::json!({
                "role": role,
                "content": msg.content
            }));
        }

        messages.push(serde_json::json!({
            "role": "user",
            "content": user_message
        }));

        let body = serde_json::json!({
            "model": config.model,
            "messages": messages,
            "temperature": 0.7,
            "max_tokens": 800,
            "top_p": 0.9
        });

        let client = reqwest::Client::new();
        let response = client
            .post(&config.api_url)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
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

        /* [064A-29] Formato OpenAI: choices[0].message.content */
        let text = json["choices"][0]["message"]["content"]
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
                if let Ok((svc_title, _, plan_name)) =
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
