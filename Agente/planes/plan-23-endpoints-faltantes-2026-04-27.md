# Plan — Implementación real de 23 endpoints faltantes (sin stubs)

**Origen tareas:** roadmap.md sección "Endpoints faltantes"
**Audit inicio:** 23 missing
**Audit objetivo:** 0 missing
**Restricciones del usuario:** "C) Implementar TODO real ahora" + "Sí, autonomía total para crear migraciones" + "no quiero stubs de nada"

## Inventario por dificultad (más difícil primero)

### Bloque A — Infraestructura externa pesada (4)
1. ✅ **274A-27** POST `/api/dashboard/payout` — implementado real en Rust: valida Connect activo, balance disponible, crea payout con Stripe `/v1/payouts` y persiste `transacciones.tipo = 'payout'`.
2. **274A-57** POST `/api/dev/recorte/generar` — ffmpeg subprocess (`std::process::Command`) + extension Chrome opcional. PHP: `CancionesController.php::generarRecorte` + `RecorteService.php`.
3. **274A-55/56** POST `/api/dev/scraper/run` + `/cola` — subproceso PHP scraper (no portable a Rust); alternativa: HTTP a un servicio scraper externo o reescribir con `scraper` crate. Decisión: invocar el binario PHP `kamples-scraper` que ya existe (`clients/kamples-scraper/`) vía `tokio::process::Command`.
4. **274A-49/50** POST `/api/admin/embeddings/generar|regenerar` — pipeline de embeddings audio. PHP: `EmbeddingsController.php`. Requiere modelo (Groq o OpenAI). Implementación inicial: encolar en `cola_ia` con tipo nuevo `embedding`.

### Bloque B — Contribuciones (5)
5. **274A-23** POST `/api/contribuciones` — crear contribución. PHP: `ContribucionesController.php::crear`.
6. **274A-24** POST `/api/contribuciones/edicion` — proponer edición de relación.
7. **274A-25** POST `/api/contribuciones/eliminacion` — proponer eliminación.
8. **274A-26** GET `/api/contribuciones/mis` — listar propias.
9. **274A-48** POST `/api/admin/contribuciones/moderar` — admin aprueba/rechaza.

Tablas requeridas: `contribuciones_pendientes`, `relaciones_sample` (revisar si ya existen).

### Bloque C — Moderación admin (5)
10. **274A-29** POST `/api/admin/moderar` — aprobar/rechazar publicación o artículo.
11. **274A-30** POST `/api/admin/reportes/resolver` — resolver reporte.
12. **274A-32** POST `/api/admin/moderacion/rechazar-pendientes` — bulk reject.
13. **274A-33** POST `/api/admin/moderacion/banear-usuario` — banear.
14. **274A-34** POST `/api/admin/moderacion/rechazar-usuario-publicaciones` — bulk reject por usuario.

### Bloque D — Operaciones admin varias (6)
15. **274A-43** POST `/api/admin/duplicados/backfill` — recalcular duplicados (puede ser sync simple sobre tabla existente o encolar).
16. **274A-46** POST `/api/admin/automatizacion/reactivar` — reactivar lote fallido.
17. **274A-51** POST `/api/admin/experimentos/generar` — generar experimento (samples sintéticos).
18. **274A-52** POST `/api/admin/procesos/benchmark` — benchmark procesos fondo.
19. **274A-53** DELETE `/api/admin/samples/todos` — operación destructiva con confirmación.
20. **274A-58** POST `/api/dev/extraccion/publicar` — publicar batch extraído.

### Bloque E — Dev destructivo (3)
21. **274A-54** DELETE `/api/dev/canciones` — wipe canciones (dev only).
22-23. ya cubiertos en bloque A (scraper, recorte).

## Procedimiento por tarea (estricto)

1. Leer PHP fuente del controlador correspondiente.
2. Identificar tablas/servicios usados; verificar existencia en `migrations/` y `src/repositories/`.
3. Si falta tabla → crear migración (`sqlx migrate add ...`).
4. Implementar handler en `src/handlers/<modulo>.rs` con `#[utoipa::path]`.
5. Si lógica >40 líneas → mover a `src/services/<dominio>.rs`.
6. Registrar path + schemas en `src/handlers/mod.rs`.
7. `cargo check` (CARGO_TARGET_DIR=C:\tmp\glory-target, SQLX_OFFLINE=true).
8. Regenerar `openapi.json`: `cargo run -- --emit-openapi openapi.json`.
9. `npm run audit:api` — verificar reduction.
10. Commit con ID, archivar en `Agente/completados/tareas-2026-04-27.md`, actualizar roadmap, push.

## Reglas operativas
- Usar `sqlx::query_as` plain (no macros) para evitar requerir `cargo sqlx prepare`.
- AppState campos disponibles: `.pool`, `.redis`, `.public_base_url`, `.storage`, `.jwt_secret`.
- Branch: `kamples` en `1ndoryu/glory-rs`. Último commit: `fee3820`.
- Frontend services en `frontend/src/legacy/services/` indican firma exacta esperada (request body + response).

## Estado
- ✅ Plan creado.
- ✅ 274A-27 completado. Audit 23 -> 22.
- 🔄 Próximo: 274A-57 (recorte ffmpeg) o 274A-55/56 (scraper), manteniendo lo más difícil primero.
