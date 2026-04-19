use crate::errors::AppError;
use crate::models::{
    ArtistDetailResponse, ArtistStats, MusicArtistRole, SampleRelationDetail,
    SampleRelationSummary, SongArtistInput, SongDetailResponse,
};
use crate::repositories::MusicRepository;
use crate::AppState;

pub(super) const DEFAULT_PAGE: i64 = 1;
pub(super) const DEFAULT_PER_PAGE: i64 = 20;
pub(super) const DEFAULT_LIMIT: i64 = 20;
pub(super) const DEFAULT_RELATION_LIMIT: i64 = 20;
pub(super) const DEFAULT_ARTIST_RELATION_LIMIT: i64 = 100;
pub(super) const DEFAULT_CHAIN_DEPTH: i32 = 5;

pub(super) async fn fetch_song_detail_by_slug(
    state: &AppState,
    slug: &str,
) -> Result<SongDetailResponse, AppError> {
    let song = MusicRepository::find_song_by_slug(&state.pool, slug)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("cancion {slug}")))?;
    fetch_song_detail_by_id(state, song.id).await
}

pub(super) async fn fetch_song_detail_by_id(
    state: &AppState,
    song_id: i32,
) -> Result<SongDetailResponse, AppError> {
    let cancion = MusicRepository::find_song_by_id(&state.pool, song_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("cancion {song_id}")))?;
    let artistas = MusicRepository::list_song_artists(&state.pool, song_id).await?;
    let samples_de = MusicRepository::list_song_relations_as_destino(
        &state.pool,
        song_id,
        DEFAULT_RELATION_LIMIT,
    )
    .await?;
    let sampleada_en = MusicRepository::list_song_relations_as_fuente(
        &state.pool,
        song_id,
        DEFAULT_RELATION_LIMIT,
    )
    .await?;

    Ok(SongDetailResponse {
        cancion,
        artistas,
        samples_de,
        sampleada_en,
    })
}

pub(super) async fn fetch_artist_detail_by_slug(
    state: &AppState,
    slug: &str,
) -> Result<ArtistDetailResponse, AppError> {
    let artist = MusicRepository::find_artist_by_slug(&state.pool, slug)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("artista {slug}")))?;
    fetch_artist_detail_by_id(state, artist.id).await
}

pub(super) async fn fetch_artist_detail_by_id(
    state: &AppState,
    artist_id: i32,
) -> Result<ArtistDetailResponse, AppError> {
    let artista = MusicRepository::find_artist_by_id(&state.pool, artist_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("artista {artist_id}")))?;
    let canciones = MusicRepository::list_artist_songs(&state.pool, artist_id).await?;
    let song_ids = canciones.iter().map(|song| song.id).collect::<Vec<_>>();
    let sampleado_por = MusicRepository::list_artist_relations_as_fuente(
        &state.pool,
        &song_ids,
        DEFAULT_ARTIST_RELATION_LIMIT,
    )
    .await?;
    let samplea_a = MusicRepository::list_artist_relations_as_destino(
        &state.pool,
        &song_ids,
        DEFAULT_ARTIST_RELATION_LIMIT,
    )
    .await?;
    let generos = MusicRepository::list_artist_genres(&state.pool, artist_id, 5).await?;

    Ok(ArtistDetailResponse {
        artista,
        canciones,
        sampleado_por: sampleado_por.clone(),
        samplea_a: samplea_a.clone(),
        estadisticas: ArtistStats {
            total_sampleado_por: sampleado_por.len(),
            total_samplea_a: samplea_a.len(),
            generos,
        },
    })
}

pub(super) async fn fetch_relation_detail(
    state: &AppState,
    relation_id: i32,
) -> Result<SampleRelationDetail, AppError> {
    let mut detail = MusicRepository::find_relation_by_id(&state.pool, relation_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("relacion {relation_id}")))?;

    let destino_samples_de = MusicRepository::list_song_relations_as_destino(
        &state.pool,
        detail.cancion_destino_id,
        DEFAULT_RELATION_LIMIT,
    )
    .await?;
    let destino_sampleada_en = MusicRepository::list_song_relations_as_fuente(
        &state.pool,
        detail.cancion_destino_id,
        DEFAULT_RELATION_LIMIT,
    )
    .await?;
    let fuente_samples_de = MusicRepository::list_song_relations_as_destino(
        &state.pool,
        detail.cancion_fuente_id,
        DEFAULT_RELATION_LIMIT,
    )
    .await?;
    let fuente_sampleada_en = MusicRepository::list_song_relations_as_fuente(
        &state.pool,
        detail.cancion_fuente_id,
        DEFAULT_RELATION_LIMIT,
    )
    .await?;

    detail.destino_samples_de = Some(without_relation(destino_samples_de, relation_id));
    detail.destino_sampleada_en = Some(without_relation(destino_sampleada_en, relation_id));
    detail.fuente_samples_de = Some(without_relation(fuente_samples_de, relation_id));
    detail.fuente_sampleada_en = Some(without_relation(fuente_sampleada_en, relation_id));

    Ok(detail)
}

pub(super) fn without_relation(
    relations: Vec<SampleRelationSummary>,
    relation_id: i32,
) -> Vec<SampleRelationSummary> {
    relations
        .into_iter()
        .filter(|relation| relation.id != relation_id)
        .collect()
}

pub(super) fn resolve_song_main_artist_id(
    explicit_artist_id: Option<i32>,
    requested_artists: Option<&[SongArtistInput]>,
    fallback_artist_id: i32,
) -> i32 {
    explicit_artist_id
        .or_else(|| {
            requested_artists.and_then(|artists| {
                artists
                    .iter()
                    .find(|artist| artist.rol == MusicArtistRole::Principal)
                    .map(|artist| artist.artista_id)
            })
        })
        .unwrap_or(fallback_artist_id)
}