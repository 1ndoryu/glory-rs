# 174A-41 — Servicio IA para metadata creativa de samples

## Objetivo

Componer la capa de negocio que faltaba entre prompts, clientes LLM y `JsonRepairer`, para que el resto del sistema pueda pedir metadata creativa sin reimplementar fallback, parsing ni clasificación de errores.

## Implementación

- Se creó `src/services/ia_service.rs` con `AudioIaService` como orquestador de:
  - `build_analysis_prompt()`;
  - `GroqClient` como proveedor primario;
  - `OpenAiClient` como fallback final;
  - `JsonRepairer` como normalizador tipado hacia `AudioCreativeMetadata`.
- El servicio expone un contrato explícito de entrada/salida:
  - `AudioIaAnalysisRequest`;
  - `AudioIaAnalysisResult`;
  - `AudioIaProvider`.
- También tipa el plano de fallos para la siguiente tarea del worker:
  - `AudioIaFailure`;
  - `OpenAiAttemptFailure`;
  - `AudioIaServiceError` con `is_retryable()` y `retry_after_seconds()`.

## Decisiones

- El servicio no depende todavía de BD ni de cola. En esta tarea solo clasifica y devuelve metadata creativa; el acople con `ia_queue` queda para `174A-42`.
- Un parse failure de `JsonRepairer` no aborta inmediatamente el flujo: si Groq responde texto inválido, se intenta OpenAI antes de dar el resultado por agotado.
- El error final conserva suficiente señal operativa para backoff futuro sin filtrar detalles HTTP dentro del worker.

## Tests

- `src/services/ia_service/tests.rs` cubre:
  - éxito directo con Groq;
  - fallback a OpenAI tras agotamiento de Groq;
  - fallback a OpenAI cuando Groq devuelve texto no reparable;
  - agregado de `retry_after_seconds()` e `is_retryable()` cuando fallan ambos proveedores.

## Validación

- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test ia_service`