# Handler `downloads` — Descargas con límites por plan + tracking

**Tarea:** 174A-61 (2026-04-18)

## Endpoints
- `POST /api/samples/{id}/descargar` → registra descarga, devuelve URL del archivo y restantes del día.
- `GET /api/descargas/limites` → informa límite del plan, usadas en 24h y restantes.

## Lógica `register_download`
1. Cargar info del sample (`creador_id`, `permitir_descarga`, `es_premium`, `precio`). 404 si no existe o eliminado.
2. Si `!permitir_descarga` y el usuario no es creador → 403.
3. Detectar `ya_descargado` (re-descargas no consumen crédito).
4. `consume_credito = !propietario && !ya_descargado`.
5. Reglas de pago:
   - `precio > 0` y no comprado:
     - Pro/Premium con `es_premium`: descarga sin crédito.
     - Resto → 403 `requiere_compra`.
   - `es_premium` sin precio + plan free → 403.
6. Si `consume_credito` y plan free supera 5/día → 429 `RateLimited`.
7. `INSERT INTO descargas` + bump `samples.total_descargas` + `usuarios_ext.total_descargas` (transacción).
8. Trigger `AlgoPlanner::register_interaction(InteractionKind::Descarga)`.

## Plan limits
- `free` → 5/día.
- `pro` / `premium` → ilimitado (`limite = None`).

NOTA: hardcodeado por ahora. Cuando se porte `StripeService::obtenerConfigPlan`, mover a configuración.

## Decisiones vs legado PHP (`DescargasController::descargar`)
| Legado | Rust |
|--------|------|
| Códigos de descarga gratis | NO portado |
| Compras individuales (TransaccionesRepository) | NO portado — todo precio>0 bloquea a free |
| Anti-abuso por IP | NO portado |
| Calidad por plan | siempre `wav` (legado también) |
| Límite específico para cuentas <3 días | NO portado |
| Stream con token HMAC | 174A-63 |

## Estructura DB
- `descargas(id, usuario_id, sample_id, calidad, tamano_bytes, created_at)`.
- Índices: `idx_descargas_usuario_dia`, `idx_descargas_usuario_sample`.
- Contadores derivados: `samples.total_descargas`, `usuarios_ext.total_descargas`.

## Estructuras Rust
- `DownloadRepository::{fetch_sample_info, already_downloaded, count_today, register, user_plan}`.
- `SampleDownloadInfo { creador_id, permitir_descarga, es_premium, precio }`.
- `DownloadResponse { ok, url, calidad, consume_credito, restantes }`.
- `DownloadLimitsResponse { plan, limite, usadas, restantes, calidad }`.
