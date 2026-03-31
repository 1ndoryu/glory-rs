/* [313A-13] Medición compartida del viewport para minimap e indicadores.
 * Evita el bug donde un viewport renderizado condicionalmente se medía como 0x0
 * porque el effect corría antes de que existiera el nodo o antes de que el layout
 * real del contenedor hubiera asentado su tamaño. */

import { useCallback, useLayoutEffect, useRef, useState, type DependencyList } from 'react';

interface ViewportSize {
  w: number;
  h: number;
}

export function useViewportSize(enabled: boolean, deps: DependencyList = []) {
  const viewportRef = useRef<HTMLDivElement | null>(null);
  const [viewportSize, setViewportSize] = useState<ViewportSize>({ w: 0, h: 0 });

  const measureViewport = useCallback(() => {
    const el = viewportRef.current;
    if (!el) return false;

    const rect = el.getBoundingClientRect();
    const next = { w: Math.round(rect.width), h: Math.round(rect.height) };
    setViewportSize((prev) => (prev.w === next.w && prev.h === next.h ? prev : next));
    return next.w > 0 && next.h > 0;
  }, []);

  useLayoutEffect(() => {
    if (!enabled) {
      setViewportSize((prev) => (prev.w === 0 && prev.h === 0 ? prev : { w: 0, h: 0 }));
      return;
    }

    const el = viewportRef.current;
    if (!el) return;

    let rafId = 0;
    let attempts = 0;
    const measureUntilReady = () => {
      attempts += 1;
      if (measureViewport() || attempts >= 12) return;
      rafId = window.requestAnimationFrame(measureUntilReady);
    };

    measureUntilReady();

    const ro = new ResizeObserver(() => {
      measureViewport();
    });
    ro.observe(el);

    return () => {
      if (rafId) window.cancelAnimationFrame(rafId);
      ro.disconnect();
    };
  }, [enabled, measureViewport, ...deps]);

  return { viewportRef, viewportSize, measureViewport };
}