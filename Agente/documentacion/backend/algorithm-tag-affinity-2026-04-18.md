# algorithm::tag_affinity — Pre-cómputo de afinidad tag↔usuario

**Tarea:** 174A-54
**Fecha:** 2026-04-18

## Decisión arquitectónica

Mantiene la decisión del legado: el cómputo pesado (UNNEST de `tags_enriquecidos` + 7 JOINs sobre likes/reproducciones/descargas/follows) corre **1× en el servidor de BD** vía un único INSERT-FROM-WITH dentro de una transacción. Los lectores (`recommender`, scoring) consultan `user_tag_scores` con un JOIN indexado de O(1).

**Por qué no portar al cómputo a Rust:**
- Materializar 26K filas intermedias en memoria del backend = red + RAM desperdiciada.
- PostgreSQL ya optimiza UNNEST + GROUP BY de forma nativa.
- El plan SQL es estable y conocido (medido en el legado).

## Schema (existente, migration `20260417000011_indexes_planificador_changelog`)

```sql
user_tag_scores(
    user_id INT, tag TEXT,
    w_likes REAL, w_repro REAL, w_tiempo REAL,
    w_descargas REAL, w_completadas REAL,
    w_dislikes REAL, w_ctx REAL,
    updated_at TIMESTAMP,
    PRIMARY KEY (user_id, tag)
)
```

## API Rust

```rust
TagAffinityService::has_recent_scores(pool, user_id) -> bool
TagAffinityService::fetch_for_user(pool, user_id) -> HashMap<String, TagWeights>
TagAffinityService::recalculate_for_user(pool, user_id) -> u64  // filas insertadas
TagAffinityService::recalculate_active(pool, limit) -> u32       // usuarios procesados
TagAffinityService::schedule_recalc(pool, user_id, &locks) -> JoinHandle<()>
TagAffinityService::new_inflight_locks() -> InflightLocks
```

`InflightLocks = Arc<Mutex<HashSet<i32>>>` evita recálculos duplicados en el mismo proceso. Para coordinación cross-proceso, 174A-55 introducirá lock vía Redis SETNX.

## Algoritmo

7 CTEs `utag_*` agregan pesos por tag desde fuentes distintas:
- `utag_likes`: peso `2` para `encanta`, `1` para `like`.
- `utag_repro`: count de reproducciones.
- `utag_tiempo`: count de reproducciones con `duracion_escuchada > 10`.
- `utag_descargas`: count de descargas.
- `utag_completadas`: count de reproducciones con `completada = true`.
- `utag_dislikes`: count de dislikes.
- `utag_ctx`: top-8 tags más frecuentes en likes (proxy de "contexto" del usuario).

`merged` mergea las 7 fuentes vía `UNION ALL → GROUP BY tag → SUM`. Resultado entra en `user_tag_scores` con `ON CONFLICT DO UPDATE`.

## Política de recálculo

- **Síncrono (`recalculate_for_user`):** llamado desde tests y CLI.
- **Asíncrono (`schedule_recalc`):** disparado tras like/play/download (174A-58).
- **Batch (`recalculate_active`):** worker `algo_planner` (174A-55) lo invoca cada N min sobre usuarios activos en últimas 24h.

## Hooks pendientes

- 174A-55: scheduling automático en worker.
- 174A-58: `schedule_recalc` post-200 en `POST /samples/{id}/play`.
- Recommender: leer `user_tag_scores` para alimentar `signals.afinidad_tags` con pesos reales (actualmente derivados de la respuesta a samples del feed).
