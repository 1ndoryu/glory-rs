use axum::extract::{Path, State};
use axum::Json;
use validator::Validate;

use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::middleware::CurrentUser;
use crate::models::{
    MusicMutationResponse, RelationSampleSide, RelationVerificationResponse, SampleLinkRequest,
    VerifyRelationRequest,
};
use crate::repositories::MusicRepository;
use crate::AppState;

#[utoipa::path(
    post,
    path = "/api/relaciones/{id}/vincular-sample",
    tag = "music",
    params(("id" = i32, Path, description = "ID de la relación")),
    request_body = SampleLinkRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Sample vinculado a la relación", body = MusicMutationResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Sin permisos sobre el sample", body = ErrorResponse),
        (status = 404, description = "Relación o sample no encontrados", body = ErrorResponse),
        (status = 409, description = "La relación ya tiene sample en ese lado", body = ErrorResponse)
    )
)]
pub async fn link_sample_to_relation(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
    Json(request): Json<SampleLinkRequest>,
) -> Result<Json<MusicMutationResponse>, AppError> {
    request
        .validate()
        .map_err(|error| AppError::Validation(error.to_string()))?;
    MusicRepository::link_sample(
        &state.pool,
        id,
        request.sample_id,
        request.lado,
        user.user_id,
    )
    .await?;
    Ok(Json(MusicMutationResponse { ok: true }))
}

#[utoipa::path(
    delete,
    path = "/api/relaciones/{id}/sample/{lado}",
    tag = "music",
    params(
        ("id" = i32, Path, description = "ID de la relación"),
        ("lado" = RelationSampleSide, Path, description = "Lado a desvincular: fuente o destino")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Sample desvinculado", body = MusicMutationResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Sin permisos sobre el sample", body = ErrorResponse),
        (status = 404, description = "Relación no encontrada", body = ErrorResponse)
    )
)]
pub async fn unlink_sample_from_relation(
    State(state): State<AppState>,
    user: CurrentUser,
    Path((id, lado)): Path<(i32, RelationSampleSide)>,
) -> Result<Json<MusicMutationResponse>, AppError> {
    MusicRepository::unlink_sample(&state.pool, id, lado, user.user_id).await?;
    Ok(Json(MusicMutationResponse { ok: true }))
}

#[utoipa::path(
    put,
    path = "/api/relaciones/{id}/verificar",
    tag = "music",
    params(("id" = i32, Path, description = "ID de la relación")),
    request_body = VerifyRelationRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Estado de verificación actualizado", body = RelationVerificationResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 403, description = "Requiere admin", body = ErrorResponse),
        (status = 404, description = "Relación no encontrada", body = ErrorResponse)
    )
)]
pub async fn verify_relation(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
    Json(request): Json<VerifyRelationRequest>,
) -> Result<Json<RelationVerificationResponse>, AppError> {
    user.require_admin()?;
    let updated = MusicRepository::verify_relation(&state.pool, id, request.verificada).await?;
    if !updated {
        return Err(AppError::NotFound(format!("relacion {id}")));
    }
    Ok(Json(RelationVerificationResponse {
        ok: true,
        verificada: request.verificada,
    }))
}
