# SPA home + auth + dashboard mínimo — 2026-04-20

## Resumen

La tarea 204A-1 reemplaza la home placeholder de `frontend/src/pages/HomePage.tsx` por una portada SPA real y añade rutas de `login`, `registro` y `dashboard` dentro del frontend Vite. La migración usa contratos reales generados por Orval contra el backend Rust ya existente; no se reintroducen fetchers manuales ni tipos escritos a mano.

## Qué se mantiene 1:1

- Contratos OpenAPI ya migrados a Rust y regenerados por Orval:
  - `GET /api/health`
  - `GET /api/samples` para descubrimiento público
  - `GET /api/me/feed` para feed autenticado
  - `POST /api/auth/login`
  - `POST /api/auth/register`
  - `GET /api/users/me`
  - `GET /api/dashboard/stats`
- Convención de sesión existente en el frontend nuevo:
  - `localStorage.token`
  - `localStorage.refresh`
  - invalidación de `getMeQueryKey()` después de login/register
- Paths SPA consistentes con la estructura de producto: `/`, `/auth/login`, `/auth/registro`, `/dashboard`, `/perfil/:username`, `/sample/:slug`.

## Qué se reescribe

- La UI de entrada y la portada no se importan 1:1 desde `frontend/src/features/` ni desde el legado WordPress.
- Se reescriben estas piezas dentro del frontend tipado y compilable:
  - `frontend/src/pages/AuthPage.tsx`
  - `frontend/src/pages/LoginPage.tsx`
  - `frontend/src/pages/RegisterPage.tsx`
  - `frontend/src/pages/DashboardPage.tsx`
  - `frontend/src/pages/HomePage.tsx`
  - `frontend/src/router.tsx`
  - `frontend/src/components/ui/{Boton,Input}.tsx`
  - `frontend/src/styles/*.css`
- Motivo de la reescritura: las islands importadas en `frontend/src/features/` siguen excluidas por `frontend/tsconfig.json` y arrastran dependencias `@app/*`, componentes UI y stores del legado que todavía no existen como base estable dentro del frontend SPA.

## Revisión del legado y decisión tomada

### Revisado

- `frontend/src/features/auth/*`
- `frontend/src/features/admin/DashboardCreadorIsland.tsx`
- `frontend/src/features/mezclador/README.md`
- hooks y modelos generados por Orval en `frontend/src/api/generated/*`

### Decisión

- **No** se importa 1:1 la UI legacy de auth/dashboard en esta fase.
- **Sí** se reutilizan 1:1 los endpoints y modelos ya migrados a Rust.
- El “dashboard mínimo” de 204A-1 es una versión nueva, pequeña y tipada sobre `GET /api/users/me` + `GET /api/dashboard/stats`.
- La adaptación completa de features legacy sigue pendiente para tareas específicas como `174A-109b-fase2`.

## Verificación

- `npm --prefix frontend run type-check` OK.
- `npm --prefix frontend run build` OK.
- `GET http://127.0.0.1:3000/api/health` OK → `{"status":"ok","version":"0.1.0"}`.
- `GET http://127.0.0.1:3000/api/samples?page=1&per_page=2` OK con datos públicos.
- `GET /api/feed` sin sesión devuelve 401; por eso la home anónima usa `GET /api/samples` y la autenticada usa `GET /api/me/feed`.

## Pendiente relacionado

- `174A-109b-fase2`: adaptar imports `@app/*` y servicios legacy de Mezclador/otras features para poder recuperar más UI 1:1 dentro de `frontend/src/features/`.
