pub mod groq;

/* [174A-37] Clientes IA específicos del pipeline de audio.
 * Aquí viven los wrappers HTTP hacia proveedores LLM/STT antes de llegar al
 * service IA de negocio. El primero en entrar es Groq con rotación de keys. */