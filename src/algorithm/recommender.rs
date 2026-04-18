/* [174A-52] MotorRecomendacion portado del legado Kamples (1013 LoC PHP →
 * arquitectura limpia en Rust). Ver plan completo en
 * `Agente/planes/plan-recommender-2026-04-18.md`.
 *
 * DECISIÓN ARQUITECTÓNICA CLAVE
 * ─────────────────────────────
 * El legado generaba SQL gigante con scoring inline (100+ líneas de SQL por
 * request, con CTEs encadenadas, ROW_NUMBER, EXISTS correlacionados).
 * Aquí invertimos: SQL hace solo lo que SQL hace bien (filtrar, agregar,
 * traer datos enriquecidos para un set de IDs), y el scoring corre en Rust
 * usando `algorithm::signals` (ya tipado y testeado en 174A-49).
 *
 * Beneficios:
 *   - Scoring testeable sin BD (ya cubierto por tests de signals).
 *   - SQL parametrizado y validado en compile-time vía sqlx::query!.
 *   - Diversidad y serendipia se aplican como funciones puras.
 *   - El recommender es composable: la Fase 5 podrá inyectar afinidad por
 *     tags (174A-54) y la Fase 4 reutiliza el mismo enriquecimiento.
 *
 * 5 fases del plan, todas en este módulo (split por commits no por archivos):
 *   Fase 1: API + cache stale-while-revalidate + scoring + fallback.
 *   Fase 2: Bulk-fetch (3 páginas calculadas en una sola pasada).
 *   Fase 3: Warm async con tokio::spawn + lock SETNX.
 *   Fase 4: similar_to_sample (motor "más como esto").
 *   Fase 5: hooks de invalidación (invalidate_user_feed). */

/* Permitidos a nivel de módulo: el dominio es algoritmo numérico con muchos
 * casts entre usize/i64/f64 (coordenadas, conteos, scores) y queries SQL
 * grandes son inherentes al motor. Otros lints siguen activos. */
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::map_unwrap_or,
    clippy::explicit_iter_loop,
    clippy::too_many_lines,
    clippy::large_types_passed_by_value,
    clippy::type_complexity
)]

use std::collections::HashSet;
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use deadpool_redis::Pool as RedisPool;
use pgvector::Vector;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, warn};
use utoipa::ToSchema;

use crate::errors::AppError;

use super::candidates::{CandidatesConfig, CandidatesService};
use super::profile::{ProfileService, UserProfile};
use super::signals::{
    content_similarity_score, novelty_signal_score, AlgorithmSignalConfig, AlgorithmSignalInput,
    BehaviorSignalInput, ContextSignalInput, SocialSignalInput, TrendSignalInput,
};

const CACHE_PREFIX_FRESH: &str = "kamples_feed_";
const CACHE_PREFIX_STALE: &str = "kamples_feed_stale_";
const WARM_LOCK_PREFIX: &str = "kamples_warm_feed_";
const PAGINAS_BULK: usize = 3;

static MEMORY_CACHE: LazyLock<Arc<DashMap<String, (Vec<RankedSample>, Instant)>>> =
    LazyLock::new(|| Arc::new(DashMap::new()));
static MEMORY_LOCKS: LazyLock<Arc<DashMap<String, Instant>>> =
    LazyLock::new(|| Arc::new(DashMap::new()));

#[derive(Debug, Clone, Copy)]
pub struct RecommenderConfig {
    /// TTL del cache fresco para página 0 (en segundos).
    pub fresh_ttl_p0: u64,
    /// TTL del cache fresco para páginas paginadas (en segundos).
    pub fresh_ttl_pn: u64,
    /// TTL del cache stale (escudo para responder rápido mientras recalcula).
    pub stale_ttl: u64,
    /// TTL del lock distribuido para warm async.
    pub warm_lock_ttl: u64,
    /// Umbral para activar el `SelectorCandidatos` (si total_activos >).
    pub umbral_candidatos: i64,
    /// Máx por creador (diversidad).
    pub max_por_creador: usize,
    /// Máx por género (diversidad).
    pub max_por_categoria: usize,
    /// Máx por tipo (diversidad — solo penaliza one-shots).
    pub max_por_tipo: usize,
    /// Configuración de límites de las 6 fuentes de candidatos.
    pub candidates: CandidatesConfig,
    /// Pesos y subpesos del scoring de las 6 señales.
    pub signal: AlgorithmSignalConfig,
}

impl RecommenderConfig {
    /// Valores idénticos al legado `algoritmoPesos.php`.
    pub const fn legacy_defaults() -> Self {
        Self {
            fresh_ttl_p0: 300,
            fresh_ttl_pn: 900,
            stale_ttl: 7200,
            warm_lock_ttl: 90,
            umbral_candidatos: 5000,
            max_por_creador: 3,
            max_por_categoria: 4,
            max_por_tipo: 5,
            candidates: CandidatesConfig::legacy_defaults(),
            signal: AlgorithmSignalConfig::legacy_current(),
        }
    }
}

