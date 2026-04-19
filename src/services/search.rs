use serde::Serialize;
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use validator::Validate;

use crate::errors::AppError;
use crate::models::{
    GlobalSearchQuery, GlobalSearchResponse, LegacyQuickSearchCollectionResult,
    LegacyQuickSearchQuery, LegacyQuickSearchRelationResult, LegacyQuickSearchRelationSide,
    LegacyQuickSearchResponse, LegacyQuickSearchSampleCreator, LegacyQuickSearchSampleResult,
    LegacyQuickSearchSongResult, LegacyQuickSearchTodoItem, LegacyQuickSearchUserResult,
    SampleCreatorSummary, SearchCollectionOwnerSummary, SearchCollectionResult,
    SearchSampleResult, SearchSongResult, SearchType, SearchUserResult,
};
use crate::repositories::{
    SearchCollectionRecord, SearchRepository, SearchSampleRecord, SearchSampleRelationRecord,
    SearchSongRecord, SearchUserRecord,
};

const DEFAULT_LIMIT_PER_TYPE: i64 = 5;
const MIN_QUERY_CHARS: usize = 2;
const MAX_TODOS: usize = 12;

pub struct SearchService;

impl SearchService {
    pub async fn global_search(
        pool: &PgPool,
        public_base_url: Option<&str>,
        query: GlobalSearchQuery,
    ) -> Result<GlobalSearchResponse, AppError> {
        query
            .validate()
            .map_err(|error| AppError::Validation(error.to_string()))?;

        let normalized_query = normalize_query(&query.q);
        let selected_types = parse_search_types(query.types.as_deref())?;
        let trimmed_query = query.q.trim().to_string();

        let Some(text_query) = normalized_query else {
            return Ok(GlobalSearchResponse::empty(trimmed_query));
        };

        let include_samples = selected_types.contains(&SearchType::Samples);
        let include_users = selected_types.contains(&SearchType::Users);
        let include_collections = selected_types.contains(&SearchType::Collections);
        let include_songs = selected_types.contains(&SearchType::Songs);

        let samples_future = async {
            if include_samples {
                SearchRepository::search_samples(pool, &text_query.like, DEFAULT_LIMIT_PER_TYPE).await
            } else {
                Ok(Vec::new())
            }
        };

        let users_future = async {
            if include_users {
                SearchRepository::search_users(pool, &text_query.like, DEFAULT_LIMIT_PER_TYPE).await
            } else {
                Ok(Vec::new())
            }
        };

        let collections_future = async {
            if include_collections {
                SearchRepository::search_collections(pool, &text_query.like, DEFAULT_LIMIT_PER_TYPE)
                    .await
            } else {
                Ok(Vec::new())
            }
        };

        let songs_future = async {
            if include_songs {
                SearchRepository::search_songs(
                    pool,
                    &text_query.original,
                    &text_query.like,
                    DEFAULT_LIMIT_PER_TYPE,
                )
                .await
            } else {
                Ok(Vec::new())
            }
        };

        let (samples, users, collections, songs) =
            tokio::try_join!(samples_future, users_future, collections_future, songs_future)?;

        Ok(GlobalSearchResponse {
            q: text_query.original,
            samples: samples
                .into_iter()
                .map(|record| map_sample_result(record, public_base_url))
                .collect(),
            users: users
                .into_iter()
                .map(|record| map_user_result(record, public_base_url))
                .collect(),
            collections: collections
                .into_iter()
                .map(|record| map_collection_result(record, public_base_url))
                .collect(),
            songs: songs
                .into_iter()
                .map(|record| map_song_result(record, public_base_url))
                .collect(),
        })
    }

