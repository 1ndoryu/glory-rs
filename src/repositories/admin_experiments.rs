/* sentinel-disable-file sqlx-query-sin-macro sqlx-query-as-sin-macro — queries batch parametrizadas runtime; SQLX_OFFLINE evita macros nuevas sin cache local. */
/* [274A-49+50+51+52] Repositorio admin para experimentos, embeddings y benchmark.
 * Mantiene SQL fuera de handlers/servicios y usa la estructura real de samples,
 * usuarios, notificaciones y pgvector. */

use pgvector::Vector;
use sqlx::{FromRow, PgPool};

use crate::errors::AppError;

pub struct AdminExperimentsRepository;

#[derive(Debug, Clone, FromRow)]
pub struct EmbeddingSampleRow {
    pub id: i32,
    pub bpm: Option<i32>,
    pub music_key: Option<String>,
    pub scale: Option<String>,
    pub duration_seconds: f32,
    pub sample_type: String,
    pub is_premium: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TestUserRow {
    pub id: i32,
    pub username: String,
    pub nombre_visible: String,
}

impl AdminExperimentsRepository {
    pub async fn list_embedding_candidates(
        pool: &PgPool,
        limit: i64,
        only_missing: bool,
    ) -> Result<Vec<EmbeddingSampleRow>, AppError> {
        let rows = sqlx::query_as::<_, EmbeddingSampleRow>(
            "SELECT id,
                    bpm,
                    key AS music_key,
                    escala AS scale,
                    duracion AS duration_seconds,
                    tipo AS sample_type,
                    COALESCE(es_premium, FALSE) AS is_premium,
                    COALESCE(tags, '{}') AS tags
             FROM samples
             WHERE eliminado_en IS NULL
               AND estado <> 'eliminado'
               AND (NOT $2 OR embedding IS NULL)
             ORDER BY id
             LIMIT $1",
        )
        .bind(limit)
        .bind(only_missing)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn update_sample_embedding(
        pool: &PgPool,
        sample_id: i32,
        embedding: Vector,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE samples
             SET embedding = $2,
                 updated_at = NOW()
             WHERE id = $1
               AND eliminado_en IS NULL",
        )
        .bind(sample_id)
        .bind(embedding)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn clear_embeddings(pool: &PgPool) -> Result<i64, AppError> {
        let result = sqlx::query(
            "UPDATE samples
             SET embedding = NULL,
                 updated_at = NOW()
             WHERE eliminado_en IS NULL",
        )
        .execute(pool)
        .await?;
        i64::try_from(result.rows_affected()).map_err(|_| {
            AppError::Internal("Cantidad de embeddings limpiados fuera de rango".into())
        })
    }

    pub async fn find_test_user(
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<TestUserRow>, AppError> {
        let row = sqlx::query_as::<_, TestUserRow>(
            "SELECT id, username, nombre_visible
             FROM usuarios_ext
             WHERE username = $1
             LIMIT 1",
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn upsert_test_user(
        pool: &PgPool,
        username: &str,
        email: &str,
        display_name: &str,
        bio: &str,
        avatar_url: &str,
    ) -> Result<TestUserRow, AppError> {
        let row = sqlx::query_as::<_, TestUserRow>(
            "INSERT INTO usuarios_ext (
                wp_user_id, username, email, nombre_visible, bio, avatar_url, rol, estado, es_seed
             ) VALUES (-2026042801, $1, $2, $3, $4, $5, 'usuario', 'activo', TRUE)
             ON CONFLICT (username) DO UPDATE
             SET email = EXCLUDED.email,
                 nombre_visible = EXCLUDED.nombre_visible,
                 bio = EXCLUDED.bio,
                 avatar_url = EXCLUDED.avatar_url,
                 estado = 'activo'
             RETURNING id, username, nombre_visible",
        )
        .bind(username)
        .bind(email)
        .bind(display_name)
        .bind(bio)
        .bind(avatar_url)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn latest_sample_id_for_creator(
        pool: &PgPool,
        creator_id: i32,
    ) -> Result<Option<i32>, AppError> {
        let sample_id = sqlx::query_scalar::<_, i32>(
            "SELECT id
             FROM samples
             WHERE creador_id = $1
               AND eliminado_en IS NULL
             ORDER BY created_at DESC
             LIMIT 1",
        )
        .bind(creator_id)
        .fetch_optional(pool)
        .await?;
        Ok(sample_id)
    }

    pub async fn active_sample_count(pool: &PgPool) -> Result<i64, AppError> {
        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint
             FROM samples
             WHERE estado = 'activo'
               AND eliminado_en IS NULL",
        )
        .fetch_one(pool)
        .await?;
        Ok(total)
    }

    pub async fn embedding_count(pool: &PgPool) -> Result<i64, AppError> {
        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint
             FROM samples
             WHERE embedding IS NOT NULL
               AND eliminado_en IS NULL",
        )
        .fetch_one(pool)
        .await?;
        Ok(total)
    }

    pub async fn benchmark_feed(pool: &PgPool, limit: i64) -> Result<i64, AppError> {
        let rows = sqlx::query_scalar::<_, i32>(
            "SELECT id
             FROM samples
             WHERE estado = 'activo'
               AND mostrar_en_comunidad = TRUE
               AND eliminado_en IS NULL
             ORDER BY publicado_at DESC NULLS LAST, id DESC
             LIMIT $1",
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;
        i64::try_from(rows.len())
            .map_err(|_| AppError::Internal("Benchmark feed fuera de rango".into()))
    }

    pub async fn benchmark_similar(pool: &PgPool, limit: i64) -> Result<i64, AppError> {
        let Some(reference_id) = sqlx::query_scalar::<_, i32>(
            "SELECT id
             FROM samples
             WHERE embedding IS NOT NULL
               AND estado = 'activo'
               AND eliminado_en IS NULL
             ORDER BY id
             LIMIT 1",
        )
        .fetch_optional(pool)
        .await?
        else {
            return Ok(0);
        };

        let rows = sqlx::query_scalar::<_, i32>(
            "SELECT s.id
             FROM samples s
             WHERE s.id <> $1
               AND s.embedding IS NOT NULL
               AND s.estado = 'activo'
               AND s.eliminado_en IS NULL
             ORDER BY s.embedding <=> (SELECT embedding FROM samples WHERE id = $1)
             LIMIT $2",
        )
        .bind(reference_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;
        i64::try_from(rows.len())
            .map_err(|_| AppError::Internal("Benchmark similares fuera de rango".into()))
    }
}
