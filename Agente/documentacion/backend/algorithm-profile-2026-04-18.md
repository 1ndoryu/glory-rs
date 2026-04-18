# Algoritmo · Perfil de usuario (`algorithm::profile`)

Fecha: 2026-04-18

## Propósito

Construir el perfil de preferencias de un usuario para alimentar al motor de
recomendación (`174A-52`) y al selector de candidatos (`174A-51`). Es el puente
entre las interacciones (likes, reproducciones, descargas) y las señales
contextuales (BPM/key/scale/tipo/creador).

Replica el contrato de `App\Kamples\Services\PerfilUsuario` (PHP) del legado
vivo, con el mismo formato de campos para que las señales y CTEs portadas en
174A-49 puedan consumirlo sin transformaciones intermedias.

## Contrato público

```rust
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

ProfileService::build(pool, redis, user_id) -> Result<UserProfile, AppError>;
ProfileService::invalidate(redis, user_id)  -> Result<(), AppError>;
UserProfile::cold_start(user_id, generos)   // helper sin interacciones
UserProfile::is_cold_start()                // interactions == 0
```

`build` resuelve en 3 queries:

1. **Géneros declarados** (`usuarios_ext.generos_favoritos` JSONB → `Vec<String>`).
2. **CTE unificada** que combina likes + reproducciones + descargas y devuelve
   en un único roundtrip: `interactions`, `bpm_avg`, `key_fav`, `scale_fav`
   (lower-cased), `type_fav`. Equivale a
   `UsuariosExtRepository::perfilCompletoParaAlgoritmo` del legado.
3. **Top 5 creadores favoritos** (sólo si hay interacciones), con pesos
   `like=1.0`, `encanta=2.0`, `reproducción=0.5`, `descarga=1.5` y mínimo
   `SUM(score) >= 2.0`.

Si `interactions == 0` se devuelve un perfil cold-start (sólo `user_id` +
`declared_genres`), saltando la query de creadores.

## Cache

- Backend dual: Redis (`deadpool_redis::Pool`) si está configurado, fallback a
  `DashMap` en memoria con expiración perezosa. Mismo patrón que
  `services::idempotency::IdempotencyStore`.
- TTL **1800 s (30 min)**, alineado con el legado y con la entrada del roadmap
  (`174A-50 — algorithm/profile.rs (PerfilUsuario, TTL 30min)`).
- Clave `kamples_perfil_usr_{user_id}` — idéntica a la PHP para coexistencia
  durante la migración.
- Invalidación explícita vía `ProfileService::invalidate(redis, user_id)` para
  que el `PlanificadorAlgoritmo` (174A-55) la dispare en recálculo rápido o
  preciso (no se invalida en cada cache miss del feed).

## Mapeo de columnas DB ↔ Rust

| Postgres                       | Rust                  |
| ------------------------------ | --------------------- |
| `samples.bpm`                  | `bpm_avg: Option<i32>`|
| `samples.key`                  | `key_fav: Option<String>` |
| `samples.escala`               | `scale_fav: Option<String>` (lower-cased en SQL) |
| `samples.tipo`                 | `type_fav: Option<String>` |
| `samples.creador_id`           | `favorite_creators: Vec<i32>` |
| `usuarios_ext.generos_favoritos` (JSONB) | `declared_genres: Vec<String>` |

## Dependencias internas

- `crate::errors::AppError` para tipado de errores.
- `sqlx::PgPool` para queries (validadas en compile-time vía `query!`).
- `deadpool_redis::Pool` opcional para cache distribuido.
- `serde_json` para parsear JSONB de géneros y para serializar el perfil al
  cache.

## Tests (`src/algorithm/profile/tests.rs`)

- `cold_start_keeps_only_user_id_and_genres`
- `cache_key_uses_legacy_prefix`
- `user_profile_round_trips_through_json`
- `invalidate_without_redis_is_noop_safe`

Las queries SQL se validan en compile-time contra la BD local (sqlx prepare),
por lo que no se duplican como tests de integración aquí; los consumidores
(174A-51/52) las ejercitarán end-to-end.

## GLORY-RS

Es **específico de Kamples**: nombres de tablas (`samples`, `likes`,
`reproducciones`, `descargas`, `usuarios_ext`), columnas (`creador_id`,
`generos_favoritos`) y semántica de pesos pertenecen al dominio de música y
descubrimiento. No procede mover a `glory-rs`.
