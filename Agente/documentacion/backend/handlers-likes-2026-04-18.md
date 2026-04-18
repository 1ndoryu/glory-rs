# Handler `likes` — Likes polimórficos

**Tarea:** 174A-59 (2026-04-18)

## Endpoints

### `POST /api/like`
Crea o actualiza la reacción del usuario autenticado al target.

**Body:**
```json
{ "tipo": "sample", "target_id": 123, "reaccion": "like" }
```

- `tipo`: `sample` | `publicacion` | `cancion` | `relacion`.
- `reaccion` (opcional, default `like`): `like` | `dislike` | `encanta`.

**Respuestas:**
- `200 LikeResponse { ok, liked: true, reaccion }`.
- `400` si `tipo` o `reaccion` inválidos.
- `401` si no autenticado.
- `404` si el target no existe.

### `DELETE /api/like?tipo=...&target_id=...`
Elimina la reacción del usuario al target. Idempotente.

**Respuestas:**
- `200 LikeResponse { ok, liked: false, reaccion: null }`.

## Flujo interno
1. Validar `tipo` (`LikeKind::from_str`) y `reaccion` (`Reaction::from_str`).
2. Verificar que el target existe (`LikeRepository::target_exists`).
3. Upsert atómico vía `INSERT ... ON CONFLICT (usuario_id, tipo, target_id) DO UPDATE`.
4. Recontar `total_likes` en la tabla destino contando solo reacciones positivas (`like` + `encanta`). Los dislikes son privados.
5. Disparar `AlgoPlanner::register_interaction(user, InteractionKind::Like)` para refrescar buckets.

## Ajuste posterior — 174A-67
- El branch `tipo=publicacion` ahora exige `eliminado_en IS NULL` en `LikeRepository::target_exists`. Un post soft-deleted deja de ser target válido para likes aunque el ID siga existiendo.

## Decisiones vs legado PHP (`SocialController::darLike/quitarLike`)
- **Dislike:** El legado llamaba `PlanificadorAlgoritmo::registrarInteraccion(user, 'dislike')`, pero `algoritmo_estado` solo tiene contador `cnt_likes`. En Rust se usa `InteractionKind::Like` para todas las reacciones (mismo bucket).
- **Rate limit (30/min):** NO portado. Pendiente cuando exista `RateLimiter` global.
- **Notificaciones al creador:** NO portado. Llega en Fase 11.
- **Verificación de ban:** NO portado. Depende de QQ71 (BanRepository por implementar).

## Estructura DB
Tabla `likes` (creada en migración `20260417000005_social_engagement.up.sql`):
- PK compuesta lógica vía UNIQUE `(usuario_id, tipo, target_id)`.
- CHECK `tipo IN ('sample','publicacion','comentario','cancion','relacion')` (comentario lo manejará `ComentariosController` aparte).
- CHECK `reaccion IN ('like','dislike','encanta')`.
- Índices: `idx_likes_target`, `idx_likes_reaccion`, `idx_likes_usuario_created`, `idx_likes_trending_24h`.

## Tablas con `total_likes` recalculado
- `samples`, `publicaciones`, `canciones`, `relaciones_sample` — todas tienen columna `total_likes INT DEFAULT 0`.
