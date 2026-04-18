# Plan 174A-52 — `algorithm/recommender.rs` (MotorRecomendacion)

> Origen: porte de `App/Kamples/Services/MotorRecomendacion.php` (1013 LoC, legado).
> Creado: 2026-04-18
> Estado: **Fase 1 en curso**

## Por qué un plan

Tarea compleja: orquestador central del feed que combina scoring (6 señales,
ya implementadas en `algorithm::signals`), perfil (`algorithm::profile`,
174A-50), candidatos (`algorithm::candidates`, 174A-51), y añade cache
stale-while-revalidate, bulk-fetch (3 páginas), warm async, fallback
recientes y similares por embedding. Portar todo en una pasada produciría
código de baja calidad. Se divide en **5 fases** que se entregan cada una
con commit independiente, dentro del mismo ID 174A-52.

## Diferencias estratégicas vs el legado

| Aspecto             | PHP legado                                | Rust 174A-52                                              |
| ------------------- | ----------------------------------------- | --------------------------------------------------------- |
| Cache               | `ServicioCache` (Redis o transient WP)    | `deadpool_redis` con fallback `DashMap` (mismo patrón A1) |
| Warm async          | `add_action('shutdown')` + `fastcgi_finish_request` | `tokio::spawn` desde el handler, lock vía Redis SETNX |
| pgvector check      | Query + transient 1h                      | Inicializado en `AppState` al boot                        |
| Scoring             | SQL gigante con 6 sub-CTEs                | `algorithm::signals` ya tipado en Rust                    |
| Bulk-fetch          | LIMIT × 3 + slice por página              | Idéntico, conservar paridad                               |
| Fallback recientes  | Branch directo en `feedPersonalizado`     | Función separada `fallback_recientes`                     |

## Fase 1 — API pública + cache + fallback simple (commit incluido)

**Alcance:** crear `src/algorithm/recommender.rs` con:

- `RecommenderConfig` (TTLs: 300s pag1, 900s paginadas, 7200s stale,
  límites warm).
- `RecommenderService::feed(pool, redis, user_id, limit, offset)`
  - Lookup cache fresh → return.
  - Lookup cache stale → return + spawn warm async.
  - Si nada → `compute_feed` (sin warm).
- `compute_feed`:
  - Construir `UserProfile`.
  - Decidir activar selector (`count_active > umbral`).
  - Si activado: `select(...)` → `score_and_rank` sobre candidatos.
  - Si no: `score_and_rank` sobre todo el set (LIMIT generoso).
  - Si scoring vacío: `fallback_recientes`.
- `score_and_rank`: aplica las 6 señales de `algorithm::signals` y
  ordena por score descendente.
- `fallback_recientes`: top N por `publicado_at`, filtro bloqueos.

**Validación:** `cargo check`, `clippy`, `cargo test --lib algorithm` con
tests unitarios para `RecommenderConfig::legacy_defaults`,
`cache_key`/`stale_key` y deduplicación.

**Salida:** commit `174A-52a: recommender.rs fase 1 (cache + scoring + fallback)`.

## Fase 2 — Bulk-fetch (3 páginas en 1 query)

Replicar `PAGINAS_BULK = 3`: `compute_feed` con `offset = 0` calcula
`limit × 3` candidatos rankeados, splittea en 3 sub-resultados y cachea
páginas 0/limit/2*limit. Evita recomputar al paginar.

## Fase 3 — Warm async (stale-while-revalidate)

Lock SETNX `kamples_warm_feed_<user>_<limit>_<offset>` con TTL 90s.
`tokio::spawn` para recálculo no bloqueante. Métricas opcionales con
`tracing` (latencia, cache hit/miss/stale).

## Fase 4 — Similares por embedding

Método `similar_to_sample(pool, sample_id, limit)` con `embedding <=>`.
Se usa para "más como esto" en detalle de sample (endpoint 174A-56).

## Fase 5 — Hooks de invalidación

Cuando un sample pasa a `activo`/`inactivo`/`eliminado`, invalidar:

- `CandidatesService::invalidate_count`.
- `RecommenderService::invalidate_user_feed(user_id)` (opcional, se
  consensuará si vale la pena vs dejar expirar el TTL de 5min).

## Estado actual

- ✅ Plan creado.
- ⏳ Fase 1 — pendiente arrancar implementación.

## GLORY-RS

No aplica — toda la lógica es específica de Kamples (feed musical con
samples, embeddings 128d, semántica bpm/key/escala/tipo).
