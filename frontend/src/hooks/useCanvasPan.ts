/* [303A-12] Hook para pan del canvas — middle-click drag o shift+click drag
 * para desplazar el viewport. Funciona con cualquier div scrollable. */

import { useCallback, useRef, useState, useEffect } from 'react';

export function useCanvasPan(scrollRef: React.RefObject<HTMLElement | null>) {
  const [panning, setPanning] = useState(false);
  const startPos = useRef({ x: 0, y: 0 });
  const startScroll = useRef({ left: 0, top: 0 });

  const onMouseDown = useCallback((e: React.MouseEvent) => {
    /* Middle-click (button=1) o shift+left-click activan el pan */
    const isMiddle = e.button === 1;
    const isShiftLeft = e.button === 0 && e.shiftKey;
    if (!isMiddle && !isShiftLeft) return;
    if (!scrollRef.current) return;

    e.preventDefault();
    startPos.current = { x: e.clientX, y: e.clientY };
    startScroll.current = {
      left: scrollRef.current.scrollLeft,
      top: scrollRef.current.scrollTop,
    };
    setPanning(true);
  }, [scrollRef]);

  useEffect(() => {
    if (!panning) return;

    const onMove = (e: MouseEvent) => {
      if (!scrollRef.current) return;
      const dx = e.clientX - startPos.current.x;
      const dy = e.clientY - startPos.current.y;
      scrollRef.current.scrollLeft = startScroll.current.left - dx;
      scrollRef.current.scrollTop = startScroll.current.top - dy;
    };

    const onUp = () => setPanning(false);

    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
    return () => {
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    };
  }, [panning, scrollRef]);

  return { panning, onPanMouseDown: onMouseDown };
}
