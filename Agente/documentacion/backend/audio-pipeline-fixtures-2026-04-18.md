# 174A-36 — Tests con audio fixtures para el pipeline

## Objetivo

Cubrir el hueco entre los tests unitarios aislados de `ffmpeg`, BPM y tonalidad, y el comportamiento real del pipeline cuando varias etapas analizan el mismo archivo de audio.

## Implementación

- Se ampliaron `src/services/audio_pipeline/tests.rs` con fixtures WAV sintéticos generados al vuelo.
- Cada fixture representa un solo asset mono con:
  - onsets claros por beat para que el detector BPM vea pulsos reales;
  - una secuencia tonal distribuida en esos beats para que el detector de key/scale lea la armonía del mismo archivo;
  - material suficiente para que `inspect_audio_file()` produzca waveform y metadata consistente.
- Se añadieron dos escenarios:
  - `pipeline_fixture_major_loop_extracts_expected_analysis`: fixture `C major`, `120 BPM`, tipo `loop`.
  - `pipeline_fixture_minor_one_shot_updates_metadata_and_embedding`: fixture `C minor`, `90 BPM`, tipo `one_shot`, premium.

## Qué validan

- El mismo WAV puede pasar por `inspect_audio_file`, `detect_bpm_from_file` y `detect_key_from_file` sin contradicciones entre módulos.
- `build_technical_analysis()` propaga correctamente formato, duración, BPM, tonalidad y escala.
- `AudioPipelineService::build_embedding()` refleja el tipo `one_shot` y la bandera premium en los slots esperados del embedding.
- `build_pipeline_metadata()` mantiene consistente la metadata parcial del pipeline sobre resultados reales de audio.

## Validación

- `cargo test pipeline_fixture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`

## Resultado

La suite del backend quedó en `32` tests. Ahora el pipeline tiene cobertura con assets de audio sintéticos que ejercitan varias etapas a la vez, sin introducir dependencias de base de datos ni requerir fixtures binarios versionados en el repo.