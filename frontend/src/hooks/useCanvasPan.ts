/* [303A-12] Hook para pan del canvas — middle-click drag o shift+click drag.
 * [303A-20] Reescrito: usa panOffset (state) + CSS transform en vez de
 * scrollLeft/scrollTop. Elimina toda dependencia de overflow:auto,
 * evitando desbordamiento de layout. */

import { useCallback, useRef, useState, useEffect } from 'react';

export interface PanOffset {
  x: number;
  y: number;
}

export function useCanvasPan() {
  const [panning, setPanning] = useState(false);
  const [panOffset, setPanOffset] = useState<PanOffset>({ x: 0, y: 0 });
  const startPos = useRef({ x: 0, y: 0 });
  const startPan = useRef({ x: 0, y: 0 });
  const panRef = useRef(panOffset);
  panRef.current = panOffset;

  const onMouseDown = useCallback((e: React.MouseEvent) => {
    /* Middle-click (button=1) o shift+left-click activan el pan */
    const isMiddle = e.button === 1;
    const isShiftLeft = e.button === 0 && e.shiftKey;
    if (!isMiddle && !isShiftLeft) return;

    e.preventDefault();
    startPos.current = { x: e.clientX, y: e.clientY };
    startPan.current = { ...panRef.current };
    setPanning(true);
  }, []);

  useEffect(() => {
    if (!panning) return;

    const onMove = (e: MouseEvent) => {
      const dx = e.clientX - startPos.current.x;
      const dy = e.clientY - startPos.current.y;
      setPanOffset({
        x: Math.max(0, startPan.current.x - dx),
        y: Math.max(0, startPan.current.y - dy),
      });
    };

    const onUp = () => setPanning(false);

    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
    return () => {
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    };
  }, [panning]);

  return { panning, panOffset, setPanOffset, onPanMouseDown: onMouseDown };
}
