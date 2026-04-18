# MotorRecomendacion — feed personalizado de samples

> **Tarea origen:** 174A-52
> **Módulo:** `src/algorithm/recommender.rs`
> **Tests:** `src/algorithm/recommender/tests.rs` (8 tests)
> **Plan:** `Agente/planes/completados/plan-recommender-2026-04-18.md`

## Propósito

Reemplazo Rust del `MotorRecomendacion.php` legado de Kamples (1013 LoC). Genera el feed personalizado para un usuario combinando perfil + candidatos + scoring + diversidad + cache stale-while-revalidate + warm async.

## Decisión arquitectónica

El legado generaba SQL gigante con scoring inline (CTEs encadenadas, 100+ líneas por request). Aquí el SQL hace solo lo que SQL hace bien (filtrar, agregar, traer datos enriquecidos para un set de IDs) y el scoring corre en Rust usando [`algorithm::signals`](algorithm-signals-2026-04-18.md). Ventajas:

- Scoring testeable sin BD.
- Queries validadas en compile-time vía `sqlx::query!`.
- Diversidad y serendipia como funciones puras.
- Composable con `algorithm::profile` (174A-50) y `algorithm::candidates` (174A-51).

## API pública

```rust
pub struct RecommenderConfig {
    pub fresh_ttl_p0: u64,        // 300
    pub fresh_ttl_pn: u64,        // 900
    pub stale_ttl: u64,           // 7200
    pub warm_lock_ttl: u64,       // 90
    pub umbral_candidatos: i64,   // 5000
    pub max_por_creador: usize,   // 3
    pub max_por_categoria: usize, // 4
    pub max_por_tipo: usize,      // 5
    pub candidates: CandidatesConfig,
    pub signal: AlgorithmSignalConfig,
}

pub struct RankedSample { /* id, creador_id, titulo, slug, tipo, bpm, key,
                           escala, tags, publicado_at, totales, verificado,
                           es_nuevo, score */ }

impl RecommenderService {
    pub async fn feed(pool, redis, user_id, limit, offset, &config) -> Result<Vec<RankedSample>>;
    pub async fn similar_to_sample(pool, sample_id, limit, viewer_id) -> Result<Vec<RankedSample>>;
    pub async fn invalidate_user_feed(redis, user_id) -> Result<()>;
    pub async fn invalidate_global(redis) -> Result<()>;
}
```

## Política de cache

```
GET feed(user, limit, offset)
 ├─ HIT cache fresh `kamples_feed_{u}_{l}_{o}` → devuelve.
 ├─ HIT cache stale `kamples_feed_stale_{u}_{l}_{o}` →
 │    devuelve + spawn warm async (lock SETNX `kamples_warm_feed_*`).
 └─ MISS → compute_and_cache (síncrono).
```

- Página 0: TTL 300s (fresh).
- Páginas paginadas: TTL 900s (fresh).
- Stale: TTL 7200s (escudo).
- Lock warm: TTL 90s para evitar stampede.
- Sin Redis cae a `DashMap` con expiración por `Instant`.

## Bulk-fetch (PAGINAS_BULK = 3)

Cuando `offset <= limit * 2`, calcula `limit * 3` candidatos rankeados en una sola pasada y splittea en 3 páginas que se cachean juntas. Ahorra 2/3 de las llamadas a BD para los primeros scrolls. Para offsets más profundos, calcula página individual.

## Pipeline interno

1. **Profile**: `ProfileService::build` (cache TTL 30min).
2. **Cold start**: si `profile.is_cold_start()` → `fallback_recientes` (top trending por engagement reciente, sin scoring complejo).
3. **Candidatos**: si `total_active > umbral_candidatos`, `CandidatesService::select` devuelve ~1000 IDs pre-filtrados.
4. **Enriched fetch**: una sola query con LEFT JOIN LATERAL trae todos los datos para scoring (totales, métricas 24h/7d, comportamiento del usuario, follow del creador, no-reproducido).
5. **Scoring**: itera y llama `AlgorithmSignalConfig::score` por fila. Multiplicadores adicionales: `verificado` ×1.15, `nunca_reproducido` ×1.20.
6. **Sort primario**: `es_nuevo DESC`, luego `score DESC`.
7. **Diversidad**: `apply_diversity` penaliza creador (>3), género (>4) y tipo oneshot (>5) con factores 0.85–0.30.
8. **Resort + slice**: re-ordena tras diversidad y devuelve la página solicitada.

## Filtro de bloqueos

`collect_blocked` une `list_blocked` + `list_blockers` (bidireccional, `repositories::ModerationRepository`). Se aplica en todas las queries (`creador_id <> ALL($blocked)`).

## `similar_to_sample`

Motor "más como esto":
1. Si el sample tiene embedding → pgvector `<=>` → top N por cercanía coseno.
2. Fallback: overlap de tags (Jaccard case-insensitive) + match de tipo.

## Hooks de invalidación

- `invalidate_user_feed(redis, user_id)`: borra `kamples_feed_{user_id}_*` (mantiene stale).
- `invalidate_global(redis)`: borra `kamples_feed_*` (publicación de sample nuevo).

Llamar desde:
- Like/dislike (cualquier usuario afectado).
- Descarga.
- Follow/unfollow (afecta perfil + candidatos).
- Bloqueo nuevo (afecta filtro bidireccional).
- Sample publicado / despublicado (global).

## Hooks pendientes (para tareas siguientes)

- **174A-53 (`PrecomputadorFeed`)**: cuando exista, conectar `signals.tendencias.*` desde `mv_*` materializadas precomputadas (ahora se llenan de 0 fuera de candidates).
- **174A-54 (`TagAffinityService`)**: poblar `signal_input.grafo_social.puntos_reacciones_seguidos`.
- **174A-55 (`algo_planner`)**: worker periódico que invoca `RecommenderService::feed` proactivamente para top-N usuarios activos.

## Tests

8 unitarios en `src/algorithm/recommender/tests.rs`:
- `legacy_defaults_match_php_constants` (paridad de constantes con `algoritmoPesos.php`).
- `cache_keys_use_legacy_prefixes` (compatibilidad con cache existente).
- `jaccard_overlap_*` (correctness del fallback similar).
- `apply_diversity_*` (penalización por repetición).
- `fresh_ttl_distinguishes_first_page_from_paginated`.
- `invalidate_user_feed_without_redis_clears_memory_cache`.
- `try_acquire_lock_in_memory_blocks_second_call` (stampede protection).

Las queries SQL se validan en compile-time vía `sqlx::query!` + `.sqlx/` cache.

## Archivos relacionados

- [src/algorithm/recommender.rs](../../../src/algorithm/recommender.rs)
- [src/algorithm/recommender/tests.rs](../../../src/algorithm/recommender/tests.rs)
- [src/algorithm/signals.rs](../../../src/algorithm/signals.rs)
- [src/algorithm/profile.rs](../../../src/algorithm/profile.rs)
- [src/algorithm/candidates.rs](../../../src/algorithm/candidates.rs)
