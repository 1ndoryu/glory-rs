# Contribuciones comunitarias - 2026-04-28

## Alcance

Este flujo porta las contribuciones legacy de Kamples a Rust sin stubs. El backend expone endpoints de usuario para proponer relaciones, ediciones y eliminaciones, mas endpoints admin para listar/moderar pendientes.

## Endpoints de usuario

- `POST /api/contribuciones`: crea una propuesta nueva en `contribuciones_pendientes`.
- `GET /api/contribuciones/mis`: lista las propuestas del usuario autenticado.
- `PUT /api/contribuciones/{id}`: permite editar una propuesta propia mientras siga pendiente.
- `DELETE /api/contribuciones/{id}`: elimina una propuesta propia pendiente.
- `POST /api/contribuciones/edicion`: crea una propuesta de cambio sobre una relacion existente.
- `POST /api/contribuciones/eliminacion`: crea una propuesta de eliminacion sobre una relacion existente.

## Endpoints admin

- `GET /api/admin/contribuciones`: lista pendientes con paginacion.
- `POST /api/admin/contribuciones/moderar`: aprueba o rechaza una contribucion pendiente.

## Reglas de negocio

- Las propuestas duplicadas para la misma relacion pendiente se rechazan con conflicto.
- Una relacion ya existente no puede proponerse como nueva otra vez.
- Las ediciones solo aceptan campos conocidos de relaciones sample; campos desconocidos se descartan.
- Las eliminaciones requieren una razon minima de 10 caracteres.
- Al aprobar una contribucion nueva con cancion nueva, el servicio crea o reutiliza artista por nombre, crea la cancion y luego crea la relacion comunitaria.
- Al aprobar una edicion, se actualiza la relacion existente via `MusicRepository::update_relation`.
- Al aprobar una eliminacion, se elimina la relacion via `MusicRepository::delete_relation`.
- La moderacion marca la contribucion como `aprobada` o `rechazada`, registra moderador, nota y relacion resultante cuando aplica.

## Implementacion

- Handler: `src/handlers/admin_contribuciones.rs` mantiene el contrato HTTP y schemas OpenAPI.
- Service: `src/services/contribuciones/mod.rs` valida reglas de negocio y aplica aprobaciones.
- Repository: `src/repositories/contribuciones.rs` encapsula SQL parametrizado contra `contribuciones_pendientes`, `canciones`, `artistas` y `relaciones_sample`.

## Gotchas

SQLx corre en modo offline durante validacion. Por eso este repositorio usa `sqlx::query` y `query_as::<_, T>()` parametrizados en runtime con suppressions justificadas, en lugar de macros nuevas que exigirian regenerar cache `.sqlx` con una base local activa.

El auditor `npm run audit:api` detecta llamadas estaticas, pero no todas las rutas template dinamicas. `PUT/DELETE /api/contribuciones/{id}` se implementaron igualmente porque los servicios frontend las invocan.
