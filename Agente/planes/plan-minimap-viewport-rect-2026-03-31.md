# Plan: Fix definitivo del rectángulo viewport en CanvasMinimap

## Problema
El rectángulo azul del minimap en PlanoOcupacion no representa correctamente la proporción del viewport visible. Se ha intentado corregir múltiples veces sin éxito.

## Historial de intentos (todos insuficientes)

### 313A-2 — Primer fix
- **Cambio:** viewport rect más visible, minimap siempre visible si hay mesas
- **Problema creado:** el rect seguía sin verse porque el overlay lo tapaba

### 313A-5 — Segundo intento
- **Cambio:** overlay vía clip path evenodd, contentBounds = bounding box de mesas (excluyó zona)
- **Problema creado:** al usar solo mesas en vez de zona, si las mesas están dispersas el "mundo" del minimap crece mucho y el rect se encoge

### 313A-7 — Tercer intento
- **Cambio:** MIN_VIEWPORT_RECT de 20px como floor para el rect
- **Problema creado:** rect con floor ≠ overlay proporcional → desajuste visual (rect más grande que zona clara)

### 313A-7b — Cuarto intento
- **Cambio:** eliminar MIN_VIEWPORT_RECT, usar proporciones exactas para todo
- **Resultado:** las proporciones son exactas pero el rect sigue siendo pequeño porque el "mundo" (contentBounds) es incorrecto

### 313A-9 — Quinto intento
- **Cambio:** contentBounds = bounding box solo de mesas reales + 40px padding
- **Problema:** si las mesas se extienden mucho más allá de la zona (o si la zona es grande y los mesas ocupan poco), las proporciones no coinciden con lo que PlanoSala muestra

## Análisis raíz

El problema NUNCA fue el CanvasMinimap — su matemática es correcta. El problema es **qué le pasamos como contentWidth/contentHeight** desde PlanoOcupacion.

### PlanoSala (funciona correctamente):
```js
contentBounds = {
  w: max(zonaData.ancho * zoom, ...mesas_right_edges * zoom),
  h: max(zonaData.alto * zoom, ...mesas_bottom_edges * zoom)
}
```
- Base: dimensiones de la zona (el "mundo" de diseño)
- Extendido: si alguna mesa se sale de la zona

### PlanoOcupacion original (antes de mis cambios):
```js
contentBounds = {
  w: zonaData.ancho * zoom,
  h: Math.max(zonaData.alto * zoom, canvasHeight)  // ← BUG: canvasHeight
}
```
- `canvasHeight` (altura del viewport) se incluía en el content height
- Cuando canvasHeight > zonaData.alto * zoom, el contenido = viewport → rect cubre 100% alto → invisible
- ESTO era el bug original que el usuario reportó

### Mis fixes divergieron del camino correcto
En vez de simplemente quitar `canvasHeight` de la fórmula, empecé a cambiar la base de zona → mesas, creando un problema nuevo.

## Solución definitiva
Hacer que PlanoOcupacion use **exactamente la misma fórmula que PlanoSala**:
```js
contentBounds = {
  w: max(zonaData.ancho * zoom, ...mesas_right_edges * zoom),
  h: max(zonaData.alto * zoom, ...mesas_bottom_edges * zoom)
}
```
- Sin `canvasHeight` (eso causaba el bug original)
- Sin ignorar zona (eso causaba los bugs de proporción)
- Con extensión por mesas fuera de zona (como PlanoSala)

## Por qué esto funciona
- El "mundo" del minimap es la zona de diseño — el usuario ya sabe cuán grande es su plano
- El viewport rect muestra qué fracción de la zona se ve — proporcional y predecible
- Si una mesa se sale de la zona, el mundo se extiende automáticamente
- Es la misma lógica de PlanoSala que está probada y funciona
