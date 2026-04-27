pub mod bpm;
pub mod embeddings;
pub mod ffmpeg;
pub mod ia;
pub mod key;

/* [174A-3+174A-30+174A-31+174A-32+174A-33+174A-37] Audio DSP + IA pipeline. Fases 4-5.
 * `ffmpeg.rs` resuelve binarios externos, duración, conversiones base y waveform.
 * `bpm.rs` detecta tempo con onset + autocorrelación sobre audio decodificado.
 * `key.rs` estima tonalidad con cromagramas FFT y perfiles major/minor.
 * `embeddings.rs` genera el vector 128d determinista para similitud y perfiles.
 * `ia/` aloja clientes LLM/STT específicos del pipeline antes del service de negocio.
 *
 * [254A-8c-refactor] Eliminado ytdlp.rs: la descarga de audio se delega al
 * scraper Python externo (clients/kamples-scraper/). El backend Rust solo
 * coordina la cola_extraccion_samples; ver src/services/extension_recorte.rs. */
