use sqlx::PgPool;
use pgvector::Vector;

/* [174A-28] Repositorio mínimo para deduplicación previa de uploads.
 * Solo expone lo necesario para el precheck; el CRUD completo de samples llega
 * en la Fase 6. */

pub struct SampleRepository;

#[derive(Debug, Clone)]
pub struct DuplicateSampleCandidate {
    pub id: i32,
    pub creador_id: i32,
    pub titulo: String,
}

#[derive(Debug, Clone)]
pub struct CreatedUploadSample {
    pub id: i32,
    pub id_corto: String,
    pub slug: String,
    pub estado: String,
    pub ruta_original: String,
}

#[derive(Debug, Clone)]
pub struct AudioPipelineSample {
    pub id: i32,
    pub id_corto: String,
    pub formato: String,
    pub tags: Vec<String>,
    pub tipo: String,
    pub es_premium: bool,
    pub metadata: serde_json::Value,
    pub estado: String,
    pub ruta_original: String,
}

pub struct SaveAudioAnalysisParams {
    pub sample_id: i32,
    pub duration_seconds: f32,
    pub formato: String,
    pub tamano: i64,
    pub bpm: Option<i32>,
    pub music_key: Option<String>,
    pub scale: Option<String>,
    pub metadata: serde_json::Value,
}

pub struct SaveAudioAssetsParams {
    pub sample_id: i32,
    pub ruta_waveform: Option<String>,
    pub ruta_optimizada: Option<String>,
    pub metadata: serde_json::Value,
}

pub struct CompleteAudioPipelineParams {
    pub sample_id: i32,
    pub embedding: Vector,
    pub metadata: serde_json::Value,
}

pub struct MarkAudioPipelineFailedParams {
    pub sample_id: i32,
    pub metadata: serde_json::Value,
}

#[allow(clippy::struct_excessive_bools)]
pub struct CreateUploadSampleParams<'a> {
    pub creador_id: i32,
    pub titulo: &'a str,
    pub slug: &'a str,
    pub id_corto: &'a str,
    pub descripcion: &'a str,
    pub formato: &'a str,
    pub tamano: i64,
    pub tags: &'a [String],
    pub audio_hash: &'a str,
    pub ruta_original: &'a str,
    pub permitir_descarga: bool,
    pub licencia_libre: bool,
    pub es_premium: bool,
    pub precio: Option<f64>,
    pub mostrar_en_comunidad: bool,
    pub metadata: serde_json::Value,
    pub sync_upload: bool,
}

impl SampleRepository {
    pub async fn find_pipeline_sample(
        pool: &PgPool,
        sample_id: i32,
    ) -> Result<Option<AudioPipelineSample>, sqlx::Error> {
        sqlx::query_as!(
            AudioPipelineSample,
            "SELECT
                id,
                id_corto as \"id_corto!\",
                formato as \"formato!\",
                COALESCE(tags, ARRAY[]::text[]) as \"tags!: Vec<String>\",
                tipo as \"tipo!\",
                es_premium as \"es_premium!\",
                COALESCE(metadata, '{}'::jsonb) as \"metadata!: serde_json::Value\",
                estado as \"estado!\",
                ruta_original as \"ruta_original!\"
             FROM samples
             WHERE id = $1
               AND eliminado_en IS NULL
             LIMIT 1",
            sample_id
        )
        .fetch_optional(pool)
        .await
    }

    /// Busca el primer sample visible (activo o en supervisión) con ese hash exacto.
    pub async fn find_duplicate_by_audio_hash(
        pool: &PgPool,
        audio_hash: &str,
    ) -> Result<Option<DuplicateSampleCandidate>, sqlx::Error> {
                sqlx::query_as!(
                        DuplicateSampleCandidate,
                        "SELECT id, creador_id, titulo
                         FROM samples
                         WHERE audio_hash = $1
                             AND eliminado_en IS NULL
                             AND estado IN ('activo', 'en_supervision')
                         ORDER BY CASE estado WHEN 'activo' THEN 0 ELSE 1 END,
                                            publicado_at DESC NULLS LAST,
                                            created_at DESC
                         LIMIT 1",
                        audio_hash
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create_upload_sample(
        pool: &PgPool,
        params: CreateUploadSampleParams<'_>,
    ) -> Result<CreatedUploadSample, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let created = sqlx::query_as!(
            CreatedUploadSample,
            "INSERT INTO samples (
                creador_id, titulo, slug, id_corto, descripcion, formato, tamano,
                tags, audio_hash, estado, ruta_original, permitir_descarga,
                licencia_libre, es_premium, precio, mostrar_en_comunidad, metadata
             )
             VALUES (
                $1, $2, $3, $4, $5, $6, $7,
                $8, $9, 'procesando', $10, $11,
                     $12, $13, ROUND(CAST($14 AS double precision)::numeric, 2), $15, $16
             )
             RETURNING id, id_corto as \"id_corto!\", slug, estado, ruta_original as \"ruta_original!\"",
            params.creador_id,
            params.titulo,
            params.slug,
            params.id_corto,
            params.descripcion,
            params.formato,
            params.tamano,
            params.tags as _,
            params.audio_hash,
            params.ruta_original,
            params.permitir_descarga,
            params.licencia_libre,
            params.es_premium,
            params.precio,
            params.mostrar_en_comunidad,
            params.metadata,
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            "UPDATE usuarios_ext
             SET total_samples = COALESCE(total_samples, 0) + 1
             WHERE id = $1",
            params.creador_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "INSERT INTO cola_procesamiento_ia (tipo, entidad_id, operacion, metadata)
             VALUES ('sample', $1, 'analisis_audio', $2)",
            created.id,
            serde_json::json!({
                "sync_upload": params.sync_upload,
                "audio_hash": params.audio_hash,
                "ruta_original": params.ruta_original,
            })
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn save_audio_analysis(
        pool: &PgPool,
        params: SaveAudioAnalysisParams,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE samples
             SET duracion = $2,
                 formato = $3,
                 tamano = $4,
                 bpm = $5,
                 key = $6,
                 escala = $7,
                 metadata = COALESCE(metadata, '{}'::jsonb) || $8
             WHERE id = $1
               AND eliminado_en IS NULL",
            params.sample_id,
            params.duration_seconds,
            params.formato,
            params.tamano,
            params.bpm,
            params.music_key,
            params.scale,
            params.metadata,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn save_audio_assets(
        pool: &PgPool,
        params: SaveAudioAssetsParams,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE samples
             SET ruta_waveform = COALESCE($2, ruta_waveform),
                 ruta_optimizada = COALESCE($3, ruta_optimizada),
                 metadata = COALESCE(metadata, '{}'::jsonb) || $4
             WHERE id = $1
               AND eliminado_en IS NULL",
            params.sample_id,
            params.ruta_waveform,
            params.ruta_optimizada,
            params.metadata,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn complete_audio_pipeline(
        pool: &PgPool,
        params: CompleteAudioPipelineParams,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE samples
             SET embedding = $2,
                 estado = 'activo',
                 publicado_at = COALESCE(publicado_at, NOW()),
                 metadata = COALESCE(metadata, '{}'::jsonb) || $3
             WHERE id = $1
               AND eliminado_en IS NULL",
            params.sample_id,
            params.embedding as _,
            params.metadata,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn mark_audio_pipeline_failed(
        pool: &PgPool,
        params: MarkAudioPipelineFailedParams,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE samples
             SET metadata = COALESCE(metadata, '{}'::jsonb) || $2
             WHERE id = $1
               AND eliminado_en IS NULL",
            params.sample_id,
            params.metadata,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}