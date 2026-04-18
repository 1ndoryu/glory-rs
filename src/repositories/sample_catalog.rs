use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

use super::sample::SampleRepository;

/* [174A-44] Listado público de samples con filtros combinables.
 * Se mantiene en un módulo separado para no seguir inflando sample.rs, que ya
 * concentra upload, deduplicación y pipeline. El query queda encapsulado acá
 * porque comparte joins, filtros base y ordenamiento estable. */

#[derive(Debug, Clone)]
pub struct SampleListFilters {
    pub page: i64,
    pub per_page: i64,
    pub search: Option<SampleTextSearch>,
    pub bpm: Option<i32>,
    pub music_key: Option<String>,
    pub sample_type: Option<String>,
    pub tags: Vec<String>,
    pub premium: Option<bool>,
    pub creator: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SampleTextSearch {
    pub query: String,
    pub title_like: String,
    pub title_prefix: String,
    pub lower_query: String,
    pub lower_like: String,
    pub normalized_like: Option<String>,
}

impl SampleTextSearch {
    pub fn new(query: String, normalized: Option<String>) -> Self {
        let lower_query = query.to_ascii_lowercase();

        Self {
            title_like: format!("%{query}%"),
            title_prefix: format!("{query}%"),
            lower_like: format!("%{lower_query}%"),
            normalized_like: normalized.map(|value| format!("%{value}%")),
            lower_query,
            query,
        }
    }
}

impl SampleListFilters {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }
}

#[derive(Debug, Clone)]
pub struct SampleListResult {
    pub items: Vec<SampleCatalogSummaryRecord>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct SampleCatalogSummaryRecord {
    pub id: i32,
    pub id_corto: Option<String>,
    pub slug: String,
    pub titulo: String,
    pub descripcion: String,
    pub bpm: Option<i32>,
    pub music_key: Option<String>,
    pub escala: Option<String>,
    pub duracion: f32,
    pub formato: String,
    pub tags: Vec<String>,
    pub tipo: String,
    pub es_premium: bool,
    pub precio: Option<f64>,
    pub verificado: bool,
    pub ruta_preview: Option<String>,
    pub ruta_waveform: Option<String>,
    pub imagen_url: Option<String>,
    pub total_descargas: i32,
    pub total_likes: i32,
    pub total_reproducciones: i32,
    pub total_comentarios: i32,
    pub publicado_at: Option<DateTime<Utc>>,
    pub creator_id: i32,
    pub creator_username: String,
    pub creator_nombre_visible: Option<String>,
    pub creator_avatar_url: Option<String>,
    pub creator_verificado: bool,
}

#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct SampleCatalogDetailRecord {
    pub id: i32,
    pub id_corto: Option<String>,
    pub slug: String,
    pub titulo: String,
    pub descripcion: String,
    pub bpm: Option<i32>,
    pub music_key: Option<String>,
    pub escala: Option<String>,
    pub duracion: f32,
    pub formato: String,
    pub tamano: i64,
    pub tags: Vec<String>,
    pub tipo: String,
    pub estado: String,
    pub es_premium: bool,
    pub precio: Option<f64>,
    pub metadata: serde_json::Value,
    pub ruta_preview: Option<String>,
    pub ruta_waveform: Option<String>,
    pub ruta_original: Option<String>,
    pub ruta_optimizada: Option<String>,
    pub permitir_descarga: bool,
    pub licencia_libre: bool,
    pub imagen_url: Option<String>,
    pub total_descargas: i32,
    pub total_likes: i32,
    pub total_reproducciones: i32,
    pub total_comentarios: i32,
    pub audio_hash: Option<String>,
    pub verificado: bool,
    pub mostrar_en_comunidad: bool,
    pub publicado_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub cancion_origen_id: Option<i32>,
    pub relacion_sampleo_id: Option<i32>,
    pub creator_id: i32,
    pub creator_username: String,
    pub creator_nombre_visible: Option<String>,
    pub creator_avatar_url: Option<String>,
    pub creator_verificado: bool,
}

