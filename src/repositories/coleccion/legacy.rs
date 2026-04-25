/* sentinel-disable-file limite-lineas — proyecciones SQL legacy de colecciones agrupadas para preservar paridad PHP; dividir cuando se cierre la migración legacy. */
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

use crate::errors::AppError;

use super::ColeccionesRepository;

const COLLECTION_TAGS_SQL: &str = r"
COALESCE(
    (
        SELECT array_agg(DISTINCT tag_val)
        FROM coleccion_samples cs_tags
        JOIN samples s_tags ON s_tags.id = cs_tags.sample_id
        CROSS JOIN LATERAL jsonb_array_elements_text(
            COALESCE(
                CASE WHEN jsonb_typeof(s_tags.metadata->'tags') = 'array' THEN s_tags.metadata->'tags' END,
                CASE WHEN jsonb_typeof(s_tags.metadata->'tags_es') = 'array' THEN s_tags.metadata->'tags_es' END,
                '[]'::jsonb
            )
        ) AS tag_val
        WHERE cs_tags.coleccion_id = c.id
          AND s_tags.estado = 'activo'
          AND s_tags.eliminado_en IS NULL
    ),
    ARRAY[]::text[]
)
";

const COLLECTION_TOTAL_ITEMS_SQL: &str = r"
(
    SELECT COUNT(*)
    FROM coleccion_samples cs_count
    JOIN samples s_count ON s_count.id = cs_count.sample_id
    WHERE s_count.estado = 'activo'
      AND s_count.eliminado_en IS NULL
      AND (
        cs_count.coleccion_id = c.id
        OR cs_count.coleccion_id IN (
            SELECT sub.id
            FROM colecciones sub
            WHERE sub.parent_id = c.id
              AND sub.eliminado_en IS NULL
        )
      )
)
";

#[derive(Debug, Clone, FromRow)]
#[allow(clippy::struct_excessive_bools)]
pub struct LegacyColeccionRecord {
    pub id: i64,
    pub usuario_id: i32,
    pub nombre: String,
    pub slug: Option<String>,
    pub descripcion: String,
    pub publica: bool,
    pub parent_id: Option<i64>,
    pub imagen_url: Option<String>,
    pub version: i32,
    pub total_samples: i32,
    pub total_items: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub username: Option<String>,
    pub nombre_visible: Option<String>,
    pub avatar_url: Option<String>,
    pub verificado: bool,
    pub esta_guardada: bool,
    pub esta_likeada: bool,
    pub total_likes: i32,
    pub contiene_el_sample: Option<bool>,
}

#[derive(Debug, Clone, FromRow)]
pub struct LegacyColeccionParentRecord {
    pub id: i64,
    pub nombre: String,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct LegacyColeccionSampleRecord {
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
    pub metadata: serde_json::Value,
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
    pub creator_id: i32,
    pub creator_username: String,
    pub creator_nombre_visible: Option<String>,
    pub creator_avatar_url: Option<String>,
    pub creator_verificado: bool,
}

impl ColeccionesRepository {
    pub async fn list_user_legacy(
        pool: &PgPool,
        usuario_id: i32,
        busqueda: Option<&str>,
    ) -> Result<Vec<LegacyColeccionRecord>, AppError> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT c.id, c.usuario_id, c.nombre, c.slug, COALESCE(c.descripcion, '') AS descripcion, \
                    c.publica, c.parent_id, c.imagen_url, c.version, c.total_samples, ",
        );
        push_collection_projection(&mut builder, None);
        builder.push(
            " FROM colecciones c \
               JOIN usuarios_ext u ON u.id = c.usuario_id \
               WHERE c.eliminado_en IS NULL \
                 AND c.usuario_id = ",
        );
        builder.push_bind(usuario_id);
        builder.push(" AND u.estado = 'activo'");
        if let Some(search) = normalize_search(busqueda) {
            push_search_filter(&mut builder, &search);
        }
        builder.push(" ORDER BY c.updated_at DESC, c.id DESC");

