import { useCallback, useEffect, useRef, useState } from 'react';

export type MesaResizeDirection = 'n' | 's' | 'e' | 'w';

export interface MesaResizeRect {
  x: number;
  y: number;
  width: number;
  height: number;
}

interface UseMesaResizeOptions {
  rect: MesaResizeRect;
  bounds: {
    width: number;
    height: number;
  };
  zoom: number;
  onCommit: (rect: MesaResizeRect) => void | Promise<void>;
  minSize?: number;
  maxSize?: number;
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

/* [014A-10] bounds solo se usa para w/n (evitar coordenadas negativas).
 * Para e/s ya no se restringe al borde de zona: el canvas es 100% ancho y
 * la mesa puede crecer libremente hasta maxSize — igual que el drag.
 * Antes: la zona imponía un techo que impedía resize cerca del borde derecho. */
function calculateNextRect(
  rect: MesaResizeRect,
  direction: MesaResizeDirection,
  dx: number,
  dy: number,
  _bounds: { width: number; height: number },
  minSize: number,
  maxSize: number,
): MesaResizeRect {
  let next = { ...rect };

  if (direction === 'e') {
    next.width = clamp(rect.width + dx, minSize, maxSize);
  }

  if (direction === 's') {
    next.height = clamp(rect.height + dy, minSize, maxSize);
  }

  if (direction === 'w') {
    const width = clamp(rect.width - dx, minSize, Math.min(maxSize, rect.x + rect.width));
    next.x = rect.x + (rect.width - width);
    next.width = width;
  }

  if (direction === 'n') {
    const height = clamp(rect.height - dy, minSize, Math.min(maxSize, rect.y + rect.height));
    next.y = rect.y + (rect.height - height);
    next.height = height;
  }

  return next;
}

export function useMesaResize({
  rect,
  bounds,
  zoom,
  onCommit,
  minSize = 30,
  maxSize = 400,
}: UseMesaResizeOptions) {
  const [direction, setDirection] = useState<MesaResizeDirection | null>(null);
  const [previewRect, setPreviewRect] = useState(rect);
  const startPointRef = useRef({ x: 0, y: 0 });
  const startRectRef = useRef(rect);
  const lastRectRef = useRef(rect);
  const commitRef = useRef(onCommit);
  commitRef.current = onCommit;

  useEffect(() => {
    if (direction) return;
    setPreviewRect(rect);
    lastRectRef.current = rect;
  }, [direction, rect.height, rect.width, rect.x, rect.y]);

  const onResizeStart = useCallback((nextDirection: MesaResizeDirection) => (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    startPointRef.current = { x: e.clientX, y: e.clientY };
    startRectRef.current = rect;
    lastRectRef.current = rect;
    setPreviewRect(rect);
    setDirection(nextDirection);
  }, [rect]);

  useEffect(() => {
    if (!direction) return;

    const onMove = (e: MouseEvent) => {
      const dx = (e.clientX - startPointRef.current.x) / zoom;
      const dy = (e.clientY - startPointRef.current.y) / zoom;
      const next = calculateNextRect(
        startRectRef.current,
        direction,
        dx,
        dy,
        bounds,
        minSize,
        maxSize,
      );
      lastRectRef.current = next;
      setPreviewRect(next);
    };

    const onUp = () => {
      const finalRect = lastRectRef.current;
      const changed =
        finalRect.x !== startRectRef.current.x ||
        finalRect.y !== startRectRef.current.y ||
        finalRect.width !== startRectRef.current.width ||
        finalRect.height !== startRectRef.current.height;
      setDirection(null);
      if (changed) {
        void commitRef.current(finalRect);
      }
    };

    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
    return () => {
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    };
  }, [bounds, direction, maxSize, minSize, zoom]);

  return {
    resizing: direction !== null,
    previewRect,
    onResizeStart,
  };
}