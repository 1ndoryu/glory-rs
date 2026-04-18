mod audio_pipeline_worker;

/* [174A-35] Workers background del backend.
 * El primero en existir consume `cola_procesamiento_ia` para ejecutar el
 * pipeline técnico de audio en background con reclamo optimista de jobs. */

pub use audio_pipeline_worker::{spawn_audio_pipeline_workers, AudioPipelineWorker};
