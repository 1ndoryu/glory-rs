/* [303A-12] Minimap del plano de sala — muestra overview de todas las mesas
 * como puntos y un rectángulo representando el viewport visible.
 * Reutilizable entre PlanoSala y PlanoOcupacion. */

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

const MINIMAP_W = 140;
const MINIMAP_H = 100;

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

  /* No mostrar si el contenido cabe completamente en el viewport */
  const needsScroll = contentWidth > viewportWidth || contentHeight > viewportHeight;
  if (!needsScroll || contentWidth === 0 || contentHeight === 0) return null;

  const scaleX = MINIMAP_W / contentWidth;
  const scaleY = MINIMAP_H / contentHeight;
  const scale = Math.min(scaleX, scaleY);

  const drawW = contentWidth * scale;
  const drawH = contentHeight * scale;

  useEffect(() => {
    const ctx = canvasElRef.current?.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, MINIMAP_W, MINIMAP_H);

    /* Fondo del área de contenido */
    ctx.fillStyle = 'rgba(128, 128, 128, 0.1)';
    ctx.fillRect(0, 0, drawW, drawH);

    /* Mesas como rectángulos pequeños */
    ctx.fillStyle = 'rgba(100, 100, 240, 0.5)';
    for (const m of mesas) {
      ctx.fillRect(m.x * scale, m.y * scale, Math.max(2, m.ancho * scale), Math.max(2, m.alto * scale));
    }

    /* Viewport rect */
    const vx = scrollLeft * scale;
    const vy = scrollTop * scale;
    const vw = Math.min(viewportWidth, contentWidth) * scale;
    const vh = Math.min(viewportHeight, contentHeight) * scale;
    ctx.strokeStyle = 'rgba(59, 130, 246, 0.8)';
    ctx.lineWidth = 1.5;
    ctx.strokeRect(vx, vy, vw, vh);
    ctx.fillStyle = 'rgba(59, 130, 246, 0.08)';
    ctx.fillRect(vx, vy, vw, vh);
  }, [mesas, contentWidth, contentHeight, viewportWidth, viewportHeight, scrollLeft, scrollTop, scale, drawW, drawH]);

  const handleClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const rect = canvasElRef.current!.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;
    /* Centrar viewport en el punto clickeado */
    const targetScrollLeft = Math.max(0, mx / scale - viewportWidth / 2);
    const targetScrollTop = Math.max(0, my / scale - viewportHeight / 2);
    onNavigate(targetScrollLeft, targetScrollTop);
  };

  return (
    <canvas
      ref={canvasElRef}
      width={MINIMAP_W}
      height={MINIMAP_H}
      className="planoMinimap"
      onClick={handleClick}
      title="Click para navegar"
    />
  );
}

export default CanvasMinimap;
