use crate::repositories::{
    IaQueueFailureDisposition, IaQueueRepository, QueuedIaJob,
};
use crate::services::{AudioIaServiceError, IaQueueProcessRequest, IaQueueService};
use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

const IA_QUEUE_WORKER_CONCURRENCY: usize = 1;
const IDLE_POLL_INTERVAL: Duration = Duration::from_secs(5);
const ERROR_POLL_INTERVAL: Duration = Duration::from_secs(10);

/* [174A-42] Worker background de metadata creativa.
 * Consume `ia_queue`, delega el job a `IaQueueService` y aplica backoff
 * exponencial respetando `retry_after` cuando un proveedor devuelve 429. */

#[derive(Clone)]
pub struct IaQueueWorker {
    worker_index: usize,
    pool: PgPool,
    service: IaQueueService,
}

pub fn spawn_ia_queue_workers(pool: &PgPool) -> Vec<JoinHandle<()>> {
    let service = match IaQueueService::from_env(pool.clone()) {
        Ok(service) => service,
        Err(AudioIaServiceError::MissingProviders) => {
            tracing::warn!("ia queue worker deshabilitado: no hay proveedores IA configurados");
            return Vec::new();
        }
        Err(error) => {
            tracing::error!(%error, "ia queue worker no pudo inicializar servicio IA");
            return Vec::new();
        }
    };

    (0..IA_QUEUE_WORKER_CONCURRENCY)
        .map(|worker_index| {
            let worker = IaQueueWorker::new(worker_index, pool.clone(), service.clone());
            tokio::spawn(async move {
                worker.run_forever().await;
            })
        })
        .collect()
}

impl IaQueueWorker {
    #[must_use]
    pub fn new(worker_index: usize, pool: PgPool, service: IaQueueService) -> Self {
        Self {
            worker_index,
            pool,
            service,
        }
    }

    pub async fn run_forever(self) {
        tracing::info!(worker = self.worker_index, "ia queue worker iniciado");

        loop {
            match IaQueueRepository::claim_next_pending(&self.pool).await {
                Ok(Some(job)) => self.process_job(job).await,
                Ok(None) => sleep(IDLE_POLL_INTERVAL).await,
                Err(error) => {
                    tracing::error!(worker = self.worker_index, %error, "error reclamando job de ia_queue");
                    sleep(ERROR_POLL_INTERVAL).await;
                }
            }
        }
    }

    async fn process_job(&self, job: QueuedIaJob) {
        tracing::info!(
            worker = self.worker_index,
            job_id = job.id,
            sample_id = job.sample_id,
            attempts = job.attempts,
            "procesando job de ia_queue"
        );

        match self
            .service
            .run(IaQueueProcessRequest {
                sample_id: job.sample_id,
                job_metadata: job.metadata.clone(),
            })
            .await
        {
            Ok(result) => {
                if let Err(error) = IaQueueRepository::mark_completed(&self.pool, job.id).await {
                    tracing::error!(
                        worker = self.worker_index,
                        job_id = job.id,
                        sample_id = job.sample_id,
                        %error,
                        "el job IA terminó pero no se pudo marcar completado"
                    );
                    return;
                }

                tracing::info!(
                    worker = self.worker_index,
                    job_id = job.id,
                    sample_id = job.sample_id,
                    provider = result.provider.as_str(),
                    model = result.model,
                    title_updated = result.title_updated,
                    "job de ia_queue completado"
                );
            }
            Err(error) => match IaQueueRepository::mark_failed(
                &self.pool,
                &job,
                &error.to_string(),
                error.is_retryable(),
                error.retry_after_seconds(),
            )
            .await
            {
                Ok(IaQueueFailureDisposition::RetryScheduled) => {
                    tracing::warn!(
                        worker = self.worker_index,
                        job_id = job.id,
                        sample_id = job.sample_id,
                        retryable = error.is_retryable(),
                        retry_after_seconds = error.retry_after_seconds(),
                        error = %error,
                        "job de ia_queue falló y quedó reprogramado"
                    );
                }
                Ok(IaQueueFailureDisposition::FinalError) => {
                    tracing::error!(
                        worker = self.worker_index,
                        job_id = job.id,
                        sample_id = job.sample_id,
                        retryable = error.is_retryable(),
                        error = %error,
                        "job de ia_queue agotó reintentos o falló de forma definitiva"
                    );
                }
                Err(queue_error) => {
                    tracing::error!(
                        worker = self.worker_index,
                        job_id = job.id,
                        sample_id = job.sample_id,
                        error = %error,
                        queue_error = %queue_error,
                        "job IA falló y tampoco se pudo actualizar ia_queue"
                    );
                }
            },
        }
    }
}