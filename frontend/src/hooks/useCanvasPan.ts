/* [303A-12] Hook para pan del canvas — middle-click drag o shift+click drag.
 * [303A-20] Reescrito: usa panOffset (state) + CSS transform en vez de
 * scrollLeft/scrollTop. Elimina toda dependencia de overflow:auto,
 * evitando desbordamiento de layout. */

import { useCallback, useRef, useState, useEffect } from 'react';

export interface PanOffset {
  x: number;
  y: number;
}

function clampOffset(offset: PanOffset, maxOffset: PanOffset): PanOffset {
  return {
    x: Math.min(Math.max(0, offset.x), Math.max(0, maxOffset.x)),
    y: Math.min(Math.max(0, offset.y), Math.max(0, maxOffset.y)),
  };
}

export function useCanvasPan(maxOffset: PanOffset) {
  const [panning, setPanning] = useState(false);
  const [panOffset, setPanOffset] = useState<PanOffset>({ x: 0, y: 0 });
  const startPos = useRef({ x: 0, y: 0 });
  const startPan = useRef({ x: 0, y: 0 });
  const panRef = useRef(panOffset);
  const maxOffsetRef = useRef(maxOffset);
  panRef.current = panOffset;
  maxOffsetRef.current = maxOffset;

  const setPanOffsetClamped = useCallback((next: PanOffset | ((prev: PanOffset) => PanOffset)) => {
    const raw = typeof next === 'function' ? next(panRef.current) : next;
    setPanOffset(clampOffset(raw, maxOffsetRef.current));
  }, []);

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
      setPanOffsetClamped({
        x: startPan.current.x - dx,
        y: startPan.current.y - dy,
      });
    };

    const onUp = () => setPanning(false);

    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
    return () => {
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    };
  }, [panning, setPanOffsetClamped]);

  useEffect(() => {
    setPanOffset((prev) => clampOffset(prev, maxOffset));
  }, [maxOffset.x, maxOffset.y]);

  return { panning, panOffset, setPanOffset: setPanOffsetClamped, onPanMouseDown: onMouseDown };
}
