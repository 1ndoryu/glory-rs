/* [303A-13] Hook para redimensionar el canvas arrastrando el borde inferior.
 * Recibe setCanvasHeight del store y retorna handlers + estado activo.
 * Usa mousedown/mousemove/mouseup en el document para capturar el drag
 * incluso cuando el cursor sale del handle. */

import { useCallback, useRef, useState, useEffect } from 'react';

interface UseCanvasResizeOptions {
  canvasHeight: number;
  setCanvasHeight: (h: number) => void;
  minHeight?: number;
  maxHeight?: number;
}

export function useCanvasResize({
  canvasHeight,
  setCanvasHeight,
  minHeight = 300,
  maxHeight = 1500,
}: UseCanvasResizeOptions) {
  const [resizing, setResizing] = useState(false);
  const startY = useRef(0);
  const startHeight = useRef(0);

  const onResizeStart = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    startY.current = e.clientY;
    startHeight.current = canvasHeight;
    setResizing(true);
  }, [canvasHeight]);

  useEffect(() => {
    if (!resizing) return;

    const onMove = (e: MouseEvent) => {
      const delta = e.clientY - startY.current;
      const next = Math.min(maxHeight, Math.max(minHeight, startHeight.current + delta));
      setCanvasHeight(next);
    };

    const onUp = () => setResizing(false);

    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
    return () => {
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    };
  }, [resizing, minHeight, maxHeight, setCanvasHeight]);

  return { resizing, onResizeStart };
}
