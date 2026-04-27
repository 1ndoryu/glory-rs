# Admin moderacion JSON render — 2026-04-27

## Contexto

La isla `AdminPanelIsland` podia romper al abrir la pestaña de moderación con el error de React `Objects are not valid as a React child`.

La traza apuntaba a `TarjetaHistorial` dentro de `TabModeracionAdmin`, especificamente al `<pre>` que muestra `moderacion_detalle`.

## Causa raiz

El frontend tipaba `moderacion_detalle` como `string | null`, pero en algunos casos el backend o la capa intermedia ya entregaban un objeto JSON parseado.

El helper `formatearJson()` intentaba hacer `JSON.parse(raw)` y, si fallaba, devolvia `raw` tal cual. Cuando `raw` era un objeto, React intentaba renderizar ese objeto dentro de `<pre>`, provocando el crash de la isla.

## Cambio aplicado

- `frontend/src/legacy/services/apiAdmin.ts`
  - `PublicacionModeracion.moderacion_detalle` pasa a `unknown` para reflejar el contrato real.
- `frontend/src/legacy/components/admin/TabModeracionAdmin.tsx`
  - `formatearJson()` ahora acepta `unknown`
  - si llega string JSON, lo parsea y reserializa bonito
  - si llega objeto/array ya parseado, lo serializa con `JSON.stringify(..., null, 2)`
  - si llega cualquier otro valor, lo convierte a string segura

## Resultado

El `<pre>` siempre recibe texto y ya no puede intentar renderizar un objeto crudo.

## Gotchas

- La traza de React mencionaba `useAdminPanel.ts` porque el cambio de estado que cargaba moderación disparaba el render, pero el problema real no estaba en el hook sino en el presenter `TarjetaHistorial`.
- El error solo aparece cuando la tab activa/restaurada es `moderacion`, no necesariamente al entrar por la tab `usuarios`.
