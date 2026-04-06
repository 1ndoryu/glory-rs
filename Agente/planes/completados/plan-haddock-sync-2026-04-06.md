# Plan: Sincronización unidireccional con Haddock POS API

## Contexto
- API de Haddock: `https://pos-api.haddock.app` (Basic Auth, 2 endpoints: POST /catalog/, POST /orders/)
- Objetivo: enviar ventas automáticamente cuando se crean/actualizan en nuestra plataforma
- Limitación: API unidireccional (solo push), no se puede borrar en Haddock

## Datos necesarios del cliente
- Token de API de Haddock (Base64 de usuario:contraseña)
- Se obtiene desde: Haddock → Configuración → Integraciones → POS API → Conectar → Generar credenciales

## Arquitectura

### Fase 1: Backend — Servicio Haddock (tarea 064A-5)
1. **Migración**: añadir campo `haddock_api_token` a `configuracion_restaurante` (texto, encriptado en reposo con serde skip_serializing)
2. **Modelo**: actualizar `ConfiguracionRestaurante` y `ActualizarConfiguracionRequest` con el nuevo campo
3. **Servicio `haddock.rs`**: cliente HTTP que habla con la API de Haddock
   - `sync_order(venta, config)` — convierte Venta a formato Haddock y hace POST /orders/
   - Mapeo de canales: comedor→dining-room, barra→bar, terraza→terrace, delivery→delivery, just_eat→justeat, eventos→events
   - Mapeo de métodos: efectivo→cash, tarjeta→card, transferencia→transfer
   - Para `items[]` (obligatorio): enviar un ítem genérico con nombre=descripcion, precio=importe_base+importe_iva
   - Para `date`: usar fecha + hora estimada por turno (mañana=09:00, mediodía=14:00, noche=21:00)
   - Retry con backoff exponencial (1s, 2s, 4s, max 3 intentos)
   - Logging de éxito/fallo sin bloquear la operación principal

### Fase 2: Integración en flujo de ventas (tarea 064A-5 cont.)
4. **Hook post-create**: después de crear venta (manual o auto), si `haddock_api_token` está configurado → spawn `sync_order` en background (tokio::spawn)
5. **Hook post-update**: después de actualizar venta → sync (Haddock reconoce por externalID y actualiza)
6. **Hook post-delete**: NO hay delete en Haddock API — loguear warning de que la venta sigue en Haddock

### Fase 3: Frontend — Config UI (tarea 064A-6)
7. **Configuración**: añadir campo "Token API Haddock" en la sección de configuración junto a `url_haddock`
8. **Toggle de sincronización**: checkbox "Sincronizar ventas con Haddock" (para poder desactivar sin borrar el token)
9. **Indicador de estado**: badge que muestre si la última sincronización fue exitosa

### Fase 4: Tabla de log de sync (tarea 064A-7, si amerita)
10. Tabla `haddock_sync_log` para registrar cada intento de sync (venta_id, status, error_msg, timestamp)
11. Vista en frontend para que el propietario vea el historial de sincronización

## Orden de implementación
1. ✅ Migración DB (haddock_api_token + haddock_sync_enabled)
2. ✅ Modelo configuracion actualizado
3. ✅ Repo configuracion actualizado ($17, $18 con COALESCE)
4. ✅ Servicio haddock.rs (cliente HTTP + mapeo + retry + 3 unit tests)
5. ✅ Hooks en VentaService (create + update → tokio::spawn sync)
6. ✅ Hook en ReservaService::crear_venta_automatica
7. ✅ .sqlx/ cache actualizado (SELECT, INSERT, UPDATE)
8. ✅ Frontend: hook useConfiguracion + campos token/toggle en Configuración
9. ✅ OpenAPI dump + Orval regen
10. ✅ cargo check + clippy limpio
11. ✅ cargo test (3 tests pasan)
12. ✅ npx tsc --noEmit + vite build
13. ⬜ Commit + push + deploy
