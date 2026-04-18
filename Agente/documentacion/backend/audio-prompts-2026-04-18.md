# 174A-40 — Prompts parametrizados para IA de audio

## Objetivo

Sacar del futuro `ia_service` los prompts largos y frágiles para que el contrato de entrada/salida quede centralizado. El objetivo práctico era evitar drift entre Groq, OpenAI, `JsonRepairer` y la metadata que realmente consume el sistema legacy.

## Implementación

- Se creó `src/audio/ia/prompts.rs` con piezas reutilizables:
  - `AUDIO_CLASSIFICATION_SYSTEM_PROMPT` como system prompt compartido por Groq y OpenAI;
  - `SHARED_JSON_FIELD_INSTRUCTIONS` con el shape JSON esperado;
  - `build_analysis_prompt()`;
  - `append_transcription_context()`;
  - `build_correction_prompt()`.
- Se añadieron los value objects livianos:
  - `AudioAnalysisPromptInput`
  - `AudioExtractionContext`
  - `MetadataCorrectionPromptInput`
- `GroqChatRequest::new()` y `OpenAiChatRequest::new()` ahora reutilizan el mismo system prompt para eliminar duplicación.

## Alineación de contrato

- Durante esta tarea apareció un drift importante: el legacy y las migraciones siguen usando `carpeta_primaria` y `carpeta_secundaria`, pero el `JsonRepairer` Rust inicial no las estaba normalizando.
- Se corrigió el contrato de `AudioCreativeMetadata` para incluir ambos campos y se añadieron defaults compatibles:
  - `carpeta_primaria = "General"` cuando no exista o sea inválida;
  - `carpeta_secundaria = "General"` cuando venga vacía.
- Esto evita que `ia_service.rs` nazca con un mismatch silencioso entre prompt, parser y JSON persistido.

## Tests

- `src/audio/ia/prompts/tests.rs` valida:
  - contexto de extracción cuando existe;
  - fallback al nombre de archivo + tags + BPM + key + duración + path de origen;
  - truncado de transcripción a 3000 caracteres;
  - prompt de corrección con metadata actual e instrucciones compartidas.
- `src/audio/ia/json_repairer/tests.rs` se actualizó para verificar también las carpetas primarias/secundarias.

## Validación

- `cargo test prompts`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`