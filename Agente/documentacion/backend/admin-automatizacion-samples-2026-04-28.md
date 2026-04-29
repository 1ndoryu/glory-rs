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

La tarjeta admin tambien muestra una estimacion de `Proxima` ejecucion usando `ultimo_lote` + `intervalo_segundos`. Si el lote actual sigue `ejecutando`, la UI informa que el siguiente ciclo correra al terminar el lote actual.

Para forzar una corrida manual, la UI usa el endpoint existente de procesos:

- `POST /api/admin/procesos/extraccion/start`
- `POST /api/admin/procesos/scraping/start`

Se envia el `limit` actual de la tarjeta para que la ejecucion manual respete el lote configurado.

La UI tambien consulta `GET /api/admin/procesos` para distinguir entre:

- automatizacion habilitada pero proceso detenido,
- proceso corriendo sin lotes todavia,
- proceso en error.

Con ese cruce, `Proxima` ya no muestra `Pendiente de primer lote` cuando no hay evidencia suficiente. Si el proceso no esta corriendo en ese instante pero la automatizacion sigue habilitada, la tarjeta muestra `Sin ejecucion en curso` y explica si esta esperando el siguiente intervalo o el primer ciclo automatico.

## Worker automatico Rust y reporte de lotes

El scheduler real vive en `workers::spawn_automation_worker` y ejecuta `AdminAutomationService::run_due()` cada 15 segundos. Ese metodo revisa `extraccion_enabled`/`scraping_enabled`, el ultimo lote y el intervalo configurado antes de lanzar un proceso nuevo.

Cuando se lanza extracción o scraping desde el scheduler o desde el boton play, `AdminProcessService` crea primero un registro `lotes_procesamiento` en estado `ejecutando` y pasa `KAMPLES_BATCH_ID` al proceso Python. El proceso Python reporta su cierre a:

```text
POST /api/admin/scraper/reporte-lote
```

Para que el reporte funcione en local sin configurar URLs duplicadas, Rust deriva automaticamente `BACKEND_URL` desde `HOST`/`PORT` (`http://127.0.0.1:3000` por defecto) cuando no existe `BACKEND_URL`, `KAMPLES_INTERNAL_URL`, `KAMPLES_SITE_URL` o `PUBLIC_BASE_URL`. El secreto se toma de `SCRAPER_SECRET` o `KAMPLES_CRON_SECRET` y se pasa al proceso hijo como ambos nombres para conservar compatibilidad legacy.

Gotcha operacional: si un proceso termina sin reportar cierre, eso indica fallo de telemetria, no necesariamente fallo de scraping/extraccion. Rust marca ese lote como `detenido` cuando el proceso no reporto error, y los lotes historicos con `termino sin reportar cierre` no inflan `fallos_consecutivos`.

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