use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::AppState;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AppVersionInfo {
    pub version: String,
    pub url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AppVersionsResponse {
    pub windows: Option<AppVersionInfo>,
    pub apk: Option<AppVersionInfo>,
    pub web: Option<AppVersionInfo>,
}

fn first_env(keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| std::env::var(key).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn build_version_info(prefix: &str, fallback_version: Option<String>) -> Option<AppVersionInfo> {
    let upper = prefix.to_ascii_uppercase();
    let version = first_env(&[
        &format!("KAMPLES_{upper}_VERSION"),
        &format!("APP_{upper}_VERSION"),
    ])
    .or(fallback_version);
    let url = first_env(&[
        &format!("KAMPLES_{upper}_URL"),
        &format!("APP_{upper}_URL"),
    ]);
    let notes = first_env(&[
        &format!("KAMPLES_{upper}_NOTES"),
        &format!("APP_{upper}_NOTES"),
    ]);

    if version.is_none() && url.is_none() && notes.is_none() {
        return None;
    }

    Some(AppVersionInfo {
        version: version.unwrap_or_default(),
        url,
        notes,
    })
}

#[utoipa::path(
    get,
    path = "/api/app/versions",
    tag = "sync",
    responses((status = 200, description = "Versiones disponibles por plataforma", body = AppVersionsResponse))
)]
pub async fn get_app_versions() -> Json<AppVersionsResponse> {
    Json(AppVersionsResponse {
        windows: build_version_info("windows", None),
        apk: build_version_info("apk", None),
        web: build_version_info("web", Some(env!("CARGO_PKG_VERSION").to_string())),
    })
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/app/versions", get(get_app_versions))
}
