# Plan: Robustez integral de sincronización Haddock

## Problema
La integración actual envía ventas a Haddock pero carece de:
- Prevención de duplicados (si hay error y retry, o si se edita y re-envía)
- Tracking de qué ventas fueron sincronizadas
- Bloqueo de eliminación de ventas (Haddock no tiene DELETE)
- Alertas al usuario cuando edita datos ya sincronizados
- Manejo de escenarios de concurrencia y errores parciales
- Informe/log visible de sincronizaciones

## Análisis de riesgos identificados

| Escenario | Riesgo actual | Impacto |
|-----------|--------------|---------|
| Crear venta → sync OK | ✅ Funciona | Bajo |
| Crear venta → sync falla 3x | ⚠️ Venta existe local pero NO en Haddock, sin registro | Alto |
| Editar venta ya sincronizada | ⚠️ Re-envía POST (¿Haddock crea duplicado o actualiza por externalId?) | **Crítico** |
| Editar nombre cliente vinculado | ⚠️ Haddock no se entera (no enviamos nombre de cliente) | Medio |
| Eliminar venta sincronizada | ⚠️ Se borra local pero persiste en Haddock → datos inconsistentes | **Crítico** |
| Crear venta + error de red + retry manual | ⚠️ Posible duplicado si primer POST llegó pero no confirmó | Alto |
| 2 ediciones rápidas concurrentes | ⚠️ 2 spawns simultáneos, posible race | Medio |
| Token inválido configurado | ⚠️ Todos los syncs fallan silenciosamente (solo logs) | Alto |

## Supuesto sobre externalId de Haddock
La API de Haddock recibe `externalId` en cada order. **Necesitamos confirmar** si Haddock:
- (A) Usa externalId como clave única (upsert) → editar = actualizar
- (B) Ignora externalId como clave → editar = duplicado

**Hasta confirmar, asumimos el peor caso (B)**: editar crea duplicado. Diseñamos para prevenir.

## Fases de implementación

### FASE 1: Tracking de sincronización (BD + modelo)
**Tarea 064A-6**

1. **Migración**: agregar columnas a tabla `ventas`:
   ```sql
   ALTER TABLE ventas
     ADD COLUMN haddock_synced BOOLEAN NOT NULL DEFAULT false,
     ADD COLUMN haddock_synced_at TIMESTAMPTZ,
     ADD COLUMN haddock_sync_error TEXT;
   ```
   - `haddock_synced`: true si la última sincronización fue exitosa
   - `haddock_synced_at`: timestamp de última sincronización exitosa
   - `haddock_sync_error`: último mensaje de error (null si OK)

2. **Modelo Venta**: agregar los 3 campos nuevos
3. **Repositorio Venta**: actualizar SELECT/INSERT queries con nuevos campos
4. **Nuevo método en repo**: `update_haddock_status(id, synced, error_msg)`
5. **HaddockService**: después de sync exitoso → marcar `haddock_synced=true`; si falla → marcar `haddock_synced=false` + guardar error 
6. **.sqlx/ cache**: regenerar todos los JSON afectados
7. **Tests**: verificar que el status se actualiza correctamente

### FASE 2: Prevención de duplicados + idempotencia
**Tarea 064A-7**

1. **Guard en sync_order**: si `venta.haddock_synced == true` Y la operación es CREATE → skip (ya fue enviada)
2. **Distinguir CREATE vs UPDATE**: el servicio debe saber si está sincronizando por primera vez o re-sincronizando tras edición
   - CREATE: solo si `haddock_synced == false`
   - UPDATE: siempre re-envía (el usuario fue advertido, ver Fase 4)
3. **Mutex por venta**: evitar 2 syncs simultáneos de la misma venta
   - Usar `tokio::sync::Mutex` en un `DashMap<Uuid, Arc<Mutex<()>>>` estático
   - Si ya hay sync en progreso para esa venta → skip silenciosamente
4. **Tests con wiremock**: 
   - Crear venta synced=true → no re-envía
   - Crear venta synced=false → envía
   - 2 syncs concurrentes → solo 1 pasa

### FASE 3: Bloqueo de eliminación de ventas sincronizadas
**Tarea 064A-8**

> Petición del cliente: "no se puedan borrar ventas en nuestra web para tener solidez con Haddock"

**Opción elegida**: Bloquear eliminación de ventas cuando Haddock sync está habilitado en config.

