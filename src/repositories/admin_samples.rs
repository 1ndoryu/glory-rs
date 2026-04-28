/* sentinel-disable-file sqlx-query-sin-macro sqlx-query-as-sin-macro — 274A-53 usa SQL runtime parametrizado para evitar cache SQLx nueva en el endpoint admin destructivo. */
/* [274A-53] Repositorio admin de limpieza masiva de samples.
 * Por que: legacy borra archivos fisicos y luego BD; Rust agrupa el borrado
 * de BD por IDs ya limpiados para evitar N roundtrips sin perder paridad. */

use sqlx::{FromRow, PgPool};

pub struct AdminSamplesRepository;

#[derive(Debug, Clone, FromRow)]
pub struct AdminSampleAssetRow {
    pub id: i32,
    pub ruta_original: Option<String>,
    pub ruta_optimizada: Option<String>,
    pub ruta_preview: Option<String>,
    pub ruta_waveform: Option<String>,
}

impl AdminSamplesRepository {
    pub async fn list_all_for_deletion(
        pool: &PgPool,
    ) -> Result<Vec<AdminSampleAssetRow>, sqlx::Error> {
        sqlx::query_as::<_, AdminSampleAssetRow>(
            r"SELECT id, ruta_original, ruta_optimizada, ruta_preview, ruta_waveform
                FROM samples
               ORDER BY id ASC",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn hard_delete_by_ids(pool: &PgPool, ids: &[i32]) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }

        let sample_ids = ids.to_vec();
        let mut tx = pool.begin().await?;

        sqlx::query(
            r"UPDATE cola_extraccion_samples
                  SET sample_id = NULL
                WHERE sample_id = ANY($1::int[])",
        )
        .bind(&sample_ids)
        .execute(&mut *tx)
        .await?;

        let deleted = sqlx::query(r"DELETE FROM samples WHERE id = ANY($1::int[])")
            .bind(&sample_ids)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            r"WITH counts AS (
                    SELECT u.id, COUNT(s.id)::int AS total
                      FROM usuarios_ext u
                      LEFT JOIN samples s
                        ON s.creador_id = u.id
                       AND s.eliminado_en IS NULL
                     GROUP BY u.id
               )
               UPDATE usuarios_ext u
                  SET total_samples = counts.total
                 FROM counts
                WHERE u.id = counts.id",
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(deleted.rows_affected())
    }
}