    pub async fn legacy_quick_search(
        pool: &PgPool,
        public_base_url: Option<&str>,
        query: LegacyQuickSearchQuery,
    ) -> Result<LegacyQuickSearchResponse, AppError> {
        query
            .validate()
            .map_err(|error| AppError::Validation(error.to_string()))?;

        let normalized_query = normalize_query(&query.q);
        let Some(text_query) = normalized_query else {
            return Ok(LegacyQuickSearchResponse::empty());
        };

        let (songs, samples, relations, users, collections) = tokio::try_join!(
            SearchRepository::search_songs(
                pool,
                &text_query.original,
                &text_query.like,
                DEFAULT_LIMIT_PER_TYPE,
            ),
            SearchRepository::search_samples(pool, &text_query.like, DEFAULT_LIMIT_PER_TYPE),
            SearchRepository::search_sample_relations(pool, &text_query.like, DEFAULT_LIMIT_PER_TYPE),
            SearchRepository::search_users(pool, &text_query.like, DEFAULT_LIMIT_PER_TYPE),
            SearchRepository::search_collections(pool, &text_query.like, DEFAULT_LIMIT_PER_TYPE),
        )?;

        let canciones = songs
            .into_iter()
            .map(|record| map_legacy_song_result(record, public_base_url))
            .collect::<Vec<_>>();
        let sample_results = samples
            .into_iter()
            .map(|record| map_legacy_sample_result(record, public_base_url))
            .collect::<Vec<_>>();
        let sampleos = relations
            .into_iter()
            .map(|record| map_legacy_relation_result(record, public_base_url))
            .collect::<Vec<_>>();
        let usuarios = users
            .into_iter()
            .map(|record| map_legacy_user_result(record, public_base_url))
            .collect::<Vec<_>>();
        let colecciones = collections
            .into_iter()
            .map(|record| map_legacy_collection_result(record, public_base_url))
            .collect::<Vec<_>>();
        let todos = build_legacy_todos(
            &text_query.lower,
            &canciones,
            &sample_results,
            &sampleos,
            &usuarios,
            &colecciones,
        );

        Ok(LegacyQuickSearchResponse {
            canciones,
            samples: sample_results,
            sampleos,
            usuarios,
            colecciones,
            todos,
        })
    }
}

struct TextQuery {
    original: String,
    lower: String,
    like: String,
}

fn normalize_query(raw: &str) -> Option<TextQuery> {
    let trimmed = raw.trim();
    if trimmed.chars().count() < MIN_QUERY_CHARS {
        return None;
    }

    Some(TextQuery {
        original: trimmed.to_string(),
        lower: trimmed.to_lowercase(),
        like: format!("%{trimmed}%"),
    })
}

fn parse_search_types(raw: Option<&str>) -> Result<Vec<SearchType>, AppError> {
    let Some(raw) = raw else {
        return Ok(SearchType::ALL.to_vec());
    };

    let mut parsed = Vec::new();

    for item in raw.split(',') {
        let normalized = item.trim().to_ascii_lowercase();
        if normalized.is_empty() {
            continue;
        }

        let search_type = match normalized.as_str() {
            "sample" | "samples" => SearchType::Samples,
            "user" | "users" | "usuario" | "usuarios" => SearchType::Users,
            "collection" | "collections" | "coleccion" | "colecciones" => {
                SearchType::Collections
            }
            "song" | "songs" | "cancion" | "canciones" => SearchType::Songs,
            other => {
                return Err(AppError::Validation(format!(
                    "types contiene un valor no soportado: {other}"
                )))
            }
        };

        if !parsed.contains(&search_type) {
            parsed.push(search_type);
        }
    }

    if parsed.is_empty() {
        return Ok(SearchType::ALL.to_vec());
    }

    Ok(parsed)
}

fn map_song_result(record: SearchSongRecord, public_base_url: Option<&str>) -> SearchSongResult {
    SearchSongResult {
        id: record.id,
        titulo: record.titulo,
        slug: record.slug,
        artista_nombre: record.artista_nombre,
        imagen_url: asset_to_public_url(public_base_url, record.imagen_url),
        total_sampleada: record.total_sampleada,
    }
}

fn map_sample_result(
    record: SearchSampleRecord,
    public_base_url: Option<&str>,
) -> SearchSampleResult {
    SearchSampleResult {
        id: record.id,
        titulo: record.titulo,
        slug: record.slug,
        imagen_url: asset_to_public_url(public_base_url, record.imagen_url),
        creador: SampleCreatorSummary {
            id: record.creator_id,
            username: record.creator_username.clone(),
            nombre_visible: record.creator_nombre_visible.clone(),
            avatar_url: asset_to_public_url(public_base_url, record.creator_avatar_url),
            verificado: record.creator_verificado,
        },
    }
}

