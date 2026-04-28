use sqlx::types::Json;
use sqlx::PgPool;

use super::support::{map_relation_summary, SampleRelationSummaryRecord};
use super::MusicRepository;
use crate::errors::AppError;
use crate::models::{MusicArtist, MusicSong, SampleRelationSummary};

impl MusicRepository {
    pub async fn top_artists(pool: &PgPool, limit: i64) -> Result<Vec<MusicArtist>, AppError> {
        let rows = sqlx::query_as!(
            MusicArtist,
            r#"SELECT id,
                      nombre,
                      slug,
                      imagen_url,
                      whosampled_slug,
                      musicbrainz_id,
                      COALESCE(metadata, '{}'::jsonb) AS "metadata!: serde_json::Value",
                      COALESCE(prioridad, 0)::smallint AS "prioridad!",
                      COALESCE(total_canciones, 0) AS "total_canciones!",
                      created_at AS "created_at!",
                      updated_at AS "updated_at!"
               FROM artistas_musicales
               ORDER BY COALESCE(total_canciones, 0) DESC, COALESCE(prioridad, 0) DESC, id DESC
               LIMIT $1"#,
            limit,
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_artist_by_slug(
        pool: &PgPool,
        slug: &str,
    ) -> Result<Option<MusicArtist>, AppError> {
        let row = sqlx::query_as!(
            MusicArtist,
            r#"SELECT id,
                      nombre,
                      slug,
                      imagen_url,
                      whosampled_slug,
                      musicbrainz_id,
                      COALESCE(metadata, '{}'::jsonb) AS "metadata!: serde_json::Value",
                      COALESCE(prioridad, 0)::smallint AS "prioridad!",
                      COALESCE(total_canciones, 0) AS "total_canciones!",
                      created_at AS "created_at!",
                      updated_at AS "updated_at!"
               FROM artistas_musicales
               WHERE slug = $1
               LIMIT 1"#,
            slug,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn find_artist_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<Option<MusicArtist>, AppError> {
        let row = sqlx::query_as!(
            MusicArtist,
            r#"SELECT id,
                      nombre,
                      slug,
                      imagen_url,
                      whosampled_slug,
                      musicbrainz_id,
                      COALESCE(metadata, '{}'::jsonb) AS "metadata!: serde_json::Value",
                      COALESCE(prioridad, 0)::smallint AS "prioridad!",
                      COALESCE(total_canciones, 0) AS "total_canciones!",
                      created_at AS "created_at!",
                      updated_at AS "updated_at!"
               FROM artistas_musicales
               WHERE id = $1
               LIMIT 1"#,
            id,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn list_artist_songs(
        pool: &PgPool,
        artist_id: i32,
    ) -> Result<Vec<MusicSong>, AppError> {
        let rows = sqlx::query_as!(
            MusicSong,
            r#"SELECT DISTINCT ON (c.id)
                      c.id,
                      c.titulo,
                      c.slug,
                      c.artista_id,
                      c.album,
                      c.sello,
                      c.anio,
                      c.duracion_segundos,
                      c.genero,
                      c.youtube_id,
                      c.spotify_id,
                      c.imagen_url,
                      c.whosampled_url,
                      c.bpm,
                      c.tonalidad,
                      COALESCE(c.metadata, '{}'::jsonb) AS "metadata!: serde_json::Value",
                      COALESCE(c.total_sampleada, 0) AS "total_sampleada!",
                      COALESCE(c.total_samplea, 0) AS "total_samplea!",
                      COALESCE(c.total_likes, 0) AS "total_likes!",
                      COALESCE(c.total_comentarios, 0) AS "total_comentarios!",
                      c.created_at AS "created_at!",
                      c.updated_at AS "updated_at!",
                      a.nombre AS "artista_nombre!",
                      a.slug AS "artista_slug!"
               FROM canciones_artistas ca
               INNER JOIN canciones c ON c.id = ca.cancion_id
               INNER JOIN artistas_musicales a ON a.id = c.artista_id
               WHERE ca.artista_id = $1
               ORDER BY c.id, c.anio DESC NULLS LAST, c.id DESC"#,
            artist_id,
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn list_artist_relations_as_fuente(
        pool: &PgPool,
        song_ids: &[i32],
        limit: i64,
    ) -> Result<Vec<SampleRelationSummary>, AppError> {
        if song_ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as!(
            SampleRelationSummaryRecord,
            r#"SELECT r.id,
                      r.cancion_destino_id,
                      r.cancion_fuente_id,
                      r.whosampled_id,
                      r.tipo_relacion AS "tipo_relacion!",
                      r.tipo_elemento,
                      COALESCE(r.timings_destino, '[]'::jsonb) AS "timings_destino!: Json<Vec<i32>>",
                      COALESCE(r.timings_fuente, '[]'::jsonb) AS "timings_fuente!: Json<Vec<i32>>",
                      COALESCE(r.aparece_en_todo, FALSE) AS "aparece_en_todo!",
                      r.sample_id,
                      r.sample_fuente_id,
                      r.sample_destino_id,
                      COALESCE(r.votos_total, 0) AS "votos_total!",
                      COALESCE(r.votos_promedio, 0)::double precision AS "votos_promedio!",
                      r.fuente AS "fuente!",
                      r.contribuidor_id,
                      u.username AS "contribuidor_username?: String",
                      COALESCE(r.verificada, FALSE) AS "verificada!",
                      COALESCE(r.total_likes, 0) AS "total_likes!",
                      COALESCE(r.total_comentarios, 0) AS "total_comentarios!",
                      r.created_at AS "created_at!",
                      r.updated_at AS "updated_at!",
                      cd.titulo AS "cancion_titulo!",
                      cd.slug AS "cancion_slug!",
                      ad.nombre AS "artista_nombre!",
                      ad.slug AS "artista_slug!",
                      cd.anio AS cancion_anio,
                      cd.imagen_url AS cancion_imagen_url
               FROM relaciones_sample r
               INNER JOIN canciones cd ON cd.id = r.cancion_destino_id
               INNER JOIN artistas_musicales ad ON ad.id = cd.artista_id
               LEFT JOIN usuarios_ext u ON u.id = r.contribuidor_id
               WHERE r.cancion_fuente_id = ANY($1)
               ORDER BY COALESCE(r.votos_total, 0) DESC, r.id DESC
               LIMIT $2"#,
            song_ids,
            limit,
        )
        .fetch_all(pool)
        .await?;

        rows.into_iter().map(map_relation_summary).collect()
    }

    pub async fn list_artist_relations_as_destino(
        pool: &PgPool,
        song_ids: &[i32],
        limit: i64,
    ) -> Result<Vec<SampleRelationSummary>, AppError> {
        if song_ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as!(
            SampleRelationSummaryRecord,
            r#"SELECT r.id,
                      r.cancion_destino_id,
                      r.cancion_fuente_id,
                      r.whosampled_id,
                      r.tipo_relacion AS "tipo_relacion!",
                      r.tipo_elemento,
                      COALESCE(r.timings_destino, '[]'::jsonb) AS "timings_destino!: Json<Vec<i32>>",
                      COALESCE(r.timings_fuente, '[]'::jsonb) AS "timings_fuente!: Json<Vec<i32>>",
                      COALESCE(r.aparece_en_todo, FALSE) AS "aparece_en_todo!",
                      r.sample_id,
                      r.sample_fuente_id,
                      r.sample_destino_id,
                      COALESCE(r.votos_total, 0) AS "votos_total!",
                      COALESCE(r.votos_promedio, 0)::double precision AS "votos_promedio!",
                      r.fuente AS "fuente!",
                      r.contribuidor_id,
                      u.username AS "contribuidor_username?: String",
                      COALESCE(r.verificada, FALSE) AS "verificada!",
                      COALESCE(r.total_likes, 0) AS "total_likes!",
                      COALESCE(r.total_comentarios, 0) AS "total_comentarios!",
                      r.created_at AS "created_at!",
                      r.updated_at AS "updated_at!",
                      cf.titulo AS "cancion_titulo!",
                      cf.slug AS "cancion_slug!",
                      af.nombre AS "artista_nombre!",
                      af.slug AS "artista_slug!",
                      cf.anio AS cancion_anio,
                      cf.imagen_url AS cancion_imagen_url
               FROM relaciones_sample r
               INNER JOIN canciones cf ON cf.id = r.cancion_fuente_id
               INNER JOIN artistas_musicales af ON af.id = cf.artista_id
               LEFT JOIN usuarios_ext u ON u.id = r.contribuidor_id
               WHERE r.cancion_destino_id = ANY($1)
               ORDER BY COALESCE(r.votos_total, 0) DESC, r.id DESC
               LIMIT $2"#,
            song_ids,
            limit,
        )
        .fetch_all(pool)
        .await?;

        rows.into_iter().map(map_relation_summary).collect()
    }

    pub async fn list_artist_genres(
        pool: &PgPool,
        artist_id: i32,
        limit: i64,
    ) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query!(
            r#"SELECT genero AS "genero!"
               FROM canciones
               WHERE artista_id = $1
                 AND genero IS NOT NULL
                 AND genero <> ''
               GROUP BY genero
               ORDER BY COUNT(*) DESC, genero ASC
               LIMIT $2"#,
            artist_id,
            limit,
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|row| row.genero).collect())
    }
}
