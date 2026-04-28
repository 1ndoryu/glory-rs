# Moderacion admin - 2026-04-28

## Alcance

El modulo de moderacion admin porta las acciones legacy de `AdminModeracionController.php` a Rust. Cubre lectura de pendientes/historial y acciones de escritura para contenido, reportes y usuarios.

## Endpoints

- `GET /api/admin/moderacion`: lista publicaciones, articulos y reportes pendientes.
- `GET /api/admin/moderacion/historial`: lista publicaciones moderadas recientemente.
- `POST /api/admin/moderar`: aprueba o rechaza `publicacion`, `comentario` o `articulo`.
- `POST /api/admin/reportes/resolver`: resuelve o descarta un reporte pendiente.
- `POST /api/admin/moderacion/rechazar-pendientes`: rechaza todas las publicaciones pendientes o en revision.
- `POST /api/admin/moderacion/banear-usuario`: aplica ban manual legacy (`baneado_hasta`, `ban_razon`).
- `POST /api/admin/moderacion/rechazar-usuario-publicaciones`: rechaza todas las publicaciones no rechazadas de un autor.

## Reglas de negocio

- Todas las rutas requieren `CurrentUser::require_admin()`.
- `accion=aprobar` mapea a `moderacion_estado='aprobado'`; `accion=rechazar` mapea a `moderacion_estado='rechazado'`.
- Rechazar publicaciones/articulos guarda razon `revision_manual` y notifica al autor.
- Aprobar articulos establece `publicado_en` si aun no existe.
- Resolver reportes solo actualiza reportes pendientes y registra `resuelto_por` + `resuelto_at`.
- Rechazar pendientes afecta publicaciones con estado `pendiente` o `revision` y no eliminadas.
- Ban manual acepta `1h`, `24h`, `7d` y `30d`; valores desconocidos caen a `24h`, igual que legacy.
- El admin no puede banear su propia cuenta.

## Implementacion

- Handler: `src/handlers/admin_moderacion.rs` traduce HTTP y expone schemas OpenAPI.
- Service: `src/services/admin_moderation/mod.rs` valida inputs, calcula duraciones, llama repositorio y crea notificaciones.
- Repository: `src/repositories/admin_moderation.rs` contiene lecturas y escrituras SQL parametrizadas.

## Gotchas

El handler legacy Rust ya tenia lecturas SQL directas. Al tocarlo, Sentinel las marco, asi que el cierre del bloque movio esas lecturas al repositorio nuevo en vez de solo agregar endpoints.

El endpoint `banear-usuario` mantiene el comportamiento legacy de ban (`baneado_hasta` y `ban_razon`) y no el flujo mas nuevo de suspension (`estado='suspendido'`).