impl Default for RecommenderConfig {
    fn default() -> Self {
        Self::legacy_defaults()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct RankedSample {
    pub id: i32,
    pub creador_id: i32,
    pub titulo: String,
    pub slug: String,
    pub tipo: String,
    pub bpm: Option<i32>,
    pub key: Option<String>,
    pub escala: Option<String>,
    pub tags: Vec<String>,
    pub publicado_at: Option<DateTime<Utc>>,
    pub total_likes: i32,
    pub total_reproducciones: i32,
    pub total_descargas: i32,
    pub verificado: bool,
    pub es_nuevo: bool,
    pub score: f64,
}

#[derive(Debug, Clone)]
struct SampleRow {
    id: i32,
    creador_id: i32,
    titulo: String,
    slug: String,
    tipo: String,
    bpm: Option<i32>,
    key: Option<String>,
    escala: Option<String>,
    tags: Vec<String>,
    publicado_at: Option<DateTime<Utc>>,
    total_likes: i32,
    total_reproducciones: i32,
    total_descargas: i32,
    verificado: bool,
    embedding_distance: Option<f64>,
    likes_24h: f64,
    reproducciones_24h: f64,
    descargas_7d: f64,
    follows_creador_7d: f64,
    creador_seguido: bool,
    likes_dados: f64,
    reproducciones: f64,
    tiempo_escucha: f64,
    descargas_propias: f64,
    completadas: f64,
    dislike_penalty: f64,
    nunca_reproducido: bool,
}

pub struct RecommenderService;

impl RecommenderService {
    /// Punto de entrada del feed personalizado. Política:
    /// 1. Mira cache fresh → si hit, devuelve.
    /// 2. Mira cache stale → si hit, devuelve + spawn warm async.
    /// 3. Computa síncronamente (`compute_feed_bulk`) y devuelve.
    pub async fn feed(
        pool: PgPool,
        redis: Option<RedisPool>,
        user_id: i32,
        limit: usize,
        offset: usize,
        config: &RecommenderConfig,
    ) -> Result<Vec<RankedSample>, AppError> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        /* [174A-57] Profiling fino para usuario QA. No-op para el resto. */
        crate::services::algo_timing::ALGO_TIMING.start(user_id);

        let cache_key = fresh_cache_key(user_id, limit, offset);
        if let Some(hit) = cache_get(&redis, &cache_key).await? {
            debug!(target: "kamples.feed", "cache hit fresh: {}", cache_key);
            crate::services::algo_timing::ALGO_TIMING.mark(user_id, "cache_fresh_hit");
            crate::services::algo_timing::ALGO_TIMING.save(
                user_id,
                serde_json::json!({"source": "cache_fresh", "items": hit.len()}),
            );
            return Ok(hit);
        }
        crate::services::algo_timing::ALGO_TIMING.mark(user_id, "cache_fresh_miss");

        let stale_key = stale_cache_key(user_id, limit, offset);
        if let Some(stale) = cache_get(&redis, &stale_key).await? {
            debug!(target: "kamples.feed", "cache hit stale: {}", stale_key);
            crate::services::algo_timing::ALGO_TIMING.mark(user_id, "cache_stale_hit");
            spawn_warm(pool.clone(), redis.clone(), user_id, limit, offset, *config);
            crate::services::algo_timing::ALGO_TIMING.save(
                user_id,
                serde_json::json!({"source": "cache_stale", "items": stale.len()}),
            );
            return Ok(stale);
        }
        crate::services::algo_timing::ALGO_TIMING.mark(user_id, "cache_stale_miss");

        let items = compute_and_cache(&pool, &redis, user_id, limit, offset, config).await?;
        crate::services::algo_timing::ALGO_TIMING.mark(user_id, "compute_and_cache");
        crate::services::algo_timing::ALGO_TIMING.save(
            user_id,
            serde_json::json!({"source": "compute", "items": items.len(), "limit": limit, "offset": offset}),
        );
        Ok(items)
    }

    /// "Más como esto": top N samples similares a uno dado, vía pgvector.
    /// Si el sample no tiene embedding o pgvector está deshabilitado, cae a
    /// scoring por overlap de tags + match de tipo (más simple que el legado).
    pub async fn similar_to_sample(
        pool: &PgPool,
        sample_id: i32,
        limit: i64,
        viewer_id: Option<i32>,
    ) -> Result<Vec<RankedSample>, AppError> {
        let blocked: Vec<i32> = if let Some(uid) = viewer_id {
            crate::repositories::ModerationRepository::list_blocked(pool, uid).await?
        } else {
            Vec::new()
        };

        let seed = sqlx::query!(
            r#"SELECT embedding AS "embedding?: Vector", tipo AS "tipo!", tags AS "tags!" FROM samples WHERE id = $1"#,
            sample_id
        )
        .fetch_optional(pool)
        .await?;

        let Some(seed) = seed else {
            return Ok(Vec::new());
        };

        if let Some(vector) = seed.embedding.as_ref() {
            let rows = fetch_similar_by_embedding(pool, sample_id, vector, &blocked, limit).await?;
            if !rows.is_empty() {
                return Ok(rows
                    .into_iter()
                    .map(|row| {
                        let dist = row.embedding_distance.unwrap_or(2.0);
                        let score = content_similarity_score(Some(dist));
                        row_to_ranked(row, score, true)
                    })
                    .collect());
            }
        }

        let rows = fetch_similar_by_tags(pool, sample_id, &seed.tags, &seed.tipo, &blocked, limit)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| {
                let overlap = jaccard_overlap(&row.tags, &seed.tags);
                let tipo_match = if row.tipo == seed.tipo { 0.4 } else { 0.0 };
                row_to_ranked(row, overlap * 0.6 + tipo_match, true)
            })
            .collect())
    }

    /// Invalida el cache fresco del feed de un usuario (todas las páginas).
    /// El stale se conserva para servir respuesta instantánea mientras
    /// recalcula. Llamar al dar like, descarga, publicar nuevo sample.
    pub async fn invalidate_user_feed(
        redis: &Option<RedisPool>,
        user_id: i32,
    ) -> Result<(), AppError> {
        let prefix = format!("{CACHE_PREFIX_FRESH}{user_id}_");
        if let Some(pool) = redis {
            let mut conn = pool
                .get()
                .await
                .map_err(|e| AppError::Internal(format!("Redis pool: {e}")))?;
            let keys: Vec<String> = conn
                .keys(format!("{prefix}*"))
                .await
                .map_err(|e| AppError::Internal(format!("Redis KEYS: {e}")))?;
            if !keys.is_empty() {
                let _: () = conn
                    .del(keys)
                    .await
                    .map_err(|e| AppError::Internal(format!("Redis DEL: {e}")))?;
            }
        } else {
            MEMORY_CACHE.retain(|k, _| !k.starts_with(&prefix));
        }
        Ok(())
    }

    /// Variante "global" (todos los usuarios). Útil al publicar sample nuevo
    /// para invalidar todos los feeds frescos. El stale se mantiene.
    pub async fn invalidate_global(redis: &Option<RedisPool>) -> Result<(), AppError> {
        if let Some(pool) = redis {
            let mut conn = pool
                .get()
                .await
                .map_err(|e| AppError::Internal(format!("Redis pool: {e}")))?;
            let keys: Vec<String> = conn
                .keys(format!("{CACHE_PREFIX_FRESH}*"))
                .await
                .map_err(|e| AppError::Internal(format!("Redis KEYS: {e}")))?;
            if !keys.is_empty() {
                let _: () = conn
                    .del(keys)
                    .await
                    .map_err(|e| AppError::Internal(format!("Redis DEL: {e}")))?;
            }
        } else {
            MEMORY_CACHE.retain(|k, _| !k.starts_with(CACHE_PREFIX_FRESH));
        }
        Ok(())
    }
}

