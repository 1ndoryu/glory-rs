pub mod groq;
pub mod json_repairer;
pub mod openai;

/* [174A-37] Clientes IA específicos del pipeline de audio.
 * Aquí viven los wrappers HTTP hacia proveedores LLM/STT antes de llegar al
 * service IA de negocio. Aquí viven Groq y el fallback final de OpenAI. */