1. **Backend VentaService::delete()**: 
   - Si `config.haddock_sync_enabled == true` → rechazar con error 409 Conflict
   - Mensaje: "No se pueden eliminar ventas mientras la sincronización con Haddock está activa. Desactívela primero en Configuración."
   - Si `config.haddock_sync_enabled == false` → permitir delete normal
2. **Handler**: retornar 409 con mensaje descriptivo
3. **Frontend ListaVentas**: 
   - Si sync habilitado: ocultar botón de eliminar o mostrarlo deshabilitado con tooltip
   - Añadir indicador visual de que la eliminación está bloqueada por Haddock
4. **Tests**: verificar que delete falla con 409 cuando sync está habilitado

### FASE 4: Alertas de edición en frontend
**Tarea 064A-9**

Cuando el usuario edita una venta que ya fue sincronizada con Haddock:

1. **Dialog de confirmación** antes de guardar:
   - Título: "⚠️ Venta sincronizada con Haddock"
   - Mensaje: "Esta venta ya fue enviada a Haddock. Si guardas los cambios, se re-enviará con los datos actualizados. Los datos anteriores en Haddock serán reemplazados."
   - Botones: "Continuar y actualizar" (primary) / "Cancelar" (secondary)
   - Solo aparece si `venta.haddock_synced == true`

2. **Campos que NO se sincronizan** (informar al usuario):
   - Si se cambia el `cliente_id` → alerta adicional: "El nombre del cliente no se sincroniza con Haddock."
   
3. **Badge de estado sync** en cada fila de ListaVentas:
   - 🟢 Sincronizada (haddock_synced=true + haddock_sync_error=null)
   - 🔴 Error de sync (haddock_synced=false + haddock_sync_error no null)
   - ⚪ No sincronizada (haddock_synced=false + sync_error=null → nunca intentado, o sync desactivado)

4. **Tooltip en badge**: muestra `haddock_synced_at` o `haddock_sync_error`

### FASE 5: Retry manual + log visible
**Tarea 064A-10**

1. **Botón "Reintentar sync"** en ventas con error de sincronización:
   - Solo visible si `haddock_synced == false && haddock_sync_error != null`
   - Llama a un nuevo endpoint: `POST /api/ventas/{id}/haddock-sync`
   - El endpoint ejecuta sync_order inmediatamente y retorna resultado

2. **Endpoint nuevo**: `POST /api/ventas/{id}/haddock-sync`
   - Handler: obtener venta + config → llamar sync_order → retornar {ok: true/false, error: string?}
   - Solo funciona si sync está habilitado y token presente

3. **Columna "Estado Haddock"** en tabla de ventas (frontend):
   - Filtrable: "Todas", "Sincronizadas", "Con error", "Sin sincronizar"

### FASE 6: Tests de simulación exhaustivos
**Tarea 064A-11**

Tests adicionales con wiremock que simulan el flujo completo:

1. **test_create_venta_syncs_and_marks**: crear → sync OK → haddock_synced=true
2. **test_create_venta_sync_fails_marks_error**: crear → sync 500 3x → haddock_synced=false + error
3. **test_edit_synced_venta_resyncs**: editar venta synced → re-envía a Haddock
4. **test_edit_unsynced_venta_syncs**: editar venta no-synced → envía a Haddock
5. **test_delete_blocked_when_sync_enabled**: eliminar → 409
6. **test_delete_allowed_when_sync_disabled**: eliminar → 204
7. **test_concurrent_syncs_deduplicate**: 2 updates rápidos → 1 sola request
8. **test_retry_manual_endpoint**: POST /haddock-sync → sync exitoso
9. **test_retry_already_synced**: retry de venta ya synced → idempotente (no duplica)
10. **test_invalid_token_marks_error**: token malo → 401 → marca error

## Orden de ejecución
1. Fase 1 (064A-6): Base de tracking — todo lo demás depende de esto
2. Fase 2 (064A-7): Prevención duplicados — seguridad antes de features
3. Fase 3 (064A-8): Bloqueo delete — petición explícita del cliente
4. Fase 4 (064A-9): Alertas UI — UX de protección
5. Fase 5 (064A-10): Retry manual + log — completar el ciclo
6. Fase 6 (064A-11): Tests exhaustivos — validar todo junto

## Notas
- Todas las fases incluyen tests unitarios/integración propios
- La Fase 6 son tests de flujo completo que validan la interacción entre fases
- Si se confirma que Haddock usa externalId como upsert, la Fase 2 se simplifica significativamente
- El plan se ejecuta secuencialmente — cada fase depende de la anterior
