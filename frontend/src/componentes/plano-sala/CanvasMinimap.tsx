/* [303A-12] Minimap del plano de sala — muestra overview de todas las mesas
 * como puntos y un rectángulo representando el viewport visible.
 * Reutilizable entre PlanoSala y PlanoOcupacion.
 * [303A-18] Fix: useEffect movido antes del early return para cumplir reglas de hooks.
 * [313A-2] Viewport rect más visible. Minimap siempre visible si hay mesas.
 * [313A-5] Fix viewport rect: reemplazado clearRect+redraw por clip path evenodd.
 * [313A-7] Fix viewport rect desproporcional: el MIN_VIEWPORT_RECT (20px floor)
 * hacía que el borde azul fuera más grande que el overlay, creando un desajuste
 * visual. Ahora overlay y rect usan las mismas dimensiones proporcionales exactas.
 * El borde azul tiene lineWidth 2.5 y el overlay es suficiente para ver el rect
 * incluso cuando es pequeño.
 * [313A-14] Quitado el log de diagnóstico y restaurado el posicionamiento absoluto
 * del canvas para que el minimap vuelva a quedar dentro de la esquina del plano. */

import { useEffect, useRef } from 'react';

interface MesaMinimapa {
  x: number;
  y: number;
  ancho: number;
  alto: number;
}

interface CanvasMinimapProps {
  mesas: MesaMinimapa[];
  contentWidth: number;
  contentHeight: number;
  viewportWidth: number;
  viewportHeight: number;
  scrollLeft: number;
  scrollTop: number;
  onNavigate: (scrollLeft: number, scrollTop: number) => void;
}

const MAX_MINIMAP_SIDE = 180;
const MIN_MINIMAP_SIDE = 60;

function CanvasMinimap({
  mesas,
  contentWidth,
  contentHeight,
  viewportWidth,
  viewportHeight,
  scrollLeft,
  scrollTop,
  onNavigate,
}: CanvasMinimapProps) {
  const canvasElRef = useRef<HTMLCanvasElement>(null);

  /* [313A-2] Mostrar minimap siempre que haya mesas y contenido real.
   * Si el contenido cabe en el viewport, no hay scroll pero el minimap
   * sigue sirviendo de referencia visual. */
  const visible = mesas.length > 0 && contentWidth > 0 && contentHeight > 0;

  /* Usar la dimensión real que el usuario puede recorrer:
   * si el contenido es menor que el viewport, aún así mostramos todo el viewport. */
  const effectiveWidth = Math.max(contentWidth, viewportWidth);
  const effectiveHeight = Math.max(contentHeight, viewportHeight);

  const aspectRatio = effectiveHeight > 0 ? effectiveWidth / effectiveHeight : 1;
  const minimapWidth = aspectRatio >= 1
    ? MAX_MINIMAP_SIDE
    : Math.max(MIN_MINIMAP_SIDE, Math.round(MAX_MINIMAP_SIDE * aspectRatio));
  const minimapHeight = aspectRatio >= 1
    ? Math.max(MIN_MINIMAP_SIDE, Math.round(MAX_MINIMAP_SIDE / Math.max(aspectRatio, 0.001)))
    : MAX_MINIMAP_SIDE;

  const scaleX = effectiveWidth > 0 ? minimapWidth / effectiveWidth : 1;
  const scaleY = effectiveHeight > 0 ? minimapHeight / effectiveHeight : 1;
  const scale = Math.min(scaleX, scaleY);
  const drawW = effectiveWidth * scale;
  const drawH = effectiveHeight * scale;
  const offsetX = (minimapWidth - drawW) / 2;
  const offsetY = (minimapHeight - drawH) / 2;

  useEffect(() => {
    if (!visible) return;
    const ctx = canvasElRef.current?.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, minimapWidth, minimapHeight);

    /* Fondo del área total del contenido */
    ctx.fillStyle = 'rgba(128, 128, 128, 0.1)';
    ctx.fillRect(offsetX, offsetY, drawW, drawH);

    /* Mesas como puntos */
    ctx.fillStyle = 'rgba(100, 100, 240, 0.5)';
    for (const m of mesas) {
      ctx.fillRect(
        offsetX + m.x * scale,
        offsetY + m.y * scale,
        Math.max(2, m.ancho * scale),
        Math.max(2, m.alto * scale),
      );
    }

    /* [313A-7] Viewport rect — mismas proporciones exactas para overlay y borde.
     * Antes el borde tenía un floor de 20px (MIN_VIEWPORT_RECT) que hacía que el
     * rectángulo azul fuera visualmente más grande que la zona clara del overlay,
     * creando un desajuste que parecía desproporcional. */
    const vx = offsetX + scrollLeft * scale;
    const vy = offsetY + scrollTop * scale;
    const vw = Math.min(viewportWidth, effectiveWidth - scrollLeft) * scale;
    const vh = Math.min(viewportHeight, effectiveHeight - scrollTop) * scale;

    /* Solo oscurecer si el viewport no cubre todo el contenido. */
    const viewportCoversAll = vw >= drawW - 1 && vh >= drawH - 1;
    if (!viewportCoversAll) {
      ctx.save();
      const region = new Path2D();
      region.rect(offsetX, offsetY, drawW, drawH);
      region.rect(vx, vy, vw, vh);
      ctx.clip(region, 'evenodd');
      ctx.fillStyle = 'rgba(0, 0, 0, 0.30)';
      ctx.fillRect(offsetX, offsetY, drawW, drawH);
      ctx.restore();
    }

    /* Borde del viewport — azul sólido, siempre visible.
     * Usa las mismas dimensiones vw/vh que el overlay para consistencia total. */
    ctx.strokeStyle = 'rgba(59, 130, 246, 1)';
    ctx.lineWidth = 2.5;
    ctx.strokeRect(vx + 1, vy + 1, vw - 2, vh - 2);
    /* Fill interior semitransparente */
    ctx.fillStyle = 'rgba(59, 130, 246, 0.12)';
    ctx.fillRect(vx + 1, vy + 1, vw - 2, vh - 2);
  }, [visible, mesas, viewportWidth, viewportHeight, scrollLeft, scrollTop, scale, drawW, drawH, minimapWidth, minimapHeight, offsetX, offsetY, effectiveWidth, effectiveHeight]);

  if (!visible) return null;

  const handleClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const rect = canvasElRef.current!.getBoundingClientRect();
    const mx = e.clientX - rect.left - offsetX;
    const my = e.clientY - rect.top - offsetY;
    const targetScrollLeft = Math.max(0, mx / scale - viewportWidth / 2);
    const targetScrollTop = Math.max(0, my / scale - viewportHeight / 2);
    onNavigate(targetScrollLeft, targetScrollTop);
  };

  return (
    <canvas
      ref={canvasElRef}
      width={minimapWidth}
      height={minimapHeight}
      className="planoMinimap"
      onClick={handleClick}
      title="Click para navegar"
    />
  );
}

export default CanvasMinimap;
