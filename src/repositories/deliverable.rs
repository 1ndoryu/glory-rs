/* [044A-38 Fase 6] Repositorio de entregables (phase_deliverables).
 * CRUD sobre archivos asociados a fases de órdenes. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::PhaseDeliverable;

/// Parámetros para crear un entregable
pub struct CreateDeliverableParams<'a> {
    pub phase_id: Uuid,
    pub uploaded_by: Uuid,
    pub file_name: &'a str,
    pub file_url: &'a str,
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<&'a str>,
    pub revision_number: i32,
    pub notes: Option<&'a str>,
}

pub struct DeliverableRepository;

impl DeliverableRepository {
    /// Guarda un registro de entregable en BD
    pub async fn create(
        pool: &PgPool,
        params: CreateDeliverableParams<'_>,
    ) -> Result<PhaseDeliverable, AppError> {
        let row = sqlx::query_as::<_, PhaseDeliverable>(
            r"INSERT INTO phase_deliverables
              (phase_id, uploaded_by, file_name, file_url, file_size_bytes, mime_type, revision_number, notes)
              VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
              RETURNING *"
        )
        .bind(params.phase_id)
        .bind(params.uploaded_by)
        .bind(params.file_name)
        .bind(params.file_url)
        .bind(params.file_size_bytes)
        .bind(params.mime_type)
        .bind(params.revision_number)
        .bind(params.notes)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row)
    }

    /// Lista entregables de una fase, ordenados por revision + fecha
    pub async fn list_by_phase(
        pool: &PgPool,
        phase_id: Uuid,
    ) -> Result<Vec<PhaseDeliverable>, AppError> {
        let rows = sqlx::query_as::<_, PhaseDeliverable>(
            r"SELECT * FROM phase_deliverables
              WHERE phase_id = $1
              ORDER BY revision_number DESC, created_at DESC"
        )
        .bind(phase_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(rows)
    }

    /// Lista entregables de una revisión específica
    pub async fn list_by_revision(
        pool: &PgPool,
        phase_id: Uuid,
        revision_number: i32,
    ) -> Result<Vec<PhaseDeliverable>, AppError> {
        let rows = sqlx::query_as::<_, PhaseDeliverable>(
            r"SELECT * FROM phase_deliverables
              WHERE phase_id = $1 AND revision_number = $2
              ORDER BY created_at ASC"
        )
        .bind(phase_id)
        .bind(revision_number)
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(rows)
    }

    /// Busca un entregable por ID
    pub async fn find_by_id(
        pool: &PgPool,
        deliverable_id: Uuid,
    ) -> Result<Option<PhaseDeliverable>, AppError> {
        let row = sqlx::query_as::<_, PhaseDeliverable>(
            "SELECT * FROM phase_deliverables WHERE id = $1"
        )
        .bind(deliverable_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row)
    }

    /// Obtiene el número de revisión actual (max `revision_number`) de una fase
    pub async fn current_revision_number(
        pool: &PgPool,
        phase_id: Uuid,
    ) -> Result<i32, AppError> {
        let row: (Option<i32>,) = sqlx::query_as(
            "SELECT MAX(revision_number) FROM phase_deliverables WHERE phase_id = $1"
        )
        .bind(phase_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row.0.unwrap_or(0))
    }
}
