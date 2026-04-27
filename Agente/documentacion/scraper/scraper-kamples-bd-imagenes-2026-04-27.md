# Scraper Kamples — BD canónica e imágenes (2026-04-27)

> Reemplaza referencias previas que asumían dos BDs (`kamples` + `glory_kamples`).

## BD canónica
- **Una sola BD:** `glory_kamples`.
- El backend Rust lee de ahí (`DATABASE_URL`).
- El scraper Python escribe ahí (`KAMPLES_PG_DBNAME=glory_kamples`).
- La BD legacy `kamples` se conservó como `kamples_legacy_2026_04_27` (solo respaldo manual; no la usa nadie).

## Imágenes (mirror local)
WhoSampled bloquea hotlinking (HTTP 403). Toda imagen debe descargarse al filesystem local.

| Variable             | Valor (dev Windows)                                                                  | Rol                                  |
|---------------------|---------------------------------------------------------------------------------------|--------------------------------------|
| `IMAGES_STORE_PATH` | `C:/Users/Owner/OneDrive/Documentos/glory-rust-template/uploads/kamples/portadas`    | Filesystem absoluto donde se guarda  |
| `IMAGES_BASE_URL`   | `/uploads/kamples/portadas`                                                           | URL pública (servida por Axum)       |

- El backend Axum monta `ServeDir::new(STORAGE_ROOT)` (default `./uploads`) en la ruta `/uploads`. El frontend solo hace `<img src="{imagen_url}">` y funciona.
- En producción el path absoluto cambia (`/app/uploads/...`), la URL relativa no.

## Flujo del scraper sobre imágenes
1. `ImageDescargaPipeline` (prioridad 200, antes de Postgres) intercepta cada `cancion_destino`/`cancion_fuente`.
2. Para cada `imagen_url` externa: descarga vía `curl_cffi` con proxy DataImpulse y headers de browser, guarda en disco con nombre `sha256(url)[:40]+ext`.
3. Reemplaza el `imagen_url` del item por la URL local (`IMAGES_BASE_URL/<hash>.<ext>`).
4. Si falla tras 3 intentos, deja `imagen_url=None` (el frontend renderiza placeholder).

Si las dos vars no están definidas, el pipeline se desactiva con un warning y los items pasan con la URL externa intacta.

## Backfill (script standalone)
`clients/kamples-scraper/scripts/backfill_imagenes.py`:
```
cd clients/kamples-scraper
.\.venv\Scripts\python.exe scripts\backfill_imagenes.py [--solo canciones|artistas] [--limite N]
```
Selecciona filas con `imagen_url LIKE 'http%whosampled%'` y las reemplaza por la URL local. Compatible con `canciones` y `artistas_musicales`.

## Migración de datos legacy (2026-04-27, 274A-3)
Si alguna vez aparecen URLs `http://glory.local/wp-content/uploads/kamples/portadas/...` (entorno WordPress local antiguo):
```sql
UPDATE canciones
SET imagen_url = REPLACE(imagen_url,
    'http://glory.local/wp-content/uploads/kamples/portadas',
    '/uploads/kamples/portadas')
WHERE imagen_url LIKE 'http://glory.local/wp-content/uploads/kamples/portadas%';
```
Y copiar los archivos físicos de `wp-content/uploads/kamples/portadas/` al `uploads/kamples/portadas/` del repo Rust.

## CHECK de `scraping_log`
La whitelist de `tipo_pagina` debe incluir `track_overview` (el spider sample_detail.py los auto-encola). Migración: `20260427000001_scraping_log_track_overview.up.sql`.
