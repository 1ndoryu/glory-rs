use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::middleware::CurrentUser;
use crate::models::{
    BlockUserRequest, PrivateProfileResponse, PublicProfileResponse, UpdateProfileRequest,
};
use crate::repositories::{ModerationRepository, ProfileRepository};
use crate::AppState;

const MAX_PROFILE_IMAGE_BYTES: usize = 5 * 1024 * 1024;

/* [174A-24] Endpoints de perfil. */

#[utoipa::path(get, path = "/api/users/me",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Perfil propio", body = PrivateProfileResponse),
        (status = 401, body = ErrorResponse)
    ))]
pub async fn me(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<PrivateProfileResponse>, AppError> {
    let p = ProfileRepository::find_by_id(&state.pool, user.user_id)
        .await?
        .ok_or(AppError::NotFound("Usuario".into()))?;
    Ok(Json(PrivateProfileResponse::from(p)))
}

#[utoipa::path(patch, path = "/api/users/me",
    request_body = UpdateProfileRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Perfil actualizado", body = PrivateProfileResponse),
        (status = 401, body = ErrorResponse),
        (status = 422, body = ErrorResponse)
    ))]
pub async fn update_me(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<PrivateProfileResponse>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let p = ProfileRepository::update(&state.pool, user.user_id, &req).await?;
    Ok(Json(PrivateProfileResponse::from(p)))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfileImageUploadResponse {
    pub ok: bool,
    pub data: PrivateProfileResponse,
    pub url: String,
}

#[derive(Debug, Clone, Copy)]
enum ProfileImageKind {
    Avatar,
    Portada,
}

impl ProfileImageKind {
    fn field_name(self) -> &'static str {
        match self {
            Self::Avatar => "avatar",
            Self::Portada => "portada",
        }
    }

    fn path_segment(self) -> &'static str {
        match self {
            Self::Avatar => "avatar",
            Self::Portada => "portada",
        }
    }
}

#[utoipa::path(post, path = "/api/users/me/avatar",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Avatar actualizado", body = ProfileImageUploadResponse),
        (status = 400, body = ErrorResponse),
        (status = 401, body = ErrorResponse)
    ))]
pub async fn upload_avatar(
    State(state): State<AppState>,
    user: CurrentUser,
    multipart: Multipart,
) -> Result<Json<ProfileImageUploadResponse>, AppError> {
    upload_profile_image(state, user.user_id, multipart, ProfileImageKind::Avatar).await
}

#[utoipa::path(post, path = "/api/users/me/portada",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Portada actualizada", body = ProfileImageUploadResponse),
        (status = 400, body = ErrorResponse),
        (status = 401, body = ErrorResponse)
    ))]
pub async fn upload_portada(
    State(state): State<AppState>,
    user: CurrentUser,
    multipart: Multipart,
) -> Result<Json<ProfileImageUploadResponse>, AppError> {
    upload_profile_image(state, user.user_id, multipart, ProfileImageKind::Portada).await
}

async fn upload_profile_image(
    state: AppState,
    user_id: i32,
    mut multipart: Multipart,
    kind: ProfileImageKind,
) -> Result<Json<ProfileImageUploadResponse>, AppError> {
    let (bytes, extension) = read_profile_image_field(&mut multipart, kind).await?;
    let key = format!(
        "usuarios/{user_id}/{}_{}.{}",
        kind.path_segment(),
        Uuid::now_v7(),
        extension
    );
    state.storage.put_bytes(&key, &bytes).await?;
    let url = public_storage_url(state.public_base_url.as_deref(), &key);
    let request = match kind {
        ProfileImageKind::Avatar => UpdateProfileRequest {
            avatar_url: Some(url.clone()),
            ..UpdateProfileRequest::default()
        },
        ProfileImageKind::Portada => UpdateProfileRequest {
            portada_url: Some(url.clone()),
            ..UpdateProfileRequest::default()
        },
    };
    let profile = ProfileRepository::update(&state.pool, user_id, &request).await?;
    Ok(Json(ProfileImageUploadResponse {
        ok: true,
        data: PrivateProfileResponse::from(profile),
        url,
    }))
}

