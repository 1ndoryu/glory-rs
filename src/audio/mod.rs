pub mod bpm;
pub mod ffmpeg;

/* [174A-3+174A-30+174A-31] Audio DSP pipeline (Symphonia + RustFFT). Fase 4.
 * `ffmpeg.rs` resuelve binarios externos, duración, conversiones base y waveform.
 * `bpm.rs` detecta tempo con onset + autocorrelación sobre audio decodificado.
 * Pendiente: key, embeddings y orquestación del pipeline en tareas siguientes. */
