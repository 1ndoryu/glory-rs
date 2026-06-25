# Mapeo de Aliases — Desktop App (Tauri 2)

> **Fecha:** 2026-06-25
> **Relacionado:** `plan-portar-desktop-app-2026-06-25.md`, `desktop-adaptacion-2026-04-22.md`
> **Propósito:** Documentar el mapeo de aliases del desktop entre el tema WordPress (legacy) y el stack Rust (actual)

---

## Contexto

La app desktop (`clients/desktop/`) se construyó originalmente dentro del tema WordPress `glorytemplate/`. Sus imports TypeScript y Vite usaban aliases que apuntaban a directorios del tema:

| Alias | Ruta WP (legacy) | Propósito |
|---|---|---|
| `@` | `Glory/assets/react/src/` | Framework Glory (core, islands, hydration) |
| `@app` | `App/React/` | Código específico de Kamples (stores, services, componentes) |
| `@mezclador` | `Mezclador/` | DAW embebible |
| `@api` | N/A | Cliente Orval generado |

## Problema

En el proyecto Rust template, esas rutas **no existen**. El tema WordPress vive en otro repositorio. El SPA frontend (`frontend/src/`) ya tiene el código equivalente portado.

## Mapeo Correcto

| Alias | Ruta Rust (correcta) | Contenido |
|---|---|---|
| `@` | `../../frontend/src/glory-core` | Framework Glory: `core/`, `core/hydration`, `core/router/navigationStore`, `index.css` |
| `@app` | `../../frontend/src/legacy` | Código Kamples: `appIslands.tsx`, `stores/authStore`, `services/apiAuth`, `components/ui/`, `hooks/`, `types/` |
| `@api` | `../../frontend/src/api/generated` | Cliente Orval generado (ya funcionaba) |
| `@desktop` | `./src` | Código específico del desktop (sin cambios) |

### Dependencias de node_modules

Las dependencias que antes se resolvían desde `Glory/assets/react/node_modules/` ahora se instalan localmente en `clients/desktop/node_modules/`:

| Dependencia | Ruta legacy (rota) | Ruta correcta |
|---|---|---|
| `zustand` | `../Glory/assets/react/node_modules/zustand` | `./node_modules/zustand` |
| `lucide-react` | `../Glory/assets/react/node_modules/lucide-react` | `./node_modules/lucide-react` |
| `soundtouchjs` | `../Glory/assets/react/node_modules/soundtouchjs` | `./node_modules/soundtouchjs` |

## Archivos modificados (256A-1)

| Archivo | Cambio | Estado |
|---|---|---|
| `clients/desktop/vite.config.ts` | `resolve.alias.@` y `@app`; `server.fs.allow`; eliminar `servirAssetsLocales()` | ✅ |
| `clients/desktop/tsconfig.json` | `paths.@/*`, `@app/*`, `zustand`, `lucide-react`, `soundtouchjs`; `include` | ✅ |
| `clients/desktop/src/types/desktop-stubs.d.ts` | Stubs para Capacitor, Mezclador, Tauri plugins que faltan en desktop | ✅ |
| `clients/Glory/` | Directorio temporal eliminado (stub index.css ya no necesario) | ✅ |

## Verificación

```bash
cd clients/desktop
npx tsc --noEmit        # 0 errores ✅
npx vite build          # Build exitoso ✅
```

## Historial

| Fecha | Cambio |
|---|---|
| 2026-06-25 | Documento inicial — diagnóstico y solución de aliases |
| 2026-06-25 | Actualizado tras ejecución 256A-1 — type check 0 errores |