#[derive(Debug, Clone)]
pub struct OwnedSampleRecord {
    pub id: i32,
    pub id_corto: Option<String>,
    pub slug: String,
    pub creator_id: i32,
}

#[derive(Debug, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct UpdateSamplePatch {
    pub titulo: Option<String>,
    pub descripcion: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sample_type: Option<String>,
    pub es_premium: Option<bool>,
    pub precio: Option<Option<f64>>,
    pub permitir_descarga: Option<bool>,
    pub licencia_libre: Option<bool>,
    pub mostrar_en_comunidad: Option<bool>,
    pub imagen_url: Option<Option<String>>,
}

impl UpdateSamplePatch {
    pub fn has_changes(&self) -> bool {
        self.titulo.is_some()
            || self.descripcion.is_some()
            || self.tags.is_some()
            || self.sample_type.is_some()
            || self.es_premium.is_some()
            || self.precio.is_some()
            || self.permitir_descarga.is_some()
            || self.licencia_libre.is_some()
            || self.mostrar_en_comunidad.is_some()
            || self.imagen_url.is_some()
    }
}

#[derive(Debug, FromRow)]
struct CountRow {
    total: i64,
}

#[derive(Debug, FromRow)]
struct SampleSummaryRow {
    id: i32,
    id_corto: Option<String>,
    slug: String,
    titulo: String,
    descripcion: String,
    bpm: Option<i32>,
    music_key: Option<String>,
    escala: Option<String>,
    duracion: f32,
    formato: String,
    tags: Vec<String>,
    tipo: String,
    es_premium: bool,
    precio: Option<f64>,
    verificado: bool,
    ruta_preview: Option<String>,
    ruta_waveform: Option<String>,
    imagen_url: Option<String>,
    total_descargas: i32,
    total_likes: i32,
    total_reproducciones: i32,
    total_comentarios: i32,
    publicado_at: Option<DateTime<Utc>>,
    creator_id: i32,
    creator_username: String,
    creator_nombre_visible: Option<String>,
    creator_avatar_url: Option<String>,
    creator_verificado: bool,
}

impl From<SampleSummaryRow> for SampleCatalogSummaryRecord {
    fn from(row: SampleSummaryRow) -> Self {
        Self {
            id: row.id,
            id_corto: row.id_corto,
            slug: row.slug,
            titulo: row.titulo,
            descripcion: row.descripcion,
            bpm: row.bpm,
            music_key: row.music_key,
            escala: row.escala,
            duracion: row.duracion,
            formato: row.formato,
            tags: row.tags,
            tipo: row.tipo,
            es_premium: row.es_premium,
            precio: row.precio,
            verificado: row.verificado,
            ruta_preview: row.ruta_preview,
            ruta_waveform: row.ruta_waveform,
            imagen_url: row.imagen_url,
            total_descargas: row.total_descargas,
            total_likes: row.total_likes,
            total_reproducciones: row.total_reproducciones,
            total_comentarios: row.total_comentarios,
            publicado_at: row.publicado_at,
            creator_id: row.creator_id,
            creator_username: row.creator_username,
            creator_nombre_visible: row.creator_nombre_visible,
            creator_avatar_url: row.creator_avatar_url,
            creator_verificado: row.creator_verificado,
        }
    }
}

impl SampleRepository {
    pub async fn list_public_samples(
        pool: &PgPool,
        filters: &SampleListFilters,
    ) -> Result<SampleListResult, sqlx::Error> {
        let total = Self::count_public_samples(pool, filters).await?;
        if total == 0 {
            return Ok(SampleListResult {
                items: Vec::new(),
                total,
            });
        }

        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT
                s.id,
                s.id_corto,
                s.slug,
                s.titulo,
                s.descripcion,
                s.bpm,
                s.key AS music_key,
                s.escala,
                s.duracion,
                s.formato,
                COALESCE(s.tags, ARRAY[]::text[]) AS tags,
                s.tipo,
                s.es_premium,
                CAST(s.precio AS double precision) AS precio,
                s.verificado,
                s.ruta_preview,
                s.ruta_waveform,
                s.imagen_url,
                COALESCE(s.total_descargas, 0) AS total_descargas,
                COALESCE(s.total_likes, 0) AS total_likes,
                COALESCE(s.total_reproducciones, 0) AS total_reproducciones,
                COALESCE(s.total_comentarios, 0) AS total_comentarios,
                s.publicado_at,
                u.id AS creator_id,
                u.username AS creator_username,
                u.nombre_visible AS creator_nombre_visible,
                u.avatar_url AS creator_avatar_url,
                COALESCE(u.verificado, FALSE) AS creator_verificado",
        );