/* ───────────── helpers internos ───────────── */

async fn compute_and_cache(
    pool: &PgPool,
    redis: &Option<RedisPool>,
    user_id: i32,
    limit: usize,
    offset: usize,
    config: &RecommenderConfig,
) -> Result<Vec<RankedSample>, AppError> {
    let max_bulk_offset = limit * (PAGINAS_BULK - 1);
    let usar_bulk = offset <= max_bulk_offset;

    let pages = if usar_bulk {
        compute_feed_bulk(pool, redis, user_id, limit, config).await?
    } else {
        let page = compute_feed_page(pool, redis, user_id, limit, offset, config).await?;
        vec![(offset, page)]
    };

    let mut response = Vec::new();
    for (page_offset, page) in pages {
        if page_offset == offset {
            response.clone_from(&page);
        }
        cache_set(redis, &fresh_cache_key(user_id, limit, page_offset), &page, fresh_ttl(config, page_offset))
            .await?;
        cache_set(
            redis,
            &stale_cache_key(user_id, limit, page_offset),
            &page,
            config.stale_ttl,
        )
        .await?;
    }
    Ok(response)
}

/// Bulk: calcula `PAGINAS_BULK * limit` candidatos rankeados en una sola
/// pasada y los splittea en páginas. Replica `PAGINAS_BULK = 3` del legado.
async fn compute_feed_bulk(
    pool: &PgPool,
    redis: &Option<RedisPool>,
    user_id: i32,
    limit: usize,
    config: &RecommenderConfig,
) -> Result<Vec<(usize, Vec<RankedSample>)>, AppError> {
    let target = limit * PAGINAS_BULK;
    let ranked = score_pipeline(pool, redis, user_id, target * 2, config).await?;
    let final_ranked = apply_diversity(ranked, config);
    let trimmed: Vec<RankedSample> = final_ranked.into_iter().take(target).collect();

    let mut out = Vec::with_capacity(PAGINAS_BULK);
    for page in 0..PAGINAS_BULK {
        let start = page * limit;
        if start >= trimmed.len() {
            out.push((start, Vec::new()));
            continue;
        }
        let end = (start + limit).min(trimmed.len());
        out.push((start, trimmed[start..end].to_vec()));
    }
    Ok(out)
}

/// Página individual (offset > rango bulk). Calcula `limit*2` para diversidad
/// y trimea al límite solicitado, replicando la rama non-bulk del legado.
async fn compute_feed_page(
    pool: &PgPool,
    redis: &Option<RedisPool>,
    user_id: i32,
    limit: usize,
    offset: usize,
    config: &RecommenderConfig,
) -> Result<Vec<RankedSample>, AppError> {
    let target = (limit + offset) * 2;
    let ranked = score_pipeline(pool, redis, user_id, target, config).await?;
    let final_ranked = apply_diversity(ranked, config);
    Ok(final_ranked.into_iter().skip(offset).take(limit).collect())
}

async fn score_pipeline(
    pool: &PgPool,
    redis: &Option<RedisPool>,
    user_id: i32,
    target: usize,
    config: &RecommenderConfig,
) -> Result<Vec<RankedSample>, AppError> {
    let profile = ProfileService::build(pool, redis, user_id).await?;

    let total_active = CandidatesService::count_active(pool, redis).await?;
    let usar_candidatos = total_active > config.umbral_candidatos;

    let candidate_ids: Option<Vec<i32>> = if usar_candidatos {
        Some(
            CandidatesService::select(pool, user_id, &profile, None, &config.candidates).await?,
        )
    } else {
        None
    };

    /* Si el usuario es cold-start, atajo: feed por trending recientes
     * (replica `feedNuevoUsuario` del legado, sin scoring complejo). */
    if profile.is_cold_start() {
        return fallback_recientes(pool, user_id, target, candidate_ids.as_deref()).await;
    }

    let rows = fetch_enriched(pool, user_id, candidate_ids.as_deref(), target * 3).await?;
    if rows.is_empty() {
        return fallback_recientes(pool, user_id, target, candidate_ids.as_deref()).await;
    }

    let signal_cfg = config.signal;
    let mut ranked: Vec<RankedSample> = rows
        .into_iter()
        .map(|row| score_row(row, &profile, signal_cfg))
        .collect();

    /* Orden primario: es_nuevo DESC (no reproducidos primero), luego score. */
    ranked.sort_by(|a, b| {
        b.es_nuevo
            .cmp(&a.es_nuevo)
            .then(b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal))
    });
    Ok(ranked)
}

