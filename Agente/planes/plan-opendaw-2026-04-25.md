# Plan — Reemplazo del DAW interno por opendaw

> **Tarea:** roadmap pendiente — "Remplazar el daw que hicimos, por https://github.com/andremichelle/opendaw"
> **Fecha:** 2026-04-25
> **Origen:** `frontend/src/mezclador/` + `frontend/src/features/mezclador/` (DAW propio actual)
> **Destino:** Integracion de opendaw como motor del DAW.
> **Repo upstream:** https://github.com/andremichelle/opendaw
> **Fork requerido:** github.com/1ndoryu/opendaw (accion manual del usuario, ver fase 0).

## Contexto

El DAW actual (`Mezclador`) es una implementacion propia con limitaciones:
- 2 carpetas paralelas (`mezclador/` y `features/mezclador/`) — el origen del duplicado y cual se usa hay que resolverlo en la fase 1.
- Funcionalidad pobre comparada con opendaw (sintesis, MIDI, mixer pro, etc.).
- Mantenerlo costaria mas que adoptar opendaw.

**Objetivo:** Que un usuario pueda arrastrar un sample desde la biblioteca al DAW y empezar a producir con un editor pro.

## Inventario de lo que se va a tirar/migrar

Lo que hay que auditar antes de empezar:
1. `frontend/src/mezclador/` — types, stores zustand, services, hooks, utils, componentes.
2. `frontend/src/features/mezclador/` — duplicado o version anterior. Revisar diferencias.
3. `frontend/src/app/styles/legacyUi.css` — dependencias UI legacy del Mezclador.
4. `frontend/src/app/stores/panelLateralStore.ts` — modo `'mezclador'` que abre el panel.
5. `frontend/src/app/stores/crearModalStore.ts` — flujo "abrir modal con archivo precargado" que el Mezclador dispara.
6. Endpoints backend que sirven al DAW: revisar `/api/samples` agrupado por tipo (`apiExplorador.ts`).
7. Persistencia de proyectos del DAW — si existe en backend, decidir si se migra o se descarta.

## Fases

### Fase 0 — Fork y exploracion del upstream (manual del usuario + investigacion)
**Bloqueante manual:** el agente NO tiene credenciales de GitHub para crear el fork. El usuario debe:
1. Ir a https://github.com/andremichelle/opendaw y darle "Fork" -> destino `1ndoryu/opendaw`.
2. Confirmar al agente que el fork existe; el agente continua desde ahi.

Mientras tanto, en investigacion (NO bloqueante):
- 0.1 Clonar localmente `andremichelle/opendaw` en `Repos-vivos/opendaw/` (fuera de glory-rust-template) y leer:
  - `package.json`, `vite.config.ts`, dependencias.
  - Estructura de modulos (motor de audio, UI, tracks, plugins).
  - Como se monta como aplicacion standalone vs. como libreria embebible.
- 0.2 Identificar el "API publico" de opendaw: ¿exporta un componente React montable? ¿es app standalone con su propia raiz? ¿carga proyectos via JSON?
- 0.3 Revisar la licencia (GPL? MIT?) y compatibilidad con kamples.
- 0.4 Identificar puntos de integracion para "drag-drop sample" — buscar handlers de file drop, file import, asset library.

### Fase 1 — Auditoria del Mezclador actual (1 sesion)
- 1.1 Diff entre `frontend/src/mezclador/` y `frontend/src/features/mezclador/`. Decidir cual es la version "buena" y borrar la otra.
- 1.2 Listar funcionalidades del Mezclador actual (tracks, sampler, transport, etc.) para mapear que cubre opendaw y que no.
- 1.3 Listar estados zustand y servicios que persisten datos del DAW al backend, si hay.
- 1.4 Documentar resultados en `Agente/documentacion/daw/auditoria-mezclador-actual-{fecha}.md`.

### Fase 2 — Estrategia de integracion (1 sesion)
Decidir entre:
- (A) **Embed:** opendaw se monta como sub-componente React en una ruta `/daw` o panel. Comunicacion via postMessage o ref.
- (B) **iframe:** opendaw corre en un iframe (su propia app) y kamples envia/recibe mensajes (drag-drop -> postMessage).
- (C) **Fork-and-merge:** copiar codigo de opendaw dentro de glory-rust-template y mergear.

Recomendacion preliminar (sin haber leido opendaw): (B) iframe — preserva el upstream, facilita merges, encapsula complejidad. (A) si opendaw exporta componentes React puros.

Documentar en `Agente/documentacion/daw/estrategia-opendaw-{fecha}.md`.

### Fase 3 — Integracion de drag-drop con biblioteca (2-3 sesiones)
- 3.1 Sample card en biblioteca expone `onDragStart` con payload tipado `{ sampleId, sampleUrl, sampleHash, duracion, bpm, key }`.
- 3.2 Receptor en opendaw (a traves de postMessage o drop handler) crea una pista nueva con el sample.
- 3.3 Backend: garantizar que `ruta_preview`/`ruta_optimizada` son CORS-friendly para que el motor de audio de opendaw pueda fetchear.
- 3.4 Tests manuales: arrastrar 5 samples diferentes, verificar reproduccion y sync.

### Fase 4 — Persistencia de proyectos (si aplica)
- 4.1 Definir formato de proyecto opendaw (probablemente JSON).
- 4.2 Backend: tabla `proyectos_daw { id, usuario_id, nombre, datos JSONB, ... }` o reutilizar existente si la auditoria revela una.
- 4.3 Endpoints CRUD `/api/me/proyectos-daw`.
- 4.4 UI de "Guardar proyecto" / "Abrir proyecto" en kamples shell.

### Fase 5 — Cleanup del Mezclador legacy
- 5.1 Eliminar `frontend/src/mezclador/` y `frontend/src/features/mezclador/` (lo que sobre).
- 5.2 Eliminar `legacyUi.css`, types compatibilidad, modos zustand obsoletos.
- 5.3 Actualizar landing (`landing.seccion.daw.subtitulo`) si cambia el copy.

### Fase 6 — Cierre
- 6.1 Mover este plan a `Agente/planes/completados/`.
- 6.2 Documentar arquitectura final en `Agente/documentacion/daw/arquitectura-opendaw-{fecha}.md`.

## Riesgos y dudas

- Licencia de opendaw: si es GPL, kamples (cerrado/comercial) podria tener conflicto. Revisar LICENSE en fase 0.
- Performance: opendaw es una app web pesada. Cargarla solo cuando el usuario abre el panel del DAW (lazy import).
- Compatibilidad de stack: opendaw probablemente usa Vite + TypeScript. Debe encajar con nuestro frontend Vite.
- Datos legacy: si hay proyectos guardados con el formato del Mezclador actual, ¿se migran? Evaluar en fase 1.

## Estado actual (2026-04-25)
- Plan inicial escrito.
- BLOQUEANTE FASE 0: el usuario debe hacer el fork manualmente en su cuenta `1ndoryu`.
- Agente puede avanzar en investigacion del upstream (clonar, leer codigo) sin bloqueante.
- No hay deadline; se prioriza despues de las tareas mas chicas del roadmap.

## Bitacora
- 2026-04-25: plan creado. Sin bloqueante de aclaracion del usuario, pero fork pendiente como accion manual.
