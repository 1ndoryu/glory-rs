use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use utoipa::ToSchema;
use validator::Validate;

use crate::models::SampleCreatorSummary;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Samples,
    Users,
    Collections,
    Songs,
}

impl SearchType {
    pub const ALL: [Self; 4] = [Self::Samples, Self::Users, Self::Collections, Self::Songs];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Samples => "samples",
            Self::Users => "users",
            Self::Collections => "collections",
            Self::Songs => "songs",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct GlobalSearchQuery {
    #[validate(length(max = 120))]
    pub q: String,
    pub types: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct LegacyQuickSearchQuery {
    #[validate(length(max = 120))]
    pub q: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SearchSongResult {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub artista_nombre: String,
    pub imagen_url: Option<String>,
    pub total_sampleada: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SearchUserResult {
    pub id: i32,
    pub username: String,
    pub nombre_visible: String,
    pub avatar_url: Option<String>,
    pub verificado: bool,
    pub total_seguidores: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SearchCollectionOwnerSummary {
    pub username: String,
    pub nombre_visible: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SearchCollectionResult {
    pub id: i64,
    pub nombre: String,
    pub slug: Option<String>,
    pub portada_url: Option<String>,
    pub total_samples: i32,
    pub creador: SearchCollectionOwnerSummary,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SearchSampleResult {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub imagen_url: Option<String>,
    pub creador: SampleCreatorSummary,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GlobalSearchResponse {
    pub q: String,
    pub samples: Vec<SearchSampleResult>,
    pub users: Vec<SearchUserResult>,
    pub collections: Vec<SearchCollectionResult>,
    pub songs: Vec<SearchSongResult>,
}

impl GlobalSearchResponse {
    pub fn empty(q: String) -> Self {
        Self {
            q,
            samples: Vec::new(),
            users: Vec::new(),
            collections: Vec::new(),
            songs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LegacyQuickSearchSongResult {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub artista_nombre: String,
    pub imagen_url: Option<String>,
    pub total_sampleada: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LegacyQuickSearchSampleCreator {
    pub username: String,
    pub nombre_visible: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LegacyQuickSearchSampleResult {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub imagen_url: Option<String>,
    pub creador: LegacyQuickSearchSampleCreator,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LegacyQuickSearchUserResult {
    pub id: i32,
    pub username: String,
    pub nombre_visible: String,
    pub avatar_url: Option<String>,
    pub verificado: bool,
    pub total_seguidores: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LegacyQuickSearchCollectionResult {
    pub id: i64,
    pub nombre: String,
    pub slug: String,
    pub portada_url: Option<String>,
    pub total_samples: i32,
    pub creador: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LegacyQuickSearchRelationSide {
    pub titulo: String,
    pub slug: String,
    pub imagen_url: Option<String>,
    pub artista: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LegacyQuickSearchRelationResult {
    pub id: i32,
    pub fuente: LegacyQuickSearchRelationSide,
    pub destino: LegacyQuickSearchRelationSide,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LegacyQuickSearchTodoItem {
    pub tipo: String,
    pub score: f32,
    #[schema(value_type = Object)]
    pub datos: JsonValue,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LegacyQuickSearchResponse {
    pub canciones: Vec<LegacyQuickSearchSongResult>,
    pub samples: Vec<LegacyQuickSearchSampleResult>,
    pub sampleos: Vec<LegacyQuickSearchRelationResult>,
    pub usuarios: Vec<LegacyQuickSearchUserResult>,
    pub colecciones: Vec<LegacyQuickSearchCollectionResult>,
    pub todos: Vec<LegacyQuickSearchTodoItem>,
}

impl LegacyQuickSearchResponse {
    pub fn empty() -> Self {
        Self {
            canciones: Vec::new(),
            samples: Vec::new(),
            sampleos: Vec::new(),
            usuarios: Vec::new(),
            colecciones: Vec::new(),
            todos: Vec::new(),
        }
    }
}