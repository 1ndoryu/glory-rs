# Plan — Extension de recortes de samples en Rust

> **Tarea:** 254A-8c
> **Origen:** `App/Kamples/Api/Controladores/ExtensionRecorteController.php` + `App/Kamples/Services/ServicioExtensionRecorte.php`
> **Destino:** Rust handlers/services dentro de `glory-rust-template`.
> **Endpoints en scope:**
>   - POST `/api/samples/{id}/extender-recorte`
>   - POST `/api/samples/{id}/generar-siguiente`
>   - POST `/api/samples/{id}/restaurar-recorte`
> **Permisos:** admin-only (mismo que el legacy).

## Resumen del flujo (referencia legacy)

1. Validar sample existe y proviene de `cola_extraccion_samples` (tiene `youtube_id`).
2. Calcular nuevo timing (inicio - segAntes, fin + segDespues).
3. Obtener audio fuente:
   a. Si hay `audio_completo` guardado en disco -> usar ese.
   b. Si no -> descargar audio completo desde YouTube via `yt-dlp`.
4. Recortar con FFmpeg al nuevo rango [inicio, fin].
5. Reemplazar archivos del sample (preview, optimizado, waveform, hash).
6. Actualizar timing en `cola_extraccion_samples` (`compas_inicio_seg`, `compas_fin_seg`).
7. Guardar timing original en `samples.metadata.timing_original` (para restaurar).
8. Limpiar tmp dir.

`generarSiguiente` reusa los pasos 3-5 pero crea un sample nuevo en lugar de reemplazar.
`restaurar` lee `samples.metadata.timing_original` y vuelve a recortar al timing inicial.

## Dependencias que faltan en Rust (al 2026-04-25)

| Pieza | Estado | Comentario |
|------|--------|-----------|
| FFmpeg base | ✅ existe | `src/audio/ffmpeg.rs` (convert_to_mp3, inspect) |
| Recorte por rango (-ss / -t) | ❌ falta | hay que agregar `cut_to_mp3(start_sec, duration_sec)` |
| yt-dlp wrapper | ❌ falta | `App/Kamples/Api/AyudanteDescargaAudio.php` no esta portado |
| `audio_completo` storage path | ⚠ parcial | falta convencion de ruta y campo en cola_extraccion_samples |
| Reemplazo de archivos del sample | ❌ falta | implica regenerar preview, optimizado, waveform, audio_hash, total reproducciones=0 |
| Generador de waveform | ⚠ parcial | existe en pipeline de upload, hay que extraerlo |
| Cola_extraccion_samples repo (write) | ⚠ parcial | hay reads pero no `actualizar_timing` ni `crear` para generarSiguiente |
| AuthMiddleware admin-only | ✅ existe | `CurrentUser` + helper de admin |

## Fases propuestas

### Fase 1 — Infraestructura de audio (bloqueante)
- 1.1 `audio::ffmpeg::cut_to_mp3(input, output, start_sec, duration_sec)` con `-ss` antes del `-i` y `-t` despues, output como MP3 320 kbps mono.
- 1.2 `services::audio::ytdlp` modulo nuevo: `download_audio_from_youtube(youtube_id, target_dir, cookies?) -> Result<PathBuf>`. Usa `yt-dlp` como binario externo, busca cookies en `state.youtube_cookies_path`. Manejo de errores: timeout, no disponible, region locked.
- 1.3 `services::audio::sample_assets`: `replace_sample_assets(pool, sample_id, source_audio, storage_root) -> ReplacedAssets`. Genera preview MP3, optimizado, waveform JSON, calcula `audio_hash`, actualiza columnas y metadata, dispara reset de `total_reproducciones` segun politica.

### Fase 2 — Repositorios
- 2.1 `repositories::cola_extraccion::find_by_sample_id(pool, sample_id)` retorna `ColaExtraccionRow` con timing, youtube_id, audio_completo_path.
- 2.2 `repositories::cola_extraccion::actualizar_timing(pool, cola_id, inicio, fin)` UPDATE.
- 2.3 `repositories::cola_extraccion::crear_para_segmento(pool, original_cola_id, nuevo_inicio, nuevo_fin, sample_id_nuevo)` (para generarSiguiente).

### Fase 3 — Servicio de extension
- 3.1 `services::extension_recorte::extender(state, sample_id, seg_antes, seg_despues) -> ExtenderResult`.
- 3.2 `services::extension_recorte::generar_siguiente(state, sample_id, duracion) -> GenerarSiguienteResult` (crea sample nuevo).
- 3.3 `services::extension_recorte::restaurar(state, sample_id) -> RestaurarResult`.

### Fase 4 — Handlers + OpenAPI
- 4.1 `handlers::extension_recorte::routes()` con tres rutas POST.
- 4.2 Schemas: `ExtenderRequest { segundos_antes, segundos_despues }`, `GenerarSiguienteRequest { duracion }`, response unificada `{ ok, mensaje, duracion?, audio_hash?, nuevo_sample_id? }`.
- 4.3 Guard admin: usar el mismo helper que `admin::*` (revisar `middleware::AuthUser` + comprobacion en handler).

### Fase 5 — Tests
- 5.1 Test unitario para `cut_to_mp3` con un fixture WAV en `assets/test/`.
- 5.2 Smoke: con un sample real de la cola, ejecutar `extender` con segAntes=2, segDespues=2 y verificar que `samples.duracion` aumento ~4s.

### Fase 6 — Cierre
- 6.1 Archivar 254A-8c en `Agente/completados/`.
- 6.2 Documentar en `Agente/documentacion/audio/extension-recorte-{fecha}.md` el flujo y las gotchas.
- 6.3 Mover este plan a `Agente/planes/completados/`.

## Estado actual (2026-04-25)
- Fase 0: investigacion completa. Plan escrito.
- Bloqueante critico: yt-dlp wrapper. Antes de implementar el endpoint, hay que tener `download_audio_from_youtube` funcionando con cookies opcionales y manejo de errores robusto. Esto solo es un mini-proyecto.
- Decision: NO implementar el endpoint en esta sesion. Continuar con las tareas mas pequenas del roadmap (md cleanup, articulos UI removal, opendaw plan) y dejar 254A-8c en este plan para una sesion dedicada.

## Bitacora de avance
- 2026-04-25: plan inicial. Tarea reabierta en roadmap con referencia a este archivo.