        push_public_filters(&mut builder, filters);
        push_public_order(&mut builder, filters);
        builder.push(" LIMIT ");
        builder.push_bind(filters.per_page);
        builder.push(" OFFSET ");
        builder.push_bind(filters.offset());

        let rows = builder
            .build_query_as::<SampleSummaryRow>()
            .fetch_all(pool)
            .await?;

        Ok(SampleListResult {
            items: rows.into_iter().map(SampleCatalogSummaryRecord::from).collect(),
            total,
        })
    }

    pub async fn find_sample_by_slug_or_short_id(
        pool: &PgPool,
        slug_or_short_id: &str,
    ) -> Result<Option<SampleCatalogDetailRecord>, sqlx::Error> {
        sqlx::query_as!(
            SampleCatalogDetailRecord,
            "SELECT
                s.id,
                s.id_corto,
                s.slug AS \"slug!\",
                s.titulo AS \"titulo!\",
                s.descripcion AS \"descripcion!\",
                s.bpm,
                s.key AS \"music_key?\",
                s.escala,
                s.duracion,
                s.formato AS \"formato!\",
                s.tamano,
                COALESCE(s.tags, ARRAY[]::text[]) AS \"tags!: Vec<String>\",
                s.tipo AS \"tipo!\",
                s.estado AS \"estado!\",
                COALESCE(s.es_premium, FALSE) AS \"es_premium!\",
                CAST(s.precio AS double precision) AS \"precio?\",
                COALESCE(s.metadata, '{}'::jsonb) AS \"metadata!: serde_json::Value\",
                s.ruta_preview,
                s.ruta_waveform,
                s.ruta_original,
                s.ruta_optimizada,
                COALESCE(s.permitir_descarga, TRUE) AS \"permitir_descarga!\",
                COALESCE(s.licencia_libre, FALSE) AS \"licencia_libre!\",
                s.imagen_url,
                COALESCE(s.total_descargas, 0) AS \"total_descargas!\",
                COALESCE(s.total_likes, 0) AS \"total_likes!\",
                COALESCE(s.total_reproducciones, 0) AS \"total_reproducciones!\",
                COALESCE(s.total_comentarios, 0) AS \"total_comentarios!\",
                s.audio_hash,
                COALESCE(s.verificado, FALSE) AS \"verificado!\",
                COALESCE(s.mostrar_en_comunidad, TRUE) AS \"mostrar_en_comunidad!\",
                s.publicado_at,
                s.created_at,
                s.cancion_origen_id,
                s.relacion_sampleo_id,
                u.id AS \"creator_id!\",
                u.username AS \"creator_username!\",
                u.nombre_visible AS \"creator_nombre_visible?\",
                u.avatar_url AS \"creator_avatar_url?\",
                COALESCE(u.verificado, FALSE) AS \"creator_verificado!\"
             FROM samples s
             INNER JOIN usuarios_ext u ON u.id = s.creador_id
             WHERE s.eliminado_en IS NULL
               AND (LOWER(s.slug) = LOWER($1) OR s.id_corto = $1)
             ORDER BY CASE WHEN LOWER(s.slug) = LOWER($1) THEN 0 ELSE 1 END, s.id DESC
             LIMIT 1",
            slug_or_short_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_random_public_sample(
        pool: &PgPool,
    ) -> Result<Option<SampleCatalogDetailRecord>, sqlx::Error> {
        sqlx::query_as!(
            SampleCatalogDetailRecord,
            "SELECT
                s.id,
                s.id_corto,
                s.slug AS \"slug!\",
                s.titulo AS \"titulo!\",
                s.descripcion AS \"descripcion!\",
                s.bpm,
                s.key AS \"music_key?\",
                s.escala,
                s.duracion,
                s.formato AS \"formato!\",
                s.tamano,
                COALESCE(s.tags, ARRAY[]::text[]) AS \"tags!: Vec<String>\",
                s.tipo AS \"tipo!\",
                s.estado AS \"estado!\",
                COALESCE(s.es_premium, FALSE) AS \"es_premium!\",
                CAST(s.precio AS double precision) AS \"precio?\",
                COALESCE(s.metadata, '{}'::jsonb) AS \"metadata!: serde_json::Value\",
                s.ruta_preview,
                s.ruta_waveform,
                s.ruta_original,
                s.ruta_optimizada,
                COALESCE(s.permitir_descarga, TRUE) AS \"permitir_descarga!\",
                COALESCE(s.licencia_libre, FALSE) AS \"licencia_libre!\",
                s.imagen_url,
                COALESCE(s.total_descargas, 0) AS \"total_descargas!\",
                COALESCE(s.total_likes, 0) AS \"total_likes!\",
                COALESCE(s.total_reproducciones, 0) AS \"total_reproducciones!\",
                COALESCE(s.total_comentarios, 0) AS \"total_comentarios!\",
                s.audio_hash,
                COALESCE(s.verificado, FALSE) AS \"verificado!\",
                COALESCE(s.mostrar_en_comunidad, TRUE) AS \"mostrar_en_comunidad!\",
                s.publicado_at,
                s.created_at,
                s.cancion_origen_id,
                s.relacion_sampleo_id,
                u.id AS \"creator_id!\",
                u.username AS \"creator_username!\",
                u.nombre_visible AS \"creator_nombre_visible?\",
                u.avatar_url AS \"creator_avatar_url?\",
                COALESCE(u.verificado, FALSE) AS \"creator_verificado!\"
             FROM samples s
             INNER JOIN usuarios_ext u ON u.id = s.creador_id
             WHERE s.id IN (
                 SELECT id
                 FROM samples
                 WHERE eliminado_en IS NULL
                   AND estado = 'activo'
                   AND mostrar_en_comunidad = TRUE
                 ORDER BY publicado_at DESC NULLS LAST, created_at DESC, id DESC
                 LIMIT 1000
             )
             ORDER BY RANDOM()
             LIMIT 1"
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_owned_sample_by_slug_or_short_id(
        pool: &PgPool,
        slug_or_short_id: &str,
    ) -> Result<Option<OwnedSampleRecord>, sqlx::Error> {
        sqlx::query_as!(
            OwnedSampleRecord,
            "SELECT
                s.id,
                s.id_corto,
                s.slug AS \"slug!\",
                s.creador_id AS \"creator_id!\"
             FROM samples s
             WHERE s.eliminado_en IS NULL
               AND (LOWER(s.slug) = LOWER($1) OR s.id_corto = $1)
             ORDER BY CASE WHEN LOWER(s.slug) = LOWER($1) THEN 0 ELSE 1 END, s.id DESC
             LIMIT 1",
            slug_or_short_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn update_sample_metadata(
        pool: &PgPool,
        sample_id: i32,
        patch: &UpdateSamplePatch,
    ) -> Result<(), sqlx::Error> {
        let mut builder = QueryBuilder::<Postgres>::new("UPDATE samples SET ");
        {
            let mut separated = builder.separated(", ");

            if let Some(titulo) = &patch.titulo {
                separated.push("titulo = ");
                separated.push_bind(titulo.clone());
            }

            if let Some(descripcion) = &patch.descripcion {
                separated.push("descripcion = ");
                separated.push_bind(descripcion.clone());
            }

            if let Some(tags) = &patch.tags {
                separated.push("tags = ");
                separated.push_bind(tags.clone());
                separated.push("::text[]");
            }

            if let Some(sample_type) = &patch.sample_type {
                separated.push("tipo = ");
                separated.push_bind(sample_type.clone());
            }

            if let Some(es_premium) = patch.es_premium {
                separated.push("es_premium = ");
                separated.push_bind(es_premium);
            }

            if let Some(precio) = &patch.precio {
                match precio {
                    Some(value) => {
                        separated.push("precio = ");
                        separated.push_bind(*value);
                    }
                    None => {
                        separated.push("precio = NULL");
                    }
                }
            }

            if let Some(permitir_descarga) = patch.permitir_descarga {
                separated.push("permitir_descarga = ");
                separated.push_bind(permitir_descarga);
            }

            if let Some(licencia_libre) = patch.licencia_libre {
                separated.push("licencia_libre = ");
                separated.push_bind(licencia_libre);
            }

            if let Some(mostrar_en_comunidad) = patch.mostrar_en_comunidad {
                separated.push("mostrar_en_comunidad = ");
                separated.push_bind(mostrar_en_comunidad);
            }

            if let Some(imagen_url) = &patch.imagen_url {
                match imagen_url {
                    Some(value) => {
                        separated.push("imagen_url = ");
                        separated.push_bind(value.clone());
                    }
                    None => {
                        separated.push("imagen_url = NULL");
                    }
                }
            }

            separated.push("updated_at = NOW()");
        }

        builder.push(" WHERE id = ");
        builder.push_bind(sample_id);

        builder.build().execute(pool).await?;
        Ok(())
    }

    pub async fn soft_delete_owned_sample(
        pool: &PgPool,
        sample_id: i32,
        owner_id: i32,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE samples
             SET estado = 'eliminado', eliminado_en = NOW(), updated_at = NOW()
             WHERE id = $1 AND creador_id = $2 AND eliminado_en IS NULL",
            sample_id,
            owner_id,
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn count_public_samples(
        pool: &PgPool,
        filters: &SampleListFilters,
    ) -> Result<i64, sqlx::Error> {
        let mut builder = QueryBuilder::<Postgres>::new("SELECT COUNT(*) AS total");
        push_public_filters(&mut builder, filters);

        let row = builder.build_query_as::<CountRow>().fetch_one(pool).await?;
        Ok(row.total)
    }
}

fn push_public_filters(builder: &mut QueryBuilder<'_, Postgres>, filters: &SampleListFilters) {
    builder.push(
        " FROM samples s
          INNER JOIN usuarios_ext u ON u.id = s.creador_id
          WHERE s.eliminado_en IS NULL
            AND s.estado = 'activo'
            AND s.mostrar_en_comunidad = TRUE",
    );

    if let Some(bpm) = filters.bpm {
        builder.push(" AND s.bpm = ");
        builder.push_bind(bpm);
    }

    if let Some(music_key) = &filters.music_key {
        builder.push(" AND s.key = ");
        builder.push_bind(music_key.clone());
    }

    if let Some(sample_type) = &filters.sample_type {
        builder.push(" AND s.tipo = ");
        builder.push_bind(sample_type.clone());
    }

    if !filters.tags.is_empty() {
        builder.push(" AND s.tags_enriquecidos && ");
        builder.push_bind(filters.tags.clone());
        builder.push("::text[]");
    }

    if let Some(premium) = filters.premium {
        builder.push(" AND s.es_premium = ");
        builder.push_bind(premium);
    }

    if let Some(creator) = &filters.creator {
        builder.push(" AND LOWER(u.username) = LOWER(");
        builder.push_bind(creator.clone());
        builder.push(")");
    }

    if let Some(search) = &filters.search {
        push_public_search_filters(builder, search);
    }
}

fn push_public_search_filters(
    builder: &mut QueryBuilder<'_, Postgres>,
    search: &SampleTextSearch,
) {
    builder.push(
        " AND (
            to_tsvector('spanish', COALESCE(s.titulo, '') || ' ' || COALESCE(s.descripcion, ''))
                @@ plainto_tsquery('spanish', ",
    );
    builder.push_bind(search.query.clone());
    builder.push(")");

    builder.push(" OR s.titulo ILIKE ");
    builder.push_bind(search.title_like.clone());

    builder.push(
        " OR EXISTS (
            SELECT 1
            FROM UNNEST(COALESCE(s.tags, ARRAY[]::text[])) tag
            WHERE tag ILIKE ",
    );
    builder.push_bind(search.lower_like.clone());
    builder.push(")");

    builder.push(
        " OR EXISTS (
            SELECT 1
            FROM UNNEST(COALESCE(s.tags_enriquecidos, ARRAY[]::text[])) tag
            WHERE tag ILIKE ",
    );
    builder.push_bind(search.lower_like.clone());
    builder.push(")");

    builder.push(" OR word_similarity(");
    builder.push_bind(search.query.clone());
    builder.push(", s.titulo) > 0.3");

    builder.push(
        " OR EXISTS (
            SELECT 1
            FROM UNNEST(COALESCE(s.tags, ARRAY[]::text[])) tag
            WHERE similarity(tag, ",
    );
    builder.push_bind(search.lower_query.clone());
    builder.push(") > 0.4)");

    builder.push(
        " OR EXISTS (
            SELECT 1
            FROM UNNEST(COALESCE(s.tags_enriquecidos, ARRAY[]::text[])) tag
            WHERE similarity(tag, ",
    );
    builder.push_bind(search.lower_query.clone());
    builder.push(") > 0.4)");

    if let Some(normalized_like) = &search.normalized_like {
        builder.push(
            " OR EXISTS (
                SELECT 1
                FROM UNNEST(COALESCE(s.tags, ARRAY[]::text[])) tag
                WHERE tag ILIKE ",
        );
        builder.push_bind(normalized_like.clone());
        builder.push(")");

        builder.push(
            " OR EXISTS (
                SELECT 1
                FROM UNNEST(COALESCE(s.tags_enriquecidos, ARRAY[]::text[])) tag
                WHERE tag ILIKE ",
        );
        builder.push_bind(normalized_like.clone());
        builder.push(")");

        builder.push(" OR s.titulo ILIKE ");
        builder.push_bind(normalized_like.clone());
    }

    builder.push(")");
}

fn push_public_order(builder: &mut QueryBuilder<'_, Postgres>, filters: &SampleListFilters) {
    if let Some(search) = &filters.search {
        builder.push(
            " ORDER BY (
                1.0 * ts_rank(
                    to_tsvector('spanish', COALESCE(s.titulo, '') || ' ' || COALESCE(s.descripcion, '')),
                    plainto_tsquery('spanish', ",
        );
        builder.push_bind(search.query.clone());
        builder.push(")
                )
                + 0.8 * CASE WHEN EXISTS (
                    SELECT 1
                    FROM UNNEST(COALESCE(s.tags, ARRAY[]::text[])) tag
                    WHERE tag ILIKE ");
        builder.push_bind(search.lower_like.clone());
        builder.push(
            "
                ) OR EXISTS (
                    SELECT 1
                    FROM UNNEST(COALESCE(s.tags_enriquecidos, ARRAY[]::text[])) tag
                    WHERE tag ILIKE ",
        );
        builder.push_bind(search.lower_like.clone());
        builder.push(
            "
                ) THEN 1.0 ELSE 0.0 END
                + 1.5 * ts_rank(
                    to_tsvector('spanish', COALESCE(s.titulo, '')),
                    plainto_tsquery('spanish', ",
        );
        builder.push_bind(search.query.clone());
        builder.push(")
                )
                + 0.6 * word_similarity(");
        builder.push_bind(search.query.clone());
        builder.push(", s.titulo)
                + 2.0 * CASE WHEN s.titulo ILIKE ");
        builder.push_bind(search.title_prefix.clone());
        builder.push(
            " THEN 1.0 ELSE 0.0 END
            ) DESC,
            s.publicado_at DESC NULLS LAST,
            s.created_at DESC,
            s.id DESC",
        );
    } else {
        builder.push(" ORDER BY s.publicado_at DESC NULLS LAST, s.created_at DESC, s.id DESC");
    }
}
