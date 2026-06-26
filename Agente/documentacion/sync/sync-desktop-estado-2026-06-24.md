# Sync Desktop — Estado Actual (junio 2026)

> **Última actualización:** 2026-06-25
> **Relacionado:** `auditoria-sync-desktop-2026-04-25.md` (audit original), tareas 254A-7a..7d, 256A-1 (plan aliases), 256A-2 (sync report + doc)

---

## 0. Portabilidad Desktop App (junio 2026)

> **Problema detectado:** La desktop app (`clients/desktop/`) se construyó originalmente dentro del tema WordPress y sus imports apuntan a rutas que **no existen** en el proyecto Rust template (`../Glory/assets/react/src`, `../App/React`).
>
> **Estado: ✅ RESUELTO (256A-1, 2026-06-25).** Aliases `@` y `@app` repuntados al SPA Rust. Type check pasa con 0 errores. Vite dev server compila correctamente.
>
> **Plan detallado:** `Agente/planes/plan-portar-desktop-app-2026-06-25.md`

### Mapeo de aliases (target)

| Alias | Tema WP (roto) | Rust SPA (correcto) | Estado |
|---|---|---|---|
| `@` | `../Glory/assets/react/src` | `../../frontend/src/glory-core` | ✅ |
| `@app` | `../App/React` | `../../frontend/src/legacy` | ✅ |
| `@api` | — | `../../frontend/src/api/generated` (ya funciona) | ✅ |
| `@desktop` | `./src` | `./src` (sin cambios) | ✅ |

---

## 1. Arquitectura General

```
┌─────────────────────────┐     HTTP/REST      ┌─────────────────────┐     SQLx     ┌──────────────┐
│   Tauri Desktop (v2)    │ ────────────────── │   Axum Backend      │ ──────────── │  PostgreSQL  │
│                         │                    │   (Rust)            │              │              │
│  clients/desktop/src/   │                    │   src/handlers/     │              │  sync_       │
│  services/              │                    │   src/repositories/ │              │  changelog   │
│  (30+ archivos)         │                    │   src/models/       │              │              │
└─────────────────────────┘                    └─────────────────────┘              └──────────────┘
```

### Flujo de sincronización

El sistema usa un protocolo de **delta sync basado en cursor**:

1. Desktop mantiene un `cursor` local (último ID visto en `sync_changelog`).
2. Polling periódico → `GET /api/me/sync/delta?cursor=N` → retorna cambios nuevos desde ese cursor.
3. Si el cursor es inválido o muy viejo → `full_sync_required: true` → fallback a `GET /api/me/sync/colecciones`.
4. Desktop aplica cambios locales (crear/renombrar carpetas, descargar samples, eliminar archivos).

---

## 2. Estado del Backend (Rust)

### 2.1 Endpoints implementados ✅

| Endpoint | Método | Handler | Estado |
|---|---|---|---|
| `/api/me/sync/delta` | GET | `sync::get_delta` | ✅ Implementado |
| `/api/me/sync/colecciones` | GET | `sync::get_colecciones_full` | ✅ Implementado |
| `/api/sync/changelog` | GET | `sync::get_changelog_legacy` | ✅ Implementado (legacy) |

### 2.2 Repositorios

| Repositorio | Estado | Funcionalidad |
|---|---|---|
| `sync_changelog.rs` | ⚠️ **Solo lectura** | `delta()` y `ultimo_cursor()`. **NO tiene método INSERT.** |
| `sync_full.rs` | ✅ Completo | `colecciones_con_samples()` y `descargas_sin_coleccion()` — full sync con CTE + json_agg |

### 2.3 Modelos (`models/sync.rs`) ✅

Tipos definidos y correctos:
- `SyncChangelogTipo` — enum con 7 variantes: `SampleAdded`, `SampleRemoved`, `SampleUpdated`, `CollectionCreated`, `CollectionRenamed`, `CollectionDeleted`, `CollectionMerged`
- `SyncChangelogEntry` — `{id, tipo, entidad_id, metadata, created_at}`
- `SyncChangelogDelta` — `{cambios, cursor, hay_mas, full_sync_required}`
- `SyncSample` — `{id, titulo, formato, tamano, imagen_url}`
- `SyncColeccion` — `{id, nombre, parent_id, version, samples}`

### 2.4 Tabla `sync_changelog` (DDL)

