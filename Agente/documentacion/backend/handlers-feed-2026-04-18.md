# Endpoints del feed personalizado

**Fecha:** 2026-04-18 · **Tarea:** 174A-56 · **Handler:** `src/handlers/feed.rs`

## Rutas

| Método | Ruta              | Auth | Descripción                                              |
| ------ | ----------------- | ---- | -------------------------------------------------------- |
| GET    | `/api/feed`       | Sí   | Feed personalizado del usuario autenticado.              |
| GET    | `/api/me/feed`    | Sí   | Alias de `/api/feed` (compatibilidad cliente legado).    |
| GET    | `/api/samples/random` | No (opcional) | Sample aleatorio (handler en `sample_catalog`). |

## Query params

```
limit:  Option<i64>  // default 20, clamp [1..=100]
offset: Option<i64>  // default 0, clamp [0..]
```

## Response

```json
{
  "items": [RankedSample, ...],
  "limit": 20,
  "offset": 0
}
```

`RankedSample` (definido en `src/algorithm/recommender.rs`):

```
id, creador_id, titulo, slug, tipo, bpm?, key?, escala?,
tags[], publicado_at?, total_likes, total_reproducciones,
total_descargas, verificado, es_nuevo, score
```

## Pipeline interno

```
HTTP GET /api/feed
  → CurrentUser middleware (401 si no auth)
  → normalize_pagination(limit, offset)
  → RecommenderService::feed(pool, redis, user_id, limit, offset, &config)
       1. Mira cache fresh (Redis o memoria) → si hit, devuelve
       2. Mira cache stale → si hit, devuelve + spawn warm async
       3. Computa síncronamente vía CandidatesService + scoring 6 señales
  → Json(FeedResponse { items, limit, offset })
```

## Configuración

`RecommenderConfig::legacy_defaults()` se invoca en cada request. Cambios:
- TTL fresh p0: 300s, pn: 900s; stale: 7200s.
- Diversidad: max 3 por creador, 4 por género, 5 por tipo.

Si en el futuro se necesita configurabilidad por env, mover el config a
`AppState::recommender_config: Arc<RecommenderConfig>` y leerlo en el
handler vía `state.recommender_config.as_ref()`.

## Cliente generado (frontend)

Orval regeneró:
- `getFeed(params?)` y `getMeFeed(params?)`
- `getGetFeedQueryOptions` / `getGetMeFeedQueryOptions` para React Query.
- `GetFeedParams = { limit?, offset? }` y `getFeedResponse` tipado.

## Pendiente

- **174A-58** invocará `AlgoPlanner::register_interaction` desde `POST /samples/{id}/play` para disparar invalidación del cache del feed cuando el usuario interactúa.
- **Sistémico (fuera de scope):** migrar Orval a modo `tags-split` (regla del protocolo). Actualmente `frontend/src/api/generated.ts` es monolítico.
