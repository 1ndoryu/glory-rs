# Audio Pipeline — 2026-04-18

## Estado actual

- `174A-34`: existe `src/services/audio_pipeline.rs` como orquestador técnico del pipeline de audio.
- El servicio ya corre inspección de archivo, detección de BPM, detección de tonalidad, generación de waveform JSON, conversión a MP3 optimizado y embedding `128d`.
- La implementación es agnóstica al backend de storage: lee el original vía `FileStorage`, trabaja en un workspace temporal y sube de vuelta los derivados al storage lógico.

## Alcance de esta tarea

- Sí resuelto en `174A-34`:
  - semáforo global de concurrencia `2`
  - timeouts por etapa (`inspect`, `bpm`, `key`, `mp3`)
  - persistencia parcial en DB después de análisis, waveform y MP3
  - activación final del sample con `embedding` y metadata de pipeline
- Intencionalmente fuera de alcance por roadmap:
  - consumo de cola / retries / backoff (`174A-35`)
  - IA creativa y enriquecimiento de metadata (`174A-37+`)
  - preview de 30s y rename de slug/título al estilo legacy
  - deduplicación perceptual y estados de supervisión estilo PHP

## Diseño aplicado

- Entrada pública: `AudioPipelineService::run(AudioPipelineRequest)`.
- El servicio carga el sample desde `SampleRepository`, valida que tenga `ruta_original` y, salvo `force_recompute`, exige estado `procesando`.
- El original se descarga con `FileStorage::get_bytes()`, se escribe a un directorio temporal y se procesa desde ahí.
- Los resultados parciales se guardan en `metadata.audio_pipeline` sin pisar el resto del `metadata` del sample.
- Los assets derivados usan la misma carpeta lógica del original y nombres estables con `id_corto`:
  - `{id_corto}_waveform.json`
  - `{id_corto}_optimizado.mp3`

## Persistencia parcial

- Query 1: análisis técnico
  - `duracion`, `formato`, `tamano`, `bpm`, `key`, `escala`
  - `metadata.audio_pipeline.status = analyzed`
- Query 2: waveform
  - `ruta_waveform`
  - `metadata.audio_pipeline.status = waveform_ready`
- Query 3: MP3 optimizado
  - `ruta_optimizada`
  - `metadata.audio_pipeline.status = optimized_ready`
- Query 4: activación final
  - `embedding`
  - `estado = activo`
  - `publicado_at = COALESCE(publicado_at, NOW())`
  - `metadata.audio_pipeline.status = completed`
- En error:
  - no se pierden los resultados ya persistidos
  - `metadata.audio_pipeline.status = error`
  - se guarda `last_stage` y `last_error`

## Compatibilidad / diferencias con el legacy

- El servicio NO asume rutas locales en `ruta_original`; eso corrige la dependencia implícita del pipeline PHP a disco local.
- Se mantiene la semántica del pipeline técnico, pero no se porta aún la parte de IA, rename ni deduplicación dura porque el roadmap las separa en tareas posteriores.
- La cola real del repo hoy no es `pending_processing`, sino `cola_procesamiento_ia` con `operacion = analisis_audio`. El servicio quedó intencionalmente `queue-agnostic` para que `174A-35` lo reutilice sin mezclar responsabilidades.

## Validación realizada

- `cargo sqlx prepare -- --lib` OK.
- `cargo check` OK.
- `cargo clippy --all-targets -- -D warnings` OK.
- `cargo test audio_pipeline` OK.
- `cargo test` OK.

## Archivos tocados

- `src/services/audio_pipeline.rs`
- `src/services/audio_pipeline/tests.rs`
- `src/services/mod.rs`
- `src/repositories/sample.rs`
- `src/repositories/mod.rs`
- `.sqlx/query-108725a2454cc291259b2c8dd57609acb890e3d3e6f4393eb64f2a54b8780d88.json`
- `.sqlx/query-4409b35859c832d56ce7243b01473f477ba1e3482829d7a3f8a96f45459c8dc6.json`
- `.sqlx/query-7fddbdb6614a807a885c107763f28cf1f86c06e403b8e0053d467f51c685b077.json`
- `.sqlx/query-bbcbf80539f846ac22f6e9ebeb6ed5a62f7229628a815355ec2f5b66b04be20d.json`
- `.sqlx/query-e47101be3abbd50568acb1914c73d9b41f60e565ea2c1980c24ac37097d897c7.json`