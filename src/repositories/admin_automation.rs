/* sentinel-disable-file sqlx-query-sin-macro sqlx-query-as-sin-macro — 274A-46 usa SQL runtime parametrizado por SQLX_OFFLINE sin cache nueva para consultas admin. */
/* [274A-46] Repositorio de automatizacion: lotes e historial salen del handler
 * para que reactivar pueda compartir el mismo contrato de estado. */

use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};

pub struct AdminAutomationRepository;

#[derive(Debug, Clone, FromRow)]
pub struct AutomationBatchRow {
    pub id: i32,
    pub tipo: String,
    pub estado: String,
    pub iniciado_at: DateTime<Utc>,
    pub completado_at: Option<DateTime<Utc>>,
    pub exitosos: i32,
    pub fallidos: i32,
    pub recortes: i32,
    pub samples_publicados: i32,
    pub canciones_nuevas: i32,
    pub sampleos_nuevos: i32,
    pub error_mensaje: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl AdminAutomationRepository {
    pub async fn latest_batch(
        pool: &PgPool,
        process_type: &str,
    ) -> Result<Option<AutomationBatchRow>, sqlx::Error> {
        sqlx::query_as::<_, AutomationBatchRow>(
            r"SELECT id, tipo, estado, iniciado_at, completado_at,
                      exitosos, fallidos, recortes, samples_publicados,
                      canciones_nuevas, sampleos_nuevos, error_mensaje, metadata
                 FROM lotes_procesamiento
                WHERE tipo = $1
                ORDER BY iniciado_at DESC
                LIMIT 1",
        )
        .bind(process_type)
        .fetch_optional(pool)
        .await
    }

    pub async fn consecutive_failures(
        pool: &PgPool,
        process_type: &str,
    ) -> Result<i32, sqlx::Error> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r"SELECT estado
                 FROM lotes_procesamiento
                WHERE tipo = $1
                ORDER BY iniciado_at DESC
                LIMIT 20",
        )
        .bind(process_type)
        .fetch_all(pool)
        .await?;

        let mut count = 0_i32;
        for (status,) in rows {
            if status == "error" {
                count += 1;
            } else {
                break;
            }
        }

        Ok(count)
    }

    pub async fn list_history(
        pool: &PgPool,
        process_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AutomationBatchRow>, sqlx::Error> {
        sqlx::query_as::<_, AutomationBatchRow>(
            r"SELECT id, tipo, estado, iniciado_at, completado_at,
                      exitosos, fallidos, recortes, samples_publicados,
                      canciones_nuevas, sampleos_nuevos, error_mensaje, metadata
                 FROM lotes_procesamiento
                WHERE ($1::text IS NULL OR tipo = $1)
                ORDER BY iniciado_at DESC
                LIMIT $2 OFFSET $3",
        )
        .bind(process_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn count_history(
        pool: &PgPool,
        process_type: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        let total: (i64,) = sqlx::query_as(
            r"SELECT COUNT(*)::bigint
                 FROM lotes_procesamiento
                WHERE ($1::text IS NULL OR tipo = $1)",
        )
        .bind(process_type)
        .fetch_one(pool)
        .await?;

        Ok(total.0)
    }
}
