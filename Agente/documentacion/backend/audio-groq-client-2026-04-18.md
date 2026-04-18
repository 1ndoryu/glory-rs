# 174A-37 — Cliente Groq con rotación de keys y fallback de modelos

## Objetivo

Crear la capa HTTP reutilizable para Groq antes de construir el service IA de negocio. El cliente debía resolver tres problemas del sistema legacy: rotación entre varias API keys, retry controlado por modelo y fallback ordenado entre modelos LLM.

## Implementación

- Se creó `src/audio/ia/groq.rs` como cliente `reqwest` específico para `chat/completions` de Groq.
- El cliente carga keys desde entorno con la misma prioridad práctica del legacy:
  - `GROQ_API_1`, `GROQ_API_2`, `GROQ_API_3`
  - `GROQ_API` solo como fallback si no hay numeradas válidas.
- La rotación de keys ocurre por intento usando un contador atómico round-robin compartido.
- La cadena de modelos inicial replica el orden del PHP legado:
  - `openai/gpt-oss-120b`
  - `moonshotai/kimi-k2-instruct-0905`
  - `moonshotai/kimi-k2-instruct`
  - `llama-3.3-70b-versatile`
  - `qwen/qwen3-32b`
  - `meta-llama/llama-4-scout-17b-16e-instruct`
  - `openai/gpt-oss-20b`
  - `groq/compound`
- Cada intento arma payload OpenAI-compatible con `system`, `user`, `temperature`, `max_tokens` y `response_format = json_object` cuando se pide JSON estricto.
- El cliente devuelve éxito tipado (`GroqChatSuccess`) o un error agotado con historial de fallos (`GroqAttemptFailure`) para que la siguiente capa pueda decidir entre cola, backoff, OpenAI fallback o moderación.

## Tests

- `src/audio/ia/groq/tests.rs` monta un servidor HTTP local con Axum para validar comportamiento real de red sin salir a Internet.
- Casos cubiertos:
  - rotación de keys y fallback entre modelos;
  - retry dentro del mismo modelo;
  - carga de env vars numeradas;
  - parsing de `retry_after` desde el mensaje `Please retry in ...`.

## Decisiones

- La rotación se resolvió por intento, no por proceso, porque en el backend Rust el servidor es persistente y conviene distribuir carga entre keys en tiempo real.
- El cliente devuelve el texto bruto del assistant, no JSON parseado, para dejar `JsonRepairer` y la interpretación semántica en las tareas siguientes (`174A-39` y `174A-41`).
- El endpoint quedó configurable para permitir tests y futuros overrides sin tocar la lógica principal.

## Validación

- `cargo test groq`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`