use sqlx::types::Json;
use sqlx::PgPool;

use super::support::{
    map_relation_detail, parse_relation_type, RelationTypeCountRecord, SampleRelationDetailRecord,
};
use super::MusicRepository;
use crate::errors::AppError;
use crate::models::{SampleRelationDetail, SampleRelationType};

impl MusicRepository {
    pub async fn find_relation_by_id(
        pool: &PgPool,
        relation_id: i32,
    ) -> Result<Option<SampleRelationDetail>, AppError> {
        let row = sqlx::query_as!(
            SampleRelationDetailRecord,
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
                      (
                        SELECT COUNT(DISTINCT s.id)
                        FROM samples s
                        WHERE s.eliminado_en IS NULL
                          AND s.estado = 'activo'
                          AND (
                                s.relacion_sampleo_id = r.id
                             OR s.id = r.sample_id
                             OR s.id = r.sample_fuente_id
                             OR s.id = r.sample_destino_id
                          )
                      ) AS "total_samples!",
                      r.created_at AS "created_at!",
                      r.updated_at AS "updated_at!",
                      cf.titulo AS "fuente_titulo!",
                      cf.slug AS "fuente_slug!",
                      cf.anio AS fuente_anio,
                      cf.imagen_url AS fuente_imagen_url,
                      cf.youtube_id AS fuente_youtube_id,
                      cf.spotify_id AS fuente_spotify_id,
                      cf.album AS fuente_album,
                      cf.genero AS fuente_genero,
                      af.nombre AS "fuente_artista!",
                      af.slug AS "fuente_artista_slug!",
                      cd.titulo AS "destino_titulo!",
                      cd.slug AS "destino_slug!",
                      cd.anio AS destino_anio,
                      cd.imagen_url AS destino_imagen_url,
                      cd.youtube_id AS destino_youtube_id,
                      cd.spotify_id AS destino_spotify_id,
                      cd.album AS destino_album,
                      cd.genero AS destino_genero,
                      ad.nombre AS "destino_artista!",
                      ad.slug AS "destino_artista_slug!"
               FROM relaciones_sample r
               INNER JOIN canciones cf ON cf.id = r.cancion_fuente_id
               INNER JOIN artistas_musicales af ON af.id = cf.artista_id
               INNER JOIN canciones cd ON cd.id = r.cancion_destino_id
               INNER JOIN artistas_musicales ad ON ad.id = cd.artista_id
               LEFT JOIN usuarios_ext u ON u.id = r.contribuidor_id
               WHERE r.id = $1
               LIMIT 1"#,
            relation_id,
        )
        .fetch_optional(pool)
        .await?;

        row.map(map_relation_detail).transpose()
    }

    pub async fn find_relation_by_sample_id(
        pool: &PgPool,
        sample_id: i32,
    ) -> Result<Option<SampleRelationDetail>, AppError> {
        let row = sqlx::query_as!(
            SampleRelationDetailRecord,
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
                      (
                        SELECT COUNT(DISTINCT s.id)
                        FROM samples s
                        WHERE s.eliminado_en IS NULL
                          AND s.estado = 'activo'
                          AND (
                                s.relacion_sampleo_id = r.id
                             OR s.id = r.sample_id
                             OR s.id = r.sample_fuente_id
                             OR s.id = r.sample_destino_id
                          )
                      ) AS "total_samples!",
                      r.created_at AS "created_at!",
                      r.updated_at AS "updated_at!",
                      cf.titulo AS "fuente_titulo!",
                      cf.slug AS "fuente_slug!",
                      cf.anio AS fuente_anio,
                      cf.imagen_url AS fuente_imagen_url,
                      cf.youtube_id AS fuente_youtube_id,
                      cf.spotify_id AS fuente_spotify_id,
                      cf.album AS fuente_album,
                      cf.genero AS fuente_genero,
                      af.nombre AS "fuente_artista!",
                      af.slug AS "fuente_artista_slug!",
                      cd.titulo AS "destino_titulo!",
                      cd.slug AS "destino_slug!",
                      cd.anio AS destino_anio,
                      cd.imagen_url AS destino_imagen_url,
                      cd.youtube_id AS destino_youtube_id,
                      cd.spotify_id AS destino_spotify_id,
                      cd.album AS destino_album,
                      cd.genero AS destino_genero,
                      ad.nombre AS "destino_artista!",
                      ad.slug AS "destino_artista_slug!"
               FROM relaciones_sample r
               INNER JOIN canciones cf ON cf.id = r.cancion_fuente_id
               INNER JOIN artistas_musicales af ON af.id = cf.artista_id
               INNER JOIN canciones cd ON cd.id = r.cancion_destino_id
               INNER JOIN artistas_musicales ad ON ad.id = cd.artista_id
               LEFT JOIN usuarios_ext u ON u.id = r.contribuidor_id
               WHERE r.sample_id = $1
                  OR r.sample_fuente_id = $1
                  OR r.sample_destino_id = $1
                  OR EXISTS (
                        SELECT 1
                        FROM samples s
                        WHERE s.id = $1
                          AND s.relacion_sampleo_id = r.id
                  )
               LIMIT 1"#,
            sample_id,
        )
        .fetch_optional(pool)
        .await?;

        row.map(map_relation_detail).transpose()
    }

    pub async fn relation_type_counts(
        pool: &PgPool,
    ) -> Result<Vec<(SampleRelationType, i64)>, AppError> {
        let rows = sqlx::query_as!(
            RelationTypeCountRecord,
            r#"SELECT tipo_relacion AS "tipo_relacion!", COUNT(*)::bigint AS "total!"
               FROM relaciones_sample
               GROUP BY tipo_relacion
             ORDER BY COUNT(*) DESC, tipo_relacion ASC"#,
        )
        .fetch_all(pool)
        .await?;

        rows.into_iter()
            .map(|row| Ok((parse_relation_type(&row.tipo_relacion)?, row.total)))
            .collect()
    }
}
