# `POST /api/samples/{id}/play` — tracking de reproducciones (2026-04-18)

## Endpoint
`POST /api/samples/{id}/play`

### Auth
`bearer_auth` obligatorio (`CurrentUser`).

### Request body
```json
{ "duracion_escuchada": 12.5, "completada": false }
```
Validaciones (`validator`):
- `duracion_escuchada`: `0.0 ..= 86_400.0` segundos. Default `0.0`.
- `completada`: bool. Default `false`.

### Respuestas
- `201 Created` — play registrado (no debounced).
- `200 OK` — play debounced (fusionado con uno reciente).
- `400` — body inválido.
- `401` — sin auth.

### Body de respuesta
```json
{
  "ok": true,
  "debounce": false,
  "triggered": { "fast": false, "precise": false }
}
```

## Lógica
1. **Debounce 3s:** `PlayRepository::register_with_debounce` busca un play del mismo `(usuario, sample)` en los últimos 3 segundos.
   - Si existe: `UPDATE duracion_escuchada = GREATEST(...), completada = completada OR ...`. Devuelve `debounced=true`. Skipea contador y planificador.
   - Si no: `INSERT` nuevo. Devuelve `debounced=false` (+ `completed_now` si transición false→true).
2. **Contador:** `samples.total_reproducciones += 1` solo en path no-debounced.
3. **Planificador:**
   - Llama `AlgoPlanner::register_interaction(InteractionKind::Reproduccion)`.
   - Si `completed_now`, llama además con `InteractionKind::Completa`.
   - Los flags `fast`/`precise` devueltos se OR-ean y se exponen en la respuesta.

## Referencias al legado
- Origen: `App/Kamples/Api/Controladores/ReproduccionesController.php::registrar`.
- Migrado:
  - Debounce 3s ✅ (lógica idéntica).
  - Contador `samples.total_reproducciones` ✅.
  - Invalidación de cache feed ✅ (vía `AlgoPlanner::execute_fast` cuando se cruza umbral).
  - Trigger `PlanificadorAlgoritmo::registrarInteraccion('reproduccion'|'completa')` ✅.
- **No portado:**
  - Rate limit 60/min (`RateLimiter::verificarUsuario`). Pendiente cuando exista `RateLimiter` global. Comentario `TODO(174A-58 follow-up)` en el handler.

## Cambios estructurales
- `AppState` ahora incluye `algo_planner: Arc<AlgoPlanner>` (inicializado en `handlers::create_router` con `AlgoPlannerConfig::legacy_defaults()`).
- `algorithm::mod` re-exporta `AlgoPlanner`, `AlgoPlannerConfig`, `InteractionKind`, `Triggered`.
- Nuevo `repositories::PlayRepository` (+ `RegisterPlayOutcome`).

## Tarea origen
174A-58.
