pub mod admin;
pub mod mutations;
pub mod public;
mod support;

use axum::routing::{delete, get, post, put};
use axum::Router;

pub use admin::{
    create_artist, create_relation, create_song, delete_artist, delete_relation, delete_song,
    update_artist, update_relation, update_song,
};
pub use mutations::{link_sample_to_relation, unlink_sample_from_relation, verify_relation};
pub use public::{
    get_artist, get_relation, get_relation_by_sample, get_song, get_song_chain, get_song_samples,
    list_songs, relation_stats, search_songs, song_sections, top_artists, top_songs,
};

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/canciones", get(list_songs))
        .route("/canciones/buscar", get(search_songs))
        .route("/canciones/secciones", get(song_sections))
        .route("/canciones/top", get(top_songs))
        .route("/canciones/:slug/cadena", get(get_song_chain))
        .route("/canciones/:slug/samples", get(get_song_samples))
        .route("/canciones/:slug", get(get_song))
        .route("/artistas/top", get(top_artists))
        .route("/artistas/:slug", get(get_artist))
        .route(
            "/sample-discovery/relacion/:sample_id",
            get(get_relation_by_sample),
        )
        .route("/sample-discovery/estadisticas", get(relation_stats))
        .route("/relaciones/:id", get(get_relation))
        .route(
            "/relaciones/:id/vincular-sample",
            post(link_sample_to_relation),
        )
        .route(
            "/relaciones/:id/sample/:lado",
            delete(unlink_sample_from_relation),
        )
        .route("/relaciones/:id/verificar", put(verify_relation))
        .route("/admin/artistas", post(create_artist))
        .route(
            "/admin/artistas/:id",
            put(update_artist).delete(delete_artist),
        )
        .route("/admin/canciones", post(create_song))
        .route("/admin/canciones/:id", put(update_song).delete(delete_song))
        .route("/admin/relaciones", post(create_relation))
        .route(
            "/admin/relaciones/:id",
            put(update_relation).delete(delete_relation),
        )
}
