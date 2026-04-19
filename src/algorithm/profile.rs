/* [174A-50] PerfilUsuario portado del legado Kamples.
 *
 * Replica el contrato de `App\Kamples\Services\PerfilUsuario` (PHP):
 *   - CTE unificada (interacciones de likes + reproducciones + descargas) →
 *     `total`, `bpm_prom`, `key_fav`, `escala_fav`, `tipo_fav` en 1 roundtrip.
 *   - Query separada para `creadores_fav` (top 5 por afinidad ponderada).
 *   - `generos_declarados` desde `usuarios_ext.generos_favoritos` (JSONB).
 *   - Si `interacciones == 0` → respuesta cold-start (solo userId + géneros).
 *   - Cache TTL 1800s (30min) bajo clave `kamples_perfil_usr_{user_id}`,
 *     compatible con la del legado para coexistencia durante la migración.
 *   - Invalidación explícita al recalcular en el planificador (174A-55).
 *
 * Pesos para `obtener_creadores_favoritos` (idénticos al legado):
 *   like=1.0, encanta=2.0, reproducción=0.5, descarga=1.5, mínimo afinidad=2.0.
 *
 * Cache backend: Redis si está disponible, fallback a `DashMap` en memoria con
 * expiración perezosa, replicando el patrón de `services::idempotency`. */

use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

use dashmap::DashMap;
use deadpool_redis::Pool as RedisPool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;

const CACHE_PREFIX: &str = "kamples_perfil_usr_";
const CACHE_TTL_SECS: u64 = 1800;
const CREATORS_LIMIT: i64 = 5;
const CREATORS_MIN_AFFINITY: f64 = 2.0;

static MEMORY_CACHE: LazyLock<Arc<DashMap<String, (String, Instant)>>> =
    LazyLock::new(|| Arc::new(DashMap::new()));

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: i32,
    pub interactions: i64,
    pub bpm_avg: Option<i32>,
    pub key_fav: Option<String>,
    pub scale_fav: Option<String>,
    pub type_fav: Option<String>,
    pub favorite_creators: Vec<i32>,
    pub declared_genres: Vec<String>,
}

impl UserProfile {
    /// Perfil cold-start: usuario sin interacciones suficientes. Solo géneros
    /// declarados son útiles aún para el contexto del recomendador.
    #[must_use]
    pub fn cold_start(user_id: i32, declared_genres: Vec<String>) -> Self {
        Self {
            user_id,
            interactions: 0,
            bpm_avg: None,
            key_fav: None,
            scale_fav: None,
            type_fav: None,
            favorite_creators: Vec::new(),
            declared_genres,
        }
    }

    #[must_use]
    pub const fn is_cold_start(&self) -> bool {
        self.interactions == 0
    }
}

pub struct ProfileService;

impl ProfileService {
    /// Construye el perfil completo del usuario para el algoritmo.
    /// Busca en cache primero; si miss, ejecuta la CTE + creadores + géneros
    /// declarados y guarda el resultado por 30 minutos.
    pub async fn build(
        pool: &PgPool,
        redis: &Option<RedisPool>,
        user_id: i32,
    ) -> Result<UserProfile, AppError> {
        if let Some(cached) = cache_get(redis, user_id).await? {
            return Ok(cached);
        }

        let declared_genres = load_declared_genres(pool, user_id).await?;
        let row = load_profile_aggregates(pool, user_id).await?;
        let interactions = row.interactions;

        let profile = if interactions == 0 {
            UserProfile::cold_start(user_id, declared_genres)
        } else {
            let favorite_creators = load_favorite_creators(pool, user_id).await?;
            UserProfile {
                user_id,
                interactions,
                bpm_avg: row.bpm_avg,
                key_fav: row.key_fav,
                scale_fav: row.scale_fav,
                type_fav: row.type_fav,
                favorite_creators,
                declared_genres,
            }
        };

        cache_set(redis, user_id, &profile).await?;
        Ok(profile)
    }

