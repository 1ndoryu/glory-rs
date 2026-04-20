# Plan — Adaptación de clientes externos al backend Axum

> **Fecha:** 2026-04-20
> **Tareas relacionadas:** 174A-108, 174A-109, 174A-110, 174A-111
> **Estado:** Importación 1:1 completa. Adaptación por fases pendiente.

## Contexto

Los 4 clientes (`kamples-scraper`, `Mezclador`, `mobile`, `desktop`) viven
en el legado WordPress y fueron copiados 1:1 a `clients/` para versionarse
junto al backend Rust. Cada uno apunta a `wp-json/...` y necesita migrar
a `/api/...` del nuevo backend Axum.

Este plan **no implementa la migración completa** — eso se hace por cliente
en una sesión dedicada cuando esté el endpoint Rust equivalente. Este plan
es el inventario de qué cambia y en qué orden.

---

## 174A-108 — `kamples-scraper` (Python + Scrapy)

### Endpoints WordPress en uso
| Endpoint legado | Archivo | Reemplazo Rust necesario |
|-----------------|---------|--------------------------|
| `POST /wp-json/kamples/v1/dev/extraccion/publicar-auto` | `extractor/pipeline.py:308` | `POST /api/admin/scraper/publicar-auto` |
| `POST /wp-json/kamples/v1/admin/automatizacion/reporte-lote` | `extractor/pipeline.py:350`, `scripts/cron_runner.py:212` | `POST /api/admin/scraper/reporte-lote` |

### Auth
Header `X-Kamples-Secret: <secret>` — comparar contra
`AppState.scraper_secret` (env var nueva: `SCRAPER_SECRET`).

### Tareas hijas
1. Crear módulo `src/handlers/scraper_admin.rs` con los 2 endpoints.
2. Añadir `scraper_secret: Option<String>` a `AppState`.
3. Generar OpenAPI + regenerar cliente frontend (no se usa, pero para
   consistencia).
4. Cambiar URLs en `pipeline.py` y `cron_runner.py` para que lean
   `BACKEND_URL` y `SCRAPER_SECRET` en lugar de `KAMPLES_INTERNAL_URL` y
   `KAMPLES_CRON_SECRET`.
5. Smoke test con un lote de 1 sample.

### Riesgos
- El payload del reporte usa estructura PHP-friendly (snake_case ya, OK).
- Rate-limiting: el endpoint actual no tiene; añadir middleware `tower::limit`.

---

## 174A-109 — `Mezclador` (React + TS, embebible)

### Estructura
- Componentes React puros (no monta `<App>` propio).
- `services/` apunta a `App/React/services/api*.ts` del legado vía
  `tsconfig.json` paths.
- Stores Zustand independientes.

### Plan
1. Identificar qué exports son consumidos por la SPA principal:
   `MezcladorTrack`, `MezcladorPlayer`, hooks de waveform.
2. Mover esos archivos a `frontend/src/features/mezclador/` (ya existe
   stub vacío).
3. Reemplazar imports a `services/apiSamples` por hooks Orval generados
   (`useGetSample`, `useListSamples`).
4. Reemplazar fetch de waveform JSON por endpoint Rust (probablemente
   ya existe en `/api/samples/{id}/waveform`).

### Riesgos
- Web Audio API + `AudioWorklet` — verificar que Vite dev server sirve
  los `.worklet.js` con MIME correcto.
- Tamaño bundle: el Mezclador trae ~80kb extra; cargarlo lazy.

---

## 174A-110 — `mobile` (Capacitor)

### Configuración actual
- `capacitor.config.js` → `webDir: 'www'`, `server.url` apunta a
  producción WP.
- `package.json` → `@capacitor/android`, `@capacitor/push-notifications`,
  `@capacitor/app` (deep links).

### Plan
1. Cambiar `webDir` a `../../frontend/dist` (build de la SPA Rust).
2. Eliminar `server.url` para que use el bundle local en producción
   (modo offline-first).
3. FCM:
   - Token se obtiene con `PushNotifications.register()`.
   - Enviarlo a `POST /api/fcm/register-token` (endpoint ya existe en
     `src/handlers/fcm.rs`).
4. Deep links:
   - `appLinks` en `AndroidManifest.xml` → dominio de producción nuevo.
   - Capacitor `App.addListener('appUrlOpen')` ya implementado en
     `mobile/www/js/deep-links.js` (legado) — copiar lógica a
     `frontend/src/hooks/useDeepLinks.ts`.
5. Build APK firmado con `npx cap build android`.

### Riesgos
- Permisos Android 13+ (`POST_NOTIFICATIONS`) — añadir runtime request.
- Token FCM rotates: implementar refresh listener.

---

## 174A-111 — `desktop` (Tauri 2)

### Configuración actual
- `src-tauri/tauri.conf.json` → `build.frontendDist: "dist"`.
- `vite.config.ts` proxy a producción WP.
- `services/apiDesktopAdmin.ts`, `googleAuthMobile.ts`, `syncGuards.ts`,
  `sync.tsx` apuntan a `wp-json/`.

### Plan
1. Añadir submódulo o symlink que comparta `frontend/src/api/generated/`
   con la SPA principal (evita duplicar cliente Orval).
2. Reemplazar `apiDesktopAdmin` por hooks Orval correspondientes.
3. Google OAuth: usar `useGooglePkce` ya generado (más seguro que el
   flujo `googleAuthMobile` actual).
4. `sync.tsx`: reemplazar polling cada 5min por WebSocket (`useWebSocket`
   ya existe).
5. Auto-updates Tauri:
   - Endpoint `GET /api/desktop/latest` en backend que devuelva manifest
     firmado (`{ version, url, signature }`).
   - Configurar `tauri.conf.json → updater.endpoints`.
6. Smoke test: build Windows + macOS.

### Riesgos
- Tauri 2 firma de updates: requiere generar par de llaves
  (`tauri signer generate`) y guardar pública en repo, privada en
  GitHub Secrets.
- macOS notarization requiere cuenta Apple Developer.

---

## Orden recomendado de ejecución

1. **174A-108 scraper** primero — es el más simple (2 endpoints, auth por
   secret, sin UI).
2. **174A-110 mobile** — reusa SPA + FCM ya implementado; bajo riesgo.
3. **174A-109 Mezclador** — refactor interno frontend, ningún backend nuevo.
4. **174A-111 desktop** — el más complejo (auto-updates + signing).

## Criterios de cierre

Cada tarea cierra cuando:
- Endpoint Rust correspondiente existe y tiene test.
- Cliente compila contra el nuevo backend.
- Smoke test manual con flujo end-to-end exitoso.
- Documentación actualizada en `Agente/documentacion/clientes/`.
