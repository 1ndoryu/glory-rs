use std::collections::HashSet;

use sqlx::types::Json;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::{
    MusicArtistRole, RelationChainNode, SampleRelationDetail, SampleRelationElementType,
    SampleRelationSource, SampleRelationSummary, SampleRelationType, SongArtistInput,
    SongArtistLink,
};

#[derive(Debug)]
pub(super) struct SongArtistLinkRecord {
    pub(super) artista_id: i32,
    pub(super) nombre: String,
    pub(super) slug: String,
    pub(super) rol: String,
}

#[derive(Debug)]
pub(super) struct SampleRelationSummaryRecord {
    pub(super) id: i32,
    pub(super) cancion_destino_id: i32,
    pub(super) cancion_fuente_id: i32,
    pub(super) whosampled_id: Option<i32>,
    pub(super) tipo_relacion: String,
    pub(super) tipo_elemento: Option<String>,
    pub(super) timings_destino: Json<Vec<i32>>,
    pub(super) timings_fuente: Json<Vec<i32>>,
    pub(super) aparece_en_todo: bool,
    pub(super) sample_id: Option<i32>,
    pub(super) sample_fuente_id: Option<i32>,
    pub(super) sample_destino_id: Option<i32>,
    pub(super) votos_total: i32,
    pub(super) votos_promedio: f64,
    pub(super) fuente: String,
    pub(super) contribuidor_id: Option<i32>,
    pub(super) contribuidor_username: Option<String>,
    pub(super) verificada: bool,
    pub(super) total_likes: i32,
    pub(super) total_comentarios: i32,
    pub(super) created_at: chrono::DateTime<chrono::Utc>,
    pub(super) updated_at: chrono::DateTime<chrono::Utc>,
    pub(super) cancion_titulo: String,
    pub(super) cancion_slug: String,
    pub(super) artista_nombre: String,
    pub(super) artista_slug: String,
    pub(super) cancion_anio: Option<i16>,
    pub(super) cancion_imagen_url: Option<String>,
}

#[derive(Debug)]
pub(super) struct SampleRelationDetailRecord {
    pub(super) id: i32,
    pub(super) cancion_destino_id: i32,
    pub(super) cancion_fuente_id: i32,
    pub(super) whosampled_id: Option<i32>,
    pub(super) tipo_relacion: String,
    pub(super) tipo_elemento: Option<String>,
    pub(super) timings_destino: Json<Vec<i32>>,
    pub(super) timings_fuente: Json<Vec<i32>>,
    pub(super) aparece_en_todo: bool,
    pub(super) sample_id: Option<i32>,
    pub(super) sample_fuente_id: Option<i32>,
    pub(super) sample_destino_id: Option<i32>,
    pub(super) votos_total: i32,
    pub(super) votos_promedio: f64,
    pub(super) fuente: String,
    pub(super) contribuidor_id: Option<i32>,
    pub(super) contribuidor_username: Option<String>,
    pub(super) verificada: bool,
    pub(super) total_likes: i32,
    pub(super) total_comentarios: i32,
    pub(super) total_samples: i64,
    pub(super) created_at: chrono::DateTime<chrono::Utc>,
    pub(super) updated_at: chrono::DateTime<chrono::Utc>,
    pub(super) fuente_titulo: String,
    pub(super) fuente_slug: String,
    pub(super) fuente_anio: Option<i16>,
    pub(super) fuente_imagen_url: Option<String>,
    pub(super) fuente_youtube_id: Option<String>,
    pub(super) fuente_spotify_id: Option<String>,
    pub(super) fuente_album: Option<String>,
    pub(super) fuente_genero: Option<String>,
    pub(super) fuente_artista: String,
    pub(super) fuente_artista_slug: String,
    pub(super) destino_titulo: String,
    pub(super) destino_slug: String,
    pub(super) destino_anio: Option<i16>,
    pub(super) destino_imagen_url: Option<String>,
    pub(super) destino_youtube_id: Option<String>,
    pub(super) destino_spotify_id: Option<String>,
    pub(super) destino_album: Option<String>,
    pub(super) destino_genero: Option<String>,
    pub(super) destino_artista: String,
    pub(super) destino_artista_slug: String,
}

#[derive(Debug)]
pub(super) struct RelationTypeCountRecord {
    pub(super) tipo_relacion: String,
    pub(super) total: i64,
}