    /// Invalida el perfil cacheado para un usuario.
    /// Lo invocará `PlanificadorAlgoritmo` (174A-55) al disparar recálculo.
    pub async fn invalidate(redis: &Option<RedisPool>, user_id: i32) -> Result<(), AppError> {
        let key = cache_key(user_id);
        if let Some(pool) = redis {
            let mut conn = pool
                .get()
                .await
                .map_err(|error| AppError::Internal(format!("Redis pool: {error}")))?;
            let _: () = conn
                .del(&key)
                .await
                .map_err(|error| AppError::Internal(format!("Redis DEL: {error}")))?;
        } else {
            MEMORY_CACHE.remove(&key);
        }
        Ok(())
    }
}

struct ProfileAggregateRow {
    interactions: i64,
    bpm_avg: Option<i32>,
    key_fav: Option<String>,
    scale_fav: Option<String>,
    type_fav: Option<String>,
}

async fn load_profile_aggregates(
    pool: &PgPool,
    user_id: i32,
) -> Result<ProfileAggregateRow, AppError> {
    /* CTE legacy: el set "interacciones" UNE samples likeados (like|encanta) +
     * samples reproducidos. Sobre ese set se calcula AVG(bpm), top-1 key,
     * top-1 escala (lower-cased) y top-1 tipo. El contador "total" suma
     * likes + reproducciones + descargas de forma independiente (paridad
     * exacta con `UsuariosExtRepository::perfilCompletoParaAlgoritmo`). */
    let row = sqlx::query!(
        r#"
        WITH interacciones AS (
            SELECT target_id AS sample_id
              FROM likes
             WHERE usuario_id = $1
               AND tipo = 'sample'
               AND reaccion IN ('like', 'encanta')
            UNION
            SELECT sample_id
              FROM reproducciones
             WHERE usuario_id = $1
        ),
        total AS (
            SELECT
                (SELECT COUNT(*) FROM likes WHERE usuario_id = $1 AND tipo = 'sample')
              + (SELECT COUNT(*) FROM reproducciones WHERE usuario_id = $1)
              + (SELECT COUNT(*) FROM descargas WHERE usuario_id = $1) AS val
        ),
        bpm AS (
            SELECT AVG(s.bpm)::int AS val
              FROM samples s
              JOIN interacciones i ON s.id = i.sample_id
             WHERE s.bpm IS NOT NULL
        ),
        key_fav AS (
            SELECT s.key AS val
              FROM samples s
              JOIN interacciones i ON s.id = i.sample_id
             WHERE s.key IS NOT NULL
             GROUP BY s.key
             ORDER BY COUNT(*) DESC
             LIMIT 1
        ),
        escala_fav AS (
            SELECT LOWER(s.escala) AS val
              FROM samples s
              JOIN interacciones i ON s.id = i.sample_id
             WHERE s.escala IS NOT NULL AND s.escala <> ''
             GROUP BY LOWER(s.escala)
             ORDER BY COUNT(*) DESC
             LIMIT 1
        ),
        tipo_fav AS (
            SELECT s.tipo AS val
              FROM samples s
              JOIN interacciones i ON s.id = i.sample_id
             GROUP BY s.tipo
             ORDER BY COUNT(*) DESC
             LIMIT 1
        )
        SELECT
            (SELECT val FROM total)      AS "interactions!",
            (SELECT val FROM bpm)        AS "bpm_avg?",
            (SELECT val FROM key_fav)    AS "key_fav?",
            (SELECT val FROM escala_fav) AS "scale_fav?",
            (SELECT val FROM tipo_fav)   AS "type_fav?"
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(ProfileAggregateRow {
        interactions: row.interactions,
        bpm_avg: row.bpm_avg,
        key_fav: row.key_fav,
        scale_fav: row.scale_fav,
        type_fav: row.type_fav,
    })
}

async fn load_favorite_creators(pool: &PgPool, user_id: i32) -> Result<Vec<i32>, AppError> {
    /* Top creadores por afinidad ponderada (legacy):
     *   like=1.0, encanta=2.0, reproducción=0.5, descarga=1.5.
     * Excluye al propio usuario (HAVING SUM(score) >= 2.0, ORDER DESC LIMIT 5). */
    let rows = sqlx::query!(
        r#"
        SELECT s.creador_id AS "creador_id!"
          FROM (
              SELECT s.creador_id,
                     CASE WHEN l.reaccion = 'encanta' THEN 2.0::float8 ELSE 1.0::float8 END AS score
                FROM likes l
                JOIN samples s ON l.target_id = s.id
               WHERE l.usuario_id = $1
                 AND l.tipo = 'sample'
                 AND l.reaccion IN ('like', 'encanta')
              UNION ALL
              SELECT s.creador_id, 0.5::float8 AS score
                FROM reproducciones r
                JOIN samples s ON r.sample_id = s.id
               WHERE r.usuario_id = $1
              UNION ALL
              SELECT s.creador_id, 1.5::float8 AS score
                FROM descargas d
                JOIN samples s ON d.sample_id = s.id
               WHERE d.usuario_id = $1
          ) s
         WHERE s.creador_id <> $1
         GROUP BY s.creador_id
        HAVING SUM(s.score) >= $2
         ORDER BY SUM(s.score) DESC
         LIMIT $3
        "#,
        user_id,
        CREATORS_MIN_AFFINITY,
        CREATORS_LIMIT,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|row| row.creador_id).collect())
}

