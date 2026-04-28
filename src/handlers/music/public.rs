use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::support::{
    fetch_artist_detail_by_slug, fetch_relation_detail, fetch_song_detail_by_slug,
    DEFAULT_CHAIN_DEPTH, DEFAULT_LIMIT, DEFAULT_PAGE, DEFAULT_PER_PAGE,
};
use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::models::{
    ArtistDetailResponse, LimitQuery, ListSongsQuery, MusicArtist, MusicArtistsResponse,
    MusicPagination, MusicSong, MusicSongsResponse, RelationChainQuery, RelationChainResponse,
    RelationSampleSide, RelationStatsResponse, RelationTypeCount, SampleRelationDetail,
    SampleRelationLookupResponse, SampleSummary, SearchSongsQuery, SongDetailResponse,
    SongListResponse,
};
use crate::repositories::{MusicRepository, SampleRepository};
use crate::services::build_sample_summary;
use crate::AppState;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct SongSectionsQuery {
    pub por_seccion: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SongSection {
    pub tipo: String,
    pub titulo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genero: Option<String>,
    pub canciones: Vec<MusicSong>,
    pub artistas: Vec<MusicArtist>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SongSectionsResponse {
    pub data: Vec<SongSection>,
}

#[utoipa::path(
    get,
    path = "/api/canciones/secciones",
    tag = "music",
    params(SongSectionsQuery),
    responses((status = 200, description = "Secciones legacy de canciones", body = SongSectionsResponse))
)]
pub async fn song_sections(
    State(state): State<AppState>,
    Query(query): Query<SongSectionsQuery>,
) -> Result<Json<SongSectionsResponse>, AppError> {
    let per_section = query.por_seccion.unwrap_or(DEFAULT_LIMIT).clamp(1, 30);
    let top = MusicRepository::top_songs(&state.pool, per_section).await?;
    let recent = MusicRepository::list_songs(&state.pool, per_section * 3, 0).await?;
    let artists = MusicRepository::top_artists(&state.pool, per_section).await?;
    let take_count = usize::try_from(per_section).unwrap_or(30);

    let mut sections = vec![
        SongSection {
            tipo: "top".into(),
            titulo: "Más sampleadas".into(),
            genero: None,
            canciones: top,
            artistas: Vec::new(),
        },
        SongSection {
            tipo: "recientes".into(),
            titulo: "Descubrimientos recientes".into(),
            genero: None,
            canciones: recent.iter().take(take_count).cloned().collect(),
            artistas: Vec::new(),
        },
    ];

    for genre in top_genres(&recent, 3) {
        let songs = recent
            .iter()
            .filter(|song| song.genero.as_deref() == Some(genre.as_str()))
            .take(take_count)
            .cloned()
            .collect::<Vec<_>>();
        if !songs.is_empty() {
            sections.push(SongSection {
                tipo: "genero".into(),
                titulo: genre.clone(),
                genero: Some(genre),
                canciones: songs,
                artistas: Vec::new(),
            });
        }
    }

    sections.push(SongSection {
        tipo: "artistas".into(),
        titulo: "Artistas destacados".into(),
        genero: None,
        canciones: Vec::new(),
        artistas: artists,
    });

    Ok(Json(SongSectionsResponse { data: sections }))
}

fn top_genres(songs: &[MusicSong], limit: usize) -> Vec<String> {
    let mut counts = std::collections::HashMap::<String, usize>::new();
    for song in songs {
        if let Some(genre) = song
            .genero
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            *counts.entry(genre.to_owned()).or_default() += 1;
        }
    }
    let mut pairs = counts.into_iter().collect::<Vec<_>>();
    pairs.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    pairs
        .into_iter()
        .take(limit)
        .map(|(genre, _)| genre)
        .collect()
}