```sql
CREATE TABLE sync_changelog (
    id          BIGSERIAL PRIMARY KEY,
    usuario_id  INT NOT NULL REFERENCES users(id),
    tipo        TEXT NOT NULL CHECK (tipo IN (
        'sample_added','sample_removed','sample_updated',
        'collection_created','collection_renamed','collection_deleted','collection_merged'
    )),
    entidad_id  INT NOT NULL,
    metadata    JSONB DEFAULT '{}',
    created_at  TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_sync_changelog_usuario ON sync_changelog (usuario_id, id);
CREATE INDEX idx_sync_changelog_created ON sync_changelog (created_at);
```

---

## 3. 🔴 PROBLEMA CRÍTICO: sync_changelog nunca se inserta desde handlers

### Descripción

El repositorio `SyncChangelogRepository` **sí tiene** `insert_entry()` (implementado en tarea 246A-1), pero **ninguno de los 12 handlers que mutan datos lo llama**.

Esto significa que:

- **El delta sync siempre devuelve vacío** o `full_sync_required: true` (si el cursor es 0 o fue purgado).
- **El desktop nunca recibe cambios incrementales** — cada sync es un full sync costoso.
- **Los 7 tipos de `SyncChangelogTipo` existen en el modelo y el método de inserción está listo, pero nunca se invoca.**

### Puntos de escritura faltantes (12 handlers)

Los siguientes handlers necesitan llamar a `SyncChangelogRepository::insert_entry()` después de su operación exitosa:

| # | Handler | Operación | Tipo changelog | Entidad |
|---|---|---|---|---|
| 1 | `samples::upload` | Crear sample | `sample_added` | `sample.id` |
| 2 | `samples::update` | Actualizar sample | `sample_updated` | `sample.id` |
| 3 | `samples::delete` | Eliminar sample | `sample_removed` | `sample.id` |
| 4 | `samples::update_status` | Cambiar estado | `sample_updated` | `sample.id` |
| 5 | `colecciones::create_coleccion` | Crear colección | `collection_created` | `coleccion.id` |
| 6 | `colecciones::update_coleccion` | Renombrar colección | `collection_renamed` | `coleccion.id` |
| 7 | `colecciones::delete_coleccion` | Eliminar colección | `collection_deleted` | `coleccion.id` |
| 8 | `colecciones::add_sample` | Agregar sample a colección | `sample_added` | `sample_id` |
| 9 | `colecciones::remove_sample` | Quitar sample de colección | `sample_removed` | `sample_id` |
| 10 | `colecciones::merge_coleccion` | Fusionar colecciones | `collection_merged` | `source_id` |
| 11 | `biblioteca` (likes/downloads) | Descarga/like de sample | `sample_added` | `sample.id` |
| 12 | `downloads::register_download` | Registrar descarga | `sample_added` | `sample.id` |

### Impacto

- Sin changelog, el desktop hace **full sync cada vez** → O(n) con todos los samples del usuario.
- Con changelog, el delta sería O(1) por cambio → sync instantánea.
- Para usuarios con miles de samples, la diferencia es ~500ms (delta) vs ~2-5s (full sync).

---

## 4. Estado del Desktop (Tauri v2)

### 4.1 Servicios de sync (30+ archivos)

| Servicio | Archivo | Estado | Responsabilidad |
|---|---|---|---|
| **Orquestador** | `syncOrchestratorService.ts` | ✅ Completo | Lock sync, retry colas, circuit breaker, delegar a v1/v2 |
| **Watcher** | `syncWatcherSetup.ts` | ✅ Completo | File watcher local + polling estructura + callbacks archivo/carpeta |
| **Colecciones** | `syncCollectionService.ts` | ✅ Completo | Mapeo servidor↔carpeta, descarga samples, reconciliación, renombres |
| **Descarga v1** | `syncDownloadV1.ts` | ✅ Completo | Fallback full sync descarga |
| **Upload queue** | `uploadQueueService.ts` | ✅ Completo | Cola de subida con reintentos, prioridades, batching |
| **Offline queue** | `offlineQueueService.ts` | ✅ Completo | Cola de operaciones API offline → retry al reconectar |
| **Tracking** | `syncTrackingService.ts` | ✅ Completo | Persistencia local de estado de archivos y colecciones |
| **Guards** | `syncGuards.ts` | ✅ Completo | Locks, flags, headers sync, detección de conflictos |
| **Logger** | `syncLogger.ts` | ✅ Completo | Logging estructurado con niveles y categorías |
| **Estado** | `syncState.ts` | ✅ Completo | Estado global del sync (config, cursor, tracking, collectionModule) |
| **Config panel** | `configSyncService.ts` | ✅ Completo | Configuración avanzada de sync |
| **Hash** | `hashService.ts` | ✅ Completo | SHA256 de archivos, verificación de tamaño |
| **Papelera** | `papeleraService.ts` | ✅ Completo | Papelera local con expiración |
| **Journal** | `syncJournalService.ts` | ✅ Completo | Registro de operaciones para debug |
| **Rehidratación** | `syncRehidratacionService.ts` | ✅ Completo | Reintentar descargas de imágenes pendientes |
| **Registro** | `syncRegistroService.ts` | ✅ Completo | Registro de descargas completadas |