#[derive(Debug)]
pub(super) struct RelationChainNodeRecord {
    pub(super) id: i32,
    pub(super) cancion_fuente_id: i32,
    pub(super) cancion_destino_id: i32,
    pub(super) tipo_relacion: String,
    pub(super) nivel: i32,
    pub(super) fuente_titulo: String,
    pub(super) fuente_slug: String,
    pub(super) fuente_artista: String,
    pub(super) destino_titulo: String,
    pub(super) destino_slug: String,
    pub(super) destino_artista: String,
}

#[derive(Debug)]
pub(super) struct SampleOwnerRecord {
    pub(super) creador_id: i32,
    pub(super) estado: String,
    pub(super) relacion_sampleo_id: Option<i32>,
    pub(super) cancion_origen_id: Option<i32>,
}

#[derive(Debug)]
pub(super) struct RelationLinkContextRecord {
    pub(super) cancion_fuente_id: i32,
    pub(super) cancion_destino_id: i32,
    pub(super) sample_fuente_id: Option<i32>,
    pub(super) sample_destino_id: Option<i32>,
}

pub(super) fn collect_artist_ids(main_artist_id: i32, artists: &[SongArtistInput]) -> Vec<i32> {
    let mut ids = Vec::with_capacity(artists.len() + 1);
    ids.push(main_artist_id);
    ids.extend(artists.iter().map(|artist| artist.artista_id));
    dedupe_i32(&ids)
}

pub(super) fn normalize_song_artists(
    main_artist_id: i32,
    requested: Option<&[SongArtistInput]>,
    existing: &[(i32, MusicArtistRole)],
) -> Result<Vec<(i32, MusicArtistRole)>, AppError> {
    let mut normalized = Vec::new();
    let mut seen = HashSet::new();

    normalized.push((main_artist_id, MusicArtistRole::Principal));
    seen.insert((main_artist_id, artist_role_db(MusicArtistRole::Principal)));

    match requested {
        Some(items) => {
            for item in items {
                if item.rol == MusicArtistRole::Principal && item.artista_id != main_artist_id {
                    continue;
                }
                let role_db = artist_role_db(item.rol);
                if seen.insert((item.artista_id, role_db)) {
                    normalized.push((item.artista_id, item.rol));
                }
            }
        }
        None => {
            for (artist_id, role) in existing {
                if *role == MusicArtistRole::Principal {
                    continue;
                }
                let role_db = artist_role_db(*role);
                if seen.insert((*artist_id, role_db)) {
                    normalized.push((*artist_id, *role));
                }
            }
        }
    }

    if normalized.is_empty() {
        return Err(AppError::Validation(
            "La cancion debe tener al menos un artista principal".into(),
        ));
    }

    Ok(normalized)
}

pub(super) fn map_song_artist_link(row: SongArtistLinkRecord) -> Result<SongArtistLink, AppError> {
    Ok(SongArtistLink {
        artista_id: row.artista_id,
        nombre: row.nombre,
        slug: row.slug,
        rol: parse_artist_role(&row.rol)?,
    })
}

pub(super) fn map_relation_summary(
    record: SampleRelationSummaryRecord,
) -> Result<SampleRelationSummary, AppError> {
    Ok(SampleRelationSummary {
        id: record.id,
        cancion_destino_id: record.cancion_destino_id,
        cancion_fuente_id: record.cancion_fuente_id,
        whosampled_id: record.whosampled_id,
        tipo_relacion: parse_relation_type(&record.tipo_relacion)?,
        tipo_elemento: parse_relation_element(record.tipo_elemento.as_deref())?,
        timings_destino: record.timings_destino.0,
        timings_fuente: record.timings_fuente.0,
        aparece_en_todo: record.aparece_en_todo,
        sample_id: record.sample_id,
        sample_fuente_id: record.sample_fuente_id,
        sample_destino_id: record.sample_destino_id,
        votos_total: record.votos_total,
        votos_promedio: record.votos_promedio,
        fuente: parse_relation_source(&record.fuente)?,
        contribuidor_id: record.contribuidor_id,
        contribuidor_username: record.contribuidor_username,
        verificada: record.verificada,
        total_likes: record.total_likes,
        total_comentarios: record.total_comentarios,
        created_at: record.created_at,
        updated_at: record.updated_at,
        cancion_titulo: record.cancion_titulo,
        cancion_slug: record.cancion_slug,
        artista_nombre: record.artista_nombre,
        artista_slug: record.artista_slug,
        cancion_anio: record.cancion_anio,
        cancion_imagen_url: record.cancion_imagen_url,
    })
}

