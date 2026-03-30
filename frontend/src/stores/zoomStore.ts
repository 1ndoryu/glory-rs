/* [283A-36] Store de zoom del plano de sala — Zustand + localStorage.
 * Compartido entre PlanoSala y PlanoOcupacion para que estén sincronizados.
 * Persistido en localStorage para mantener el zoom entre sesiones.
 * [303A-11] Añadido canvasHeight: altura fija del viewport del canvas,
 * independiente del zoom. Sincronizada entre ambos paneles via store. */

import { create } from 'zustand';

const ZOOM_KEY = 'plano-zoom';
const HEIGHT_KEY = 'plano-canvas-height';
const MIN_ZOOM = 0.5;
const MAX_ZOOM = 2;
const STEP = 0.1;
const DEFAULT_CANVAS_HEIGHT = 600;
const MIN_CANVAS_HEIGHT = 300;
const MAX_CANVAS_HEIGHT = 1500;

function loadZoom(): number {
  try {
    const v = localStorage.getItem(ZOOM_KEY);
    if (v) {
      const n = parseFloat(v);
      if (!isNaN(n) && n >= MIN_ZOOM && n <= MAX_ZOOM) return n;
    }
  } catch { /* ignore */ }
  return 1;
}

function loadCanvasHeight(): number {
  try {
    const v = localStorage.getItem(HEIGHT_KEY);
    if (v) {
      const n = parseInt(v, 10);
      if (!isNaN(n) && n >= MIN_CANVAS_HEIGHT && n <= MAX_CANVAS_HEIGHT) return n;
    }
  } catch { /* ignore */ }
  return DEFAULT_CANVAS_HEIGHT;
}

interface ZoomState {
  zoom: number;
  canvasHeight: number;
  setZoom: (fn: (prev: number) => number) => void;
  setCanvasHeight: (h: number) => void;
  zoomIn: () => void;
  zoomOut: () => void;
}

export { MIN_CANVAS_HEIGHT, MAX_CANVAS_HEIGHT };

export const useZoomStore = create<ZoomState>((set) => ({
  zoom: loadZoom(),
  canvasHeight: loadCanvasHeight(),
  setZoom: (fn) => set((state) => {
    const next = Math.round(Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, fn(state.zoom))) * 100) / 100;
    localStorage.setItem(ZOOM_KEY, String(next));
    return { zoom: next };
  }),
  setCanvasHeight: (h) => set(() => {
    const clamped = Math.min(MAX_CANVAS_HEIGHT, Math.max(MIN_CANVAS_HEIGHT, Math.round(h)));
    localStorage.setItem(HEIGHT_KEY, String(clamped));
    return { canvasHeight: clamped };
  }),
  zoomIn: () => set((state) => {
    const next = Math.round(Math.min(MAX_ZOOM, state.zoom + STEP) * 100) / 100;
    localStorage.setItem(ZOOM_KEY, String(next));
    return { zoom: next };
  }),
  zoomOut: () => set((state) => {
    const next = Math.round(Math.max(MIN_ZOOM, state.zoom - STEP) * 100) / 100;
    localStorage.setItem(ZOOM_KEY, String(next));
    return { zoom: next };
  }),
}));
