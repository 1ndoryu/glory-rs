/* [054A-1] Handler para gestión de usuarios desde panel admin.
 * Endpoints: listar (paginado + search + filtros), cambiar rol, cambiar status (ban/unban).
 * Solo accesible por admins. */

use axum::{
    extract::{Path, Query, State},
    routing::{get, patch},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    AdminUserItem, ChangeRoleRequest, ChangeStatusRequest, PaginatedUsers, UserResponse, UserRole,
};
use crate::repositories::UserRepository;
use crate::AppState;

/// Query params para listar usuarios con paginación y filtros
#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
}

/// Lista usuarios con paginación, búsqueda y filtros
#[utoipa::path(
    get,
    path = "/api/admin/users",
    params(
        ("page" = Option<i64>, Query, description = "Página (1-based)"),
        ("per_page" = Option<i64>, Query, description = "Resultados por página (max 100)"),
        ("search" = Option<String>, Query, description = "Buscar por email o nombre"),
        ("role" = Option<String>, Query, description = "Filtrar por rol: admin, employee, client"),
        ("status" = Option<String>, Query, description = "Filtrar por status: active, banned, suspended"),
    ),
    responses(
        (status = 200, description = "Lista paginada de usuarios", body = PaginatedUsers)
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_users(
    auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<ListUsersQuery>,
) -> Result<Json<PaginatedUsers>, AppError> {
    auth.require_role(&[UserRole::Admin])?;

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    /* Parsear el filtro de rol si se proporcionó */
    let role_filter = params.role.as_deref().and_then(|r| match r {
        "admin" => Some(UserRole::Admin),
        "employee" => Some(UserRole::Employee),
        "client" => Some(UserRole::Client),
        _ => None,
    });

    let rows = UserRepository::list_all(
        &state.pool,
        params.search.as_deref(),
        role_filter,
        params.status.as_deref(),
        per_page,
        offset,
    )
    .await?;

    let total = rows.first().map_or(0, |r| r.total_count);

    let users = rows
        .into_iter()
        .map(|r| AdminUserItem {
            id: r.id,
            email: r.email,
            role: r.role,
            status: r.status,
            email_verified: r.email_verified,
            avatar_url: r.avatar_url,
            display_name: r.display_name,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(PaginatedUsers {
        users,
        total,
        page,
        per_page,
    }))
}

/// Cambia el rol de un usuario
#[utoipa::path(
    patch,
    path = "/api/admin/users/{user_id}/role",
    request_body = ChangeRoleRequest,
    params(
        ("user_id" = Uuid, Path, description = "ID del usuario")
    ),
    responses(
        (status = 200, description = "Rol actualizado", body = UserResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn change_role(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<ChangeRoleRequest>,
) -> Result<Json<UserResponse>, AppError> {
    auth.require_role(&[UserRole::Admin])?;

    /* No permitir que un admin se cambie el rol a sí mismo */
    if user_id == auth.user_id {
        return Err(AppError::BadRequest(
            "No puedes cambiar tu propio rol".into(),
        ));
    }

    let user = UserRepository::update_role(&state.pool, user_id, body.role)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Usuario no encontrado".into()),
            other => AppError::Database(other),
        })?;

    Ok(Json(UserResponse::from(user)))
}

/// Cambia el status de un usuario (ban, suspend, reactivar)
#[utoipa::path(
    patch,
    path = "/api/admin/users/{user_id}/status",
    request_body = ChangeStatusRequest,
    params(
        ("user_id" = Uuid, Path, description = "ID del usuario")
    ),
    responses(
        (status = 200, description = "Status actualizado", body = UserResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn change_status(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<ChangeStatusRequest>,
) -> Result<Json<UserResponse>, AppError> {
    auth.require_role(&[UserRole::Admin])?;

    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    /* No permitir que un admin se banee a sí mismo */
    if user_id == auth.user_id {
        return Err(AppError::BadRequest(
            "No puedes cambiar tu propio status".into(),
        ));
    }

    let user = UserRepository::update_status(&state.pool, user_id, &body.status)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Usuario no encontrado".into()),
            other => AppError::Database(other),
        })?;

    Ok(Json(UserResponse::from(user)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin/users", get(list_users))
        .route("/admin/users/:user_id/role", patch(change_role))
        .route("/admin/users/:user_id/status", patch(change_status))
}
