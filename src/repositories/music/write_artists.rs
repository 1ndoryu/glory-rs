use sqlx::PgPool;

use super::support::generate_unique_slug;
use super::MusicRepository;
use crate::errors::AppError;
use crate::models::{CreateArtistRequest, UpdateArtistRequest};

impl MusicRepository {
    pub async fn create_artist(pool: &PgPool, request: &CreateArtistRequest) -> Result<i32, AppError> {
        let slug = match request.slug.as_deref() {
            Some(value) if !value.trim().is_empty() => value.trim().to_string(),
            _ => Self::generate_unique_artist_slug(pool, &request.nombre, None).await?,
        };

        let artist_id = sqlx::query_scalar!(
            r#"INSERT INTO artistas_musicales (
                    nombre,
                    slug,
                    imagen_url,
                    whosampled_slug,
                    musicbrainz_id,
                    metadata,
                    prioridad
               )
                       VALUES ($1, $2, $3, $4, $5, COALESCE($6::jsonb, '{}'::jsonb), COALESCE($7::smallint, 0::smallint))
               RETURNING id AS "id!""#,
            request.nombre.trim(),
            slug,
            request.imagen_url.as_deref().map(str::trim),
            request.whosampled_slug.as_deref().map(str::trim),
            request.musicbrainz_id.as_deref().map(str::trim),
            request.metadata.clone(),
            request.prioridad,
        )
        .fetch_one(pool)
        .await?;

        Ok(artist_id)
    }

    pub async fn update_artist(
        pool: &PgPool,
        artist_id: i32,
        request: &UpdateArtistRequest,
    ) -> Result<bool, AppError> {
        let slug = if let Some(slug) = request.slug.as_deref() {
            Some(slug.trim().to_string())
        } else if let Some(nombre) = request.nombre.as_deref() {
            Some(Self::generate_unique_artist_slug(pool, nombre, Some(artist_id)).await?)
        } else {
            None
        };

        let updated = sqlx::query_scalar!(
            r#"UPDATE artistas_musicales
               SET nombre = COALESCE($2, nombre),
                   slug = COALESCE($3, slug),
                   imagen_url = COALESCE($4, imagen_url),
                   whosampled_slug = COALESCE($5, whosampled_slug),
                   musicbrainz_id = COALESCE($6, musicbrainz_id),
                   metadata = COALESCE($7::jsonb, metadata),
                   prioridad = COALESCE($8::smallint, prioridad),
                   updated_at = NOW()
               WHERE id = $1
               RETURNING id AS "id!""#,
            artist_id,
            request.nombre.as_deref().map(str::trim),
            slug,
            request.imagen_url.as_deref().map(str::trim),
            request.whosampled_slug.as_deref().map(str::trim),
            request.musicbrainz_id.as_deref().map(str::trim),
            request.metadata.clone(),
            request.prioridad,
        )
        .fetch_optional(pool)
        .await?;

        Ok(updated.is_some())
    }

    pub async fn delete_artist(pool: &PgPool, artist_id: i32) -> Result<bool, AppError> {
        let primary_songs = sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!" FROM canciones WHERE artista_id = $1"#,
            artist_id,
        )
        .fetch_one(pool)
        .await?;

        if primary_songs > 0 {
            return Err(AppError::Conflict(
                "El artista tiene canciones principales asociadas".into(),
            ));
        }

        let deleted = sqlx::query!(r#"DELETE FROM artistas_musicales WHERE id = $1"#, artist_id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(deleted > 0)
    }

    async fn generate_unique_artist_slug(
        pool: &PgPool,
        name: &str,
        exclude_id: Option<i32>,
    ) -> Result<String, AppError> {
        generate_unique_slug(pool, name, exclude_id, "artista", true).await
    }
}