        let rows = builder
            .build_query_as::<LegacyColeccionRecord>()
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }

    pub async fn list_explore_legacy(
        pool: &PgPool,
        viewer_id: Option<i32>,
        busqueda: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LegacyColeccionRecord>, AppError> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT c.id, c.usuario_id, c.nombre, c.slug, COALESCE(c.descripcion, '') AS descripcion, \
                    c.publica, c.parent_id, c.imagen_url, c.version, c.total_samples, ",
        );
        push_collection_projection(&mut builder, viewer_id);
        builder.push(
            " FROM colecciones c \
               JOIN usuarios_ext u ON u.id = c.usuario_id \
               WHERE c.eliminado_en IS NULL \
                 AND u.estado = 'activo' \
                 AND (c.publica = TRUE",
        );
        if let Some(current_user_id) = viewer_id {
            builder.push(" OR c.usuario_id = ");
            builder.push_bind(current_user_id);
        }
        builder.push(")");
        if let Some(search) = normalize_search(busqueda) {
            push_search_filter(&mut builder, &search);
        }
        builder.push(" ORDER BY ");
        if let Some(current_user_id) = viewer_id {
            builder.push("CASE WHEN c.usuario_id = ");
            builder.push_bind(current_user_id);
            builder.push(" THEN 0 ELSE 1 END, ");
        }
        builder.push("c.updated_at DESC, c.id DESC LIMIT ");
        builder.push_bind(limit);
        builder.push(" OFFSET ");
        builder.push_bind(offset);

        let rows = builder
            .build_query_as::<LegacyColeccionRecord>()
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }

    pub async fn list_saved_legacy(
        pool: &PgPool,
        usuario_id: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LegacyColeccionRecord>, AppError> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT c.id, c.usuario_id, c.nombre, c.slug, COALESCE(c.descripcion, '') AS descripcion, \
                    c.publica, c.parent_id, c.imagen_url, c.version, c.total_samples, ",
        );
        push_collection_projection_common(&mut builder, Some(usuario_id));
        builder.push(
            " FROM colecciones_guardadas g \
               JOIN colecciones c ON c.id = g.coleccion_id \
               JOIN usuarios_ext u ON u.id = c.usuario_id \
               WHERE g.usuario_id = ",
        );
        builder.push_bind(usuario_id);
        builder.push(
            " AND c.eliminado_en IS NULL AND u.estado = 'activo' ORDER BY g.created_at DESC LIMIT ",
        );
        builder.push_bind(limit);
        builder.push(" OFFSET ");
        builder.push_bind(offset);

        let rows = builder
            .build_query_as::<LegacyColeccionRecord>()
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }

    pub async fn count_saved_legacy(pool: &PgPool, usuario_id: i32) -> Result<i64, AppError> {
        let total = sqlx::query_scalar::<_, i64>(
            r"SELECT COUNT(*)
               FROM colecciones_guardadas g
               JOIN colecciones c ON c.id = g.coleccion_id
               JOIN usuarios_ext u ON u.id = c.usuario_id
               WHERE g.usuario_id = $1
                 AND c.eliminado_en IS NULL
                                 AND u.estado = 'activo'",
        )
        .bind(usuario_id)
        .fetch_one(pool)
        .await?;
        Ok(total)
    }

    pub async fn tags_frecuentes_user_legacy(
        pool: &PgPool,
        usuario_id: i32,
        limit: i64,
    ) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query_scalar::<_, String>(
            r"SELECT tag_val
               FROM coleccion_samples cs
               JOIN colecciones c ON c.id = cs.coleccion_id
               JOIN samples s ON s.id = cs.sample_id
               CROSS JOIN LATERAL jsonb_array_elements_text(
                    COALESCE(
                        CASE WHEN jsonb_typeof(s.metadata->'tags') = 'array' THEN s.metadata->'tags' END,
                        CASE WHEN jsonb_typeof(s.metadata->'tags_es') = 'array' THEN s.metadata->'tags_es' END,
                        '[]'::jsonb
                    )
               ) AS tag_val
               WHERE c.usuario_id = $1
                 AND c.eliminado_en IS NULL
                 AND s.estado = 'activo'
                 AND s.eliminado_en IS NULL
               GROUP BY tag_val
               ORDER BY COUNT(*) DESC, tag_val ASC
                             LIMIT $2",
        )
        .bind(usuario_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn tags_frecuentes_explorar_legacy(
        pool: &PgPool,
        limit: i64,
    ) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query_scalar::<_, String>(
            r"SELECT tag_val
               FROM coleccion_samples cs
               JOIN colecciones c ON c.id = cs.coleccion_id
               JOIN usuarios_ext u ON u.id = c.usuario_id
               JOIN samples s ON s.id = cs.sample_id
               CROSS JOIN LATERAL jsonb_array_elements_text(
                    COALESCE(
                        CASE WHEN jsonb_typeof(s.metadata->'tags') = 'array' THEN s.metadata->'tags' END,
                        CASE WHEN jsonb_typeof(s.metadata->'tags_es') = 'array' THEN s.metadata->'tags_es' END,
                        '[]'::jsonb
                    )
               ) AS tag_val
               WHERE c.publica = TRUE
                 AND c.eliminado_en IS NULL
                 AND u.estado = 'activo'
                 AND s.estado = 'activo'
                 AND s.eliminado_en IS NULL
               GROUP BY tag_val
               ORDER BY COUNT(*) DESC, tag_val ASC
                             LIMIT $1",
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn fetch_legacy_by_id(
        pool: &PgPool,
        id: i64,
        viewer_id: Option<i32>,
    ) -> Result<Option<LegacyColeccionRecord>, AppError> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT c.id, c.usuario_id, c.nombre, c.slug, COALESCE(c.descripcion, '') AS descripcion, \
                    c.publica, c.parent_id, c.imagen_url, c.version, c.total_samples, ",
        );
        push_collection_projection(&mut builder, viewer_id);
        builder.push(
            " FROM colecciones c \
               JOIN usuarios_ext u ON u.id = c.usuario_id \
               WHERE c.id = ",
        );
        builder.push_bind(id);
        builder.push(" AND c.eliminado_en IS NULL AND u.estado = 'activo'");

        let row = builder
            .build_query_as::<LegacyColeccionRecord>()
            .fetch_optional(pool)
            .await?;
        Ok(row)
    }

    pub async fn fetch_legacy_by_slug(
        pool: &PgPool,
        slug: &str,
        viewer_id: Option<i32>,
    ) -> Result<Option<LegacyColeccionRecord>, AppError> {
        let maybe_id = parse_slug_collection_id(slug);
        if let Some(id) = maybe_id {
            if let Some(row) = Self::fetch_legacy_by_id(pool, id, viewer_id).await? {
                return Ok(Some(row));
            }
        }

        Self::fetch_legacy_by_slug_exact(pool, slug, viewer_id).await
    }

    pub async fn fetch_legacy_by_slug_exact(
        pool: &PgPool,
        slug: &str,
        viewer_id: Option<i32>,
    ) -> Result<Option<LegacyColeccionRecord>, AppError> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT c.id, c.usuario_id, c.nombre, c.slug, COALESCE(c.descripcion, '') AS descripcion, \
                    c.publica, c.parent_id, c.imagen_url, c.version, c.total_samples, ",
        );
        push_collection_projection(&mut builder, viewer_id);
        builder.push(
            " FROM colecciones c \
               JOIN usuarios_ext u ON u.id = c.usuario_id \
               WHERE c.slug = ",
        );
        builder.push_bind(slug.trim().to_owned());
        builder.push(" AND c.eliminado_en IS NULL AND u.estado = 'activo'");

        let row = builder
            .build_query_as::<LegacyColeccionRecord>()
            .fetch_optional(pool)
            .await?;
        Ok(row)
    }

    pub async fn list_relevant_for_sample_legacy(
        pool: &PgPool,
        usuario_id: i32,
        sample_id: i32,
        limit: i64,
    ) -> Result<Vec<LegacyColeccionRecord>, AppError> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "WITH reference_tags AS (
                SELECT DISTINCT tag_val
                FROM samples rs
                CROSS JOIN LATERAL jsonb_array_elements_text(
                    COALESCE(
                        CASE WHEN jsonb_typeof(rs.metadata->'tags') = 'array' THEN rs.metadata->'tags' END,
                        CASE WHEN jsonb_typeof(rs.metadata->'tags_es') = 'array' THEN rs.metadata->'tags_es' END,
                        to_jsonb(COALESCE(rs.tags_enriquecidos, ARRAY[]::text[])),
                        '[]'::jsonb
                    )
                ) AS tag_val
                WHERE rs.id = ",
        );
        builder.push_bind(sample_id);
        builder.push(
            " AND rs.eliminado_en IS NULL
            )
            SELECT c.id, c.usuario_id, c.nombre, c.slug, COALESCE(c.descripcion, '') AS descripcion,
                   c.publica, c.parent_id, c.imagen_url, c.version, c.total_samples, ",
        );
        push_collection_projection(&mut builder, Some(usuario_id));
        builder.push(
            ", EXISTS(
                SELECT 1 FROM coleccion_samples cs_contains
                WHERE cs_contains.coleccion_id = c.id AND cs_contains.sample_id = ",
        );
        builder.push_bind(sample_id);
        builder.push(
            ") AS contiene_el_sample
             FROM colecciones c
             JOIN usuarios_ext u ON u.id = c.usuario_id
             LEFT JOIN LATERAL (
                SELECT COUNT(*) AS overlap
                FROM coleccion_samples cs_rel
                JOIN samples s_rel ON s_rel.id = cs_rel.sample_id
                CROSS JOIN LATERAL jsonb_array_elements_text(
                    COALESCE(
                        CASE WHEN jsonb_typeof(s_rel.metadata->'tags') = 'array' THEN s_rel.metadata->'tags' END,
                        CASE WHEN jsonb_typeof(s_rel.metadata->'tags_es') = 'array' THEN s_rel.metadata->'tags_es' END,
                        to_jsonb(COALESCE(s_rel.tags_enriquecidos, ARRAY[]::text[])),
                        '[]'::jsonb
                    )
                ) AS rel_tag
                WHERE cs_rel.coleccion_id = c.id
                  AND rel_tag IN (SELECT tag_val FROM reference_tags)
                  AND s_rel.estado = 'activo'
                  AND s_rel.eliminado_en IS NULL
             ) rel ON TRUE
             WHERE c.usuario_id = ",
        );
        builder.push_bind(usuario_id);
        builder.push(
            " AND c.eliminado_en IS NULL AND u.estado = 'activo'
             ORDER BY contiene_el_sample DESC, COALESCE(rel.overlap, 0) DESC, c.updated_at DESC, c.id DESC
             LIMIT ",
        );
        builder.push_bind(limit.clamp(1, 50));

        let rows = builder
            .build_query_as::<LegacyColeccionRecord>()
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }

    pub async fn list_subcollection_ids_legacy(
        pool: &PgPool,
        parent_id: i64,
    ) -> Result<Vec<i64>, AppError> {
        let rows = sqlx::query_scalar::<_, i64>(
            r"SELECT id
               FROM colecciones
               WHERE parent_id = $1
                 AND eliminado_en IS NULL
                             ORDER BY updated_at DESC, id DESC",
        )
        .bind(parent_id)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn list_subcollections_legacy(
        pool: &PgPool,
        parent_id: i64,
        viewer_id: Option<i32>,
    ) -> Result<Vec<LegacyColeccionRecord>, AppError> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT c.id, c.usuario_id, c.nombre, c.slug, COALESCE(c.descripcion, '') AS descripcion, \
                    c.publica, c.parent_id, c.imagen_url, c.version, c.total_samples, ",
        );
        push_collection_projection(&mut builder, viewer_id);
        builder.push(
            " FROM colecciones c \
               JOIN usuarios_ext u ON u.id = c.usuario_id \
               WHERE c.parent_id = ",
        );
        builder.push_bind(parent_id);
        builder.push(" AND c.eliminado_en IS NULL AND u.estado = 'activo' ORDER BY c.updated_at DESC, c.id DESC");

        let rows = builder
            .build_query_as::<LegacyColeccionRecord>()
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }

    pub async fn fetch_parent_legacy(
        pool: &PgPool,
        id: i64,
    ) -> Result<Option<LegacyColeccionParentRecord>, AppError> {
        let row = sqlx::query_as!(
            LegacyColeccionParentRecord,
            r#"SELECT id, nombre, slug
                             FROM colecciones
                             WHERE id = $1
                                 AND eliminado_en IS NULL"#,
            id,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn list_detail_samples_legacy(
        pool: &PgPool,
        coleccion_ids: &[i64],
    ) -> Result<Vec<LegacyColeccionSampleRecord>, AppError> {
        if coleccion_ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT s.id, s.id_corto, s.slug, s.titulo, COALESCE(s.descripcion, '') AS descripcion, \
                    s.bpm, s.key AS music_key, s.escala, s.duracion, s.formato, \
                    COALESCE(s.metadata, '{}'::jsonb) AS metadata, \
                    COALESCE(s.tags_enriquecidos, ARRAY[]::text[]) AS tags, \
                    s.tipo, s.es_premium, s.precio, COALESCE(s.verificado, FALSE) AS verificado, \
                    s.ruta_preview, s.ruta_waveform, s.imagen_url, \
                    COALESCE(s.total_descargas, 0) AS total_descargas, \
                    COALESCE(s.total_likes, 0) AS total_likes, \
                    COALESCE(s.total_reproducciones, 0) AS total_reproducciones, \
                    COALESCE(s.total_comentarios, 0) AS total_comentarios, \
                    u.id AS creator_id, u.username AS creator_username, u.nombre_visible AS creator_nombre_visible, \
                    u.avatar_url AS creator_avatar_url, COALESCE(u.verificado, FALSE) AS creator_verificado \
               FROM coleccion_samples cs \
               JOIN samples s ON s.id = cs.sample_id \
               JOIN usuarios_ext u ON u.id = s.creador_id \
               WHERE cs.coleccion_id = ANY(",
        );
        builder.push_bind(coleccion_ids.to_vec());
        builder.push(") AND s.estado = 'activo' AND s.eliminado_en IS NULL AND u.estado = 'activo' ORDER BY array_position(");
        builder.push_bind(coleccion_ids.to_vec());
        builder.push(", cs.coleccion_id), cs.orden ASC, cs.added_at ASC");

        let rows = builder
            .build_query_as::<LegacyColeccionSampleRecord>()
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

fn push_collection_projection(builder: &mut QueryBuilder<Postgres>, viewer_id: Option<i32>) {
    push_collection_projection_common(builder, viewer_id);
    builder.push(", NULL::boolean AS contiene_el_sample");
}

fn push_collection_projection_common(builder: &mut QueryBuilder<Postgres>, viewer_id: Option<i32>) {
    builder.push(COLLECTION_TOTAL_ITEMS_SQL);
    builder.push(" AS total_items, c.created_at, c.updated_at, ");
    builder.push(COLLECTION_TAGS_SQL);
    builder.push(" AS tags, u.username, u.nombre_visible, u.avatar_url, COALESCE(u.verificado, FALSE) AS verificado, ");
    match viewer_id {
        Some(current_user_id) => {
            builder.push("EXISTS(SELECT 1 FROM colecciones_guardadas cg WHERE cg.coleccion_id = c.id AND cg.usuario_id = ");
            builder.push_bind(current_user_id);
            builder.push(") AS esta_guardada, ");
        }
        None => {
            builder.push("FALSE AS esta_guardada, ");
        }
    }
    builder.push("FALSE AS esta_likeada, 0 AS total_likes");
}

fn normalize_search(busqueda: Option<&str>) -> Option<String> {
    let trimmed = busqueda?.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(format!("%{trimmed}%"))
    }
}

fn push_search_filter(builder: &mut QueryBuilder<Postgres>, search: &str) {
    builder.push(" AND (c.nombre ILIKE ");
    builder.push_bind(search.to_owned());
    builder.push(" OR COALESCE(c.descripcion, '') ILIKE ");
    builder.push_bind(search.to_owned());
    builder.push(")");
}

fn parse_slug_collection_id(slug: &str) -> Option<i64> {
    let trimmed = slug.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Ok(id) = trimmed.parse::<i64>() {
        return Some(id);
    }

    trimmed
        .rsplit('-')
        .next()
        .and_then(|fragment| fragment.parse::<i64>().ok())
}
