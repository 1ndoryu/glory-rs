# Auditoría sync desktop (Tauri) — 254A-7

Fecha: 2026-04-25
Carpeta auditada: `clients/desktop/src/services/sync*.ts`

## Hallazgo principal

El cliente desktop hace fetch directo a `${origenServidor}/wp-json/kamples/v1/...`. El backend Rust expone rutas bajo `/api/*`, NO bajo `/wp-json/kamples/v1/*`. Resultado: si el desktop apunta al backend Rust nuevo, **toda la sync falla con 404**. Solo funciona contra el WordPress legacy en `kamples.com`.

## Configuración base

- `clients/desktop/src/sync.tsx:135-140` define `__KAMPLES_CONFIG__.serverUrl = ${origen}/wp-json` (fallback a `https://kamples.com/wp-json`).
- `syncGuards.ts:165` `obtenerBaseUrlSync()` retorna `ctx?.apiUrl ?? '/wp-json'`.
- Las URLs construidas son `${baseUrl}/kamples/v1/...`.

## Endpoints usados por desktop vs estado en Rust

| Endpoint usado por desktop | Existe en Rust? | Equivalente Rust |
|----|----|----|
| `GET /wp-json/kamples/v1/colecciones` | sí, otra ruta | `GET /api/colecciones` |
| `POST /wp-json/kamples/v1/samples/:id/descargar` | sí, otra ruta | `POST /api/samples/:id/descargar` |
| `GET /wp-json/kamples/v1/colecciones/:id/samples` | sí, otra ruta | `GET /api/colecciones/:id/samples` |
| `POST /wp-json/kamples/v1/colecciones/:id/samples` | sí, otra ruta | `POST /api/colecciones/:id/samples` |
| `DELETE /wp-json/kamples/v1/colecciones/:id` | sí, otra ruta | `DELETE /api/colecciones/:id` |
| `GET /wp-json/kamples/v1/me/sync/colecciones` | **NO** | falta — endpoint específico de sync incremental |
| `GET /wp-json/kamples/v1/me/sync/delta?cursor=N` | **NO** | falta — protocolo delta para watcher |
| `GET /wp-json/kamples/v1/me/coleccionados/carpetas` | **NO** | falta |
| `GET /wp-json/kamples/v1/me/coleccionados?...` | **NO** | falta |
| `POST /wp-json/kamples/v1/me/coleccionados/:sampleId/carpeta` | **NO** | falta |
| `GET /wp-json/kamples/v1/samples/:id` | parcial | hay `/api/samples` (verificar shape) |

## Conclusión

El desktop NO funciona contra el backend Rust hoy. Funciona solo si `__KAMPLES_CONFIG__.serverUrl` apunta al WordPress legacy (kamples.com con tema instalado).

Para que el desktop funcione contra Rust hacen falta TRES bloques de trabajo (cada uno una tarea independiente):

1. **Adapter de path** en el desktop: o bien crear un wrapper `fetch` análogo a `wpJsonStub` que reescriba `/wp-json/kamples/v1/*` → `/api/*` aplicando casos especiales, o migrar todos los `fetch` a un `apiDesktop` que use `/api/` directamente.
2. **Implementar endpoints faltantes en Rust**: 
   - `GET /api/me/sync/colecciones`
   - `GET /api/me/sync/delta?cursor=...`
   - `GET /api/me/coleccionados` y `GET /api/me/coleccionados/carpetas`
   - `POST /api/me/coleccionados/:id/carpeta`
3. **Validar shapes**: cada endpoint legacy WP devuelve un JSON específico que la lógica de sync espera. Hay que comparar campo a campo con el contrato Rust.

## Estado funcional actual

- Modo legacy (WP backend): funciona si el servidor original sigue arriba.
- Modo Rust: roto (404 en todos los `/me/sync/*` y `/me/coleccionados/*`).

## Recomendación

Crear tareas separadas (no bloquear esta auditoría):
- 254A-7a: implementar `GET /api/me/sync/delta` con protocolo cursor compatible
- 254A-7b: implementar `GET /api/me/sync/colecciones`
- 254A-7c: implementar `GET/POST /api/me/coleccionados/*`
- 254A-7d: adapter de path en desktop (`apiDesktopAdapter.ts` ya existe, ampliar)

No se hicieron cambios de código en esta auditoría. La tarea era "revisar detalladamente que vaya a funcionar".
