use sqlx::PgPool;

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
}