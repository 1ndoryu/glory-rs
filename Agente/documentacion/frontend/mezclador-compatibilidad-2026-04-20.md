# Mezclador — compatibilidad `@app` + adaptador Orval (2026-04-20)

## Contexto

`frontend/src/features/mezclador/` fue importado 1:1 desde el tema WordPress legado, pero no podía compilar porque dependía de:

- aliases `@app/*` que solo existían en el frontend legacy del tema
- fetchers contra `wp-json/...`
- componentes UI y stores no presentes en la SPA nueva

La meta de `174A-109b-fase2` fue destrabar la compilación sin reescribir la feature completa ni montarla todavía en el router.

## Qué se hizo

### 1. Alias `@app`

Se añadió resolución de `@app/*` en:

- `frontend/tsconfig.json`
- `frontend/vite.config.ts`

El alias apunta a `frontend/src/app/`.

### 2. Capa mínima de compatibilidad

Se creó `frontend/src/app/` con contratos mínimos para el Mezclador:

- `components/ui/`
  - `BotonBase.tsx`
  - `Input.tsx`
  - `CampoTexto.tsx`
  - `SelectorBase.tsx`
  - `MenuContextual.tsx`
- `stores/`
  - `panelLateralStore.ts`
  - `crearModalStore.ts`
- `types/index.ts`
- `services/apiExplorador.ts`
- `styles/legacyUi.css`

La intención no es recrear todo el shell legacy, sino solo exponer la API mínima que hoy consume la feature.

### 3. Salida de `wp-json/...`

No había un `services/apiExplorador.ts` dentro del Mezclador portado; el browser dependía de un servicio heredado del tema. Se resolvió creando `@app/services/apiExplorador`, que:

- usa `GET /api/samples` vía Orval
- agrupa el catálogo por `tipo` para reconstruir carpetas navegables
- normaliza `SampleSummary` al contrato `SampleResumen`

Con esto, el Mezclador deja de requerir el backend WordPress para su browser básico.

### 4. Validación aislada

Se añadió `frontend/tsconfig.mezclador.json` para compilar:

- `src/app/**/*`
- `src/features/mezclador/**/*`
- `src/vite-env.d.ts`

Comando de validación usado:

```bash
Push-Location frontend; npm exec -- tsc --noEmit -p tsconfig.mezclador.json; Pop-Location
```

## Decisiones

- Se eligió compatibilidad por alias en lugar de reescribir decenas de imports internos del Mezclador.
- La capa `@app` vive dentro del frontend nuevo para que la próxima fase pueda reemplazar módulos concretos sin romper toda la feature de golpe.
- El browser se alimenta con catálogo público porque ya existe y evita bloquear la migración esperando endpoints más ricos.

## Limitaciones conocidas

- La feature sigue sin ruta SPA; eso queda para `174A-109b-fase4`.
- El build/type-check global del frontend sigue excluyendo `src/features/**/*`; la validación del Mezclador es dedicada.
- El explorador reconstruye carpetas desde `sample.tipo`; si luego se necesita taxonomía más rica o colecciones privadas, debe ampliarse el adaptador desde el backend Rust.