fn map_user_result(record: SearchUserRecord, public_base_url: Option<&str>) -> SearchUserResult {
    SearchUserResult {
        id: record.id,
        username: record.username,
        nombre_visible: record.nombre_visible,
        avatar_url: asset_to_public_url(public_base_url, record.avatar_url),
        verificado: record.verificado,
        total_seguidores: record.total_seguidores,
    }
}

fn map_collection_result(
    record: SearchCollectionRecord,
    public_base_url: Option<&str>,
) -> SearchCollectionResult {
    SearchCollectionResult {
        id: record.id,
        nombre: record.nombre,
        slug: record.slug,
        portada_url: asset_to_public_url(public_base_url, record.portada_url),
        total_samples: record.total_samples,
        creador: SearchCollectionOwnerSummary {
            username: record.creator_username,
            nombre_visible: record.creator_nombre_visible,
        },
    }
}

fn map_legacy_song_result(
    record: SearchSongRecord,
    public_base_url: Option<&str>,
) -> LegacyQuickSearchSongResult {
    LegacyQuickSearchSongResult {
        id: record.id,
        titulo: record.titulo,
        slug: record.slug,
        artista_nombre: record.artista_nombre,
        imagen_url: asset_to_public_url(public_base_url, record.imagen_url),
        total_sampleada: record.total_sampleada,
    }
}

fn map_legacy_sample_result(
    record: SearchSampleRecord,
    public_base_url: Option<&str>,
) -> LegacyQuickSearchSampleResult {
    let display_name = record
        .creator_nombre_visible
        .clone()
        .unwrap_or_else(|| record.creator_username.clone());

    LegacyQuickSearchSampleResult {
        id: record.id,
        titulo: record.titulo,
        slug: record.slug,
        imagen_url: asset_to_public_url(public_base_url, record.imagen_url),
        creador: LegacyQuickSearchSampleCreator {
            username: record.creator_username,
            nombre_visible: display_name,
            avatar_url: asset_to_public_url(public_base_url, record.creator_avatar_url),
        },
    }
}

fn map_legacy_user_result(
    record: SearchUserRecord,
    public_base_url: Option<&str>,
) -> LegacyQuickSearchUserResult {
    LegacyQuickSearchUserResult {
        id: record.id,
        username: record.username,
        nombre_visible: record.nombre_visible,
        avatar_url: asset_to_public_url(public_base_url, record.avatar_url),
        verificado: record.verificado,
        total_seguidores: record.total_seguidores,
    }
}

fn map_legacy_collection_result(
    record: SearchCollectionRecord,
    public_base_url: Option<&str>,
) -> LegacyQuickSearchCollectionResult {
    let creator = if record.creator_nombre_visible.is_empty() {
        record.creator_username.clone()
    } else {
        record.creator_nombre_visible.clone()
    };

    LegacyQuickSearchCollectionResult {
        id: record.id,
        nombre: record.nombre,
        slug: record.slug.unwrap_or_else(|| record.id.to_string()),
        portada_url: asset_to_public_url(public_base_url, record.portada_url),
        total_samples: record.total_samples,
        creador: creator,
    }
}

fn map_legacy_relation_result(
    record: SearchSampleRelationRecord,
    public_base_url: Option<&str>,
) -> LegacyQuickSearchRelationResult {
    LegacyQuickSearchRelationResult {
        id: record.id,
        fuente: LegacyQuickSearchRelationSide {
            titulo: record.fuente_titulo,
            slug: record.fuente_slug,
            imagen_url: asset_to_public_url(public_base_url, record.fuente_imagen_url),
            artista: record.fuente_artista,
        },
        destino: LegacyQuickSearchRelationSide {
            titulo: record.destino_titulo,
            slug: record.destino_slug,
            imagen_url: asset_to_public_url(public_base_url, record.destino_imagen_url),
            artista: record.destino_artista,
        },
    }
}

