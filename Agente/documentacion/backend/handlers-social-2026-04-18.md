# Handler `social` — Follows + Blocks

**Tarea:** 174A-60 (2026-04-18)

## Endpoints

### Follows
- `POST /api/follow/{userId}` → seguir usuario.
- `DELETE /api/follow/{userId}` → dejar de seguir.

### Blocks
- `POST /api/block/{userId}` body opcional `{ razon }` → bloquear (también unfollow mutuo).
- `DELETE /api/block/{userId}` → desbloquear.
- `GET /api/me/bloqueados` → lista de usuarios bloqueados con username y razón.

## Flujo Follow
1. Validar self-follow (400).
2. Verificar target existe (404).
3. `INSERT ... ON CONFLICT DO NOTHING` (idempotente).
4. Recalcular `total_seguidores` del target y `total_seguidos` del follower.
5. Disparar `AlgoPlanner::register_interaction(InteractionKind::Follow)`.

## Flujo Block
1. Validar self-block (400).
2. Verificar target existe (404).
3. `INSERT ... ON CONFLICT DO UPDATE` (permite actualizar razón).
4. Unfollow mutuo + recalc de contadores.

## Decisiones vs legado PHP
- **Rate limit** (20 follows/min, 10 bloqueos/min): NO portado.
- **Notificación al target** (`ServicioNotificaciones::follow`): desde `174A-78` sí está portada vía `NotificationFanoutService`.
- **Verificación de ban** (`AuthMiddleware::verificarCuentaActiva`): NO portado (depende de QQ71).
- **Block trigger en AlgoPlanner**: NO portado (legado tampoco lo hace).

## Ajuste posterior — 174A-78
- `POST /api/follow/{userId}` ahora dispara fanout legacy-compatible después del follow persistido:
	- inserción en `notificaciones`
	- websocket `notificacion`
	- Web Push / FCM si existen runtimes
- El side-effect es best-effort: si falla un canal secundario, el follow HTTP sigue respondiendo `200` y el error queda logueado.

## Estructura DB
- `follows(seguidor_id, seguido_id, created_at)` PK compuesta.
- `bloqueos(id, bloqueador_id, bloqueado_id, razon, created_at)` UNIQUE(par), CHECK no-autoblock.
- `usuarios_ext.total_seguidores`, `total_seguidos` recontados después de cada follow/unfollow.

## Estructuras Rust
- `FollowRepository::{follow, unfollow, recount, is_following, user_exists}`.
- `BlockRepository::{block, unblock, list}` + `BlockedUser { id, bloqueado_id, username, razon, created_at }`.
- `OkResponse { ok }`, `BlockRequest { razon? }`, `BlockedListResponse { data }`.
