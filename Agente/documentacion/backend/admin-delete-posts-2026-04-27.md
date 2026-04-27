# Admin delete de publicaciones ajenas — 2026-04-27

## Contexto

En comunidad aparecían publicaciones de usuarios de smoke test que no podían borrarse desde la UI aunque el usuario autenticado fuese admin.

## Causa raíz

`DELETE /api/publicaciones/:id` reutilizaba `ensure_owner_original_post()`, que exige coincidencia exacta entre `autor_id` y `user_id`. Eso bloqueaba tanto a usuarios normales como a admins por igual.

## Cambio aplicado

- `src/handlers/posts.rs`
  - `delete_post()` ahora usa `ensure_can_delete_original_post()`
  - el nuevo guard permite borrar si:
    - el usuario es el autor original, o
    - el usuario tiene `rol == "admin"`
  - se mantiene la restricción que impide borrar reposts por este endpoint

## Resultado

Los admins pueden limpiar publicaciones ajenas desde la UI sin romper la regla de que los reposts se eliminan por su endpoint específico.

## Gotchas

- El repositorio ya hacía el soft delete recibiendo `autor_id`; por eso el guard nuevo devuelve `owner_id` y `delete_post()` lo reusa al persistir, en vez de pasar `user.user_id` cuando borra un admin.
- El navegador del agente no estaba autenticado, así que la validación final fue por `cargo check`, no por interacción visual directa.
