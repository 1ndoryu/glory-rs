# services::algo_timing — profiling fino del feed (2026-04-18)

## Propósito
Port directo del legado `AlgoTimingLogger.php` (solo activo para `userId=1`).
Permite medir cada etapa del pipeline `RecommenderService::feed` para un único usuario QA sin sobrecarga en el resto.

## Diseño
- **Singleton in-process** `pub static ALGO_TIMING: LazyLock<AlgoTimingLogger>`.
- **Target user** configurable via env `KAMPLES_ALGO_TIMING_USER_ID` (default 1).
- **Estado en vuelo:** `DashMap<i32, RequestState>` con `Instant` de inicio + última marca.
- **Historial:** `RwLock<VecDeque<TimingEntry>>` con cap 100, más reciente al frente.
- **Etapas:** `Vec<TimingStage { name, ms }>` preserva orden de inserción sin deps extras.

## API pública
```rust
ALGO_TIMING.start(user_id);                        // no-op si != target
ALGO_TIMING.mark(user_id, "etapa");                // mide delta desde última marca
ALGO_TIMING.save(user_id, json!({ "k": v }));      // cierra request, push a historial
ALGO_TIMING.history(50);                            // últimas N (más reciente primero)
ALGO_TIMING.target_user_id();                      // cuál user mide
```

## Hooks integrados (recommender.rs::feed)
1. `start(user_id)` al entrar.
2. `mark("cache_fresh_hit"|"cache_fresh_miss")`.
3. `mark("cache_stale_hit"|"cache_stale_miss")`.
4. `mark("compute_and_cache")` tras computar.
5. `save(user_id, {source, items, limit?, offset?})` antes de retornar.

## Endpoint admin
- `GET /api/admin/algo-timing?limit=N` (1..=100, default 50).
- Requiere `user.require_admin()` → 401/403.
- Retorna `Vec<TimingEntry>` (más reciente primero).
- Schemas: `TimingEntry`, `TimingStage`.

## Pendiente / no portado
- **EXPLAIN ANALYZE:** El legado capturaba el plan SQL del recommender con throttle 5min vía `ServicioCache`. No portado: requiere snapshot de queries activas y persistencia (Redis/PG). Documentado como hook futuro — agregar `mark_with_query` que ejecute `EXPLAIN ANALYZE` cuando user == target y throttle key haya expirado.
- **Persistencia:** Historial in-process se pierde al restart. Aceptable para QA puntual; si se necesita durabilidad, mover a tabla dedicada o Redis.

## Tests
- `ignores_non_target_user`
- `records_target_user_cycle`
- `history_is_capped_and_ordered`

## Tarea origen
174A-57.
