# Uploads y Storage — 2026-04-18

## Estado actual

- `174A-26`: existe `FileStorage` con `LocalFs` path-safe.
- `174A-27`: existe `S3Storage` feature-gated con selector por env (`STORAGE_BACKEND=local|s3`).
- `174A-28`: existe `POST /api/samples/check-duplicate`.
- `174A-29`: existe `POST /api/samples/upload` con multipart + idempotencia + validación MIME.

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

## `POST /api/samples/upload`

- Ruta autenticada: requiere bearer token.
- Content-Type: `multipart/form-data`.
- Campos relevantes:
  - `audio` obligatorio.
  - `tags` obligatorio con mínimo 2 tags. Acepta CSV o JSON array.
  - `titulo`, `contenido` opcionales.
  - `permitir_descarga`, `licencia_libre`, `es_premium`, `mostrar_en_comunidad`, `sync_upload` opcionales.
  - `origen_subida`, `precio` opcionales.
  - `X-Idempotency-Key` opcional en header; si se repite durante 1 hora para el mismo usuario, el backend devuelve el mismo resultado sin crear un sample nuevo.
- Validaciones actuales:
  - extensiones permitidas: `wav`, `mp3`, `flac`, `aiff`/`aif`, `ogg`.
  - magic bytes obligatorios y coherentes con el contenido real.
  - tamaño máximo: 50 MiB.
  - la cuenta debe estar `activa`.
  - límite por plan: `free=100`, `pro/premium=20000`.
- Efectos persistentes:
  - guarda el archivo original en `storage_root` bajo `samples/{user_id}/{yyyy}/{mm}/{slug}.{ext}`.
  - calcula y persiste `audio_hash` SHA-256 exacto.
  - crea el sample en estado `procesando`.
  - incrementa `usuarios_ext.total_samples`.
  - encola `cola_procesamiento_ia(tipo='sample', operacion='analisis_audio')` con metadata del upload.
- Respuesta:
  - `ok`
  - `sample_id`
  - `id_corto`
  - `slug`
  - `url`
  - `estado`

## Servido de archivos y tooling

- Cuando `STORAGE_BACKEND=local`, el backend publica `storage_root` en `/uploads/*` mediante `ServeDir`.
- Si `PUBLIC_BASE_URL` está presente, las respuestas devuelven URL absoluta; si no, devuelven URL relativa `/uploads/...`.
- El cliente frontend ahora regenera contra `../openapi.json` en vez de depender de `http://localhost:3000/api-docs/openapi.json`.
- `frontend/src/api/axios-instance.ts` adapta la firma `url + options` que genera Orval y normaliza headers/cancelación para React Query.

## Notas de migración

- El sistema legacy usaba `hashParcial` (primeros 8KB + últimos 8KB + tamaño). La migración cambia a SHA-256 exacto porque reduce colisiones y simplifica el flujo entre web/desktop/backend.
- Desktop ya no depende de este precheck para bloquear subidas, pero la ruta queda disponible para optimizaciones futuras y para clientes que quieran evitar uploads innecesarios.
- El flujo de upload mantiene la semántica clave del legado (idempotencia, límites de plan, tags mínimos), pero evita acoplarse a WordPress y deja la orquestación posterior en la cola del pipeline Rust.