# Auditoría menú contextual de samples — 254A-8

Fecha: 2026-04-25
Origen: `frontend/src/legacy/utils/construirItemsMenuSample.ts` + `useMenuContextualSample.ts`

## Inventario completo de items y estado funcional

| Item | Acción | Endpoint Rust | Estado |
|---|---|---|---|
| reproducir | `reproductorStore.reproducir` | — (cliente) | OK |
| detalle | `navegar('/sample/:slug/')` | — (cliente) | OK |
| coleccion-original | `navegar('/coleccion/:slug/')` | — (cliente) | OK |
| coleccion (añadir) | abre `coleccionPicker` → `POST /api/colecciones/:id/samples` | sí | OK |
| **descargar** | `descargarSample(id, codigo?)` → `POST /api/samples/:id/descargar` | sí | OK |
| **descargar-svg** | `descargarWaveformSvg(rutaWaveform)` — fetch directo al asset | — (cliente, asset) | OK siempre que `rutaWaveform` exista |
| creador | `navegar('/perfil/:username/')` | — (cliente) | OK |
| compartir (copiar enlace) | `clipboard.copiar` | — (cliente) | OK |
| sugerencias | `panelLateralStore.abrirSugerencias` | — (cliente) | OK |
| abrir-panel | `panelLateralStore.abrirDetalle` | — (cliente) | OK |
| youtube | `window.open` | — (cliente) | OK |
| **editar** | abre modal → `actualizarSample(id, datos)` → `PUT /samples/{id}` | `PATCH /samples/:slug` | **ROTO**: método PUT vs PATCH y `id` numérico vs `slug` string |
| **corregir-ia** (admin) | `POST /samples/:id/corregir-ia` | **NO EXISTE** | **ROTO**: 404 |
| **extender-recorte** (admin) | `POST /samples/:id/extender-recorte` | **NO EXISTE** | **ROTO**: 404 |
| **verificar/quitar-verificacion** (admin) | `actualizarSample(id, { verificado })` → `PUT /samples/{id}` | `PATCH /samples/:slug` | **ROTO**: mismo problema que editar |
| quitar-sampleo | `desvincularSample(rId, lado)` → `DELETE /relaciones/:id/sample/:lado` | sí (`src/handlers/music/mod.rs`) | OK |
| inspeccionar (admin) | `setSampleInspeccion` (modal local) | — (cliente) | OK |
| compartir-gratis (admin) | `generarCodigo('sample', id)` → `POST /api/codigos-gratis/generar` | sí (`free_codes.rs`) | OK |
| invalidar-enlace (admin) | `invalidarCodigo('sample', id)` → `POST /api/codigos-gratis/invalidar` | sí | OK |
| **eliminar** | `eliminarSample(slug)` → `DELETE /samples/{slug}` | sí (`sample_catalog.rs`) | OK |
| reportar | `reportarStore.abrir('sample', id, titulo)` → posiblemente `/reportes` | parcial (`/api/admin/reportes/legales` solo lista; falta endpoint público para crear) | **A VERIFICAR** |

## Hallazgos críticos

### 1. PUT vs PATCH + id vs slug en `actualizarSample`
- Frontend: `apiPut('/samples/{id}', datos)` con id numérico.
- Backend Rust: `PATCH /samples/:slug`.
- Impacto: items **editar** y **verificar** (toggle verificado, admin) fallarán contra el backend Rust con 405 Method Not Allowed o 404 (slug=numero no encuentra sample).
- Nota: las acciones de edición de samples son una superficie crítica. Hay que decidir entre:
  - (a) Migrar el backend a `PUT /samples/:id` (numeric) — rompe coherencia con `GET /samples/:slug`.
  - (b) Migrar el frontend a `PATCH /samples/:slug` y resolver `slug` antes del request.
  - Opción recomendada: **(b)**, evita el doble identificador y mantiene REST coherente.

### 2. Endpoints faltantes en Rust
- `POST /samples/:id/corregir-ia` — usado por menú admin "Corregir IA".
- `POST /samples/:id/extender-recorte` — usado por menú admin "Extender recorte".
- (Y derivados que no son del menú contextual pero el frontend usa: `/samples/:id/imagen`, `/samples/:id/generar-siguiente`, `/samples/:id/restaurar-recorte`).

### 3. Reportes
- El frontend llama `reportarStore.abrir(...)`. Hay que confirmar a qué endpoint llega exactamente. `/api/admin/reportes/legales` solo es GET de lista. Falta endpoint público de creación de reporte.

## Conclusión

De ~21 items del menú contextual:
- **15 funcionan** (cliente puro, o backend Rust ya lo cubre).
- **3 están rotos** (editar, verificar, corregir-ia, extender-recorte — los dos últimos por endpoint inexistente; los dos primeros por mismatch PUT/PATCH + id/slug).
- **1 a verificar** (reportar).

## Tareas derivadas (a programar)

- 254A-8a — Migrar `actualizarSample` (frontend) de `PUT /samples/{id}` a `PATCH /samples/{slug}`. Resolver slug a partir de id en el call site. Tocar `apiSamples.ts` y todas las acciones del menú que pasan id numérico.
- 254A-8b — Implementar `POST /api/samples/:id/corregir-ia` en Rust (handler IA con cola de moderación).
- 254A-8c — Implementar `POST /api/samples/:id/extender-recorte` en Rust (handler de re-extracción/re-recorte).
- 254A-8d — Auditar y completar el flujo `reportar`: confirmar endpoint, implementar `POST /api/reportes` si falta.

No se hicieron cambios de código en esta auditoría. Tarea era inventariar y verificar.