fn score_row(
    row: SampleRow,
    profile: &UserProfile,
    config: AlgorithmSignalConfig,
) -> RankedSample {
    let dias_desde_pub = row.publicado_at.map(|ts| {
        let secs = (Utc::now() - ts).num_seconds().max(0) as f64;
        secs / 86_400.0
    });

    let bpm_match = row.bpm.map(f64::from);
    let key_match = match (&row.key, &profile.key_fav) {
        (Some(a), Some(b)) => Some(a.eq_ignore_ascii_case(b)),
        _ => None,
    };
    let escala_match = match (&row.escala, &profile.scale_fav) {
        (Some(a), Some(b)) => Some(a.eq_ignore_ascii_case(b)),
        _ => None,
    };
    let tipo_match = profile
        .type_fav
        .as_deref()
        .map(|t| t.eq_ignore_ascii_case(&row.tipo));
    let creador_afin = profile.favorite_creators.contains(&row.creador_id);

    let genero_match = if profile.declared_genres.is_empty() {
        0.0
    } else {
        let row_lower: HashSet<String> = row.tags.iter().map(|t| t.to_lowercase()).collect();
        let hits = profile
            .declared_genres
            .iter()
            .filter(|g| row_lower.contains(&g.to_lowercase()))
            .count();
        (hits as f64) / (profile.declared_genres.len() as f64)
    };

    let signal_input = AlgorithmSignalInput {
        distancia_coseno_contenido: row.embedding_distance,
        comportamiento: BehaviorSignalInput {
            likes_dados: row.likes_dados,
            reproducciones: row.reproducciones,
            tiempo_escucha: row.tiempo_escucha,
            descargas: row.descargas_propias,
            completadas: row.completadas,
            dislike_penalty: row.dislike_penalty,
        },
        contexto: ContextSignalInput {
            bpm_candidato: bpm_match,
            bpm_promedio_usuario: profile.bpm_avg.map(f64::from),
            key_match,
            escala_match,
            genero_match,
            tipo_match,
            creador_afin,
        },
        tendencias: TrendSignalInput {
            likes_24h: row.likes_24h,
            reproducciones_24h: row.reproducciones_24h,
            descargas_7d: row.descargas_7d,
            follows_creador_7d: row.follows_creador_7d,
        },
        grafo_social: SocialSignalInput {
            creador_seguido: row.creador_seguido,
            puntos_reacciones_seguidos: 0.0, /* se conectará en 174A-54 */
        },
        dias_desde_publicacion: dias_desde_pub,
    };
    let breakdown = config.score(signal_input);

    /* Multiplicadores que en el legado iban inline en SQL: verificado, novedad
     * (parte del breakdown) y boost para no reproducidos. */
    let mult_verificado = if row.verificado { 1.15 } else { 1.0 };
    let mult_no_reproducido = if row.nunca_reproducido { 1.20 } else { 1.0 };
    let mut total = breakdown.total * mult_verificado * mult_no_reproducido;

    /* Garantizar score positivo (signals puede dar negativo en dislike). */
    if total.is_nan() || total.is_infinite() {
        total = 0.0;
    }

    row_to_ranked(row, total, false)
}

fn row_to_ranked(row: SampleRow, score: f64, force_es_nuevo: bool) -> RankedSample {
    let es_nuevo = force_es_nuevo || row.nunca_reproducido;
    RankedSample {
        id: row.id,
        creador_id: row.creador_id,
        titulo: row.titulo,
        slug: row.slug,
        tipo: row.tipo,
        bpm: row.bpm,
        key: row.key,
        escala: row.escala,
        tags: row.tags,
        publicado_at: row.publicado_at,
        total_likes: row.total_likes,
        total_reproducciones: row.total_reproducciones,
        total_descargas: row.total_descargas,
        verificado: row.verificado,
        es_nuevo,
        score,
    }
}

/// Diversidad post-scoring: penaliza creador/género/tipo repetidos en el
/// orden actual (paridad con `aplicarDiversidadPHP` legacy, simplificado).
fn apply_diversity(mut ranked: Vec<RankedSample>, config: &RecommenderConfig) -> Vec<RankedSample> {
    let mut count_creador: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    let mut count_genero: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let mut count_tipo: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for item in ranked.iter_mut() {
        let rc = count_creador.entry(item.creador_id).or_insert(0);
        *rc += 1;
        let factor_creador = if *rc <= config.max_por_creador {
            1.0
        } else {
            (1.0 - (*rc - config.max_por_creador) as f64 * 0.15).max(0.3)
        };

        /* Género dominante: primer tag (proxy razonable; el legado leía
         * `metadata->'genero'` JSONB, aquí mantenemos paridad funcional). */
        let genero = item
            .tags
            .first()
            .map(|t| t.to_lowercase())
            .unwrap_or_else(|| "other".to_string());
        let rg = count_genero.entry(genero).or_insert(0);
        *rg += 1;
        let factor_genero = if *rg <= config.max_por_categoria {
            1.0
        } else {
            (1.0 - (*rg - config.max_por_categoria) as f64 * 0.10).max(0.5)
        };

        let rt = count_tipo.entry(item.tipo.clone()).or_insert(0);
        *rt += 1;
        let factor_tipo = if item.tipo == "oneshot" && *rt > config.max_por_tipo {
            (1.0 - (*rt - config.max_por_tipo) as f64 * 0.12).max(0.5)
        } else {
            1.0
        };

        item.score *= factor_creador * factor_genero * factor_tipo;
    }

    ranked.sort_by(|a, b| {
        b.es_nuevo
            .cmp(&a.es_nuevo)
            .then(b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal))
    });
    ranked
}

