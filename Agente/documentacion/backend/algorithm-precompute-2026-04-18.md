# algorithm::precompute — Vista materializada de tendencias

**Tarea:** 174A-53
**Fecha:** 2026-04-18

## Decisión arquitectónica

El legado (`PrecomputadorFeed.php`) generaba 18 CTEs SQL inline para evitar
O(N×M) al calcular tendencias por sample. En la arquitectura Rust el scoring
corre en código (`algorithm::recommender` + `algorithm::signals`), así que
precompute = vista materializada `mv_trending_samples`.

**Por qué MV en vez de tabla normal:**
- `REFRESH MATERIALIZED VIEW CONCURRENTLY` no bloquea SELECTs (UNIQUE INDEX
  obligatorio, ya definido).
- Refresh atómico — no hay estado intermedio inconsistente.
- Postgres optimiza la query interna sin necesidad de mantener triggers.

## Schema (`migrations/20260418000020_mv_trending_samples.up.sql`)

7 métricas por sample:
- `likes_24h` (excluye dislikes; los conta `dislikes_7d`).
- `reproducciones_24h`.
- `descargas_7d`.
- `follows_creador_7d`.
- `tiempo_escucha_24h` (suma de `duracion_escuchada`).
- `completadas_24h` (reproducciones marcadas `completada=true`).
- `dislikes_7d`.

Índices:
- `idx_mv_trending_samples_pk(sample_id)` — UNIQUE, requerido para `CONCURRENTLY`.
- 3 índices secundarios (likes_24h DESC, reproducciones_24h DESC, creador_id).

## API Rust

```rust
PrecomputeService::refresh(pool) -> Result<(), AppError>
PrecomputeService::fetch_trends(pool, &[i32]) -> HashMap<i32, TrendStats>
```

`refresh` cae a no-CONCURRENTLY si la MV nunca fue populada (Postgres exige
una primera carga sin lock).

## Integración con recommender

`recommender::fetch_enriched` ahora hace `LEFT JOIN mv_trending_samples mv ON
mv.sample_id = s.id` y popula los campos `likes_24h`, `reproducciones_24h`,
`descargas_7d`, `follows_creador_7d` directamente desde la MV. Samples nuevos
no presentes aún en la MV caen a `0` por `COALESCE`.

## Política de refresh

- 174A-55 (worker `algo_planner`) ejecutará `refresh` cada N minutos.
- Mientras tanto, refresh manual: `cargo run --bin <admin>` o cron.

## Hooks pendientes

- 174A-55: scheduling automático en worker.
- 174A-58 (`POST /samples/{id}/play`): considerar refresh incremental tras
  bursts de actividad.
