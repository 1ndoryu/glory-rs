# 174A-35 — Worker async de `cola_procesamiento_ia`

## Objetivo

Conectar el nuevo `AudioPipelineService` al flujo real del backend para que los uploads encolados con `operacion = 'analisis_audio'` se procesen en background sin depender de WP-Cron ni de rutas locales del disco.

## Implementación

- Se creó `src/repositories/processing_queue.rs` para encapsular el acceso a `cola_procesamiento_ia` con macros `sqlx::query!` / `query_as!`.
- El claim del siguiente job usa `FOR UPDATE SKIP LOCKED`, filtrando solo `tipo = 'sample'` y `operacion = 'analisis_audio'` en estados `pendiente` o `error_reintento` cuyo `proximo_intento` ya venció.
- Se añadió `src/workers/audio_pipeline_worker.rs` con dos consumidores Tokio (`AUDIO_PIPELINE_WORKER_CONCURRENCY = 2`). Cada worker:
  - reclama un job elegible;
  - llama a `AudioPipelineService::run(sample_id, force_recompute = false)`;
  - marca la cola como `completado` al terminar;
  - o programa `error_reintento` / `error_final` según `is_retryable()` y `max_intentos`.
- El backoff replica la semántica legacy de la cola compartida: `15m`, `30m`, `60m`, y luego cap en `120m`.
- `src/main.rs` ahora arranca los workers automáticamente junto al servidor HTTP, reusando el mismo `PgPool` y el mismo backend de `FileStorage`.

## Decisiones

- El roadmap hablaba de `pending_processing`, pero el sistema real del repo usa `cola_procesamiento_ia`; el worker quedó alineado con la tabla y los estados existentes para evitar otra capa de compatibilidad ficticia.
- La coordinación multi-instancia se resuelve con row locking en PostgreSQL, no con un lock distribuido externo.
- La concurrencia quedó en `2` para respetar el límite ya introducido en `AudioPipelineService` y el objetivo original de no saturar FFmpeg/IO.

## Validación

- `cargo sqlx prepare -- --lib`
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test backoff_caps_at_two_hours`
- `cargo test`

## Pendiente inmediato

- `174A-36` debe cubrir el worker/pipeline con fixtures de audio reales o semi-reales para validar el flujo extremo a extremo desde la cola hasta la activación del sample.