async fn fetch_enriched(
    pool: &PgPool,
    user_id: i32,
    candidates: Option<&[i32]>,
    limit: usize,
) -> Result<Vec<SampleRow>, AppError> {
    /* Una sola query agregada que devuelve todo lo que el scoring necesita.
     * Filtra por candidatos si están definidos, o por `estado = 'activo'`
     * cruzando bloqueos bidireccionales. Las métricas de comportamiento se
     * agregan con LEFT JOIN LATERAL para no inflar la cardinalidad. */
    let limit_i64 = limit as i64;
    let blocked = collect_blocked(pool, user_id).await?;

    if let Some(ids) = candidates {
        let rows = sqlx::query!(
            r#"
            SELECT
                s.id            AS "id!",
                s.creador_id    AS "creador_id!",
                s.titulo        AS "titulo!",
                s.slug          AS "slug!",
                s.tipo          AS "tipo!",
                s.bpm           AS "bpm?",
                s.key           AS "key?",
                s.escala        AS "escala?",
                s.tags          AS "tags!",
                s.publicado_at  AS "publicado_at?",
                s.total_likes,
                s.total_reproducciones,
                s.total_descargas,
                s.verificado,
                NULL::float8    AS "embedding_distance?",
                COALESCE(mv.likes_24h, 0)::float8          AS "likes_24h!",
                COALESCE(mv.reproducciones_24h, 0)::float8 AS "reproducciones_24h!",
                COALESCE(mv.descargas_7d, 0)::float8       AS "descargas_7d!",
                COALESCE(mv.follows_creador_7d, 0)::float8 AS "follows_creador_7d!",
                EXISTS (SELECT 1 FROM follows f WHERE f.seguidor_id = $1 AND f.seguido_id = s.creador_id) AS "creador_seguido!",
                COALESCE(b.likes_dados, 0)::float8         AS "likes_dados!",
                COALESCE(b.reproducciones, 0)::float8      AS "reproducciones!",
                COALESCE(b.tiempo_escucha, 0)::float8      AS "tiempo_escucha!",
                COALESCE(b.descargas_propias, 0)::float8   AS "descargas_propias!",
                COALESCE(b.completadas, 0)::float8         AS "completadas!",
                COALESCE(b.dislike_penalty, 0)::float8     AS "dislike_penalty!",
                NOT EXISTS (SELECT 1 FROM reproducciones r WHERE r.usuario_id = $1 AND r.sample_id = s.id) AS "nunca_reproducido!"
            FROM samples s
            LEFT JOIN mv_trending_samples mv ON mv.sample_id = s.id
            LEFT JOIN LATERAL (
                SELECT
                    COUNT(DISTINCT lk.target_id) FILTER (WHERE lk.target_id = s.id) AS likes_dados,
                    COUNT(*) FILTER (WHERE rp.sample_id = s.id) AS reproducciones,
                    COALESCE(SUM(rp.duracion_escuchada) FILTER (WHERE rp.sample_id = s.id), 0)::float8 AS tiempo_escucha,
                    COUNT(*) FILTER (WHERE dl.sample_id = s.id) AS descargas_propias,
                    COUNT(*) FILTER (WHERE rp.sample_id = s.id AND rp.completada) AS completadas,
                    COUNT(*) FILTER (WHERE lk.target_id = s.id AND lk.reaccion = 'dislike') AS dislike_penalty
                FROM (SELECT 1) _
                LEFT JOIN likes lk          ON lk.usuario_id = $1 AND lk.tipo = 'sample'
                LEFT JOIN reproducciones rp ON rp.usuario_id = $1
                LEFT JOIN descargas dl      ON dl.usuario_id = $1
            ) b ON TRUE
            WHERE s.estado = 'activo'
              AND s.id = ANY($2::int[])
              AND s.creador_id <> ALL($3::int[])
            ORDER BY s.publicado_at DESC NULLS LAST
            LIMIT $4
            "#,
            user_id,
            ids,
            &blocked,
            limit_i64,
        )
        .fetch_all(pool)
        .await?;

        return Ok(rows
            .into_iter()
            .map(|r| SampleRow {
                id: r.id,
                creador_id: r.creador_id,
                titulo: r.titulo,
                slug: r.slug,
                tipo: r.tipo,
                bpm: r.bpm,
                key: r.key,
                escala: r.escala,
                tags: r.tags,
                publicado_at: r.publicado_at,
                total_likes: r.total_likes.unwrap_or(0),
                total_reproducciones: r.total_reproducciones.unwrap_or(0),
                total_descargas: r.total_descargas.unwrap_or(0),
                verificado: r.verificado.unwrap_or(false),
                embedding_distance: r.embedding_distance,
                likes_24h: r.likes_24h,
                reproducciones_24h: r.reproducciones_24h,
                descargas_7d: r.descargas_7d,
                follows_creador_7d: r.follows_creador_7d,
                creador_seguido: r.creador_seguido,
                likes_dados: r.likes_dados,
                reproducciones: r.reproducciones,
                tiempo_escucha: r.tiempo_escucha,
                descargas_propias: r.descargas_propias,
                completadas: r.completadas,
                dislike_penalty: r.dislike_penalty,
                nunca_reproducido: r.nunca_reproducido,
            })
            .collect());
    }

    /* Sin candidatos: scan completo con JOIN a la MV de tendencias. Limita
     * por `publicado_at DESC` para priorizar fresh y mantener cardinalidad
     * acotada. La MV puede no tener todas las filas (samples nuevos sin
     * refrescar aún); COALESCE garantiza fallback a 0. */
    let rows = sqlx::query!(
        r#"
        SELECT
            s.id            AS "id!",
            s.creador_id    AS "creador_id!",
            s.titulo        AS "titulo!",
            s.slug          AS "slug!",
            s.tipo          AS "tipo!",
            s.bpm           AS "bpm?",
            s.key           AS "key?",
            s.escala        AS "escala?",
            s.tags          AS "tags!",
            s.publicado_at  AS "publicado_at?",
            s.total_likes,
            s.total_reproducciones,
            s.total_descargas,
            s.verificado,
            NULL::float8    AS "embedding_distance?",
            COALESCE(mv.likes_24h, 0)::float8          AS "likes_24h!",
            COALESCE(mv.reproducciones_24h, 0)::float8 AS "reproducciones_24h!",
            COALESCE(mv.descargas_7d, 0)::float8       AS "descargas_7d!",
            COALESCE(mv.follows_creador_7d, 0)::float8 AS "follows_creador_7d!",
            EXISTS (SELECT 1 FROM follows f WHERE f.seguidor_id = $1 AND f.seguido_id = s.creador_id) AS "creador_seguido!",
            0.0::float8 AS "likes_dados!",
            0.0::float8 AS "reproducciones!",
            0.0::float8 AS "tiempo_escucha!",
            0.0::float8 AS "descargas_propias!",
            0.0::float8 AS "completadas!",
            0.0::float8 AS "dislike_penalty!",
            NOT EXISTS (SELECT 1 FROM reproducciones r WHERE r.usuario_id = $1 AND r.sample_id = s.id) AS "nunca_reproducido!"
        FROM samples s
        LEFT JOIN mv_trending_samples mv ON mv.sample_id = s.id
        WHERE s.estado = 'activo'
          AND s.creador_id <> ALL($2::int[])
        ORDER BY s.publicado_at DESC NULLS LAST
        LIMIT $3
        "#,
        user_id,
        &blocked,
        limit_i64,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| SampleRow {
            id: r.id,
            creador_id: r.creador_id,
            titulo: r.titulo,
            slug: r.slug,
            tipo: r.tipo,
            bpm: r.bpm,
            key: r.key,
            escala: r.escala,
            tags: r.tags,
            publicado_at: r.publicado_at,
            total_likes: r.total_likes.unwrap_or(0),
            total_reproducciones: r.total_reproducciones.unwrap_or(0),
            total_descargas: r.total_descargas.unwrap_or(0),
            verificado: r.verificado.unwrap_or(false),
            embedding_distance: r.embedding_distance,
            likes_24h: r.likes_24h,
            reproducciones_24h: r.reproducciones_24h,
            descargas_7d: r.descargas_7d,
            follows_creador_7d: r.follows_creador_7d,
            creador_seguido: r.creador_seguido,
            likes_dados: r.likes_dados,
            reproducciones: r.reproducciones,
            tiempo_escucha: r.tiempo_escucha,
            descargas_propias: r.descargas_propias,
            completadas: r.completadas,
            dislike_penalty: r.dislike_penalty,
            nunca_reproducido: r.nunca_reproducido,
        })
        .collect())
}

