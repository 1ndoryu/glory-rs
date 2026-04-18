# Stream firmado HMAC para descargas — `GET /api/descargas/stream`

**Tarea:** 174A-63 (2026-04-18)

## Endpoint
- `GET /api/descargas/stream?token={token}` → entrega el archivo de un sample.
- Sin `Authorization` header — el token HMAC firmado es la auth.

## Token (`services::download_token`)
- Formato (URL-safe base64 sin padding): `{sample_id}:{user_id}:{exp}:{hex(hmac_sha256)}`
- Firma sobre `{sample_id}:{user_id}:{exp}` con `state.jwt_secret`.
- TTL por defecto 5 min cuando se emite desde `register_download`.
- Comparación de firma en tiempo constante (anti-timing).

## Integración con `register_download`
- Antes devolvía `url: /api/samples/{id}/file` (placeholder).
- Ahora devuelve `url: /api/descargas/stream?token=...` con TTL 5 min.
- El frontend hace `<a href={url} download>` o `window.location = url`.

## Servicio del archivo
- Lee `samples.ruta_original` (fallback `ruta_optimizada`) → `storage_key`.
- Carga vía `state.storage.get_bytes(key)` (ver TODO).
- Devuelve `application/octet-stream` con `Content-Disposition: attachment; filename="{titulo_sanitizado}.{ext}"`.
- Headers anti-cache (`no-store`) y `X-Content-Type-Options: nosniff`.

## Tests (`services::download_token::tests`)
- `generate_and_verify_round_trip`: ida y vuelta válida.
- `verify_rejects_wrong_secret`: rechaza firma con secret distinto.
- `verify_rejects_expired`: rechaza token expirado (manualmente construido).
- `verify_rejects_garbage`: base64 inválido y formato inválido.

## Limitaciones (TODO)
- Sin soporte de Range/206 — `storage.get_bytes` carga todo en memoria; problemático para audio multi-MB. Hay que extender el trait `FileStorage` con `get_stream(key, range) -> impl Stream`.
- Sin chequeo de cuenta activa (ban/suspensión) — el legado lo verificaba con `AuthMiddleware::verificarCuentaActiva`. Pendiente cuando se porte el flag de suspensión a Rust.
- CORS para Tauri/Capacitor: el endpoint usa `IntoResponse` estándar, así que el `CorsLayer` global aplica.

## Decisiones vs legado PHP (`DescargasStreamController`)
| Legado | Rust |
|--------|------|
| `AUTH_SALT` global | `state.jwt_secret` |
| `base64(":"+sig)` con `hash_equals` | `URL_SAFE_NO_PAD` + `constant_time_eq` |
| `readfile()` streaming | `storage.get_bytes()` (TODO streaming) |
| Verificar `cuentaActiva` | NO portado todavía |
| Headers CORS manuales por origen | Layer global Axum |