async fn load_declared_genres(pool: &PgPool, user_id: i32) -> Result<Vec<String>, AppError> {
    /* `usuarios_ext.generos_favoritos` es JSONB con un array de strings. */
    let row = sqlx::query!(
        r#"
        SELECT generos_favoritos AS "generos!: serde_json::Value"
          FROM usuarios_ext
         WHERE id = $1
        "#,
        user_id,
    )
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(Vec::new());
    };
    let Some(array) = row.generos.as_array() else {
        return Ok(Vec::new());
    };
    Ok(array
        .iter()
        .filter_map(|value| value.as_str().map(str::to_owned))
        .collect())
}

fn cache_key(user_id: i32) -> String {
    format!("{CACHE_PREFIX}{user_id}")
}

async fn cache_get(
    redis: &Option<RedisPool>,
    user_id: i32,
) -> Result<Option<UserProfile>, AppError> {
    let key = cache_key(user_id);
    let payload: Option<String> = if let Some(pool) = redis {
        let mut conn = pool
            .get()
            .await
            .map_err(|error| AppError::Internal(format!("Redis pool: {error}")))?;
        conn.get(&key)
            .await
            .map_err(|error| AppError::Internal(format!("Redis GET: {error}")))?
    } else {
        cleanup_memory();
        MEMORY_CACHE
            .get(&key)
            .filter(|entry| Instant::now() < entry.value().1)
            .map(|entry| entry.value().0.clone())
    };
    payload
        .map(|json| {
            serde_json::from_str::<UserProfile>(&json)
                .map_err(|error| AppError::Internal(format!("deserializar perfil: {error}")))
        })
        .transpose()
}

async fn cache_set(
    redis: &Option<RedisPool>,
    user_id: i32,
    profile: &UserProfile,
) -> Result<(), AppError> {
    let key = cache_key(user_id);
    let payload = serde_json::to_string(profile)
        .map_err(|error| AppError::Internal(format!("serializar perfil: {error}")))?;
    if let Some(pool) = redis {
        let mut conn = pool
            .get()
            .await
            .map_err(|error| AppError::Internal(format!("Redis pool: {error}")))?;
        let _: () = conn
            .set_ex(&key, payload, CACHE_TTL_SECS)
            .await
            .map_err(|error| AppError::Internal(format!("Redis SETEX: {error}")))?;
    } else {
        MEMORY_CACHE.insert(
            key,
            (
                payload,
                Instant::now() + Duration::from_secs(CACHE_TTL_SECS),
            ),
        );
        cleanup_memory();
    }
    Ok(())
}

fn cleanup_memory() {
    let now = Instant::now();
    MEMORY_CACHE.retain(|_, (_, expires_at)| *expires_at > now);
}

#[cfg(test)]
mod tests;
