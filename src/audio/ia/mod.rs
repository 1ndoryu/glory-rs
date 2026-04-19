pub mod groq;
pub mod json_repairer;
pub mod openai;
pub mod prompts;

/* [174A-37] Clientes IA específicos del pipeline de audio.
 * Aquí viven los wrappers HTTP hacia proveedores LLM/STT antes de llegar al
 * service IA de negocio. Aquí viven Groq, OpenAI, prompts y JsonRepairer. */