fn build_legacy_todos(
    lower_query: &str,
    canciones: &[LegacyQuickSearchSongResult],
    samples: &[LegacyQuickSearchSampleResult],
    sampleos: &[LegacyQuickSearchRelationResult],
    usuarios: &[LegacyQuickSearchUserResult],
    colecciones: &[LegacyQuickSearchCollectionResult],
) -> Vec<LegacyQuickSearchTodoItem> {
    let mut todos = Vec::new();

    for (position, song) in canciones.iter().enumerate() {
        let match_text = format!("{} {}", song.titulo, song.artista_nombre).to_lowercase();
        todos.push(LegacyQuickSearchTodoItem {
            tipo: "cancion".into(),
            score: calculate_score(lower_query, &match_text, position),
            datos: to_json_value(song),
        });
    }

    for (position, sample) in samples.iter().enumerate() {
        let match_text = sample.titulo.to_lowercase();
        todos.push(LegacyQuickSearchTodoItem {
            tipo: "sample".into(),
            score: calculate_score(lower_query, &match_text, position),
            datos: to_json_value(sample),
        });
    }

    for (position, relation) in sampleos.iter().enumerate() {
        let match_text = format!(
            "{} {} {} {}",
            relation.fuente.titulo,
            relation.fuente.artista,
            relation.destino.titulo,
            relation.destino.artista,
        )
        .to_lowercase();
        todos.push(LegacyQuickSearchTodoItem {
            tipo: "sampleo".into(),
            score: calculate_score(lower_query, &match_text, position),
            datos: to_json_value(relation),
        });
    }

    for (position, user) in usuarios.iter().enumerate() {
        let match_text = format!("{} {}", user.username, user.nombre_visible).to_lowercase();
        todos.push(LegacyQuickSearchTodoItem {
            tipo: "usuario".into(),
            score: calculate_score(lower_query, &match_text, position),
            datos: to_json_value(user),
        });
    }

    for (position, collection) in colecciones.iter().enumerate() {
        let match_text = collection.nombre.to_lowercase();
        todos.push(LegacyQuickSearchTodoItem {
            tipo: "coleccion".into(),
            score: calculate_score(lower_query, &match_text, position),
            datos: to_json_value(collection),
        });
    }

    todos.sort_by(|left, right| right.score.total_cmp(&left.score));
    todos.truncate(MAX_TODOS);
    todos
}

fn calculate_score(query: &str, text: &str, position: usize) -> f32 {
    let base = if text == query {
        100.0
    } else if text.starts_with(query) {
        80.0
    } else if text.contains(query) {
        60.0
    } else {
        40.0
    };

    base - (position as f32 * 2.0)
}

fn to_json_value<T: Serialize>(value: &T) -> JsonValue {
    match serde_json::to_value(value) {
        Ok(json_value) => json_value,
        Err(_) => JsonValue::Null,
    }
}

fn asset_to_public_url(public_base_url: Option<&str>, raw: Option<String>) -> Option<String> {
    let raw = raw?.trim().replace('\\', "/");
    if raw.is_empty() {
        return None;
    }

    if raw.starts_with("http://") || raw.starts_with("https://") {
        return Some(raw);
    }

    let path = if raw.starts_with('/') {
        raw
    } else {
        format!("/uploads/{raw}")
    };

    Some(match public_base_url {
        Some(base) => format!("{}{}", base.trim_end_matches('/'), path),
        None => path,
    })
}

#[cfg(test)]
mod tests {
    use super::{calculate_score, normalize_query, parse_search_types};
    use crate::models::SearchType;

    #[test]
    fn normalize_query_requires_two_characters() {
        assert!(normalize_query("a").is_none());
        assert!(normalize_query("ab").is_some());
    }

    #[test]
    fn parse_search_types_accepts_aliases_and_deduplicates() {
        let parsed = parse_search_types(Some("samples,usuarios,colecciones,canciones,samples"))
            .expect("types validos");
        assert_eq!(
            parsed,
            vec![
                SearchType::Samples,
                SearchType::Users,
                SearchType::Collections,
                SearchType::Songs,
            ]
        );
    }

    #[test]
    fn calculate_score_prioritizes_prefix_matches() {
        let prefix = calculate_score("trap", "trap drums", 0);
        let contains = calculate_score("trap", "dark trap drums", 0);
        assert!(prefix > contains);
    }
}