use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::repositories::{ArticleCategoryCount, ArticleDetail, ArticleSummary};

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateArticleMultipartRequestDoc {
    pub titulo: String,
    pub contenido: String,
    pub extracto: String,
    pub categoria: String,
    #[schema(value_type = String, format = Binary)]
    pub portada: Option<Vec<u8>>,
    pub portada_url: Option<String>,
    pub embeds: Option<String>,
    pub descarga_publica: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateArticleRequest {
    pub titulo: Option<String>,
    pub contenido: Option<String>,
    pub extracto: Option<String>,
    pub categoria: Option<String>,
    pub portada_url: Option<String>,
    pub embeds: Option<String>,
    pub descarga_publica: Option<bool>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ArticleListData {
    pub articulos: Vec<ArticleSummary>,
    pub total: i64,
    pub hay_mas: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ArticleListResponse {
    pub ok: bool,
    pub data: ArticleListData,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ArticleResponse {
    pub ok: bool,
    pub data: ArticleDetail,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ArticleCategoriesResponse {
    pub ok: bool,
    pub data: Vec<ArticleCategoryCount>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DeleteArticleData {
    pub eliminado: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DeleteArticleResponse {
    pub ok: bool,
    pub data: DeleteArticleData,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ToggleArticleLikeResponse {
    pub ok: bool,
    pub liked: bool,
    pub total: i32,
}
