/* [303A-12] Minimap del plano de sala — muestra overview de todas las mesas
 * como puntos y un rectángulo representando el viewport visible.
 * Reutilizable entre PlanoSala y PlanoOcupacion.
 * [303A-18] Fix: useEffect movido antes del early return para cumplir reglas de hooks.
 * [313A-2] Viewport rect más visible (opacidad subida, área fuera del viewport oscurecida).
 * Minimap siempre visible si hay mesas, aunque no haya scroll. */

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

const MAX_MINIMAP_SIDE = 140;
const MIN_MINIMAP_SIDE = 48;

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

    /* [313A-2] Viewport rect — oscurecer fuera del viewport para que el rect resalte.
     * Luego dibujar borde azul + fill semitransparente dentro. */
    const vx = offsetX + scrollLeft * scale;
    const vy = offsetY + scrollTop * scale;
    const vw = Math.min(viewportWidth, effectiveWidth - scrollLeft) * scale;
    const vh = Math.min(viewportHeight, effectiveHeight - scrollTop) * scale;

    /* Oscurecer todo excepto viewport */
    ctx.fillStyle = 'rgba(0, 0, 0, 0.25)';
    ctx.fillRect(offsetX, offsetY, drawW, drawH);

    /* Limpiar la zona del viewport (quitar el oscurecimiento) */
    ctx.clearRect(vx, vy, vw, vh);

    /* Redibujar fondo claro + mesas dentro del viewport */
    ctx.fillStyle = 'rgba(128, 128, 128, 0.1)';
    ctx.fillRect(vx, vy, vw, vh);
    ctx.fillStyle = 'rgba(100, 100, 240, 0.5)';
    for (const m of mesas) {
      const mx = offsetX + m.x * scale;
      const my = offsetY + m.y * scale;
      const mw = Math.max(2, m.ancho * scale);
      const mh = Math.max(2, m.alto * scale);
      /* Solo redibujar si está dentro del viewport rect */
      if (mx + mw > vx && mx < vx + vw && my + mh > vy && my < vy + vh) {
        ctx.fillRect(mx, my, mw, mh);
      }
    }

    /* Borde del viewport — azul, bien visible */
    ctx.strokeStyle = 'rgba(59, 130, 246, 0.9)';
    ctx.lineWidth = 2;
    ctx.strokeRect(vx, vy, vw, vh);
    ctx.fillStyle = 'rgba(59, 130, 246, 0.12)';
    ctx.fillRect(vx, vy, vw, vh);
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
