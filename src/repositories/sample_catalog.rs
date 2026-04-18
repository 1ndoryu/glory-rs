use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

use crate::models::{SampleCreatorSummary, SampleSummary};

use super::sample::SampleRepository;

/* [174A-44] Listado público de samples con filtros combinables.
 * Se mantiene en un módulo separado para no seguir inflando sample.rs, que ya
 * concentra upload, deduplicación y pipeline. El query queda encapsulado acá
 * porque comparte joins, filtros base y ordenamiento estable. */

#[derive(Debug, Clone)]
pub struct SampleListFilters {
    pub page: i64,
    pub per_page: i64,
    pub bpm: Option<i32>,
    pub music_key: Option<String>,
    pub sample_type: Option<String>,
    pub tags: Vec<String>,
    pub premium: Option<bool>,
    pub creator: Option<String>,
}

impl SampleListFilters {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }
}

#[derive(Debug, Clone)]
pub struct SampleListResult {
    pub items: Vec<SampleSummary>,
    pub total: i64,
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

impl From<SampleSummaryRow> for SampleSummary {
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
            creador: SampleCreatorSummary {
                id: row.creator_id,
                username: row.creator_username,
                nombre_visible: row.creator_nombre_visible,
                avatar_url: row.creator_avatar_url,
                verificado: row.creator_verificado,
            },
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
        builder
            .push(" ORDER BY s.publicado_at DESC NULLS LAST, s.created_at DESC, s.id DESC LIMIT ");
        builder.push_bind(filters.per_page);
        builder.push(" OFFSET ");
        builder.push_bind(filters.offset());

        let rows = builder
            .build_query_as::<SampleSummaryRow>()
            .fetch_all(pool)
            .await?;

        Ok(SampleListResult {
            items: rows.into_iter().map(SampleSummary::from).collect(),
            total,
        })
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
}
