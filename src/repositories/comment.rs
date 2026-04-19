use serde::Serialize;
use sqlx::PgPool;
use std::str::FromStr;
use utoipa::ToSchema;

use crate::errors::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentTargetKind {
    Sample,
    Publicacion,
    Cancion,
    Relacion,
    Articulo,
}

impl CommentTargetKind {
    pub const fn as_db_str(self) -> &'static str {
        match self {
            Self::Sample => "sample",
            Self::Publicacion => "publicacion",
            Self::Cancion => "cancion",
            Self::Relacion => "relacion",
            Self::Articulo => "articulo",
        }
    }
}

impl FromStr for CommentTargetKind {
    type Err = AppError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "sample" => Ok(Self::Sample),
            "publicacion" => Ok(Self::Publicacion),
            "cancion" => Ok(Self::Cancion),
            "relacion" => Ok(Self::Relacion),
            "articulo" => Ok(Self::Articulo),
            other => Err(AppError::Validation(format!(
                "tipo de comentario inválido: {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentContentKind {
    Texto,
    Imagen,
    Audio,
}

impl CommentContentKind {
    pub const fn as_db_str(self) -> &'static str {
        match self {
            Self::Texto => "texto",
            Self::Imagen => "imagen",
            Self::Audio => "audio",
        }
    }
}

impl FromStr for CommentContentKind {
    type Err = AppError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "texto" => Ok(Self::Texto),
            "imagen" => Ok(Self::Imagen),
            "audio" => Ok(Self::Audio),
            other => Err(AppError::Validation(format!(
                "tipo_contenido inválido: {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateCommentParams<'a> {
    pub autor_id: i32,
    pub target_kind: CommentTargetKind,
    pub target_id: i32,
    pub contenido: &'a str,
    pub content_kind: CommentContentKind,
    pub media_url: Option<&'a str>,
    pub media_metadata: Option<serde_json::Value>,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct CommentContext {
    pub id: i32,
    pub autor_id: i32,
    pub target_kind: CommentTargetKind,
    pub target_id: i32,
    pub parent_id: Option<i32>,
    pub media_url: Option<String>,
    pub content_kind: CommentContentKind,
}

#[derive(Debug)]
struct CommentContextRow {
    id: i32,
    autor_id: i32,
    tipo: String,
    target_id: i32,
    parent_id: Option<i32>,
    media_url: Option<String>,
    tipo_contenido: String,
}

#[derive(Debug)]
struct CommentRow {
    id: i32,
    autor_id: i32,
    tipo: String,
    target_id: i32,
    contenido: String,
    tipo_contenido: String,
    media_url: Option<String>,
    media_metadata: Option<serde_json::Value>,
    parent_id: Option<i32>,
    total_likes: i32,
    total_respuestas: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
    author_id: i32,
    author_username: String,
    author_display_name: Option<String>,
    author_avatar_url: Option<String>,
    author_verified: bool,
    mi_reaccion: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CommentAuthorSummary {
    pub id: i32,
    pub username: String,
    pub nombre_visible: Option<String>,
    pub avatar_url: Option<String>,
    pub verificado: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CommentDetail {
    pub id: i32,
    pub autor_id: i32,
    pub tipo: String,
    pub target_id: i32,
    pub contenido: String,
    pub tipo_contenido: String,
    pub media_url: Option<String>,
    #[schema(value_type = Object)]
    pub media_metadata: Option<serde_json::Value>,
    pub parent_id: Option<i32>,
    pub total_likes: i32,
    pub total_respuestas: i32,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub liked: bool,
    pub mi_reaccion: Option<String>,
    pub autor: CommentAuthorSummary,
}

pub struct CommentRepository;

/* [174A-68] Comentarios polimórficos en un repositorio dedicado para mantener
 * list/get/create/delete con el mismo shape SQL validado en compilación.
 * Gotcha: los contadores viven en tablas distintas según `tipo`, por eso el recount usa ramas explícitas.
 * Pendiente: moderación IA/notificaciones se integrarán en tareas posteriores. */

impl CommentRepository {
    pub async fn target_exists(
        pool: &PgPool,
        kind: CommentTargetKind,
        target_id: i32,
    ) -> Result<bool, AppError> {
        let exists = match kind {
            CommentTargetKind::Sample => sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM samples WHERE id = $1 AND eliminado_en IS NULL) AS \"e!\"",
                target_id,
            )
            .fetch_one(pool)
            .await?,
            CommentTargetKind::Publicacion => sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM publicaciones WHERE id = $1 AND eliminado_en IS NULL) AS \"e!\"",
                target_id,
            )
            .fetch_one(pool)
            .await?,
            CommentTargetKind::Cancion => sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM canciones WHERE id = $1) AS \"e!\"",
                target_id,
            )
            .fetch_one(pool)
            .await?,
            CommentTargetKind::Relacion => sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM relaciones_sample WHERE id = $1) AS \"e!\"",
                target_id,
            )
            .fetch_one(pool)
            .await?,
            CommentTargetKind::Articulo => sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM articulos WHERE id = $1 AND eliminado_en IS NULL) AS \"e!\"",
                target_id,
            )
            .fetch_one(pool)
            .await?,
        };
        Ok(exists)
    }

    pub async fn validate_parent_context(
        pool: &PgPool,
        parent_id: i32,
        kind: CommentTargetKind,
        target_id: i32,
    ) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(
                    SELECT 1
                    FROM comentarios
                    WHERE id = $1
                      AND tipo = $2
                      AND target_id = $3
                      AND (moderacion_estado IS NULL OR moderacion_estado != 'rechazado')
                                ) AS "e!""#,
            parent_id,
            kind.as_db_str(),
            target_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(exists)
    }

    pub async fn create(pool: &PgPool, params: CreateCommentParams<'_>) -> Result<i32, AppError> {
        let id = sqlx::query_scalar!(
            r#"INSERT INTO comentarios (
                    autor_id, tipo, target_id, contenido, tipo_contenido, media_url, media_metadata, parent_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id AS "id!""#,
            params.autor_id,
            params.target_kind.as_db_str(),
            params.target_id,
            params.contenido,
            params.content_kind.as_db_str(),
            params.media_url,
            params.media_metadata,
            params.parent_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(id)
    }

    pub async fn update_content(
        pool: &PgPool,
        comment_id: i32,
        autor_id: i32,
        contenido: &str,
    ) -> Result<bool, AppError> {
        let updated = sqlx::query!(
            r#"UPDATE comentarios
               SET contenido = $3,
                   updated_at = NOW()
               WHERE id = $1 AND autor_id = $2"#,
            comment_id,
            autor_id,
            contenido,
        )
        .execute(pool)
        .await?
        .rows_affected();
        Ok(updated > 0)
    }

    pub async fn delete(pool: &PgPool, comment_id: i32) -> Result<bool, AppError> {
        let deleted = sqlx::query!("DELETE FROM comentarios WHERE id = $1", comment_id)
            .execute(pool)
            .await?
            .rows_affected();
        Ok(deleted > 0)
    }

    pub async fn increment_replies(pool: &PgPool, comment_id: i32) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE comentarios SET total_respuestas = total_respuestas + 1 WHERE id = $1",
            comment_id,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn decrement_replies(pool: &PgPool, comment_id: i32) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE comentarios SET total_respuestas = GREATEST(total_respuestas - 1, 0) WHERE id = $1",
            comment_id,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn recount_target(
        pool: &PgPool,
        kind: CommentTargetKind,
        target_id: i32,
    ) -> Result<(), AppError> {
        match kind {
            CommentTargetKind::Sample => {
                sqlx::query!(
                    "UPDATE samples SET total_comentarios = (SELECT COUNT(*) FROM comentarios WHERE tipo = 'sample' AND target_id = $1) WHERE id = $1",
                    target_id,
                )
                .execute(pool)
                .await?;
            }
            CommentTargetKind::Publicacion => {
                sqlx::query!(
                    "UPDATE publicaciones SET total_comentarios = (SELECT COUNT(*) FROM comentarios WHERE tipo = 'publicacion' AND target_id = $1) WHERE id = $1",
                    target_id,
                )
                .execute(pool)
                .await?;
            }
            CommentTargetKind::Cancion => {
                sqlx::query!(
                    "UPDATE canciones SET total_comentarios = (SELECT COUNT(*) FROM comentarios WHERE tipo = 'cancion' AND target_id = $1) WHERE id = $1",
                    target_id,
                )
                .execute(pool)
                .await?;
            }
            CommentTargetKind::Relacion => {
                sqlx::query!(
                    "UPDATE relaciones_sample SET total_comentarios = (SELECT COUNT(*) FROM comentarios WHERE tipo = 'relacion' AND target_id = $1) WHERE id = $1",
                    target_id,
                )
                .execute(pool)
                .await?;
            }
            CommentTargetKind::Articulo => {
                sqlx::query!(
                    "UPDATE articulos SET total_comentarios = (SELECT COUNT(*) FROM comentarios WHERE tipo = 'articulo' AND target_id = $1) WHERE id = $1",
                    target_id,
                )
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    pub async fn get(
        pool: &PgPool,
        viewer_id: Option<i32>,
        comment_id: i32,
        blocked_ids: &[i32],
    ) -> Result<Option<CommentDetail>, AppError> {
        let row = sqlx::query_as!(
            CommentRow,
            r#"SELECT
                    c.id AS "id!",
                    c.autor_id AS "autor_id!",
                    c.tipo AS "tipo!",
                    c.target_id AS "target_id!",
                    COALESCE(c.contenido, '') AS "contenido!",
                    COALESCE(c.tipo_contenido, 'texto') AS "tipo_contenido!",
                    c.media_url,
                    c.media_metadata AS "media_metadata?: serde_json::Value",
                    c.parent_id,
                    COALESCE(c.total_likes, 0) AS "total_likes!",
                    COALESCE(c.total_respuestas, 0) AS "total_respuestas!",
                    c.created_at AS "created_at!: chrono::DateTime<chrono::Utc>",
                    c.updated_at AS "updated_at?: chrono::DateTime<chrono::Utc>",
                    u.id AS "author_id!",
                    u.username AS "author_username!",
                    u.nombre_visible AS "author_display_name",
                    u.avatar_url AS "author_avatar_url",
                    COALESCE(u.verificado, FALSE) AS "author_verified!",
                    (SELECT reaccion FROM likes WHERE usuario_id = $1 AND tipo = 'comentario' AND target_id = c.id LIMIT 1) AS "mi_reaccion"
               FROM comentarios c
               JOIN usuarios_ext u ON u.id = c.autor_id
               WHERE c.id = $2
                 AND (c.moderacion_estado IS NULL OR c.moderacion_estado != 'rechazado')
                 AND NOT (c.autor_id = ANY($3::int[]))"#,
            viewer_id,
            comment_id,
            blocked_ids,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row.map(map_comment_row))
    }

    pub async fn list_roots(
        pool: &PgPool,
        viewer_id: Option<i32>,
        target_kind: CommentTargetKind,
        target_id: i32,
        blocked_ids: &[i32],
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CommentDetail>, AppError> {
        let rows = sqlx::query_as!(
            CommentRow,
            r#"SELECT
                    c.id AS "id!",
                    c.autor_id AS "autor_id!",
                    c.tipo AS "tipo!",
                    c.target_id AS "target_id!",
                    COALESCE(c.contenido, '') AS "contenido!",
                    COALESCE(c.tipo_contenido, 'texto') AS "tipo_contenido!",
                    c.media_url,
                    c.media_metadata AS "media_metadata?: serde_json::Value",
                    c.parent_id,
                    COALESCE(c.total_likes, 0) AS "total_likes!",
                    COALESCE(c.total_respuestas, 0) AS "total_respuestas!",
                    c.created_at AS "created_at!: chrono::DateTime<chrono::Utc>",
                    c.updated_at AS "updated_at?: chrono::DateTime<chrono::Utc>",
                    u.id AS "author_id!",
                    u.username AS "author_username!",
                    u.nombre_visible AS "author_display_name",
                    u.avatar_url AS "author_avatar_url",
                    COALESCE(u.verificado, FALSE) AS "author_verified!",
                    (SELECT reaccion FROM likes WHERE usuario_id = $1 AND tipo = 'comentario' AND target_id = c.id LIMIT 1) AS "mi_reaccion"
               FROM comentarios c
               JOIN usuarios_ext u ON u.id = c.autor_id
               WHERE c.tipo = $2
                 AND c.target_id = $3
                 AND c.parent_id IS NULL
                 AND (c.moderacion_estado IS NULL OR c.moderacion_estado != 'rechazado')
                 AND NOT (c.autor_id = ANY($4::int[]))
               ORDER BY c.total_likes DESC, c.created_at DESC
               LIMIT $5 OFFSET $6"#,
            viewer_id,
            target_kind.as_db_str(),
            target_id,
            blocked_ids,
            limit,
            offset,
        )
        .fetch_all(pool)
        .await?;
        Ok(rows.into_iter().map(map_comment_row).collect())
    }

    pub async fn list_replies(
        pool: &PgPool,
        viewer_id: Option<i32>,
        parent_id: i32,
        blocked_ids: &[i32],
        limit: i64,
    ) -> Result<Vec<CommentDetail>, AppError> {
        let rows = sqlx::query_as!(
            CommentRow,
            r#"SELECT
                    c.id AS "id!",
                    c.autor_id AS "autor_id!",
                    c.tipo AS "tipo!",
                    c.target_id AS "target_id!",
                    COALESCE(c.contenido, '') AS "contenido!",
                    COALESCE(c.tipo_contenido, 'texto') AS "tipo_contenido!",
                    c.media_url,
                    c.media_metadata AS "media_metadata?: serde_json::Value",
                    c.parent_id,
                    COALESCE(c.total_likes, 0) AS "total_likes!",
                    COALESCE(c.total_respuestas, 0) AS "total_respuestas!",
                    c.created_at AS "created_at!: chrono::DateTime<chrono::Utc>",
                    c.updated_at AS "updated_at?: chrono::DateTime<chrono::Utc>",
                    u.id AS "author_id!",
                    u.username AS "author_username!",
                    u.nombre_visible AS "author_display_name",
                    u.avatar_url AS "author_avatar_url",
                    COALESCE(u.verificado, FALSE) AS "author_verified!",
                    (SELECT reaccion FROM likes WHERE usuario_id = $1 AND tipo = 'comentario' AND target_id = c.id LIMIT 1) AS "mi_reaccion"
               FROM comentarios c
               JOIN usuarios_ext u ON u.id = c.autor_id
               WHERE c.parent_id = $2
                 AND (c.moderacion_estado IS NULL OR c.moderacion_estado != 'rechazado')
                 AND NOT (c.autor_id = ANY($3::int[]))
               ORDER BY c.created_at ASC
               LIMIT $4"#,
            viewer_id,
            parent_id,
            blocked_ids,
            limit,
        )
        .fetch_all(pool)
        .await?;
        Ok(rows.into_iter().map(map_comment_row).collect())
    }

    pub async fn find_context(
        pool: &PgPool,
        comment_id: i32,
    ) -> Result<Option<CommentContext>, AppError> {
        let row = sqlx::query_as!(
            CommentContextRow,
            r#"SELECT
                    id AS "id!",
                    autor_id AS "autor_id!",
                    tipo AS "tipo!",
                    target_id AS "target_id!",
                    parent_id,
                    media_url,
                    COALESCE(tipo_contenido, 'texto') AS "tipo_contenido!"
               FROM comentarios
               WHERE id = $1"#,
            comment_id,
        )
        .fetch_optional(pool)
        .await?;

        row.map(TryInto::try_into).transpose()
    }

    pub async fn list_media_urls_for_thread(
        pool: &PgPool,
        comment_id: i32,
    ) -> Result<Vec<String>, AppError> {
        let urls = sqlx::query_scalar!(
            r#"WITH RECURSIVE comment_tree AS (
                    SELECT id, media_url
                    FROM comentarios
                    WHERE id = $1
                    UNION ALL
                    SELECT c.id, c.media_url
                    FROM comentarios c
                    JOIN comment_tree ct ON c.parent_id = ct.id
                )
                SELECT media_url AS "media_url!"
                FROM comment_tree
                WHERE media_url IS NOT NULL"#,
            comment_id,
        )
        .fetch_all(pool)
        .await?;
        Ok(urls)
    }
}

impl TryFrom<CommentContextRow> for CommentContext {
    type Error = AppError;

    fn try_from(row: CommentContextRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            autor_id: row.autor_id,
            target_kind: CommentTargetKind::from_str(&row.tipo)?,
            target_id: row.target_id,
            parent_id: row.parent_id,
            media_url: row.media_url,
            content_kind: CommentContentKind::from_str(&row.tipo_contenido)?,
        })
    }
}

fn map_comment_row(row: CommentRow) -> CommentDetail {
    CommentDetail {
        id: row.id,
        autor_id: row.autor_id,
        tipo: row.tipo,
        target_id: row.target_id,
        contenido: row.contenido,
        tipo_contenido: row.tipo_contenido,
        media_url: row.media_url,
        media_metadata: row.media_metadata,
        parent_id: row.parent_id,
        total_likes: row.total_likes,
        total_respuestas: row.total_respuestas,
        created_at: row.created_at,
        updated_at: row.updated_at,
        liked: row.mi_reaccion.is_some(),
        mi_reaccion: row.mi_reaccion,
        autor: CommentAuthorSummary {
            id: row.author_id,
            username: row.author_username,
            nombre_visible: row.author_display_name,
            avatar_url: row.author_avatar_url,
            verificado: row.author_verified,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{CommentContentKind, CommentTargetKind};
    use std::str::FromStr;

    #[test]
    fn parses_comment_target_kind() {
        assert_eq!(
            CommentTargetKind::from_str("sample").expect("sample"),
            CommentTargetKind::Sample
        );
        assert_eq!(
            CommentTargetKind::from_str("articulo").expect("articulo"),
            CommentTargetKind::Articulo
        );
        assert!(CommentTargetKind::from_str("otro").is_err());
    }

    #[test]
    fn parses_comment_content_kind() {
        assert_eq!(
            CommentContentKind::from_str("texto").expect("texto"),
            CommentContentKind::Texto
        );
        assert_eq!(
            CommentContentKind::from_str("audio").expect("audio"),
            CommentContentKind::Audio
        );
        assert!(CommentContentKind::from_str("video").is_err());
    }
}
