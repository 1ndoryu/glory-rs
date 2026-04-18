/* [174A-53] mv_trending_samples — vista materializada que pre-calcula
 * tendencias agregadas por sample para alimentar `algorithm::signals`.
 *
 * El legado (PrecomputadorFeed.php) generaba 18 CTEs SQL inline por request
 * para evitar O(N×M) en correlated subqueries. En la arquitectura Rust el
 * scoring se hace en código, así que precompute = vista materializada
 * refrescable por worker (174A-55) o cron.
 *
 * Métricas pre-calculadas:
 *   - likes_24h            : COUNT likes/encanta últimas 24h
 *   - reproducciones_24h   : COUNT reproducciones últimas 24h
 *   - descargas_7d         : COUNT descargas últimos 7 días
 *   - follows_creador_7d   : COUNT follows al creador últimos 7 días
 *   - tiempo_escucha_24h   : SUM duracion_escuchada últimas 24h (segundos)
 *   - completadas_24h      : COUNT reproducciones completadas últimas 24h
 *   - dislikes_7d          : COUNT dislikes últimos 7 días
 *
 * Refresh: `REFRESH MATERIALIZED VIEW CONCURRENTLY mv_trending_samples`
 * (requiere UNIQUE INDEX, definido abajo).
 *
 * Costo aproximado en BD con 100k samples + 1M interacciones: <2s. */

BEGIN;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_trending_samples AS
SELECT
    s.id AS sample_id,
    s.creador_id,
    COALESCE((
        SELECT COUNT(*) FROM likes l
         WHERE l.tipo = 'sample'
           AND l.target_id = s.id
           AND l.reaccion IN ('like', 'encanta')
           AND l.created_at > NOW() - INTERVAL '24 hours'
    ), 0)::int AS likes_24h,
    COALESCE((
        SELECT COUNT(*) FROM reproducciones r
         WHERE r.sample_id = s.id
           AND r.created_at > NOW() - INTERVAL '24 hours'
    ), 0)::int AS reproducciones_24h,
    COALESCE((
        SELECT COUNT(*) FROM descargas d
         WHERE d.sample_id = s.id
           AND d.created_at > NOW() - INTERVAL '7 days'
    ), 0)::int AS descargas_7d,
    COALESCE((
        SELECT COUNT(*) FROM follows f
         WHERE f.seguido_id = s.creador_id
           AND f.created_at > NOW() - INTERVAL '7 days'
    ), 0)::int AS follows_creador_7d,
    COALESCE((
        SELECT SUM(r.duracion_escuchada) FROM reproducciones r
         WHERE r.sample_id = s.id
           AND r.created_at > NOW() - INTERVAL '24 hours'
    ), 0)::float8 AS tiempo_escucha_24h,
    COALESCE((
        SELECT COUNT(*) FROM reproducciones r
         WHERE r.sample_id = s.id
           AND r.completada = TRUE
           AND r.created_at > NOW() - INTERVAL '24 hours'
    ), 0)::int AS completadas_24h,
    COALESCE((
        SELECT COUNT(*) FROM likes l
         WHERE l.tipo = 'sample'
           AND l.target_id = s.id
           AND l.reaccion = 'dislike'
           AND l.created_at > NOW() - INTERVAL '7 days'
    ), 0)::int AS dislikes_7d,
    NOW() AS refrescado_at
FROM samples s
WHERE s.estado = 'activo';

/* UNIQUE INDEX requerido para REFRESH CONCURRENTLY. */
CREATE UNIQUE INDEX IF NOT EXISTS idx_mv_trending_samples_pk
    ON mv_trending_samples (sample_id);

CREATE INDEX IF NOT EXISTS idx_mv_trending_samples_likes_24h
    ON mv_trending_samples (likes_24h DESC) WHERE likes_24h > 0;
CREATE INDEX IF NOT EXISTS idx_mv_trending_samples_repro_24h
    ON mv_trending_samples (reproducciones_24h DESC) WHERE reproducciones_24h > 0;
CREATE INDEX IF NOT EXISTS idx_mv_trending_samples_creador
    ON mv_trending_samples (creador_id);

COMMIT;