### 4.2 Adaptador API (WP → Rust)

`wpJsonRustAdapter.ts` reescribe rutas legacy de WordPress a las rutas de Rust:

```
/wp-json/kamples/v1/sync/changelog  → /api/sync/changelog
/wp-json/kamples/v1/me/sync/delta   → /api/me/sync/delta
/wp-json/kamples/v1/me/sync/colecciones → /api/me/sync/colecciones
```

Activado cuando `VITE_KAMPLES_BACKEND === 'rust'` o en builds Tauri nativas.

### 4.3 Ventanas Tauri

| Ventana | HTML | Propósito |
|---|---|---|
| Principal | `index.html` | App completa (SPA), upload queue activa |
| Sync panel | `sync.html` | Panel de sincronización dedicado |
| Config sync | `config-sync.html` | Configuración avanzada de sync |

### 4.4 Reporte de verificación sync (`window.__KAMPLES_SYNC_REPORT__`)

Añadido en 256A-2. Expone una `async function` que recolecta estado de todos los subsistemas y retorna un `SyncReport` estructurado:

| Campo | Descripción |
|---|---|
| `entorno` | Desktop mode, versión, ventana actual |
| `auth` | Estado de login (`logueado`, `userId`, `tokenValido`) |
| `config` | Carpeta seleccionada, sync activa, cursor delta, polling |
| `backend` | Conectividad HTTP, cursor delta remoto, `fullSyncRequired` |
| `tracking` | Archivos/colecciones locales, espacio ocupado |
| `uploadQueue` | Items pendientes, subiendo, con error |
| `journal` | Estado del journal de operaciones |
| `circuitBreaker` | Estado del circuito (abierto/medio/cerrado), fallos |
| `diagnosticos` | Array de `{nivel, componente, mensaje}` — auto-diagnóstico con alertas |

**Disponible desde:**
- `window.__KAMPLES_SYNC_REPORT__()` en ventana principal (`main.tsx`)
- `window.__KAMPLES_SYNC_REPORT__()` en panel sync (`sync.tsx`)
- Consola del agente: `await window.__KAMPLES_SYNC_REPORT__()`

**Persistencia a disco:** Cada vez que se llama, el reporte se guarda automáticamente en el directorio `AppData` de Tauri:
- `sync-report-{timestamp}.json` — copia única por fecha (histórico)
- `sync-report-latest.json` — siempre sobrescrito, fácil de leer desde el agente

**Ruta en Windows:**
```
C:\Users\{usuario}\AppData\Roaming\{bundle-id}\sync-report-latest.json
```

Para leerlo desde el agente (una vez que el usuario ejecuta `__KAMPLES_SYNC_REPORT__()`):
```js
// Si el agente tiene acceso al filesystem:
read_file('C:/Users/{user}/AppData/Roaming/com.kamples.desktop/sync-report-latest.json')

// O desde Playwright en la app:
const report = await page.evaluate(() => window.__KAMPLES_SYNC_REPORT__())
```

**Uso para verificación:**
1. Abrir la app desktop → consola DevTools (Ctrl+Shift+I).
2. Ejecutar `const r = await window.__KAMPLES_SYNC_REPORT__(); console.table(r.diagnosticos);`
3. El agente puede leer `sync-report-latest.json` directamente desde el FS.
4. El reporte incluye detección automática de problemas: auth faltante, carpeta no seleccionada, circuit breaker abierto, backend caído, errores en cola de subida.

---

## 5. Historial de Implementación

### Tareas completadas

#### 256A-2 — Sync report + documentación (2026-06-25)

| Tarea | Descripción | Estado |
|---|---|---|
| 256A-2a | `generarReporteSync()` + `SyncReport` interface en `syncService.ts` | ✅ |
| 256A-2b | `__KAMPLES_SYNC_REPORT__` en `Window` interface (`global.d.ts`) | ✅ |
| 256A-2c | Exposición en `main.tsx` y `sync.tsx` | ✅ |
| 256A-2d | Documentación actualizada | ✅ |

#### 254A-7 serie — 2026-04-25

