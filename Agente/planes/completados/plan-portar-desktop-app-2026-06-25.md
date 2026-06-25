# Plan — Portar Desktop App (Tauri 2) al Stack Rust

> **Fecha:** 2026-06-25
> **Tarea relacionada:** 256A-1
> **Estado:** Pendiente — diagnóstico completo, solución diseñada
> **Dependencias:** La SPA frontend (`frontend/src/`) ya tiene `glory-core/` y `legacy/` portados del tema WordPress

---

## 0. Diagnóstico: ¿Qué pasa?

La app desktop (`clients/desktop/`) se construyó originalmente **dentro del tema WordPress** (`glorytemplate`) y sus imports apuntan a rutas que **no existen** en el proyecto Rust:

| Alias actual | Resuelve a | ¿Existe? |
|---|---|---|
| `@` | `../Glory/assets/react/src` (tema WP) | ❌ |
| `@app` | `../App/React` (tema WP) | ❌ |
| `@mezclador` | `../Mezclador` (tema WP) | ❌ |
| `zustand` | `../Glory/assets/react/node_modules/zustand` | ❌ |
| `lucide-react` | `../Glory/assets/react/node_modules/lucide-react` | ❌ |
| `soundtouchjs` | `../Glory/assets/react/node_modules/soundtouchjs` | ❌ |
| `@desktop` | `./src` (local) | ✅ |
| `@api` | `../../frontend/src/api/generated` | ✅ |

**El SPA frontend (`frontend/src/`) YA tiene TODO lo que el desktop necesita:**
- `glory-core/` — core del framework (islandRegistry, hydration, router stores)
- `legacy/` — código portado de `App/React/` (appIslands, stores, services, componentes UI)

**Solución:** Repuntar los aliases del desktop para que apunten al SPA frontend existente en vez del tema WP.

---

## 1. Fase 1 — Arreglar aliases (BLOQUEANTE)

### 1.1 vite.config.ts

Cambiar en `resolve.alias`:

```ts
// ANTES (tema WordPress — no existe en Rust template)
'@': resolve(__dirname, '../Glory/assets/react/src'),
'@app': resolve(__dirname, '../App/React'),
'@mezclador': resolve(__dirname, '../Mezclador'),

// DESPUÉS (SPA Rust — ya existe)
'@': resolve(__dirname, '../../frontend/src/glory-core'),
'@app': resolve(__dirname, '../../frontend/src/legacy'),
// '@mezclador' se elimina (no se usa en desktop)
```

Y actualizar `server.fs.allow` para que Vite pueda servir archivos de `frontend/src/`:

```ts
fs: {
    allow: [
        '.',
        '..',
        '../../frontend/src',
        // Rutas legacy ya no necesarias:
        // '../App/React', '../Glory/assets/react/src', etc.
    ],
},
```

### 1.2 tsconfig.json

```json
"paths": {
    "@/*": ["../../frontend/src/glory-core/*"],
    "@app/*": ["../../frontend/src/legacy/*"],
    "@desktop/*": ["./src/*"],
    // Dependencias: usar node_modules del desktop en vez del tema WP
    "zustand": ["./node_modules/zustand"],
    "zustand/*": ["./node_modules/zustand/*"],
    "lucide-react": ["./node_modules/lucide-react"],
    "soundtouchjs": ["./node_modules/soundtouchjs"],
    // @mezclador se elimina
}
```

Y actualizar `include`:

```json
"include": [
    "src/**/*.ts",
    "src/**/*.tsx",
    "src/**/*.d.ts",
    // Estos ya no existen:
    // "../App/React/**/*.ts",
    // "../Mezclador/**/*.ts",
]
```

### 1.3 Limpiar plugin `servirAssetsLocales`

El plugin `servirAssetsLocales()` sirve assets del tema WordPress via `THEME_ROOT`. Ya no es necesario porque los assets ahora viven en `frontend/src/`. **Eliminar el plugin** y la constante `THEME_ROOT`/`THEME_URL_PREFIX`.

### 1.4 Verificar dependencias

Instalar en `clients/desktop/` las dependencias que antes se resolvían desde el tema WP:

```bash
cd clients/desktop
npm install zustand lucide-react soundtouchjs
```

Algunas ya existen en el `package.json` del desktop, otras no. Verificar cada una.

### 1.5 Probar compilación

```bash
cd clients/desktop
npx tsc --noEmit          # Type check
npx vite build            # Build production
```

---

## 2. Fase 2 — Verificar integración sync

Una vez que la app desktop compila, probar el flujo de sincronización que motivó todo esto (246A-1):

1. Arrancar backend: `cargo run --bin glory-backend`
2. Arrancar desktop: `cd clients/desktop && npm run dev`
3. Iniciar sesión con JWT
4. Verificar que el panel de sync se conecta al backend
5. Verificar delta sync (cursor-based)

---

## 3. Fase 3 — Limpieza de código muerto

Una vez que los aliases funcionan:

1. **Eliminar `clients/Glory/`** — directorio creado temporalmente con el stub de `index.css` (ya no necesario)
2. **Eliminar proxy `/wp-json`** de vite.config.ts si ningún service legacy lo usa (verificar con grep)
3. **Simplificar `tauri.conf.json`** — revisar CSP, endpoints, ventanas

---

## 4. Fase 4 — Consolidar documentación

Actualizar:
1. **`Agente/documentacion/sync/sync-desktop-estado-2026-06-24.md`** — agregar sección "Desktop App portability"
2. **`Agente/documentacion/frontend/desktop-adaptacion-2026-04-22.md`** — marcar tarea A como completable tras esta migración
3. **`Agente/planes/plan-clients-adapters-2026-04-20.md`** — actualizar sección 174A-111
4. **`Agente/documentacion/frontend/`** — nuevo MD `alias-mapping-2026-06-25.md` documentando el mapeo de aliases

---

## 5. Riesgos

| Riesgo | Probabilidad | Mitigación |
|---|---|---|
| `legacy/` imports rotos por diferencias entre WP y Rust context | Media | TypeScript check + smoke test de cada servicio importado |
| Zustand/lucide-react version mismatch (desktop vs frontend) | Baja | Usar `dedupe` en vite.config y verificar `npm ls` |
| `glory-core/index.css` imports dependen de `legacy-assets/` y `glory-css/` que están relativos a `frontend/src/` | Baja | Vite resuelve correctamente imports CSS relativos desde el alias |
| `@tauri-apps/*` plugins se siguen importando desde legacy services | Media | Los aliases ya existen en vite.config; verificar que sigan funcionando |
| CSP en tauri.conf.json necesita ajuste para nuevos dominios | Baja | Ya configurado para localhost:3000 y api.kamples.com |

---

## Orden de ejecución

1. **256A-1a** — Arreglar aliases en `vite.config.ts` + `tsconfig.json` (fase 1)
2. **256A-1b** — Instalar dependencias faltantes (zustand, lucide-react, soundtouchjs)
3. **256A-1c** — Type check + build verification
4. **256A-1d** — Smoke test sync desktop contra backend local
5. **256A-1e** — Limpieza (`clients/Glory/`, proxy `/wp-json`, plugin legacy)
6. **256A-1f** — Actualizar documentación

---

## Criterios de cierre

- ✅ `npm run dev` en `clients/desktop/` abre ventana Tauri sin errores de Vite
- ✅ Los 3 imports críticos resuelven: `@/core`, `@/core/hydration`, `@app/appIslands`
- ✅ Las stores y servicios de `@app/` se importan correctamente
- ✅ El panel de sync se conecta al backend local
- ✅ `npx tsc --noEmit` sin errores
- ✅ Documentación actualizada
