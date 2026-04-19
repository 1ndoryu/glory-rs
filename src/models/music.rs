use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MusicArtistRole {
    Principal,
    Featuring,
    Producer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SampleRelationType {
    Sample,
    Cover,
    Remix,
    Interpolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SampleRelationElementType {
    HookRiff,
    VocalsLyrics,
    Drums,
    Bass,
    KeysSynth,
    SoundEffect,
    MultipleElements,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SampleRelationSource {
    Scraping,
    Comunidad,
    Musicbrainz,
    Import,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RelationSampleSide {
    Fuente,
    Destino,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MusicArtist {
    pub id: i32,
    pub nombre: String,
    pub slug: String,
    pub imagen_url: Option<String>,
    pub whosampled_slug: Option<String>,
    pub musicbrainz_id: Option<String>,
    #[schema(value_type = Object)]
    pub metadata: serde_json::Value,
    pub prioridad: i16,
    pub total_canciones: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MusicSong {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub artista_id: i32,
    pub album: Option<String>,
    pub sello: Option<String>,
    pub anio: Option<i16>,
    pub duracion_segundos: Option<i16>,
    pub genero: Option<String>,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub imagen_url: Option<String>,
    pub whosampled_url: Option<String>,
    pub bpm: Option<i16>,
    pub tonalidad: Option<String>,
    #[schema(value_type = Object)]
    pub metadata: serde_json::Value,
    pub total_sampleada: i32,
    pub total_samplea: i32,
    pub total_likes: i32,
    pub total_comentarios: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub artista_nombre: String,
    pub artista_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SongArtistLink {
    pub artista_id: i32,
    pub nombre: String,
    pub slug: String,
    pub rol: MusicArtistRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SampleRelationSummary {
    pub id: i32,
    pub cancion_destino_id: i32,
    pub cancion_fuente_id: i32,
    pub whosampled_id: Option<i32>,
    pub tipo_relacion: SampleRelationType,
    pub tipo_elemento: Option<SampleRelationElementType>,
    pub timings_destino: Vec<i32>,
    pub timings_fuente: Vec<i32>,
    pub aparece_en_todo: bool,
    pub sample_id: Option<i32>,
    pub sample_fuente_id: Option<i32>,
    pub sample_destino_id: Option<i32>,
    pub votos_total: i32,
    pub votos_promedio: f64,
    pub fuente: SampleRelationSource,
    pub contribuidor_id: Option<i32>,
    pub contribuidor_username: Option<String>,
    pub verificada: bool,
    pub total_likes: i32,
    pub total_comentarios: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cancion_titulo: String,
    pub cancion_slug: String,
    pub artista_nombre: String,
    pub artista_slug: String,
    pub cancion_anio: Option<i16>,
    pub cancion_imagen_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SampleRelationDetail {
    pub id: i32,
    pub cancion_destino_id: i32,
    pub cancion_fuente_id: i32,
    pub whosampled_id: Option<i32>,
    pub tipo_relacion: SampleRelationType,
    pub tipo_elemento: Option<SampleRelationElementType>,
    pub timings_destino: Vec<i32>,
    pub timings_fuente: Vec<i32>,
    pub aparece_en_todo: bool,
    pub sample_id: Option<i32>,
    pub sample_fuente_id: Option<i32>,
    pub sample_destino_id: Option<i32>,
    pub votos_total: i32,
    pub votos_promedio: f64,
    pub fuente: SampleRelationSource,
    pub contribuidor_id: Option<i32>,
    pub contribuidor_username: Option<String>,
    pub verificada: bool,
    pub total_likes: i32,
    pub total_comentarios: i32,
    pub total_samples: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub fuente_titulo: String,
    pub fuente_slug: String,
    pub fuente_anio: Option<i16>,
    pub fuente_imagen_url: Option<String>,
    pub fuente_youtube_id: Option<String>,
    pub fuente_spotify_id: Option<String>,
    pub fuente_album: Option<String>,
    pub fuente_genero: Option<String>,
    pub fuente_artista: String,
    pub fuente_artista_slug: String,
    pub destino_titulo: String,
    pub destino_slug: String,
    pub destino_anio: Option<i16>,
    pub destino_imagen_url: Option<String>,
    pub destino_youtube_id: Option<String>,
    pub destino_spotify_id: Option<String>,
    pub destino_album: Option<String>,
    pub destino_genero: Option<String>,
    pub destino_artista: String,
    pub destino_artista_slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destino_samples_de: Option<Vec<SampleRelationSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destino_sampleada_en: Option<Vec<SampleRelationSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuente_samples_de: Option<Vec<SampleRelationSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuente_sampleada_en: Option<Vec<SampleRelationSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lado_extraccion: Option<RelationSampleSide>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MusicPagination {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MusicSongsResponse {
    pub data: Vec<MusicSong>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SongListResponse {
    pub data: Vec<MusicSong>,
    pub pagination: MusicPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MusicArtistsResponse {
    pub data: Vec<MusicArtist>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SongDetailResponse {
    pub cancion: MusicSong,
    pub artistas: Vec<SongArtistLink>,
    pub samples_de: Vec<SampleRelationSummary>,
    pub sampleada_en: Vec<SampleRelationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArtistStats {
    pub total_sampleado_por: usize,
    pub total_samplea_a: usize,
    pub generos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArtistDetailResponse {
    pub artista: MusicArtist,
    pub canciones: Vec<MusicSong>,
    pub sampleado_por: Vec<SampleRelationSummary>,
    pub samplea_a: Vec<SampleRelationSummary>,
    pub estadisticas: ArtistStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelationTypeCount {
    pub tipo_relacion: SampleRelationType,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelationStatsResponse {
    pub relaciones_por_tipo: Vec<RelationTypeCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SampleRelationLookupResponse {
    pub data: Option<SampleRelationDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelationChainNode {
    pub id: i32,
    pub cancion_fuente_id: i32,
    pub cancion_destino_id: i32,
    pub tipo_relacion: SampleRelationType,
    pub nivel: i32,
    pub fuente_titulo: String,
    pub fuente_slug: String,
    pub fuente_artista: String,
    pub destino_titulo: String,
    pub destino_slug: String,
    pub destino_artista: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelationChainResponse {
    pub cancion_raiz: MusicSong,
    pub cadena: Vec<RelationChainNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MusicMutationResponse {
    pub ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RelationVerificationResponse {
    pub ok: bool,
    pub verificada: bool,
}

#[derive(Debug, Clone, Deserialize, IntoParams, Validate, ToSchema)]
pub struct ListSongsQuery {
    #[serde(default)]
    #[validate(range(min = 1, max = 10_000))]
    pub page: Option<i64>,
    #[serde(default)]
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams, Validate, ToSchema)]
pub struct SearchSongsQuery {
    #[validate(length(min = 1, max = 120))]
    pub q: String,
    #[serde(default, alias = "limit")]
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams, Validate, ToSchema)]
pub struct LimitQuery {
    #[serde(default)]
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams, Validate, ToSchema)]
pub struct RelationChainQuery {
    #[serde(default)]
    #[validate(range(min = 1, max = 10))]
    pub profundidad: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateArtistRequest {
    #[validate(length(min = 1, max = 300))]
    pub nombre: String,
    #[validate(length(min = 1, max = 350))]
    pub slug: Option<String>,
    #[validate(length(max = 2_000))]
    pub imagen_url: Option<String>,
    #[validate(length(max = 350))]
    pub whosampled_slug: Option<String>,
    #[validate(length(max = 36))]
    pub musicbrainz_id: Option<String>,
    #[schema(value_type = Object)]
    pub metadata: Option<serde_json::Value>,
    #[validate(range(min = 0, max = 999))]
    pub prioridad: Option<i16>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema, Default)]
pub struct UpdateArtistRequest {
    #[validate(length(min = 1, max = 300))]
    pub nombre: Option<String>,
    #[validate(length(min = 1, max = 350))]
    pub slug: Option<String>,
    #[validate(length(max = 2_000))]
    pub imagen_url: Option<String>,
    #[validate(length(max = 350))]
    pub whosampled_slug: Option<String>,
    #[validate(length(max = 36))]
    pub musicbrainz_id: Option<String>,
    #[schema(value_type = Object)]
    pub metadata: Option<serde_json::Value>,
    #[validate(range(min = 0, max = 999))]
    pub prioridad: Option<i16>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct SongArtistInput {
    #[validate(range(min = 1))]
    pub artista_id: i32,
    pub rol: MusicArtistRole,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateSongRequest {
    #[validate(length(min = 1, max = 500))]
    pub titulo: String,
    #[validate(length(min = 1, max = 550))]
    pub slug: Option<String>,
    #[validate(range(min = 1))]
    pub artista_id: i32,
    #[validate(length(max = 500))]
    pub album: Option<String>,
    #[validate(length(max = 200))]
    pub sello: Option<String>,
    #[validate(range(min = 0, max = 3000))]
    pub anio: Option<i32>,
    #[validate(range(min = 0, max = 7200))]
    pub duracion_segundos: Option<i32>,
    #[validate(length(max = 100))]
    pub genero: Option<String>,
    #[validate(length(max = 20))]
    pub youtube_id: Option<String>,
    #[validate(length(max = 30))]
    pub spotify_id: Option<String>,
    #[validate(length(max = 2_000))]
    pub imagen_url: Option<String>,
    #[validate(length(max = 500))]
    pub whosampled_url: Option<String>,
    #[validate(range(min = 0, max = 300))]
    pub bpm: Option<i32>,
    #[validate(length(max = 5))]
    pub tonalidad: Option<String>,
    #[schema(value_type = Object)]
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub artistas: Vec<SongArtistInput>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema, Default)]
pub struct UpdateSongRequest {
    #[validate(length(min = 1, max = 500))]
    pub titulo: Option<String>,
    #[validate(length(min = 1, max = 550))]
    pub slug: Option<String>,
    #[validate(range(min = 1))]
    pub artista_id: Option<i32>,
    #[validate(length(max = 500))]
    pub album: Option<String>,
    #[validate(length(max = 200))]
    pub sello: Option<String>,
    #[validate(range(min = 0, max = 3000))]
    pub anio: Option<i32>,
    #[validate(range(min = 0, max = 7200))]
    pub duracion_segundos: Option<i32>,
    #[validate(length(max = 100))]
    pub genero: Option<String>,
    #[validate(length(max = 20))]
    pub youtube_id: Option<String>,
    #[validate(length(max = 30))]
    pub spotify_id: Option<String>,
    #[validate(length(max = 2_000))]
    pub imagen_url: Option<String>,
    #[validate(length(max = 500))]
    pub whosampled_url: Option<String>,
    #[validate(range(min = 0, max = 300))]
    pub bpm: Option<i32>,
    #[validate(length(max = 5))]
    pub tonalidad: Option<String>,
    #[schema(value_type = Object)]
    pub metadata: Option<serde_json::Value>,
    pub artistas: Option<Vec<SongArtistInput>>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateRelationRequest {
    #[validate(range(min = 1))]
    pub cancion_destino_id: i32,
    #[validate(range(min = 1))]
    pub cancion_fuente_id: i32,
    pub whosampled_id: Option<i32>,
    pub tipo_relacion: SampleRelationType,
    pub tipo_elemento: Option<SampleRelationElementType>,
    #[serde(default)]
    pub timings_destino: Vec<i32>,
    #[serde(default)]
    pub timings_fuente: Vec<i32>,
    pub aparece_en_todo: Option<bool>,
    pub sample_id: Option<i32>,
    pub sample_fuente_id: Option<i32>,
    pub sample_destino_id: Option<i32>,
    pub votos_total: Option<i32>,
    pub votos_promedio: Option<f64>,
    pub fuente: Option<SampleRelationSource>,
    pub contribuidor_id: Option<i32>,
    pub verificada: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema, Default)]
pub struct UpdateRelationRequest {
    pub cancion_destino_id: Option<i32>,
    pub cancion_fuente_id: Option<i32>,
    pub whosampled_id: Option<i32>,
    pub tipo_relacion: Option<SampleRelationType>,
    pub tipo_elemento: Option<SampleRelationElementType>,
    pub timings_destino: Option<Vec<i32>>,
    pub timings_fuente: Option<Vec<i32>>,
    pub aparece_en_todo: Option<bool>,
    pub sample_id: Option<i32>,
    pub sample_fuente_id: Option<i32>,
    pub sample_destino_id: Option<i32>,
    pub votos_total: Option<i32>,
    pub votos_promedio: Option<f64>,
    pub fuente: Option<SampleRelationSource>,
    pub contribuidor_id: Option<i32>,
    pub verificada: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct SampleLinkRequest {
    #[validate(range(min = 1))]
    pub sample_id: i32,
    pub lado: RelationSampleSide,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct VerifyRelationRequest {
    pub verificada: bool,
}