| Tarea | Descripción | Estado |
|---|---|---|
| 254A-7a | Endpoints sync Rust (`/sync/changelog`, `/me/sync/delta`, `/me/sync/colecciones`) | ✅ Completada |
| 254A-7b | Migración desktop: WP JSON → API Rust (`wpJsonRustAdapter.ts`) | ✅ Completada |
| 254A-7c | Sync collection service adaptado a nuevos endpoints | ✅ Completada |
| 254A-7d | Tests y verificación de endpoints | ✅ Completada |

### Auditoría original (2026-04-25)

La auditoría encontró que **todos los endpoints sync devolvían 404** contra el backend Rust. Se implementaron los 3 endpoints y se adaptó el desktop. Sin embargo, el audit **no identificó** que `sync_changelog` nunca se escribía — porque en el flujo original PHP, los triggers de BD poblaban la tabla directamente.

---

## 6. Diferencias PHP → Rust

| Aspecto | PHP (legacy) | Rust (actual) |
|---|---|---|
| Escritura changelog | Triggers PostgreSQL + hooks en controllers | ❌ **No implementado** |
| Lectura changelog | REST endpoint + cursor | ✅ Implementado |
| Full sync | `ColeccionesCrudController::sync` | ✅ `sync_full.rs` |
| Delta format | `{cambios, cursor, hay_mas}` | ✅ `{data: {cambios, cursor, hayMas, fullSyncRequired}}` |
| Auth | WP nonce + cookie | ✅ Bearer JWT |
| Upload sync | `sync_upload` flag en metadata | ✅ Preservado |

---

## 7. Próximos Pasos (implementación)

### 7.1 Agregar `insert_entry()` a `SyncChangelogRepository`

```rust
pub async fn insert_entry(
    pool: &PgPool,
    usuario_id: i32,
    tipo: SyncChangelogTipo,
    entidad_id: i64,
    metadata: serde_json::Value,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!(
        "INSERT INTO sync_changelog (usuario_id, tipo, entidad_id, metadata) \
         VALUES ($1, $2, $3, $4) RETURNING id",
        usuario_id,
        tipo.as_str(),
        entidad_id,
        metadata
    )
    .fetch_one(pool)
    .await?;
    Ok(row.id)
}
```

### 7.2 Llamar desde los 12 handlers

Cada handler mutante debe llamar `insert_entry()` después de la operación exitosa, usando el mismo `pool` y `user.user_id`. Ejemplo:

```rust
// En create_coleccion, después del INSERT exitoso:
SyncChangelogRepository::insert_entry(
    &state.pool,
    user.user_id,
    SyncChangelogTipo::CollectionCreated,
    col.id as i64,
    serde_json::json!({"nombre": col.nombre}),
).await.ok(); // ok() para no bloquear la respuesta al usuario
```

### 7.3 Consideraciones

- **Idempotencia:** Los uploads desktop ya tienen `X-Idempotency-Key`. El changelog no necesita dedup adicional porque el cursor es incremental.
- **Performance:** `insert_entry` es fire-and-forget desde el handler (`.ok()` o log de warning). Un fallo de changelog no debe bloquear la operación del usuario.
- **Purga:** El endpoint delta ya tiene lógica de purge detection (cursor < MIN(id) → full_sync). Considerar purgar entradas > 30 días periódicamente.
- **Sample moves entre colecciones:** Actualmente `add_sample` y `remove_sample` se tratan como `sample_added`/`sample_removed`. Podrían necesitar un tipo `sample_moved` futuro, pero el protocolo actual lo maneja como par add+remove.

---

## 8. Estado Resumen

| Componente | Estado | Notas |
|---|---|---|
| Backend endpoints | ✅ | 3 endpoints funcionales |
| Backend repos (lectura) | ✅ | Delta + full sync |
| Backend repos (escritura) | ❌ | **Falta `insert_entry()`** |
| Modelos/tipos | ✅ | Completos y correctos |
| DDL tabla + índices | ✅ | Ya existe en migraciones |
| Desktop adapter (WP→Rust) | ✅ | `wpJsonRustAdapter.ts` |
| Desktop sync services | ✅ | 30+ archivos completos |
| Desktop upload queue | ✅ | Cola con reintentos y batching |
| Desktop file watcher | ✅ | Tauri FS plugin + polling |
| Desktop sync report | ✅ | `window.__KAMPLES_SYNC_REPORT__()` (256A-2) |
| **Changelog population** | ❌ | **Bloqueante: sin esto, delta sync es inútil. `insert_entry()` existe pero handlers no lo llaman (12 puntos).** |

**El único bloqueante real es la escritura de `sync_changelog` desde los handlers.** Todo lo demás está implementado y funcional. El reporte de verificación permite diagnosticar el estado actual de cada subsistema.
