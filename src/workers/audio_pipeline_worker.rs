use crate::repositories::{
    ProcessingQueueRepository, QueueFailureDisposition, QueuedAudioProcessingJob,
};
use crate::services::{AudioPipelineRequest, AudioPipelineService, FileStorage};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

const AUDIO_PIPELINE_WORKER_CONCURRENCY: usize = 2;
const IDLE_POLL_INTERVAL: Duration = Duration::from_secs(5);
const ERROR_POLL_INTERVAL: Duration = Duration::from_secs(10);

/* [174A-35] Worker async del pipeline técnico de audio.
 * Consume `cola_procesamiento_ia` con `operacion=analisis_audio`, reclama jobs
 * vía `FOR UPDATE SKIP LOCKED` y delega el procesamiento al AudioPipelineService. */

#[derive(Clone)]
pub struct AudioPipelineWorker {
    worker_index: usize,
    pool: PgPool,
    service: AudioPipelineService,
}

pub fn spawn_audio_pipeline_workers(
    pool: &PgPool,
    storage: &Arc<dyn FileStorage>,
) -> Vec<JoinHandle<()>> {
    (0..AUDIO_PIPELINE_WORKER_CONCURRENCY)
        .map(|worker_index| {
            let worker = AudioPipelineWorker::new(worker_index, pool.clone(), storage.clone());
            tokio::spawn(async move {
                worker.run_forever().await;
            })
        })
        .collect()
}

impl AudioPipelineWorker {
    #[must_use]
    pub fn new(worker_index: usize, pool: PgPool, storage: Arc<dyn FileStorage>) -> Self {
        let service = AudioPipelineService::new(pool.clone(), storage);
        Self {
            worker_index,
            pool,
            service,
        }
    }

    pub async fn run_forever(self) {
        tracing::info!(worker = self.worker_index, "audio pipeline worker iniciado");

        loop {
            match ProcessingQueueRepository::claim_next_audio_analysis(&self.pool).await {
                Ok(Some(job)) => self.process_job(job).await,
                Ok(None) => sleep(IDLE_POLL_INTERVAL).await,
                Err(error) => {
                    tracing::error!(worker = self.worker_index, %error, "error reclamando job de audio pipeline");
                    sleep(ERROR_POLL_INTERVAL).await;
                }
            }
        }
    }

    async fn process_job(&self, job: QueuedAudioProcessingJob) {
        tracing::info!(
            worker = self.worker_index,
            job_id = job.id,
            sample_id = job.sample_id,
            attempts = job.intentos,
            "procesando job de audio pipeline"
        );

        match self
            .service
            .run(AudioPipelineRequest {
                sample_id: job.sample_id,
                force_recompute: false,
            })
            .await
        {
            Ok(result) => {
                if let Err(error) =
                    ProcessingQueueRepository::mark_audio_analysis_completed(&self.pool, job.id)
                        .await
                {
                    tracing::error!(
                        worker = self.worker_index,
                        job_id = job.id,
                        sample_id = job.sample_id,
                        %error,
                        "el pipeline terminó pero no se pudo marcar completado en cola"
                    );
                    return;
                }

                tracing::info!(
                    worker = self.worker_index,
                    job_id = job.id,
                    sample_id = job.sample_id,
                    activated = result.activated,
                    "job de audio pipeline completado"
                );
            }
            Err(error) => match ProcessingQueueRepository::mark_audio_analysis_failed(
                &self.pool,
                &job,
                &error.to_string(),
                error.is_retryable(),
            )
            .await
            {
                Ok(QueueFailureDisposition::RetryScheduled) => {
                    tracing::warn!(
                        worker = self.worker_index,
                        job_id = job.id,
                        sample_id = job.sample_id,
                        retryable = error.is_retryable(),
                        error = %error,
                        "job de audio pipeline falló y quedó reprogramado"
                    );
                }
                Ok(QueueFailureDisposition::FinalError) => {
                    tracing::error!(
                        worker = self.worker_index,
                        job_id = job.id,
                        sample_id = job.sample_id,
                        retryable = error.is_retryable(),
                        error = %error,
                        "job de audio pipeline agotó reintentos o falló de forma definitiva"
                    );
                }
                Err(queue_error) => {
                    tracing::error!(
                        worker = self.worker_index,
                        job_id = job.id,
                        sample_id = job.sample_id,
                        error = %error,
                        queue_error = %queue_error,
                        "job falló y tampoco se pudo actualizar la cola"
                    );
                }
            },
        }
    }
}
