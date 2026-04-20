mod audio_pipeline_worker;
mod billing_cleanup_worker;
mod ia_queue_worker;
mod scraping_queue_worker;

/* [174A-35] Workers background del backend.
 * El primero en existir consume `cola_procesamiento_ia` para ejecutar el
 * pipeline técnico de audio en background con reclamo optimista de jobs.
 * [174A-92] billing_cleanup_worker expira suscripciones vencidas.
 * [174A-95] scraping_queue_worker reagenda rescrapes elegibles. */

pub use audio_pipeline_worker::{spawn_audio_pipeline_workers, AudioPipelineWorker};
pub use billing_cleanup_worker::spawn_billing_cleanup_worker;
pub use ia_queue_worker::{spawn_ia_queue_workers, IaQueueWorker};
pub use scraping_queue_worker::spawn_scraping_queue_worker;
