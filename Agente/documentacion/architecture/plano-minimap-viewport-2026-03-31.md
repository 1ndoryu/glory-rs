# Plano / Minimap / Viewport — 2026-03-31

## Objetivo
Documentar el contrato entre PlanoSala, PlanoOcupacion y CanvasMinimap para que el rectángulo azul del minimapa represente siempre la vista real del plano.

## Componentes implicados
- `frontend/src/componentes/PlanoSala.tsx`
- `frontend/src/componentes/PlanoOcupacion.tsx`
- `frontend/src/componentes/plano-sala/CanvasMinimap.tsx`
- `frontend/src/hooks/useViewportSize.ts`

## Contrato correcto
1. `contentBounds` representa el mundo del plano, no el viewport.
2. La base del mundo es `zona.ancho * zoom` y `zona.alto * zoom`.
3. Si una mesa se sale de la zona, el mundo se extiende hasta cubrir su borde derecho/inferior.
4. `canvasHeight` y el tamaño visible del contenedor no deben entrar en `contentBounds`.
5. El rectángulo azul usa `viewportWidth` y `viewportHeight` medidos sobre el DOM real del viewport.

## Medición del viewport
`useViewportSize` es la única estrategia permitida para medir el viewport del plano.

Razones:
- El viewport puede renderizarse de forma condicional cuando la zona aún no existe.
- Un `useLayoutEffect(..., [])` one-shot puede medir `0x0` y dejar el minimapa roto todo el ciclo.
- El layout real puede asentarse un frame después del montaje.

`useViewportSize` evita esos fallos con:
- `getBoundingClientRect()` sobre el nodo real
- reintentos con `requestAnimationFrame` hasta obtener tamaño no nulo
- `ResizeObserver` para cambios posteriores de layout
- reset limpio a `0x0` cuando el viewport deja de existir

## Reglas visuales del minimapa
- `CanvasMinimap` debe renderizar directamente el `<canvas className="planoMinimap" />`.
- El posicionamiento del minimapa depende de `.planoMinimap` en CSS.
- No usar `style={{ position: 'relative' }}` ni wrappers extra que anulen el overlay absoluto.
- El borde azul y el overlay oscuro deben usar las mismas proporciones del viewport.

## Errores que no deben reintroducirse
- Incluir `canvasHeight` dentro de `contentBounds`.
- Medir el viewport con un effect de una sola ejecución.
- Hacer `setZonaActiva(...)` durante render.
- Calcular el mundo solo con el bounding box de mesas si la zona sigue definiendo el espacio de diseño.
- Sobrescribir desde JSX la posición del canvas del minimapa.

## Verificación mínima
1. En PlanoSala y en Reservas, el minimapa debe quedar en la esquina inferior derecha del plano.
2. El rectángulo azul debe cambiar de tamaño si cambia el tamaño visible del viewport.
3. El rectángulo azul debe desplazarse correctamente al hacer pan.
4. `viewportWidth` y `viewportHeight` no deben quedarse en `0` tras cargar una zona con mesas.