#[utoipa::path(
    get,
    path = "/api/canciones",
    tag = "music",
    params(ListSongsQuery),
    responses(
        (status = 200, description = "Listado paginado de canciones", body = SongListResponse),
        (status = 422, description = "Query inválida", body = ErrorResponse)
    )
)]
pub async fn list_songs(
    State(state): State<AppState>,
    Query(query): Query<ListSongsQuery>,
) -> Result<Json<SongListResponse>, AppError> {
    query
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;

    let page = query.page.unwrap_or(DEFAULT_PAGE).max(1);
    let per_page = query.per_page.unwrap_or(DEFAULT_PER_PAGE).clamp(1, 100);
    let offset = (page - 1) * per_page;
    let total = MusicRepository::count_songs(&state.pool).await?;
    let data = MusicRepository::list_songs(&state.pool, per_page, offset).await?;
    let pages = if total == 0 {
        0
    } else {
        (total + per_page - 1) / per_page
    };

    Ok(Json(SongListResponse {
        data,
        pagination: MusicPagination {
            page,
            per_page,
            total,
            pages,
        },
    }))
}

#[utoipa::path(
    get,
    path = "/api/canciones/buscar",
    tag = "music",
    params(SearchSongsQuery),
    responses(
        (status = 200, description = "Búsqueda textual de canciones", body = MusicSongsResponse),
        (status = 422, description = "Query inválida", body = ErrorResponse)
    )
)]
pub async fn search_songs(
    State(state): State<AppState>,
    Query(query): Query<SearchSongsQuery>,
) -> Result<Json<MusicSongsResponse>, AppError> {
    query
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;

    let trimmed = query.q.trim();
    if trimmed.chars().count() < 2 {
        return Ok(Json(MusicSongsResponse { data: Vec::new() }));
    }

    let per_page = query.per_page.unwrap_or(DEFAULT_LIMIT).clamp(1, 100);
    let data =
        MusicRepository::search_songs(&state.pool, trimmed, &format!("%{trimmed}%"), per_page)
            .await?;
    Ok(Json(MusicSongsResponse { data }))
}

#[utoipa::path(
    get,
    path = "/api/canciones/top",
    tag = "music",
    params(LimitQuery),
    responses((status = 200, description = "Canciones más sampleadas", body = MusicSongsResponse))
)]
pub async fn top_songs(
    State(state): State<AppState>,
    Query(query): Query<LimitQuery>,
) -> Result<Json<MusicSongsResponse>, AppError> {
    query
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, 100);
    let data = MusicRepository::top_songs(&state.pool, limit).await?;
    Ok(Json(MusicSongsResponse { data }))
}

#[utoipa::path(
    get,
    path = "/api/canciones/{slug}",
    tag = "music",
    params(("slug" = String, Path, description = "Slug público de la canción")),
    responses(
        (status = 200, description = "Detalle de canción", body = SongDetailResponse),
        (status = 404, description = "Canción no encontrada", body = ErrorResponse)
    )
)]
pub async fn get_song(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<SongDetailResponse>, AppError> {
    Ok(Json(fetch_song_detail_by_slug(&state, slug.trim()).await?))
}

#[utoipa::path(
    get,
    path = "/api/canciones/{slug}/cadena",
    tag = "music",
    params(
        ("slug" = String, Path, description = "Slug público de la canción"),
        RelationChainQuery
    ),
    responses(
        (status = 200, description = "Cadena recursiva de sampleos", body = RelationChainResponse),
        (status = 404, description = "Canción no encontrada", body = ErrorResponse),
        (status = 422, description = "Query inválida", body = ErrorResponse)
    )
)]
pub async fn get_song_chain(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<RelationChainQuery>,
) -> Result<Json<RelationChainResponse>, AppError> {
    query
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;
    let song = MusicRepository::find_song_by_slug(&state.pool, slug.trim())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("cancion {}", slug.trim())))?;
    let depth = query
        .profundidad
        .unwrap_or(DEFAULT_CHAIN_DEPTH)
        .clamp(1, 10);
    let chain = MusicRepository::relation_chain(&state.pool, song.id, depth).await?;
    Ok(Json(RelationChainResponse {
        cancion_raiz: song,
        cadena: chain,
    }))
}

#[utoipa::path(
    get,
    path = "/api/artistas/top",
    tag = "music",
    params(LimitQuery),
    responses((status = 200, description = "Top artistas por catálogo", body = MusicArtistsResponse))
)]
pub async fn top_artists(
    State(state): State<AppState>,
    Query(query): Query<LimitQuery>,
) -> Result<Json<MusicArtistsResponse>, AppError> {
    query
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, 100);
    let data = MusicRepository::top_artists(&state.pool, limit).await?;
    Ok(Json(MusicArtistsResponse { data }))
}

