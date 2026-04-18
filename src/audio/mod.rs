pub mod bpm;
pub mod embeddings;
pub mod ffmpeg;
pub mod key;

/* [174A-3+174A-30+174A-31+174A-32+174A-33] Audio DSP pipeline (Symphonia + RustFFT). Fase 4.
 * `ffmpeg.rs` resuelve binarios externos, duración, conversiones base y waveform.
 * `bpm.rs` detecta tempo con onset + autocorrelación sobre audio decodificado.
 * `key.rs` estima tonalidad con cromagramas FFT y perfiles major/minor.
 * `embeddings.rs` genera el vector 128d determinista para similitud y perfiles.
 * Pendiente: orquestación del pipeline en tareas siguientes. */
