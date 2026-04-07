/* [074A-13] Modelo de miembros del equipo.
 * Mapa BD → API: name→name, slug→slug, role→role, bio→bio.
 * Más simple que Project: sin JSONB, campos string directos. */

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, FromRow)]
pub struct TeamMember {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub role: String,
    pub bio: String,
    pub avatar: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub status: String,
    pub sort_order: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TeamMemberResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub role: String,
    pub bio: String,
    pub avatar: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub status: String,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl TeamMember {
    #[must_use]
    pub fn into_response(self) -> TeamMemberResponse {
        TeamMemberResponse {
            id: self.id,
            name: self.name,
            slug: self.slug,
            role: self.role,
            bio: self.bio,
            avatar: self.avatar,
            linkedin: self.linkedin,
            twitter: self.twitter,
            github: self.github,
            status: self.status,
            sort_order: self.sort_order,
            created_at: self.created_at.to_rfc3339(),
            updated_at: self.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTeamMemberRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 1, max = 200))]
    pub slug: String,
    pub role: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub status: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTeamMemberRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub role: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub status: Option<String>,
    pub sort_order: Option<i32>,
}