#[utoipa::path(
    get,
    path = "/api/artistas/{slug}",
    tag = "music",
    params(("slug" = String, Path, description = "Slug público del artista")),
    responses(
        (status = 200, description = "Detalle de artista", body = ArtistDetailResponse),
        (status = 404, description = "Artista no encontrado", body = ErrorResponse)
    )
)]
pub async fn get_artist(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ArtistDetailResponse>, AppError> {
    Ok(Json(
        fetch_artist_detail_by_slug(&state, slug.trim()).await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/relaciones/{id}",
    tag = "music",
    params(("id" = i32, Path, description = "ID de la relación")),
    responses(
        (status = 200, description = "Detalle de relación", body = SampleRelationDetail),
        (status = 404, description = "Relación no encontrada", body = ErrorResponse)
    )
)]
pub async fn get_relation(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<SampleRelationDetail>, AppError> {
    Ok(Json(fetch_relation_detail(&state, id).await?))
}

#[utoipa::path(
    get,
    path = "/api/sample-discovery/relacion/{sample_id}",
    tag = "music",
    params(("sample_id" = i32, Path, description = "ID del sample vinculado")),
    responses((status = 200, description = "Relación vinculada a un sample", body = SampleRelationLookupResponse))
)]
pub async fn get_relation_by_sample(
    State(state): State<AppState>,
    Path(sample_id): Path<i32>,
) -> Result<Json<SampleRelationLookupResponse>, AppError> {
    let relation = MusicRepository::find_relation_by_sample_id(&state.pool, sample_id).await?;
    let data = match relation {
        Some(mut detail) => {
            if detail.sample_fuente_id == Some(sample_id) {
                detail.lado_extraccion = Some(RelationSampleSide::Fuente);
            } else if detail.sample_destino_id == Some(sample_id) {
                detail.lado_extraccion = Some(RelationSampleSide::Destino);
            }
            let mut enriched = fetch_relation_detail(&state, detail.id).await?;
            enriched.lado_extraccion = detail.lado_extraccion;
            Some(enriched)
        }
        None => None,
    };
    Ok(Json(SampleRelationLookupResponse { data }))
}

#[utoipa::path(
    get,
    path = "/api/sample-discovery/estadisticas",
    tag = "music",
    responses((status = 200, description = "Conteo agregado por tipo de relación", body = RelationStatsResponse))
)]
pub async fn relation_stats(
    State(state): State<AppState>,
) -> Result<Json<RelationStatsResponse>, AppError> {
    let relaciones_por_tipo = MusicRepository::relation_type_counts(&state.pool)
        .await?
        .into_iter()
        .map(|(tipo_relacion, total)| RelationTypeCount {
            tipo_relacion,
            total,
        })
        .collect();
    Ok(Json(RelationStatsResponse {
        relaciones_por_tipo,
    }))
}

/* [274A-7] GET /canciones/:slug/samples — samples de audio vinculados a una canción.
 * La canción se resuelve por slug; los samples se filtran por cancion_origen_id.
 * Devuelve array vacío si la canción no existe o no tiene samples activos. */
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SongSamplesResponse {
    pub data: Vec<SampleSummary>,
}

#[utoipa::path(
    get,
    path = "/api/canciones/{slug}/samples",
    tag = "music",
    params(("slug" = String, Path, description = "Slug de la canción")),
    responses(
        (status = 200, description = "Samples activos de la canción", body = SongSamplesResponse),
        (status = 404, description = "Canción no encontrada", body = ErrorResponse)
    )
)]
pub async fn get_song_samples(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<SongSamplesResponse>, AppError> {
    let cancion = MusicRepository::find_song_by_slug(&state.pool, &slug)
        .await?
        .ok_or_else(|| AppError::NotFound("Canción no encontrada".into()))?;

    let records = SampleRepository::list_by_cancion_id(&state.pool, cancion.id, None)
        .await
        .map_err(AppError::from)?;

    let data = records
        .into_iter()
        .map(|r| build_sample_summary(r, state.public_base_url.as_deref()))
        .collect();

    Ok(Json(SongSamplesResponse { data }))
}
