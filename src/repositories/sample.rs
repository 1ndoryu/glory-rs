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
}