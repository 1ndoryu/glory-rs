# Audio FFmpeg — 2026-04-18

## Estado actual

- `174A-30`: existe `src/audio/ffmpeg.rs` como base del pipeline de audio de Fase 4.
- El módulo detecta `ffmpeg`/`ffprobe`, inspecciona duración y formato, genera waveform peaks y expone conversiones base a MP3/FLAC.
- La integración con colas/workers queda para `174A-34` y `174A-35`.

## Capacidades implementadas

- `FFmpegBinaries::detect()` busca binarios en este orden:
  - `FFMPEG_PATH` y `FFPROBE_PATH`
  - `PATH`
  - rutas comunes de Windows/Linux
  - paquete WinGet `Gyan.FFmpeg*` en Windows
- `inspect_audio_file()` devuelve:
  - `format`
  - `duration_seconds`
  - `sample_rate_hz`
  - `channels`
  - `file_size_bytes`
  - `waveform_peaks`
- `convert_to_mp3()` usa `libmp3lame` a `320k` y `44100 Hz`.
- `convert_to_flac()` genera un derivado FLAC lossless.

## Estrategia híbrida

- Si `ffprobe` está disponible, la duración se toma desde CLI y se corrige con ese valor.
- Si `ffmpeg` está disponible, el waveform usa un render temporal `s16le mono 8kHz` y luego reduce a 120 barras.
- Si falla o no existe FFmpeg, metadata y waveform caen a Symphonia con un path totalmente Rust.
- La conversión a MP3/FLAC no tiene fallback silencioso: si falta `ffmpeg`, el módulo devuelve `AudioError::FfmpegNotFound`.

## Testing validado

- `cargo clippy -- -D warnings` OK.
- `cargo test` OK.
- Hay tests para:
  - override por env en detección de binarios
  - inspección WAV sin FFmpeg usando Symphonia
  - error explícito cuando se intenta convertir sin FFmpeg
  - conversión real a MP3 y FLAC cuando `ffmpeg` está disponible en la máquina local

## Notas de migración

- El legado PHP usaba FFprobe para duración y FFmpeg para waveform/MP3 preview. La migración conserva esa dirección, pero evita acoplar toda la capa a procesos externos.
- Este módulo todavía no escribe DB ni storage; solo encapsula procesamiento de archivo. La persistencia/orquestación se hará en `audio_pipeline.rs` y el worker de cola.
- La parte de detección/conversión es suficientemente agnóstica para evaluarse como candidata a `glory-rs`, pero conviene esperar a fijar la interfaz que consumirá `174A-34`.