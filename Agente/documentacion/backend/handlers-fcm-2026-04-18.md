# 174A-76 - FCM Android

## Alcance
- Se porto el canal FCM Android base con registro, baja y envio directo a tokens activos del usuario.
- El backend expone los endpoints legacy `POST /api/fcm/registrar` y `POST /api/fcm/eliminar`.
- Se agrego un runtime opcional que solo se habilita cuando existe `FCM_SERVICE_ACCOUNT_JSON` o su alias legacy `KAMPLES_FCM_SERVICE_ACCOUNT_JSON`.

## Contrato HTTP
- `POST /api/fcm/registrar`
  - Request: `{ token, plataforma? }`
  - Requiere bearer auth.
  - `plataforma` se normaliza a `android` por defecto.
- `POST /api/fcm/eliminar`
  - Request: `{ token }`
  - Requiere bearer auth.
  - Respuesta legacy: `{ ok: true }`.

## Reglas de negocio portadas
- El registro conserva el upsert por `token` del legado y reactiva el dispositivo si ya existia.
- La baja exige `usuario_id + token` para evitar que un usuario elimine el token de otra cuenta.
- El envio usa FCM HTTP v1 con OAuth service-account y cachea el access token en memoria.
- Todos los campos de `data` se serializan a string, igual que en PHP.
- `tipo=mensaje_nuevo` usa canal Android `mensajes`; el resto usa `notificaciones`.
- El payload Android incluye `icon=ic_notification`, `default_sound=true`, `notification_count=1` e imagen opcional desde `actorAvatarUrl`.
- Tokens invalidados o desregistrados por FCM se marcan inactivos automaticamente.

## Implementacion
- `src/repositories/fcm.rs`
  - upsert por token, delete scoped por usuario, listado activo y desactivacion por token.
- `src/services/fcm.rs`
  - validacion de token y plataforma
  - runtime OAuth con JWT RS256 firmado localmente
  - cache de access token con margen de refresco
  - envio FCM HTTP v1 y desactivacion automatica de tokens invalidos
- `src/handlers/fcm.rs`
  - endpoints HTTP y DTOs OpenAPI
- `src/config/mod.rs`, `src/main.rs`, `src/lib.rs`
  - wiring del runtime opcional dentro de `AppState`

## Decision tecnica importante
- No se agrego un crate Google adicional. El runtime se resolvio con `jsonwebtoken + reqwest`, que ya estaban en el repo y cubren la firma RS256 y el intercambio OAuth sin introducir otra dependencia pesada.
- Configuracion invalida del service-account ahora falla al arrancar, en vez de degradar silenciosamente. Si la variable no existe, el canal queda deshabilitado; si existe pero esta rota, se prefiere fail-fast.

## Diferido explicito
- Este corte deja el canal listo para enviar, pero todavia no conecta productores concretos al fanout comun. Esa orquestacion queda para `174A-78`.
- No se implementa registro mobile, deep links ni bridge Android en este corte; solo contrato backend y runtime del canal.

## Validacion ejecutada
- `cargo sqlx prepare`
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo run -- --emit-openapi openapi.json`
- `npm --prefix frontend run codegen`
- `npm --prefix frontend run type-check`
- `npm run self-check -- -TareaId 174A-76`