pub(super) fn map_relation_detail(
    record: SampleRelationDetailRecord,
) -> Result<SampleRelationDetail, AppError> {
    Ok(SampleRelationDetail {
        id: record.id,
        cancion_destino_id: record.cancion_destino_id,
        cancion_fuente_id: record.cancion_fuente_id,
        whosampled_id: record.whosampled_id,
        tipo_relacion: parse_relation_type(&record.tipo_relacion)?,
        tipo_elemento: parse_relation_element(record.tipo_elemento.as_deref())?,
        timings_destino: record.timings_destino.0,
        timings_fuente: record.timings_fuente.0,
        aparece_en_todo: record.aparece_en_todo,
        sample_id: record.sample_id,
        sample_fuente_id: record.sample_fuente_id,
        sample_destino_id: record.sample_destino_id,
        votos_total: record.votos_total,
        votos_promedio: record.votos_promedio,
        fuente: parse_relation_source(&record.fuente)?,
        contribuidor_id: record.contribuidor_id,
        contribuidor_username: record.contribuidor_username,
        verificada: record.verificada,
        total_likes: record.total_likes,
        total_comentarios: record.total_comentarios,
        total_samples: record.total_samples,
        created_at: record.created_at,
        updated_at: record.updated_at,
        fuente_titulo: record.fuente_titulo,
        fuente_slug: record.fuente_slug,
        fuente_anio: record.fuente_anio,
        fuente_imagen_url: record.fuente_imagen_url,
        fuente_youtube_id: record.fuente_youtube_id,
        fuente_spotify_id: record.fuente_spotify_id,
        fuente_album: record.fuente_album,
        fuente_genero: record.fuente_genero,
        fuente_artista: record.fuente_artista,
        fuente_artista_slug: record.fuente_artista_slug,
        destino_titulo: record.destino_titulo,
        destino_slug: record.destino_slug,
        destino_anio: record.destino_anio,
        destino_imagen_url: record.destino_imagen_url,
        destino_youtube_id: record.destino_youtube_id,
        destino_spotify_id: record.destino_spotify_id,
        destino_album: record.destino_album,
        destino_genero: record.destino_genero,
        destino_artista: record.destino_artista,
        destino_artista_slug: record.destino_artista_slug,
        destino_samples_de: None,
        destino_sampleada_en: None,
        fuente_samples_de: None,
        fuente_sampleada_en: None,
        lado_extraccion: None,
    })
}

pub(super) fn map_relation_chain_node(
    record: RelationChainNodeRecord,
) -> Result<RelationChainNode, AppError> {
    Ok(RelationChainNode {
        id: record.id,
        cancion_fuente_id: record.cancion_fuente_id,
        cancion_destino_id: record.cancion_destino_id,
        tipo_relacion: parse_relation_type(&record.tipo_relacion)?,
        nivel: record.nivel,
        fuente_titulo: record.fuente_titulo,
        fuente_slug: record.fuente_slug,
        fuente_artista: record.fuente_artista,
        destino_titulo: record.destino_titulo,
        destino_slug: record.destino_slug,
        destino_artista: record.destino_artista,
    })
}

pub(super) fn parse_artist_role(value: &str) -> Result<MusicArtistRole, AppError> {
    match value {
        "principal" => Ok(MusicArtistRole::Principal),
        "featuring" => Ok(MusicArtistRole::Featuring),
        "producer" => Ok(MusicArtistRole::Producer),
        other => Err(AppError::Internal(format!(
            "rol de artista no soportado: {other}"
        ))),
    }
}

pub(super) fn parse_relation_type(value: &str) -> Result<SampleRelationType, AppError> {
    match value {
        "sample" => Ok(SampleRelationType::Sample),
        "cover" => Ok(SampleRelationType::Cover),
        "remix" => Ok(SampleRelationType::Remix),
        "interpolation" => Ok(SampleRelationType::Interpolation),
        other => Err(AppError::Internal(format!(
            "tipo de relacion no soportado: {other}"
        ))),
    }
}

pub(super) fn parse_relation_element(
    value: Option<&str>,
) -> Result<Option<SampleRelationElementType>, AppError> {
    match value {
        None => Ok(None),
        Some("hook_riff") => Ok(Some(SampleRelationElementType::HookRiff)),
        Some("vocals_lyrics") => Ok(Some(SampleRelationElementType::VocalsLyrics)),
        Some("drums") => Ok(Some(SampleRelationElementType::Drums)),
        Some("bass") => Ok(Some(SampleRelationElementType::Bass)),
        Some("keys_synth") => Ok(Some(SampleRelationElementType::KeysSynth)),
        Some("sound_effect") => Ok(Some(SampleRelationElementType::SoundEffect)),
        Some("multiple_elements") => Ok(Some(SampleRelationElementType::MultipleElements)),
        Some("other") => Ok(Some(SampleRelationElementType::Other)),
        Some(other) => Err(AppError::Internal(format!(
            "tipo de elemento no soportado: {other}"
        ))),
    }
}