async fn fallback_recientes(
    pool: &PgPool,
    user_id: i32,
    limit: usize,
    candidates: Option<&[i32]>,
) -> Result<Vec<RankedSample>, AppError> {
    /* Cold start o scoring vacío: top trending del último mes. Usa la novedad
     * como score. Replica el espíritu de `feedNuevoUsuario` legacy con menos
     * complejidad (la diversidad la aplica `apply_diversity` posteriormente). */
    let blocked = collect_blocked(pool, user_id).await?;
    let limit_i64 = limit as i64;

    let rows = if let Some(ids) = candidates {
        sqlx::query!(
            r#"
            SELECT id AS "id!", creador_id AS "creador_id!", titulo AS "titulo!",
                   slug AS "slug!", tipo AS "tipo!", bpm AS "bpm?", key AS "key?",
                   escala AS "escala?", tags AS "tags!", publicado_at AS "publicado_at?",
                   total_likes, total_reproducciones, total_descargas, verificado
              FROM samples
             WHERE estado = 'activo'
               AND id = ANY($1::int[])
               AND creador_id <> ALL($2::int[])
             ORDER BY (total_likes * 2 + total_reproducciones + total_descargas * 3) DESC
             LIMIT $3
            "#,
            ids,
            &blocked,
            limit_i64,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| (r.id, r.creador_id, r.titulo, r.slug, r.tipo, r.bpm, r.key, r.escala, r.tags,
                  r.publicado_at, r.total_likes.unwrap_or(0), r.total_reproducciones.unwrap_or(0),
                  r.total_descargas.unwrap_or(0), r.verificado.unwrap_or(false)))
        .collect::<Vec<_>>()
    } else {
        sqlx::query!(
            r#"
            SELECT id AS "id!", creador_id AS "creador_id!", titulo AS "titulo!",
                   slug AS "slug!", tipo AS "tipo!", bpm AS "bpm?", key AS "key?",
                   escala AS "escala?", tags AS "tags!", publicado_at AS "publicado_at?",
                   total_likes, total_reproducciones, total_descargas, verificado
              FROM samples
             WHERE estado = 'activo'
               AND creador_id <> ALL($1::int[])
             ORDER BY (total_likes * 2 + total_reproducciones + total_descargas * 3) DESC
             LIMIT $2
            "#,
            &blocked,
            limit_i64,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| (r.id, r.creador_id, r.titulo, r.slug, r.tipo, r.bpm, r.key, r.escala, r.tags,
                  r.publicado_at, r.total_likes.unwrap_or(0), r.total_reproducciones.unwrap_or(0),
                  r.total_descargas.unwrap_or(0), r.verificado.unwrap_or(false)))
        .collect::<Vec<_>>()
    };

    let dias_boost = AlgorithmSignalConfig::legacy_current().parametros.novedad_dias_boost;
    Ok(rows
        .into_iter()
        .map(|r| {
            let dias = r.9.map(|ts| {
                ((Utc::now() - ts).num_seconds().max(0) as f64) / 86_400.0
            });
            let novelty = novelty_signal_score(dias, dias_boost);
            RankedSample {
                id: r.0,
                creador_id: r.1,
                titulo: r.2,
                slug: r.3,
                tipo: r.4,
                bpm: r.5,
                key: r.6,
                escala: r.7,
                tags: r.8,
                publicado_at: r.9,
                total_likes: r.10,
                total_reproducciones: r.11,
                total_descargas: r.12,
                verificado: r.13,
                es_nuevo: true,
                score: novelty + 0.5,
            }
        })
        .collect())
}

