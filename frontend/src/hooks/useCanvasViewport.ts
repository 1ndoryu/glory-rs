/* [134A-15+16] Hook para handlers del viewport del canvas según herramienta activa.
 * Extraído de PlanoSala para mantener < 300 líneas.
 * Gestiona: click (crear mesa / deseleccionar / combinar),
 * mousedown (pan / dibujar pared), mousemove (preview pared),
 * mouseup global (terminar dibujo pared). */

import { useCallback, useEffect, useRef } from 'react';
import type { CanvasTool } from '../componentes/plano-sala/CanvasToolbar';

interface ViewportToolsConfig {
  activeTool: CanvasTool;
  wallDrawStart: { x: number; y: number } | null;
  zonaActiva: string | null;
  panOffset: { x: number; y: number };
  zoom: number;
  setMesaSeleccionada: (m: null) => void;
  setParedSeleccionada: (p: null) => void;
  handleCrearCombinacion: () => void;
  handleCrearMesaRapida: (pos: { x: number; y: number }, forma: string) => void;
  onPanMouseDown: (e: React.MouseEvent) => void;
  handleWallDrawStart: (x: number, y: number) => void;
  handleWallDrawMove: (x: number, y: number) => void;
  handleWallDrawEnd: () => void;
}

export function useCanvasViewport(config: ViewportToolsConfig) {
  const {
    activeTool, wallDrawStart, zonaActiva, panOffset, zoom,
    setMesaSeleccionada, setParedSeleccionada,
    handleCrearCombinacion, handleCrearMesaRapida,
    onPanMouseDown, handleWallDrawStart, handleWallDrawMove, handleWallDrawEnd,
  } = config;

  /* Ref estable para handleWallDrawEnd — evita stale closures en el listener global */
  const wallDrawEndRef = useRef(handleWallDrawEnd);
  wallDrawEndRef.current = handleWallDrawEnd;

  /* Document-level mouseup para wall draw — funcional incluso si el cursor sale del viewport */
  const isDrawingWall = !!wallDrawStart;
  useEffect(() => {
    if (!isDrawingWall) return;
    const onUp = () => wallDrawEndRef.current();
    document.addEventListener('mouseup', onUp);
    return () => document.removeEventListener('mouseup', onUp);
  }, [isDrawingWall]);

  /** Click en viewport: deseleccionar (select), crear mesa (mesa-*), abrir diálogo (combine) */
  const onViewportClick = useCallback((e: React.MouseEvent) => {
    if (activeTool === 'select') {
      setMesaSeleccionada(null);
      setParedSeleccionada(null);
      return;
    }
    if (activeTool === 'combine') {
      handleCrearCombinacion();
      return;
    }
    if (activeTool === 'mesa-cuadrada' || activeTool === 'mesa-redonda') {
      if (!zonaActiva) return;
      const target = e.target as HTMLElement;
      if (target !== e.currentTarget && !target.classList.contains('planoCanvasContent')) return;
      const viewRect = (e.currentTarget as HTMLElement).getBoundingClientRect();
      const x = Math.round((e.clientX - viewRect.left + panOffset.x) / zoom);
      const y = Math.round((e.clientY - viewRect.top + panOffset.y) / zoom);
      handleCrearMesaRapida({ x, y }, activeTool === 'mesa-cuadrada' ? 'cuadrada' : 'redonda');
    }
  }, [activeTool, zonaActiva, panOffset, zoom, setMesaSeleccionada, setParedSeleccionada, handleCrearMesaRapida, handleCrearCombinacion]);

  /** Mousedown: pan (shift/middle/tool) o iniciar dibujo de pared */
  const onViewportMouseDown = useCallback((e: React.MouseEvent) => {
    if (activeTool === 'pan' || e.shiftKey || e.button === 1) {
      onPanMouseDown(e);
      return;
    }
    if (activeTool === 'pared' && e.button === 0) {
      const target = e.target as HTMLElement;
      if (target !== e.currentTarget && !target.classList.contains('planoCanvasContent')) return;
      const viewRect = (e.currentTarget as HTMLElement).getBoundingClientRect();
      const x = (e.clientX - viewRect.left + panOffset.x) / zoom;
      const y = (e.clientY - viewRect.top + panOffset.y) / zoom;
      handleWallDrawStart(x, y);
      e.preventDefault();
    }
  }, [activeTool, onPanMouseDown, panOffset, zoom, handleWallDrawStart]);

  /** Mousemove: actualizar preview de pared durante dibujo A→B */
  const onViewportMouseMove = useCallback((e: React.MouseEvent) => {
    if (!wallDrawStart || activeTool !== 'pared') return;
    const viewRect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    const x = (e.clientX - viewRect.left + panOffset.x) / zoom;
    const y = (e.clientY - viewRect.top + panOffset.y) / zoom;
    handleWallDrawMove(x, y);
  }, [wallDrawStart, activeTool, panOffset, zoom, handleWallDrawMove]);

  return { onViewportClick, onViewportMouseDown, onViewportMouseMove };
}
