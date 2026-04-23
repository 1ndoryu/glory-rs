# Adaptación cliente Desktop (Tauri 2) al backend Axum

> **Fecha:** 2026-04-22
> **Tarea:** 174A-111b
> **Estado:** Base estructural lista. Migración granular de services pendiente.

## Objetivo

Migrar el cliente `clients/desktop/` (Kamples Desktop, Tauri 2) del backend
WordPress (`wp-json/...`) al backend Axum (`/api/...`) preservando:

- Auto-updates con firma minisign (Tauri 2 plugin updater).
- Google OAuth PKCE (deep links `kamples://auth`).
- Sync de carpetas + WebSocket en tiempo real.
- Acceso a recursos compartidos del SPA Rust (cliente Orval + tipos).

## Lo que ya está hecho (174A-111b)

### 1. Cliente Orval compartido

`clients/desktop/vite.config.ts` ahora expone el alias:

```ts
'@api': resolve(__dirname, '../../frontend/src/api/generated')
```

Cualquier import del desktop puede usar los hooks/funciones generados del
SPA Rust sin duplicar el codegen:

```ts
import { useGetSample, useListSamples } from '@api/sample-catalog/sample-catalog';
import { googlePkce } from '@api/auth/auth';
```

### 2. Endpoint de auto-updates en backend

`src/handlers/app_versions.rs` añade:

```
GET /api/app/updater/:target/:arch/:current_version
```

Devuelve manifest Tauri 2 (`{ version, notes, pub_date, url, signature }`)
si `KAMPLES_DESKTOP_*_VERSION` está mayor que la versión actual del cliente,
o `204 No Content` si está al día.

Variables de entorno (resolución por especificidad):

| Más específica → genérica | Ejemplo |
|---------------------------|---------|
| `KAMPLES_DESKTOP_<TARGET>_<ARCH>_VERSION` | `KAMPLES_DESKTOP_WINDOWS_X86_64_VERSION` |
| `KAMPLES_DESKTOP_<TARGET>_VERSION`        | `KAMPLES_DESKTOP_WINDOWS_VERSION` |
| `KAMPLES_DESKTOP_VERSION`                 | (fallback global) |

Mismo patrón para `URL`, `SIGNATURE`, `PUB_DATE`, `NOTES`.

### 3. tauri.conf.json actualizado

- `updater.endpoints` → `https://api.kamples.com/api/app/updater/{{target}}/{{arch}}/{{current_version}}`
- CSP: dominios `kamples.com` reemplazados por `api.kamples.com` y se
  añade `http://localhost:3000` + `ws://localhost:3000` para dev.
- `pubkey` minisign mantiene la clave pública existente.

### 4. Vite proxy `/api`

El dev server del desktop ahora proxia `/api/*` y `/uploads/*` al backend
Rust (`http://localhost:3000` por defecto, override con `KAMPLES_API_TARGET`).

`/wp-json/*` se mantiene como proxy transicional para que los services
legacy aún funcionen mientras se migran.

## Lo que queda pendiente (tareas siguientes)

Cada bloque = 1 tarea futura, 1 commit independiente.

### A. Migrar `services/apiDesktopAdapter.ts` y derivados

Los 38 archivos de `clients/desktop/src/services/` apuntan a `wp-json/`.
Migrarlos uno a uno reemplazando fetchs manuales por hooks Orval del
alias `@api`.

Prioridad sugerida:

1. `apiDesktopAdapter.ts` (capa transversal)
2. `googleAuthDesktopService.ts` + `googleAuthDeepLink.ts` → usar `googlePkce` de `@api/auth/auth`
3. `desktopService.ts`, `syncOrchestratorService.ts` (núcleo de sync)
4. Resto en orden de uso.

### B. WebSocket sync — eliminar polling

`clients/desktop/src/sync.tsx` y archivos hermanos hacen polling cada 5min
contra `/wp-json/...`. Reemplazar por:

```ts
import { useWebSocket } from '@app/hooks/useWebSocket';
```

(donde `@app` apunte al SPA Rust, no al legacy WP — ver punto C).

El backend Axum ya expone:

- `GET /api/ws/ticket` — emite ticket HMAC autenticado.
- `WS /api/ws/upgrade?ticket=...` — canal con eventos `notif.*`, `sync.*`.

El payload de eventos sync existe en backend (`src/handlers/sync.rs`); falta
declarar el contrato de eventos en `messages-changelog` o similar.

### C. Repuntar alias `@app` al SPA Rust

Actualmente `@app` resuelve a `../App/React` (legacy WP). Una vez que la
migración de services esté completa, cambiar a:

```ts
'@app': resolve(__dirname, '../../frontend/src')
```

Esto hace que TODO el desktop comparta hooks, stores y componentes del
SPA Rust sin duplicación.

### D. Generar par de claves para auto-updates en producción

```powershell
# En máquina del release manager:
cargo install tauri-cli --version "^2.0"
cargo tauri signer generate -w ~/.tauri/kamples.key
```

- La clave pública (base64) ya está en `tauri.conf.json:updater.pubkey`.
- La privada NUNCA va al repo. Guardar en GitHub Secrets como
  `TAURI_SIGNING_PRIVATE_KEY` para que CI la use al firmar releases.

### E. Workflow CI release

Pipeline GitHub Actions que en cada tag `desktop-v*`:

1. Build Tauri (`cargo tauri build`) en runners windows-latest, macos-latest,
   ubuntu-latest.
2. Sube los `.msi`/`.dmg`/`.AppImage` a un release de GitHub.
3. Llama a `coolify-manager-rs` (o curl) para setear las env vars del backend:
   - `KAMPLES_DESKTOP_WINDOWS_VERSION`
   - `KAMPLES_DESKTOP_WINDOWS_URL` (URL al `.msi` del release)
   - `KAMPLES_DESKTOP_WINDOWS_SIGNATURE` (contenido del `.sig` que genera Tauri)

A partir de ese momento, los clientes con versión menor recibirán el manifest
y se autoactualizarán.

## Riesgos / gotchas conocidos

- **macOS notarization** requiere cuenta Apple Developer ($99/año). Sin
  notarización, los usuarios verán el warning de "app no firmada".
- **Tauri 2 cache de updater**: el plugin cachea el manifest por 24h en
  `%APPDATA%/Kamples/updater-cache.json`. Para forzar check inmediato,
  borrar ese archivo o llamar `await check({ force: true })`.
- **Comparador semver** en `app_versions.rs::is_newer_than_current` es
  simplificado (solo MAJOR.MINOR.PATCH). Si en algún momento se usan
  pre-releases (`0.2.0-beta.1`) hay que portar al crate `semver`.
- **CSP**: el dominio `api.kamples.com` es placeholder. Cuando el dominio
  real cambie, actualizar `tauri.conf.json:app.security.csp` y rebuild.
