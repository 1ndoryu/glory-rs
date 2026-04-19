use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;

pub struct ProcessingQueueRepository;

#[derive(Debug, Clone)]
pub struct QueuedAudioProcessingJob {
    pub id: i32,
    pub sample_id: i32,
    pub intentos: i32,
    pub max_intentos: i32,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueFailureDisposition {
    RetryScheduled,
    FinalError,
}

impl ProcessingQueueRepository {
    pub async fn claim_next_audio_analysis(
        pool: &PgPool,
    ) -> Result<Option<QueuedAudioProcessingJob>, sqlx::Error> {
        sqlx::query_as!(
            QueuedAudioProcessingJob,
            "WITH picked AS (
                SELECT id
                FROM cola_procesamiento_ia
                WHERE tipo = 'sample'
                  AND operacion = 'analisis_audio'
                  AND (
                    estado = 'pendiente'
                    OR (estado = 'error_reintento' AND (proximo_intento IS NULL OR proximo_intento <= NOW()))
                  )
                ORDER BY created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            UPDATE cola_procesamiento_ia AS cola
            SET estado = 'procesando'
            FROM picked
            WHERE cola.id = picked.id
            RETURNING
                cola.id,
                cola.entidad_id as \"sample_id!\",
                cola.intentos,
                cola.max_intentos,
                cola.metadata as \"metadata!: serde_json::Value\",
                cola.created_at as \"created_at!: DateTime<Utc>\"",
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn mark_audio_analysis_completed(
        pool: &PgPool,
        job_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE cola_procesamiento_ia
             SET estado = 'completado',
                 ultimo_error = NULL,
                 proximo_intento = NULL,
                 procesado_at = NOW()
             WHERE id = $1",
            job_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn mark_audio_analysis_failed(
        pool: &PgPool,
        job: &QueuedAudioProcessingJob,
        error_message: &str,
        retryable: bool,
    ) -> Result<QueueFailureDisposition, sqlx::Error> {
        let next_attempt = job.intentos + 1;
        let is_final = !retryable || next_attempt >= job.max_intentos;
        let disposition = if is_final {
            QueueFailureDisposition::FinalError
        } else {
            QueueFailureDisposition::RetryScheduled
        };
        let next_retry_at = if is_final {
            None
        } else {
            Some(Utc::now() + Duration::minutes(retry_backoff_minutes(next_attempt)))
        };
        let next_state = if is_final {
            "error_final"
        } else {
            "error_reintento"
        };
        let processed_at = if is_final { Some(Utc::now()) } else { None };

        sqlx::query!(
            "UPDATE cola_procesamiento_ia
             SET estado = $2,
                 intentos = $3,
                 ultimo_error = $4,
                 proximo_intento = $5,
                 procesado_at = COALESCE($6, procesado_at)
             WHERE id = $1",
            job.id,
            next_state,
            next_attempt,
            error_message,
            next_retry_at,
            processed_at,
        )
        .execute(pool)
        .await?;

        Ok(disposition)
    }
}

#[must_use]
pub fn retry_backoff_minutes(attempt: i32) -> i64 {
    match attempt.max(1) {
        1 => 15,
        2 => 30,
        3 => 60,
        _ => 120,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_caps_at_two_hours() {
        assert_eq!(retry_backoff_minutes(1), 15);
        assert_eq!(retry_backoff_minutes(2), 30);
        assert_eq!(retry_backoff_minutes(3), 60);
        assert_eq!(retry_backoff_minutes(4), 120);
        assert_eq!(retry_backoff_minutes(8), 120);
    }
}