async fn fetch_similar_by_embedding(
    pool: &PgPool,
    sample_id: i32,
    vector: &Vector,
    blocked: &[i32],
    limit: i64,
) -> Result<Vec<SampleRow>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT
            s.id            AS "id!",
            s.creador_id    AS "creador_id!",
            s.titulo        AS "titulo!",
            s.slug          AS "slug!",
            s.tipo          AS "tipo!",
            s.bpm           AS "bpm?",
            s.key           AS "key?",
            s.escala        AS "escala?",
            s.tags          AS "tags!",
            s.publicado_at  AS "publicado_at?",
            s.total_likes,
            s.total_reproducciones,
            s.total_descargas,
            s.verificado,
            (s.embedding <=> $1)::float8 AS "embedding_distance?"
        FROM samples s
        WHERE s.estado = 'activo'
          AND s.id <> $2
          AND s.embedding IS NOT NULL
          AND s.creador_id <> ALL($3::int[])
        ORDER BY s.embedding <=> $1
        LIMIT $4
        "#,
        vector as &Vector,
        sample_id,
        blocked,
        limit,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| SampleRow {
            id: r.id,
            creador_id: r.creador_id,
            titulo: r.titulo,
            slug: r.slug,
            tipo: r.tipo,
            bpm: r.bpm,
            key: r.key,
            escala: r.escala,
            tags: r.tags,
            publicado_at: r.publicado_at,
            total_likes: r.total_likes.unwrap_or(0),
            total_reproducciones: r.total_reproducciones.unwrap_or(0),
            total_descargas: r.total_descargas.unwrap_or(0),
            verificado: r.verificado.unwrap_or(false),
            embedding_distance: r.embedding_distance,
            likes_24h: 0.0,
            reproducciones_24h: 0.0,
            descargas_7d: 0.0,
            follows_creador_7d: 0.0,
            creador_seguido: false,
            likes_dados: 0.0,
            reproducciones: 0.0,
            tiempo_escucha: 0.0,
            descargas_propias: 0.0,
            completadas: 0.0,
            dislike_penalty: 0.0,
            nunca_reproducido: true,
        })
        .collect())
}

async fn fetch_similar_by_tags(
    pool: &PgPool,
    sample_id: i32,
    tags: &[String],
    tipo: &str,
    blocked: &[i32],
    limit: i64,
) -> Result<Vec<SampleRow>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT
            s.id            AS "id!",
            s.creador_id    AS "creador_id!",
            s.titulo        AS "titulo!",
            s.slug          AS "slug!",
            s.tipo          AS "tipo!",
            s.bpm           AS "bpm?",
            s.key           AS "key?",
            s.escala        AS "escala?",
            s.tags          AS "tags!",
            s.publicado_at  AS "publicado_at?",
            s.total_likes,
            s.total_reproducciones,
            s.total_descargas,
            s.verificado
        FROM samples s
        WHERE s.estado = 'activo'
          AND s.id <> $1
          AND (s.tags && $2::text[] OR s.tipo = $3)
          AND s.creador_id <> ALL($4::int[])
        ORDER BY s.publicado_at DESC NULLS LAST
        LIMIT $5
        "#,
        sample_id,
        tags,
        tipo,
        blocked,
        limit,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| SampleRow {
            id: r.id,
            creador_id: r.creador_id,
            titulo: r.titulo,
            slug: r.slug,
            tipo: r.tipo,
            bpm: r.bpm,
            key: r.key,
            escala: r.escala,
            tags: r.tags,
            publicado_at: r.publicado_at,
            total_likes: r.total_likes.unwrap_or(0),
            total_reproducciones: r.total_reproducciones.unwrap_or(0),
            total_descargas: r.total_descargas.unwrap_or(0),
            verificado: r.verificado.unwrap_or(false),
            embedding_distance: None,
            likes_24h: 0.0,
            reproducciones_24h: 0.0,
            descargas_7d: 0.0,
            follows_creador_7d: 0.0,
            creador_seguido: false,
            likes_dados: 0.0,
            reproducciones: 0.0,
            tiempo_escucha: 0.0,
            descargas_propias: 0.0,
            completadas: 0.0,
            dislike_penalty: 0.0,
            nunca_reproducido: true,
        })
        .collect())
}

