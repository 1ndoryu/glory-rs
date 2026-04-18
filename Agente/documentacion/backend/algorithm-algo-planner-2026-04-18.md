# algorithm::algo_planner — orquestador de recálculos del algoritmo Kamples

**Fecha:** 2026-04-18 · **Tarea:** 174A-55 · **Reemplaza:** `App/Kamples/Services/PlanificadorAlgoritmo.php` + WP-cron `glory_kamples_planificador`.

## Propósito

`AlgoPlanner` decide *cuándo* recalcular el feed personalizado de un usuario y dispara los recálculos. Tiene dos disparadores:

1. **Por evento (path caliente):** `register_interaction` se invoca tras cada like/play/completa/etc. Incrementa contadores en `algoritmo_estado` y, si cruzan umbrales, ejecuta recálculo *fast* (síncrono) o *precise* (async via `tokio::spawn`).
2. **Por tiempo (background):** `spawn_periodic_loop` lanza una tarea tokio que cada N segundos refresca la materialized view de tendencias, recalcula tag affinity de usuarios activos y procesa usuarios cuyo último recálculo expiró.

## API pública

```rust
pub struct AlgoPlannerConfig { /* triggers + intervalos */ }
impl AlgoPlannerConfig { pub const fn legacy_defaults() -> Self; }

pub enum InteractionKind { Like, Reproduccion, Completa, Descarga, Follow, Comentario }
pub struct Triggered { pub fast: bool, pub precise: bool }

pub struct AlgoPlanner { pub config, pub locks }
impl AlgoPlanner {
    pub fn new(config) -> Arc<Self>;
    pub async fn register_interaction(&self, pool, redis, user_id, kind) -> Result<Triggered>;
    pub async fn process_temporal(&self, pool, redis) -> Result<u32>;
    pub fn spawn_periodic_loop(self: Arc<Self>, pool, redis, token: CancellationToken) -> JoinHandle<()>;
}
```

## Tabla `algoritmo_estado`

12 contadores (6 fast + 6 precise: `cnt_{kind}` y `cnt_{kind}_preciso`) + 4 timestamps (`ultimo_rapido`, `ultimo_preciso`, `ultima_actividad`) + `version_perfil`. Definida en migration `20260417000011`.

`register_interaction` hace upsert atómico en una sola roundtrip: incrementa ambos contadores (fast y precise) + actualiza `ultima_actividad` + RETURNING los 12 contadores para evaluar umbrales sin segundo SELECT.

## Recálculos

- **`execute_fast(user_id)`** (inline, ~5ms): `RecommenderService::invalidate_user_feed` (borra keys Redis `CACHE_PREFIX_FRESH{user_id}_*`) + `UPDATE algoritmo_estado SET cnt_* = 0, ultimo_rapido = NOW()`.
- **`execute_precise(user_id)`** (spawn): `TagAffinityService::schedule_recalc` (async, recalcula `user_tag_scores` con 7 CTEs `utag_*`) + invalida cache + reset contadores precise + `version_perfil + 1`. Hook para `GeneradorEmbeddings` cuando se porte (174A-65).

## Loop background

```
tokio::select! sobre 3 intervals + CancellationToken:
  - tick_mv      (5min)  → PrecomputeService::refresh(pool)
  - tick_recalc  (10min) → TagAffinityService::recalculate_active(pool, 200)
  - tick_temporal (5min) → self.process_temporal(pool, redis)
  - token.cancelled()    → break (graceful shutdown)
```

Errores se loguean con `tracing::warn!` pero no abortan el loop. Cada interval usa `interval_at` con offsets distintos (10s, 30s, 0s) para evitar contención en t=0.

## Defaults vs legado

| Trigger fast       | Legado PHP | Rust default |
| ------------------ | ---------- | ------------ |
| likes              | 3          | 3            |
| reproducciones     | 8          | 8            |
| completas          | 5          | 5            |
| descargas          | 2          | 2            |
| follows            | 2          | 2            |
| comentarios        | 5          | 5            |
| precise            | 2× fast    | 2× fast      |

| Intervalo background | Legado | Rust |
| -------------------- | ------ | ---- |
| refresh MV trending  | n/a    | 5min |
| recalc tag_affinity  | 6h cron| 10min |
| process temporal     | 5min   | 5min |

## Pendiente

- **174A-58** invocará `register_interaction` desde `POST /samples/{id}/play` y endpoints de likes/follows/etc.
- **174A-65 (GeneradorEmbeddings)**: enchufar en `execute_precise` cuando se porte, tras `schedule_recalc`.
- **Bootstrap**: `main.rs` debe construir `AlgoPlanner::new(config)` y llamar `spawn_periodic_loop` tras `app.run()`. Pasar el `CancellationToken` al shutdown handler de Axum.

## Decisiones clave

1. **Inflight locks compartidos con `TagAffinityService`**: el planner instancia los locks en `new()` y los pasa a `schedule_recalc`. Evita que dos `execute_precise` concurrentes para el mismo usuario disparen dos recalc paralelos.
2. **Sin Redis distribuido para los locks**: por ahora `Arc<Mutex<HashSet<i32>>>` in-process. Si en el futuro hay >1 instancia del backend, migrar a Redis SET NX EX.
3. **`execute_precise` no bloquea `register_interaction`**: la transacción HTTP del usuario no espera a que termine el recálculo de tags (que puede tardar 200-500ms).
