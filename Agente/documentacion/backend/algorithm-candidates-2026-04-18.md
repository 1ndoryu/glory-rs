# `algorithm::candidates` — SelectorCandidatos (174A-51)

> Última actualización: 2026-04-18
> Tarea origen: 174A-51 (porte de `App\Kamples\Services\SelectorCandidatos.php`)

## Propósito

Pre-filtrar ~1000 IDs de samples vía 6 fuentes con index scans rápidos
(O(log N)) para que el recomendador (174A-52) corra el scoring solo sobre
candidatos. Permite que el feed escale a 1M+ samples sin O(N).

## Diferencias con el legado

| Aspecto                | PHP (`SelectorCandidatos`)                                                                 | Rust (`algorithm::candidates`)                              |
| ---------------------- | ------------------------------------------------------------------------------------------ | ----------------------------------------------------------- |
| Salida                 | Fragmento SQL `candidatos AS (...)` para inyectar en el CTE del recomendador               | `Vec<i32>` de IDs candidatos                                |
| Validación SQL         | Strings construidos en runtime, params con PDO                                             | Macros `sqlx::query!` validadas en compile-time (`.sqlx/`)  |
| Filtro de bloqueos     | Diferido al `MotorRecomendacion`                                                           | Aplicado dentro del selector (bidireccional)                |
| Whitelist `días`       | `in_array($d, [7,14,30,60,90])`                                                            | Match exhaustivo + fallback a 14                            |
| Cache conteo           | `ServicioCache` (transient WP)                                                             | Redis (`set_ex`) o `DashMap` en memoria, TTL 3600s          |

## API pública

```rust
pub struct CandidatesConfig { /* 6 límites + dias_trending */ }
impl CandidatesConfig {
    pub const fn legacy_defaults() -> Self;          // 300/200/200/200/100/150, 14d
    pub fn safe_dias_trending(&self) -> i32;         // whitelist [7,14,30,60,90]
}

pub struct CandidatesService;
impl CandidatesService {
    pub async fn count_active(pool, redis) -> Result<i64, AppError>;
    pub async fn invalidate_count(redis) -> Result<(), AppError>;
    pub async fn select(
        pool, user_id, profile: &UserProfile,
        profile_vector: Option<&Vector>, config: &CandidatesConfig,
    ) -> Result<Vec<i32>, AppError>;
}
```

## Las 6 fuentes

1. **Trending** — `samples` activos con `publicado_at > NOW() - INTERVAL`,
   ordenados por `(likes·2 + repro + desc·3)`.
2. **Embedding ANN** — `samples.embedding <=> $vector LIMIT N` (pgvector).
   Solo si se provee `profile_vector` (174A-52 lo calculará).
3. **Seguidos** — `creador_id IN (SELECT seguido_id FROM follows WHERE
   seguidor_id = $user_id)`, orden por `publicado_at DESC`.
4. **Top-tags** — `samples.tags && $top_tags::text[]`. Top tags se calculan
   uniendo `UserProfile.declared_genres` + tags de samples likeados (max 10).
5. **Populares all-time** — orden por `(likes + repro + desc) DESC`.
6. **No reproducidos** — `NOT EXISTS (SELECT 1 FROM reproducciones)`,
   `ORDER BY RANDOM()`. Garantiza frescura sin importar historial.

Cada fuente excluye `creador_id <> ALL($blocked::int[])` con bloqueos
bidireccionales (a quién bloqueó + quiénes lo bloquearon).

## Cache de conteo

Clave `kamples_total_samples_activos`, TTL 3600s. El recomendador llamará
`CandidatesService::count_active` para decidir si activar el selector
(`> umbral_candidatos`, default 5000) o aplicar scoring sobre todo el set.

Invalidar manualmente con `CandidatesService::invalidate_count` al
publicar/eliminar un sample (174A-52/53 conectarán el hook).

## Tests

`src/algorithm/candidates/tests.rs`:

- `legacy_defaults_match_php_constants`
- `safe_dias_trending_falls_back_to_14_for_unknown_value`
- `safe_dias_trending_keeps_whitelisted_values`
- `invalidate_count_without_redis_is_safe`

Las queries reales se cubren vía `sqlx prepare` (compile-time check contra
`glory_kamples`) + tests de integración del recomendador (174A-52 en
adelante).

## GLORY-RS

**No aplica** — la lógica es 100% específica de Kamples (samples, follows,
reproducciones, semántica de bpm/key/escala/tipo). No hay subset reusable
para `glory-rs`.

## Pendientes / siguientes tareas

- 174A-52 `recommender.rs`: consumir `select(...)` + scoring + cache
  stale-while-revalidate.
- 174A-53 `precompute.rs`: bulk LIMIT*3 para warm-up.
- Hook de invalidación: cuando un sample pasa a `activo`/sale de `activo`,
  llamar `CandidatesService::invalidate_count(&redis)`.
