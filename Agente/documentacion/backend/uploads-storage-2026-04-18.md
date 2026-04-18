# Uploads y Storage — 2026-04-18

## Estado actual

- `174A-26`: existe `FileStorage` con `LocalFs` path-safe.
- `174A-27`: existe `S3Storage` feature-gated con selector por env (`STORAGE_BACKEND=local|s3`).
- `174A-28`: existe `POST /api/samples/check-duplicate`.

## `POST /api/samples/check-duplicate`

- Ruta autenticada: requiere bearer token.
- Modos soportados:
  - `application/json` con `{ "audio_hash": "<sha256-hex>" }`
  - body binario crudo con cualquier otro `Content-Type`; el backend calcula SHA-256 en streaming.
- Respuesta:
  - `audio_hash`
  - `possible_duplicate`
  - `sample_id?`
  - `same_owner?`
  - `title?`
  - `message?`
  - `bytes_hashed`

## Reglas operativas

- El precheck busca en `samples.audio_hash` solo entre estados `activo` y `en_supervision`.
- El precheck no crea samples ni cancela uploads. Solo informa.
- El pipeline de audio sigue siendo la autoridad final para deduplicación exacta y moderación.
- Hay un límite actual de 256 MiB para el body binario del precheck.

## Notas de migración

- El sistema legacy usaba `hashParcial` (primeros 8KB + últimos 8KB + tamaño). La migración cambia a SHA-256 exacto porque reduce colisiones y simplifica el flujo entre web/desktop/backend.
- Desktop ya no depende de este precheck para bloquear subidas, pero la ruta queda disponible para optimizaciones futuras y para clientes que quieran evitar uploads innecesarios.