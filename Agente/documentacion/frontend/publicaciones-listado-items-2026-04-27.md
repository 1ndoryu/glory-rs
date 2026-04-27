# Publicaciones listadas con items en Rust — 2026-04-27

## Contexto

La página de comunidad y la pestaña de publicaciones del perfil podían quedar vacías aunque el backend ya devolvía publicaciones correctamente.

## Causa raíz

El backend Rust para `GET /api/publicaciones` devuelve el contrato:

- `items`
- `page`
- `per_page`

Los hooks legacy todavía intentaban leer el payload antiguo con `resp.data?.data ?? resp.data`, heredado del shape histórico del frontend WordPress.

Cuando `resp.data` era un objeto `{ items, page, per_page }`, el frontend no encontraba un array y terminaba renderizando `[]`.

## Cambio aplicado

- `frontend/src/legacy/hooks/useComunidadIsland.tsx`
  - normaliza `{ items }`, `{ data }` y arrays directos con `extraerPublicaciones()`
- `frontend/src/legacy/hooks/usePerfilIsland.ts`
  - aplica la misma normalización en la carga y recarga de publicaciones del perfil

## Resultado

Las publicaciones existentes del backend vuelven a aparecer tanto en comunidad como en perfil sin depender del shape viejo.

## Gotchas

- El navegador del agente no estaba autenticado, así que `/comunidad/` mostraba la landing pública en vez del feed. La validación decisiva fue el `curl` al backend local, que confirmó que había posts y que el contrato real usaba `items`.
- Este bug no estaba en la creación de publicaciones; el problema era solo de lectura/render del listado.
