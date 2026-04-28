use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, Postgres, QueryBuilder};

use super::{SampleCatalogSummaryRecord, SampleListFilters, SampleTextSearch};
use crate::repositories::AUTO_HIDE_SAMPLE_REPORT_THRESHOLD;

pub(super) const SAMPLE_SUMMARY_SELECT: &str = "SELECT
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
        COALESCE(NULLIF(s.ruta_preview, ''), s.ruta_optimizada) AS ruta_preview,
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
        COALESCE(u.verificado, FALSE) AS creator_verificado,
        COALESCE(s.metadata, '{}'::jsonb) AS metadata";

#[derive(Debug, FromRow)]
pub(super) struct CountRow {
    pub(super) total: i64,
}

#[derive(Debug, FromRow)]
pub(super) struct SampleSummaryRow {
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
    metadata: Value,
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
            metadata: row.metadata,
        }
    }
}

pub(super) fn push_public_filters(
    builder: &mut QueryBuilder<'_, Postgres>,
    filters: &SampleListFilters,
) {
    builder.push(
        " FROM samples s
          INNER JOIN usuarios_ext u ON u.id = s.creador_id
          WHERE s.eliminado_en IS NULL
            AND s.estado = 'activo'
            AND s.mostrar_en_comunidad = TRUE",
    );
    push_auto_hide_filter(builder, "s.id", "s.creador_id", filters.viewer_id);

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

pub(super) fn push_auto_hide_filter(
    builder: &mut QueryBuilder<'_, Postgres>,
    target_expr: &str,
    creator_expr: &str,
    viewer_id: Option<i32>,
) {
    builder.push(" AND ((SELECT COUNT(*) FROM reportes r WHERE r.tipo = 'sample' AND COALESCE(r.estado, 'pendiente') = 'pendiente' AND r.target_id = ");
    builder.push(target_expr);
    builder.push(") < ");
    builder.push(AUTO_HIDE_SAMPLE_REPORT_THRESHOLD.to_string());
    if let Some(viewer_id) = viewer_id {
        builder.push(" OR ");
        builder.push(creator_expr);
        builder.push(" = ");
        builder.push_bind(viewer_id);
    }
    builder.push(")");
}

fn push_public_search_filters(builder: &mut QueryBuilder<'_, Postgres>, search: &SampleTextSearch) {
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

pub(super) fn push_public_order(
    builder: &mut QueryBuilder<'_, Postgres>,
    filters: &SampleListFilters,
) {
    if let Some(search) = &filters.search {
        builder.push(
            " ORDER BY (
                1.0 * ts_rank(
                    to_tsvector('spanish', COALESCE(s.titulo, '') || ' ' || COALESCE(s.descripcion, '')),
                    plainto_tsquery('spanish', ",
        );
        builder.push_bind(search.query.clone());
        builder.push(
            ")
                )
                + 0.8 * CASE WHEN EXISTS (
                    SELECT 1
                    FROM UNNEST(COALESCE(s.tags, ARRAY[]::text[])) tag
                    WHERE tag ILIKE ",
        );
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
        builder.push(
            ")
                )
                + 0.6 * word_similarity(",
        );
        builder.push_bind(search.query.clone());
        builder.push(
            ", s.titulo)
                + 2.0 * CASE WHEN s.titulo ILIKE ",
        );
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