pub(super) fn parse_relation_source(value: &str) -> Result<SampleRelationSource, AppError> {
    match value {
        "scraping" => Ok(SampleRelationSource::Scraping),
        "comunidad" => Ok(SampleRelationSource::Comunidad),
        "musicbrainz" => Ok(SampleRelationSource::Musicbrainz),
        "import" => Ok(SampleRelationSource::Import),
        other => Err(AppError::Internal(format!(
            "fuente de relacion no soportada: {other}"
        ))),
    }
}

pub(super) fn artist_role_db(value: MusicArtistRole) -> &'static str {
    match value {
        MusicArtistRole::Principal => "principal",
        MusicArtistRole::Featuring => "featuring",
        MusicArtistRole::Producer => "producer",
    }
}

pub(super) fn relation_type_db(value: SampleRelationType) -> &'static str {
    match value {
        SampleRelationType::Sample => "sample",
        SampleRelationType::Cover => "cover",
        SampleRelationType::Remix => "remix",
        SampleRelationType::Interpolation => "interpolation",
    }
}

pub(super) fn relation_element_db(value: SampleRelationElementType) -> &'static str {
    match value {
        SampleRelationElementType::HookRiff => "hook_riff",
        SampleRelationElementType::VocalsLyrics => "vocals_lyrics",
        SampleRelationElementType::Drums => "drums",
        SampleRelationElementType::Bass => "bass",
        SampleRelationElementType::KeysSynth => "keys_synth",
        SampleRelationElementType::SoundEffect => "sound_effect",
        SampleRelationElementType::MultipleElements => "multiple_elements",
        SampleRelationElementType::Other => "other",
    }
}

pub(super) fn relation_source_db(value: SampleRelationSource) -> &'static str {
    match value {
        SampleRelationSource::Scraping => "scraping",
        SampleRelationSource::Comunidad => "comunidad",
        SampleRelationSource::Musicbrainz => "musicbrainz",
        SampleRelationSource::Import => "import",
    }
}

pub(super) fn to_i16(value: Option<i32>, field: &str) -> Result<Option<i16>, AppError> {
    value
        .map(|item| {
            i16::try_from(item).map_err(|_| {
                AppError::Validation(format!("{field} esta fuera del rango permitido"))
            })
        })
        .transpose()
}

pub(super) fn serialize_json_option(
    values: Option<&Vec<i32>>,
) -> Result<Option<serde_json::Value>, AppError> {
    values
        .map(|list| {
            serde_json::to_value(list)
                .map_err(|error| AppError::Internal(format!("serializar json: {error}")))
        })
        .transpose()
}

pub(super) fn dedupe_i32(values: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut result = Vec::with_capacity(values.len());
    for value in values {
        if seen.insert(*value) {
            result.push(*value);
        }
    }
    result
}

pub(super) async fn generate_unique_slug(
    pool: &PgPool,
    raw: &str,
    exclude_id: Option<i32>,
    fallback: &str,
    artist: bool,
) -> Result<String, AppError> {
    let mut base_slug = slug::slugify(raw);
    if base_slug.is_empty() {
        base_slug = fallback.to_string();
    }

    let max_len = if artist { 350 } else { 550 };
    if base_slug.len() > max_len - 8 {
        base_slug.truncate(max_len - 8);
        base_slug = base_slug.trim_matches('-').to_string();
    }
    if base_slug.is_empty() {
        base_slug = fallback.to_string();
    }

    for counter in 0..100 {
        let candidate = if counter == 0 {
            base_slug.clone()
        } else {
            format!("{base_slug}-{counter}")
        };

        let exists = if artist {
            sqlx::query_scalar!(
                r#"SELECT COUNT(*) AS "count!"
                   FROM artistas_musicales
                   WHERE slug = $1
                     AND ($2::int IS NULL OR id <> $2)"#,
                candidate.as_str(),
                exclude_id,
            )
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_scalar!(
                r#"SELECT COUNT(*) AS "count!"
                   FROM canciones
                   WHERE slug = $1
                     AND ($2::int IS NULL OR id <> $2)"#,
                candidate.as_str(),
                exclude_id,
            )
            .fetch_one(pool)
            .await?
        };

        if exists == 0 {
            return Ok(candidate);
        }
    }

    Ok(format!("{}-{}", base_slug, uuid::Uuid::new_v4().simple()))
}