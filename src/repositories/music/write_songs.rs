use sqlx::{PgPool, Postgres, Transaction};

use super::support::{
    artist_role_db, collect_artist_ids, dedupe_i32, generate_unique_slug,
    normalize_song_artists, parse_artist_role, to_i16,
};
use super::MusicRepository;
use crate::errors::AppError;
use crate::models::{CreateSongRequest, MusicArtistRole, UpdateSongRequest};

impl MusicRepository {
    pub async fn create_song(pool: &PgPool, request: &CreateSongRequest) -> Result<i32, AppError> {
        let main_artist_id = request.artista_id;
        Self::ensure_artists_exist(pool, &collect_artist_ids(main_artist_id, &request.artistas)).await?;

        let slug = match request.slug.as_deref() {
            Some(value) if !value.trim().is_empty() => value.trim().to_string(),
            _ => Self::generate_unique_song_slug(pool, &request.titulo, None).await?,
        };

        let mut tx = pool.begin().await?;
        let song_id = sqlx::query_scalar!(
            r#"INSERT INTO canciones (
                    titulo,
                    slug,
                    artista_id,
                    album,
                    sello,
                    anio,
                    duracion_segundos,
                    genero,
                    youtube_id,
                    spotify_id,
                    imagen_url,
                    whosampled_url,
                    bpm,
                    tonalidad,
                    metadata
               )
               VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8,
                    $9, $10, $11, $12, $13, $14, COALESCE($15::jsonb, '{}'::jsonb)
               )
               RETURNING id AS "id!""#,
            request.titulo.trim(),
            slug,
            main_artist_id,
            request.album.as_deref().map(str::trim),
            request.sello.as_deref().map(str::trim),
            to_i16(request.anio, "anio")?,
            to_i16(request.duracion_segundos, "duracion_segundos")?,
            request.genero.as_deref().map(str::trim),
            request.youtube_id.as_deref().map(str::trim),
            request.spotify_id.as_deref().map(str::trim),
            request.imagen_url.as_deref().map(str::trim),
            request.whosampled_url.as_deref().map(str::trim),
            to_i16(request.bpm, "bpm")?,
            request.tonalidad.as_deref().map(str::trim),
            request.metadata.clone(),
        )
        .fetch_one(&mut *tx)
        .await?;

        let assignments = normalize_song_artists(main_artist_id, Some(&request.artistas), &[])?;
        Self::replace_song_artists(&mut tx, song_id, &assignments).await?;
        Self::recount_artist_totals_tx(&mut tx, &[main_artist_id]).await?;
        tx.commit().await?;
        Ok(song_id)
    }

    pub async fn update_song(
        pool: &PgPool,
        song_id: i32,
        main_artist_id: i32,
        request: &UpdateSongRequest,
    ) -> Result<bool, AppError> {
        let existing = Self::find_song_by_id(pool, song_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("cancion {song_id}")))?;
        let current_assignments = Self::load_song_artist_assignments(pool, song_id).await?;
        let requested_artist_ids = collect_artist_ids(main_artist_id, request.artistas.as_deref().unwrap_or(&[]));
        Self::ensure_artists_exist(pool, &requested_artist_ids).await?;

        let slug = if let Some(value) = request.slug.as_deref() {
            Some(value.trim().to_string())
        } else if let Some(title) = request.titulo.as_deref() {
            Some(Self::generate_unique_song_slug(pool, title, Some(song_id)).await?)
        } else {
            None
        };

        let mut tx = pool.begin().await?;
        let updated = sqlx::query_scalar!(
            r#"UPDATE canciones
               SET titulo = COALESCE($2, titulo),
                   slug = COALESCE($3, slug),
                   artista_id = COALESCE($4, artista_id),
                   album = COALESCE($5, album),
                   sello = COALESCE($6, sello),
                   anio = COALESCE($7, anio),
                   duracion_segundos = COALESCE($8, duracion_segundos),
                   genero = COALESCE($9, genero),
                   youtube_id = COALESCE($10, youtube_id),
                   spotify_id = COALESCE($11, spotify_id),
                   imagen_url = COALESCE($12, imagen_url),
                   whosampled_url = COALESCE($13, whosampled_url),
                   bpm = COALESCE($14, bpm),
                   tonalidad = COALESCE($15, tonalidad),
                   metadata = COALESCE($16::jsonb, metadata),
                   updated_at = NOW()
               WHERE id = $1
               RETURNING id AS "id!""#,
            song_id,
            request.titulo.as_deref().map(str::trim),
            slug,
            if request.artista_id.is_some() || request.artistas.is_some() {
                Some(main_artist_id)
            } else {
                None
            },
            request.album.as_deref().map(str::trim),
            request.sello.as_deref().map(str::trim),
            to_i16(request.anio, "anio")?,
            to_i16(request.duracion_segundos, "duracion_segundos")?,
            request.genero.as_deref().map(str::trim),
            request.youtube_id.as_deref().map(str::trim),
            request.spotify_id.as_deref().map(str::trim),
            request.imagen_url.as_deref().map(str::trim),
            request.whosampled_url.as_deref().map(str::trim),
            to_i16(request.bpm, "bpm")?,
            request.tonalidad.as_deref().map(str::trim),
            request.metadata.clone(),
        )
        .fetch_optional(&mut *tx)
        .await?;

        if updated.is_none() {
            return Ok(false);
        }

        if request.artista_id.is_some() || request.artistas.is_some() {
            let assignments = normalize_song_artists(
                main_artist_id,
                request.artistas.as_deref(),
                &current_assignments,
            )?;
            Self::replace_song_artists(&mut tx, song_id, &assignments).await?;
            Self::recount_artist_totals_tx(&mut tx, &[existing.artista_id, main_artist_id]).await?;
        }

        tx.commit().await?;
        Ok(true)
    }

    pub async fn delete_song(pool: &PgPool, song_id: i32) -> Result<bool, AppError> {
        let song = Self::find_song_by_id(pool, song_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("cancion {song_id}")))?;

        let mut tx = pool.begin().await?;
        let relation_ids = sqlx::query_scalar!(
            r#"SELECT id AS "id!"
               FROM relaciones_sample
               WHERE cancion_fuente_id = $1 OR cancion_destino_id = $1"#,
            song_id,
        )
        .fetch_all(&mut *tx)
        .await?;

        if !relation_ids.is_empty() {
            sqlx::query!(
                r#"DELETE FROM likes WHERE tipo = 'relacion' AND target_id = ANY($1)"#,
                &relation_ids,
            )
            .execute(&mut *tx)
            .await?;

            sqlx::query!(
                r#"DELETE FROM comentarios WHERE tipo = 'relacion' AND target_id = ANY($1)"#,
                &relation_ids,
            )
            .execute(&mut *tx)
            .await?;
        }

        sqlx::query!(r#"DELETE FROM likes WHERE tipo = 'cancion' AND target_id = $1"#, song_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!(r#"DELETE FROM comentarios WHERE tipo = 'cancion' AND target_id = $1"#, song_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!(
            r#"DELETE FROM relaciones_sample WHERE cancion_fuente_id = $1 OR cancion_destino_id = $1"#,
            song_id,
        )
        .execute(&mut *tx)
        .await?;

        let deleted = sqlx::query!(r#"DELETE FROM canciones WHERE id = $1"#, song_id)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        if deleted == 0 {
            return Ok(false);
        }

        Self::recount_artist_totals_tx(&mut tx, &[song.artista_id]).await?;
        tx.commit().await?;
        Ok(true)
    }

    async fn ensure_artists_exist(pool: &PgPool, artist_ids: &[i32]) -> Result<(), AppError> {
        let unique_ids = dedupe_i32(artist_ids);
        if unique_ids.is_empty() {
            return Ok(());
        }

        let found = sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!" FROM artistas_musicales WHERE id = ANY($1)"#,
            &unique_ids,
        )
        .fetch_one(pool)
        .await?;

        if found != i64::try_from(unique_ids.len()).unwrap_or(i64::MAX) {
            return Err(AppError::Validation(
                "Uno o mas artistas no existen".into(),
            ));
        }

        Ok(())
    }

    async fn load_song_artist_assignments(
        pool: &PgPool,
        song_id: i32,
    ) -> Result<Vec<(i32, MusicArtistRole)>, AppError> {
        let rows = sqlx::query!(
            r#"SELECT artista_id, rol AS "rol!" FROM canciones_artistas WHERE cancion_id = $1"#,
            song_id,
        )
        .fetch_all(pool)
        .await?;

        rows.into_iter()
            .map(|row| Ok((row.artista_id, parse_artist_role(&row.rol)?)))
            .collect()
    }

    async fn replace_song_artists(
        tx: &mut Transaction<'_, Postgres>,
        song_id: i32,
        assignments: &[(i32, MusicArtistRole)],
    ) -> Result<(), AppError> {
        sqlx::query!(r#"DELETE FROM canciones_artistas WHERE cancion_id = $1"#, song_id)
            .execute(&mut **tx)
            .await?;

        for (artist_id, role) in assignments {
            sqlx::query!(
                r#"INSERT INTO canciones_artistas (cancion_id, artista_id, rol)
                   VALUES ($1, $2, $3)"#,
                song_id,
                artist_id,
                artist_role_db(*role),
            )
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    async fn recount_artist_totals_tx(
        tx: &mut Transaction<'_, Postgres>,
        artist_ids: &[i32],
    ) -> Result<(), AppError> {
        let unique_ids = dedupe_i32(artist_ids);
        if unique_ids.is_empty() {
            return Ok(());
        }

        sqlx::query!(
            r#"UPDATE artistas_musicales a
               SET total_canciones = (
                    SELECT COUNT(*)::int
                    FROM canciones c
                    WHERE c.artista_id = a.id
               ),
                   updated_at = NOW()
               WHERE a.id = ANY($1)"#,
            &unique_ids,
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn generate_unique_song_slug(
        pool: &PgPool,
        title: &str,
        exclude_id: Option<i32>,
    ) -> Result<String, AppError> {
        generate_unique_slug(pool, title, exclude_id, "cancion", false).await
    }
}