async fn collect_blocked(pool: &PgPool, user_id: i32) -> Result<Vec<i32>, AppError> {
    let mut blocked = crate::repositories::ModerationRepository::list_blocked(pool, user_id).await?;
    let blockers = crate::repositories::ModerationRepository::list_blockers(pool, user_id).await?;
    blocked.extend(blockers);
    blocked.sort_unstable();
    blocked.dedup();
    Ok(blocked)
}

fn jaccard_overlap(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    let set_a: HashSet<String> = a.iter().map(|t| t.to_lowercase()).collect();
    let set_b: HashSet<String> = b.iter().map(|t| t.to_lowercase()).collect();
    let intersection = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();
    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/* ───────────── cache ───────────── */

fn fresh_cache_key(user_id: i32, limit: usize, offset: usize) -> String {
    format!("{CACHE_PREFIX_FRESH}{user_id}_{limit}_{offset}")
}

fn stale_cache_key(user_id: i32, limit: usize, offset: usize) -> String {
    format!("{CACHE_PREFIX_STALE}{user_id}_{limit}_{offset}")
}

fn warm_lock_key(user_id: i32, limit: usize, offset: usize) -> String {
    format!("{WARM_LOCK_PREFIX}{user_id}_{limit}_{offset}")
}

fn fresh_ttl(config: &RecommenderConfig, page_offset: usize) -> u64 {
    if page_offset == 0 {
        config.fresh_ttl_p0
    } else {
        config.fresh_ttl_pn
    }
}

async fn cache_get(
    redis: &Option<RedisPool>,
    key: &str,
) -> Result<Option<Vec<RankedSample>>, AppError> {
    if let Some(pool) = redis {
        let mut conn = pool
            .get()
            .await
            .map_err(|e| AppError::Internal(format!("Redis pool: {e}")))?;
        let raw: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| AppError::Internal(format!("Redis GET: {e}")))?;
        match raw {
            None => Ok(None),
            Some(s) => serde_json::from_str(&s)
                .map(Some)
                .map_err(|e| AppError::Internal(format!("Cache JSON parse: {e}"))),
        }
    } else {
        cleanup_memory();
        Ok(MEMORY_CACHE
            .get(key)
            .filter(|entry| Instant::now() < entry.value().1)
            .map(|entry| entry.value().0.clone()))
    }
}

async fn cache_set(
    redis: &Option<RedisPool>,
    key: &str,
    value: &[RankedSample],
    ttl_secs: u64,
) -> Result<(), AppError> {
    if value.is_empty() {
        return Ok(());
    }
    if let Some(pool) = redis {
        let mut conn = pool
            .get()
            .await
            .map_err(|e| AppError::Internal(format!("Redis pool: {e}")))?;
        let json = serde_json::to_string(value)
            .map_err(|e| AppError::Internal(format!("Cache JSON serialize: {e}")))?;
        let _: () = conn
            .set_ex(key, json, ttl_secs)
            .await
            .map_err(|e| AppError::Internal(format!("Redis SETEX: {e}")))?;
    } else {
        MEMORY_CACHE.insert(
            key.to_owned(),
            (value.to_vec(), Instant::now() + Duration::from_secs(ttl_secs)),
        );
        cleanup_memory();
    }
    Ok(())
}

fn cleanup_memory() {
    let now = Instant::now();
    MEMORY_CACHE.retain(|_, (_, expires_at)| *expires_at > now);
    MEMORY_LOCKS.retain(|_, expires_at| *expires_at > now);
}

/// Lock distribuido para warm async. Devuelve `true` si el lock se adquirió.
/// Implementación: SETNX + EX en Redis, o DashMap con TTL en memoria.
async fn try_acquire_lock(
    redis: &Option<RedisPool>,
    key: &str,
    ttl_secs: u64,
) -> Result<bool, AppError> {
    if let Some(pool) = redis {
        let mut conn = pool
            .get()
            .await
            .map_err(|e| AppError::Internal(format!("Redis pool: {e}")))?;
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg("1")
            .arg("NX")
            .arg("EX")
            .arg(ttl_secs)
            .query_async(&mut *conn)
            .await
            .map_err(|e| AppError::Internal(format!("Redis SETNX: {e}")))?;
        Ok(result.is_some())
    } else {
        cleanup_memory();
        let now = Instant::now();
        if let Some(entry) = MEMORY_LOCKS.get(key) {
            if *entry.value() > now {
                return Ok(false);
            }
        }
        MEMORY_LOCKS.insert(key.to_owned(), now + Duration::from_secs(ttl_secs));
        Ok(true)
    }
}

async fn release_lock(redis: &Option<RedisPool>, key: &str) {
    if let Some(pool) = redis {
        if let Ok(mut conn) = pool.get().await {
            let _: Result<i32, _> = conn.del(key).await;
        }
    } else {
        MEMORY_LOCKS.remove(key);
    }
}

fn spawn_warm(
    pool: PgPool,
    redis: Option<RedisPool>,
    user_id: i32,
    limit: usize,
    offset: usize,
    config: RecommenderConfig,
) {
    tokio::spawn(async move {
        let lock_key = warm_lock_key(user_id, limit, offset);
        match try_acquire_lock(&redis, &lock_key, config.warm_lock_ttl).await {
            Ok(true) => {}
            Ok(false) => return,
            Err(error) => {
                warn!(target: "kamples.feed", "warm lock error: {error}");
                return;
            }
        }
        let result = compute_and_cache(&pool, &redis, user_id, limit, offset, &config).await;
        if let Err(error) = result {
            warn!(target: "kamples.feed", "warm async fallo user={user_id} limit={limit} offset={offset}: {error}");
        }
        release_lock(&redis, &lock_key).await;
    });
}

#[cfg(test)]
mod tests;