async fn read_profile_image_field(
    multipart: &mut Multipart,
    kind: ProfileImageKind,
) -> Result<(Vec<u8>, &'static str), AppError> {
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|error| AppError::BadRequest(format!("multipart invalido: {error}")))?
    {
        if field.name() != Some(kind.field_name()) {
            continue;
        }
        let content_type = field.content_type().map(ToString::to_string);
        let extension = image_extension(content_type.as_deref())?;
        let mut bytes = Vec::new();
        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|error| AppError::BadRequest(format!("multipart invalido: {error}")))?
        {
            if bytes.len().saturating_add(chunk.len()) > MAX_PROFILE_IMAGE_BYTES {
                return Err(AppError::BadRequest("la imagen supera 5MB".into()));
            }
            bytes.extend_from_slice(&chunk);
        }
        if bytes.is_empty() {
            return Err(AppError::BadRequest("imagen vacia".into()));
        }
        return Ok((bytes, extension));
    }
    Err(AppError::BadRequest(format!(
        "campo multipart '{}' requerido",
        kind.field_name()
    )))
}

fn image_extension(content_type: Option<&str>) -> Result<&'static str, AppError> {
    match content_type {
        Some("image/jpeg" | "image/jpg") => Ok("jpg"),
        Some("image/png") => Ok("png"),
        Some("image/webp") => Ok("webp"),
        Some("image/gif") => Ok("gif"),
        _ => Err(AppError::BadRequest(
            "formato de imagen no permitido; usa jpg, png, webp o gif".into(),
        )),
    }
}

fn public_storage_url(public_base_url: Option<&str>, key: &str) -> String {
    let path = format!("/uploads/{key}");
    match public_base_url {
        Some(base) => format!("{}{}", base.trim_end_matches('/'), path),
        None => path,
    }
}

#[utoipa::path(get, path = "/api/users/{username}",
    params(("username" = String, Path, description = "Username publico")),
    responses(
        (status = 200, description = "Perfil publico", body = PublicProfileResponse),
        (status = 404, body = ErrorResponse)
    ))]
pub async fn public_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<PublicProfileResponse>, AppError> {
    let p = ProfileRepository::find_by_username(&state.pool, &username)
        .await?
        .ok_or(AppError::NotFound(format!("usuario {username}")))?;
    if p.estado != "activo" {
        return Err(AppError::NotFound(format!("usuario {username}")));
    }
    Ok(Json(PublicProfileResponse::from(p)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(me).patch(update_me))
        .route("/users/me/avatar", post(upload_avatar))
        .route("/users/me/portada", post(upload_portada))
        .route("/users/me/blocked", get(list_blocked))
        .route("/users/:username", get(public_profile))
        .route("/users/:username/block", post(block).delete(unblock))
        .layer(DefaultBodyLimit::max(MAX_PROFILE_IMAGE_BYTES + 1024 * 256))
}

#[utoipa::path(post, path = "/api/users/{username}/block",
    params(("username" = String, Path, description = "Username")),
    request_body = BlockUserRequest,
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Bloqueado"), (status = 400, description = "auto-bloqueo"), (status = 401, description = "no auth"), (status = 404, description = "no encontrado")))]
pub async fn block(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(username): Path<String>,
    Json(req): Json<BlockUserRequest>,
) -> Result<StatusCode, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let target = ProfileRepository::find_by_username(&state.pool, &username)
        .await?
        .ok_or(AppError::NotFound(format!("usuario {username}")))?;
    let razon = req.razon.unwrap_or_default();
    ModerationRepository::block(&state.pool, user.user_id, target.id, &razon).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/users/{username}/block",
    params(("username" = String, Path, description = "Username")),
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Desbloqueado"), (status = 401, description = "no auth"), (status = 404, description = "no encontrado")))]
pub async fn unblock(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(username): Path<String>,
) -> Result<StatusCode, AppError> {
    let target = ProfileRepository::find_by_username(&state.pool, &username)
        .await?
        .ok_or(AppError::NotFound(format!("usuario {username}")))?;
    ModerationRepository::unblock(&state.pool, user.user_id, target.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/users/me/blocked",
    security(("bearer_auth" = [])),
    responses((status = 200, description = "Lista de IDs bloqueados", body = Vec<i32>), (status = 401, description = "no auth")))]
pub async fn list_blocked(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<Vec<i32>>, AppError> {
    let ids = ModerationRepository::list_blocked(&state.pool, user.user_id).await?;
    Ok(Json(ids))
}
