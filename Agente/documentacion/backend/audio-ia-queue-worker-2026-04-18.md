# 174A-42 — Cola y worker de metadata creativa

## Objetivo

Desacoplar la fase creativa de IA del pipeline técnico de audio. En Rust, `cola_procesamiento_ia/analisis_audio` ya quedó dedicada a DSP + embedding, así que hacía falta una cola propia para LLMs con su propio scheduling y backoff.

## Implementación

- Se creó la migración `migrations/20260418000010_ia_queue.up.sql` con la tabla `ia_queue`:
  - `status` (`pending`, `processing`, `retry_scheduled`, `completed`, `failed`)
  - `attempts`, `max_attempts`, `next_retry_at`, `last_error`, `metadata`
  - índice FIFO para pendientes/reintentos
  - índice único parcial por `sample_id` para no duplicar jobs activos.
- Se agregó `src/repositories/ia_queue.rs` para:
  - encolar jobs de sample;
  - reclamar el siguiente job con `FOR UPDATE SKIP LOCKED`;
  - marcar completado o error final/reprogramado;
  - calcular backoff exponencial respetando `retry_after` si el proveedor lo devuelve.
- `SampleRepository::complete_audio_pipeline()` ahora activa el sample y encola el job IA dentro de la misma transacción.
- Se agregó `src/services/ia_queue.rs` como capa que:
  - carga el sample listo para IA;
  - construye `AudioIaAnalysisRequest` desde `samples` + `metadata`;
  - llama a `AudioIaService`;
  - preserva carpetas manuales existentes;
  - persiste metadata creativa, título, slug, descripción y tipo.
- Se creó `src/workers/ia_queue_worker.rs`, exportado desde `src/workers/mod.rs` y arrancado desde `src/main.rs`.

## Decisiones

- El worker no arranca si no hay proveedores IA configurados. El servidor sigue levantando, pero la cola queda deshabilitada con log explícito.
- El sample queda con `ia_pending=true` y `ia_queue_status="pending"` al cerrar el pipeline técnico; cuando la IA completa, esos flags se actualizan a estado final.
- La persistencia IA no toca `tags` manuales: solo mergea `metadata`, y el trigger ya existente recalcula `tags_enriquecidos`.

## Tests

- `src/repositories/ia_queue.rs` valida el backoff exponencial con y sin `retry_after`.
- `src/services/ia_queue/tests.rs` valida:
  - composición del título final con BPM + key menor;
  - preservación de carpetas manuales aunque la IA sugiera otras.

## Validación

- `sqlx migrate run`
- `cargo sqlx prepare`
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test ia_queue`