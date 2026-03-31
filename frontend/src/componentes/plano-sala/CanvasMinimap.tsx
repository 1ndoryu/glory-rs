/* [303A-12] Minimap del plano de sala — muestra overview de todas las mesas
 * como puntos y un rectángulo representando el viewport visible.
 * Reutilizable entre PlanoSala y PlanoOcupacion.
 * [303A-18] Fix: useEffect movido antes del early return para cumplir reglas de hooks. */

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

  const needsScroll = contentWidth > viewportWidth || contentHeight > viewportHeight;
  const visible = needsScroll && contentWidth > 0 && contentHeight > 0;

  const scaleX = contentWidth > 0 ? MINIMAP_W / contentWidth : 1;
  const scaleY = contentHeight > 0 ? MINIMAP_H / contentHeight : 1;
  const scale = Math.min(scaleX, scaleY);
  const drawW = contentWidth * scale;
  const drawH = contentHeight * scale;

  useEffect(() => {
    if (!visible) return;
    const ctx = canvasElRef.current?.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, MINIMAP_W, MINIMAP_H);

    ctx.fillStyle = 'rgba(128, 128, 128, 0.1)';
    ctx.fillRect(0, 0, drawW, drawH);

    ctx.fillStyle = 'rgba(100, 100, 240, 0.5)';
    for (const m of mesas) {
      ctx.fillRect(m.x * scale, m.y * scale, Math.max(2, m.ancho * scale), Math.max(2, m.alto * scale));
    }

    const vx = scrollLeft * scale;
    const vy = scrollTop * scale;
    const vw = Math.min(viewportWidth, contentWidth) * scale;
    const vh = Math.min(viewportHeight, contentHeight) * scale;
    ctx.strokeStyle = 'rgba(59, 130, 246, 0.8)';
    ctx.lineWidth = 1.5;
    ctx.strokeRect(vx, vy, vw, vh);
    ctx.fillStyle = 'rgba(59, 130, 246, 0.08)';
    ctx.fillRect(vx, vy, vw, vh);
  }, [visible, mesas, contentWidth, contentHeight, viewportWidth, viewportHeight, scrollLeft, scrollTop, scale, drawW, drawH]);

  if (!visible) return null;

  const handleClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const rect = canvasElRef.current!.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;
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
