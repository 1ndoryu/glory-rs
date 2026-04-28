use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use validator::Validate;

use super::support::{fetch_relation_detail, fetch_song_detail_by_id, resolve_song_main_artist_id};
use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::middleware::CurrentUser;
use crate::models::{
    CreateArtistRequest, CreateRelationRequest, CreateSongRequest, MusicArtist,
    MusicMutationResponse, SampleRelationDetail, SongDetailResponse, UpdateArtistRequest,
    UpdateRelationRequest, UpdateSongRequest,
};
use crate::repositories::MusicRepository;
use crate::AppState;

#[utoipa::path(
    post,
    path = "/api/admin/artistas",
    tag = "music",
    request_body = CreateArtistRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Artista creado", body = MusicArtist),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 422, description = "Payload inválido", body = ErrorResponse)
    )
)]
pub async fn create_artist(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(request): Json<CreateArtistRequest>,
) -> Result<(StatusCode, Json<MusicArtist>), AppError> {
    user.require_admin()?;
    request
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;

    let artist_id = MusicRepository::create_artist(&state.pool, &request).await?;
    let artist = MusicRepository::find_artist_by_id(&state.pool, artist_id)
        .await?
        .ok_or_else(|| {
            AppError::Internal(format!("artista {artist_id} recien creado no visible"))
        })?;
    Ok((StatusCode::CREATED, Json(artist)))
}

#[utoipa::path(
    put,
    path = "/api/admin/artistas/{id}",
    tag = "music",
    params(("id" = i32, Path, description = "ID del artista")),
    request_body = UpdateArtistRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Artista actualizado", body = MusicArtist),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 404, description = "Artista no encontrado", body = ErrorResponse),
        (status = 422, description = "Payload inválido", body = ErrorResponse)
    )
)]
pub async fn update_artist(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
    Json(request): Json<UpdateArtistRequest>,
) -> Result<Json<MusicArtist>, AppError> {
    user.require_admin()?;
    request
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;

    let updated = MusicRepository::update_artist(&state.pool, id, &request).await?;
    if !updated {
        return Err(AppError::NotFound(format!("artista {id}")));
    }

    let artist = MusicRepository::find_artist_by_id(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("artista {id}")))?;
    Ok(Json(artist))
}

#[utoipa::path(
    delete,
    path = "/api/admin/artistas/{id}",
    tag = "music",
    params(("id" = i32, Path, description = "ID del artista")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Artista eliminado", body = MusicMutationResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 404, description = "Artista no encontrado", body = ErrorResponse),
        (status = 409, description = "El artista sigue referenciado por canciones", body = ErrorResponse)
    )
)]
pub async fn delete_artist(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
) -> Result<Json<MusicMutationResponse>, AppError> {
    user.require_admin()?;
    let deleted = MusicRepository::delete_artist(&state.pool, id).await?;
    if !deleted {
        return Err(AppError::NotFound(format!("artista {id}")));
    }
    Ok(Json(MusicMutationResponse { ok: true }))
}

#[utoipa::path(
    post,
    path = "/api/admin/canciones",
    tag = "music",
    request_body = CreateSongRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Canción creada", body = SongDetailResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 422, description = "Payload inválido", body = ErrorResponse)
    )
)]
pub async fn create_song(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(request): Json<CreateSongRequest>,
) -> Result<(StatusCode, Json<SongDetailResponse>), AppError> {
    user.require_admin()?;
    request
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;
    let song_id = MusicRepository::create_song(&state.pool, &request).await?;
    Ok((
        StatusCode::CREATED,
        Json(fetch_song_detail_by_id(&state, song_id).await?),
    ))
}

#[utoipa::path(
    put,
    path = "/api/admin/canciones/{id}",
    tag = "music",
    params(("id" = i32, Path, description = "ID de la canción")),
    request_body = UpdateSongRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Canción actualizada", body = SongDetailResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 404, description = "Canción no encontrada", body = ErrorResponse),
        (status = 422, description = "Payload inválido", body = ErrorResponse)
    )
)]
pub async fn update_song(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
    Json(request): Json<UpdateSongRequest>,
) -> Result<Json<SongDetailResponse>, AppError> {
    user.require_admin()?;
    request
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;

    let existing = MusicRepository::find_song_by_id(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("cancion {id}")))?;
    let main_artist_id = resolve_song_main_artist_id(
        request.artista_id,
        request.artistas.as_deref(),
        existing.artista_id,
    );
    let updated = MusicRepository::update_song(&state.pool, id, main_artist_id, &request).await?;
    if !updated {
        return Err(AppError::NotFound(format!("cancion {id}")));
    }

    Ok(Json(fetch_song_detail_by_id(&state, id).await?))
}

#[utoipa::path(
    delete,
    path = "/api/admin/canciones/{id}",
    tag = "music",
    params(("id" = i32, Path, description = "ID de la canción")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Canción eliminada", body = MusicMutationResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 404, description = "Canción no encontrada", body = ErrorResponse)
    )
)]
pub async fn delete_song(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
) -> Result<Json<MusicMutationResponse>, AppError> {
    user.require_admin()?;
    let deleted = MusicRepository::delete_song(&state.pool, id).await?;
    if !deleted {
        return Err(AppError::NotFound(format!("cancion {id}")));
    }
    Ok(Json(MusicMutationResponse { ok: true }))
}

#[utoipa::path(
    post,
    path = "/api/admin/relaciones",
    tag = "music",
    request_body = CreateRelationRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Relación creada", body = SampleRelationDetail),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 409, description = "La relación ya existe", body = ErrorResponse),
        (status = 422, description = "Payload inválido", body = ErrorResponse)
    )
)]
pub async fn create_relation(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(request): Json<CreateRelationRequest>,
) -> Result<(StatusCode, Json<SampleRelationDetail>), AppError> {
    user.require_admin()?;
    request
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;
    let relation_id = MusicRepository::create_relation(&state.pool, &request).await?;
    Ok((
        StatusCode::CREATED,
        Json(fetch_relation_detail(&state, relation_id).await?),
    ))
}

#[utoipa::path(
    put,
    path = "/api/admin/relaciones/{id}",
    tag = "music",
    params(("id" = i32, Path, description = "ID de la relación")),
    request_body = UpdateRelationRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Relación actualizada", body = SampleRelationDetail),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 404, description = "Relación no encontrada", body = ErrorResponse),
        (status = 409, description = "La relación ya existe con esos datos", body = ErrorResponse),
        (status = 422, description = "Payload inválido", body = ErrorResponse)
    )
)]
pub async fn update_relation(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
    Json(request): Json<UpdateRelationRequest>,
) -> Result<Json<SampleRelationDetail>, AppError> {
    user.require_admin()?;
    request
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;

    let updated = MusicRepository::update_relation(&state.pool, id, &request).await?;
    if !updated {
        return Err(AppError::NotFound(format!("relacion {id}")));
    }
    Ok(Json(fetch_relation_detail(&state, id).await?))
}

#[utoipa::path(
    delete,
    path = "/api/admin/relaciones/{id}",
    tag = "music",
    params(("id" = i32, Path, description = "ID de la relación")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Relación eliminada", body = MusicMutationResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 404, description = "Relación no encontrada", body = ErrorResponse)
    )
)]
pub async fn delete_relation(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
) -> Result<Json<MusicMutationResponse>, AppError> {
    user.require_admin()?;
    let deleted = MusicRepository::delete_relation(&state.pool, id).await?;
    if !deleted {
        return Err(AppError::NotFound(format!("relacion {id}")));
    }
    Ok(Json(MusicMutationResponse { ok: true }))
}
