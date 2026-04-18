# 174A-38 — Cliente OpenAI como fallback final

## Objetivo

Agregar el cliente OpenAI que entra al final de la cadena de proveedores IA. Su papel es ofrecer una salida robusta cuando Groq no pueda completar la clasificación y mantener una interfaz simple para la futura capa `service IA`.

## Implementación

- Se creó `src/audio/ia/openai.rs` como wrapper de `POST /v1/chat/completions`.
- El cliente carga `OPENAI_API_KEY` desde entorno y usa `gpt-4o-mini` como modelo por defecto.
- `OpenAiChatRequest` mantiene el mismo shape operativo esperado por la fase IA:
  - prompt de usuario;
  - prompt de sistema configurable;
  - `temperature`;
  - `max_tokens`;
  - bandera `require_json_object`.
- Si OpenAI responde `400` con error `json_validate_failed`, el cliente reintenta una sola vez sin `response_format=json_object`. Esto replica el problema real del legacy y deja que la siguiente tarea (`JsonRepairer`) resuelva la reparación semántica.
- Los errores HTTP conservan `status_code`, `retry_after_seconds` y `retryable` para que el worker IA futuro pueda decidir entre backoff, requeue o abandono.

## Tests

- `src/audio/ia/openai/tests.rs` usa un servidor HTTP local con Axum para validar:
  - request exitosa con `Authorization: Bearer`;
  - retry sin `response_format` tras `json_validate_failed`;
  - lectura de `OPENAI_API_KEY`;
  - preservación de `Retry-After` en errores `429`.

## Decisiones

- Se evitó una abstracción prematura entre Groq y OpenAI. Ambos clientes comparten semántica, pero todavía tienen diferencias reales de error handling y política de fallback; unificarlos antes de `174A-41` metería ruido.
- La respuesta sigue siendo texto bruto del assistant. El parseo robusto queda reservado para `174A-39` (`JsonRepairer`).
- El endpoint quedó configurable para pruebas y posibles proxies internos.

## Validación

- `cargo test openai`
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`