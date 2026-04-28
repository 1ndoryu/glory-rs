# Admin automatizacion y borrado masivo de samples - 2026-04-28

## Alcance

Este documento cubre los endpoints admin finales portados desde el legacy PHP para cerrar `npm run audit:api` en cero faltantes:

- `POST /api/admin/automatizacion/reactivar`
- `DELETE /api/admin/samples/todos`

Ambos requieren usuario autenticado con rol admin.

## Automatizacion admin

`POST /api/admin/automatizacion/reactivar` acepta:

```json
{ "tipo": "extraccion" }
```

`tipo` solo puede ser `extraccion` o `scraping`. La respuesta mantiene la forma legacy:

```json
{ "ok": true, "mensaje": "Extractor de Audio reactivado correctamente." }
```

La implementacion usa `app_config` como reemplazo real de las options de WordPress:

- `extraccion_enabled = true`
- `scraping_enabled = true`
- `scraping_fallos_consecutivos = 0` al reactivar scraping

El estado e historial de automatizacion tambien pasan por `AdminAutomationService`, para que `estado`, `historial` y `reactivar` compartan validacion de tipo y acceso a `lotes_procesamiento`.

## Configuracion desde UI admin

La pestaña de historial de lotes consume tambien los endpoints existentes de `admin_config`:

- `PUT /api/admin/config/extraccion`
- `PUT /api/admin/config/scraping`

Cada tarjeta de proceso permite guardar:

```json
{ "enabled": true, "lote_size": 20, "intervalo_seg": 3600 }
```

La UI usa la palabra `Habilitado` en vez de `Activo`: ese valor significa que el ciclo automatico esta permitido por `app_config`, no que un proceso este corriendo en ese instante. La prueba operativa visual sigue siendo el ultimo lote registrado y el historial de `lotes_procesamiento`.

Al guardar, el hook refresca `GET /api/admin/automatizacion/estado` para rehidratar la tarjeta con los valores persistidos. Los errores de API se muestran con toast.

## Borrado masivo de samples

`DELETE /api/admin/samples/todos` devuelve:

```json
{ "ok": true, "eliminados": 0, "errores": 0 }
```

El flujo es deliberadamente conservador:

1. Lista todos los samples y sus rutas fisicas.
2. Normaliza las claves de storage quitando prefijos `/` y convirtiendo `\\` a `/`.
3. Borra `ruta_original`, `ruta_optimizada`, `ruta_preview`, `ruta_waveform` y el waveform derivado `.json` del original.
4. Solo agrega a borrado de BD los samples cuyos assets se limpiaron correctamente.
5. En una transaccion, nulifica referencias de `cola_extraccion_samples`, borra `samples` por IDs y recalcula `usuarios_ext.total_samples`.

Si falla el borrado fisico de un sample, ese registro no se elimina de BD y se suma en `errores`, evitando filas huerfanas o assets perdidos sin trazabilidad.

## Validacion de contrato

Despues del port:

- `cargo test --all-targets`: 181 tests OK.
- `cargo run -- --emit-openapi openapi.json`: contrato actualizado.
- `npm run codegen`: cliente Orval actualizado.
- `npm --prefix frontend run type-check`: frontend compila.
- `npm run audit:api`: 110 calls escaneados, 110 matched, 0 missing.