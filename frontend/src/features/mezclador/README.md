# Mezclador (DAW)

Origen: `glorytemplate/Mezclador/` (legado WordPress).

[174A-109b] Movido 1:1 a `frontend/src/features/mezclador/` (era `clients/Mezclador/`). El `tsconfig.json` interno se eliminó porque ya está cubierto por el del frontend, que excluye `src/features/**/*` del type-check global hasta que cada feature complete su adaptación.

## Estado actual

- ✅ Código importado y ubicado físicamente en su destino final.
- ❌ NO compila aún: depende de `@app/*` (paths legacy del tema WP) y del backend `wp-json/...`.
- ❌ NO está conectado a ninguna ruta de la SPA.

## Pendientes (sub-tareas reagendadas)

1. **174A-109b-fase2 — sustituir imports `@app/*`:**
   - `@app/stores/panelLateralStore`, `@app/stores/crearModalStore`, `@app/components/ui/BotonBase`, `@app/components/ui/Input`, etc.
   - Decisión: o bien (a) migrar esas dependencias a `frontend/src/components/ui/` y `frontend/src/stores/`, o (b) re-implementarlas mínimamente solo para Mezclador.
2. **174A-109b-fase3 — migrar `services/api*.ts` a hooks Orval:**
   - Reemplazar fetchers que apuntan a `wp-json/...` por los hooks generados desde `src/api/` (Orval contra el OpenAPI Rust).
   - Identificar qué endpoints faltan en el backend Rust y crearlos.
3. **174A-109b-fase4 — montar ruta SPA:**
   - Añadir `<Route path="/mezclador" element={<MezcladorPanel />} />` al router.
   - Lazy-load la feature porque pesa ≥100 archivos + dependencias de audio (`soundtouchjs`, `lucide-react`).
4. **174A-109b-fase5 — quitar `exclude: src/features/**/*` del `tsconfig.json` cuando todas las features compilen.**

## Estructura

```
mezclador/
  components/         (ChannelRack, Mixer, PianoRoll, BloqueSample, …)
  hooks/              (useMezclador, useBrowserDaw, …)
  services/           (audio engine + persistencia)
  stores/             (Zustand: mezcladorStore)
  styles/             (CSS in espanol camelCase)
  types/
  utils/
  featureFlags.ts
```
