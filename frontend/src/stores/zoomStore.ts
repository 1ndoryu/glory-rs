/* [283A-36] Store de zoom del plano de sala — Zustand + localStorage.
 * Compartido entre PlanoSala y PlanoOcupacion para que estén sincronizados.
 * Persistido en localStorage para mantener el zoom entre sesiones. */

import { create } from 'zustand';

const STORAGE_KEY = 'plano-zoom';
const MIN_ZOOM = 0.5;
const MAX_ZOOM = 2;
const STEP = 0.1;

function loadZoom(): number {
  try {
    const v = localStorage.getItem(STORAGE_KEY);
    if (v) {
      const n = parseFloat(v);
      if (!isNaN(n) && n >= MIN_ZOOM && n <= MAX_ZOOM) return n;
    }
  } catch { /* ignore */ }
  return 1;
}

interface ZoomState {
  zoom: number;
  setZoom: (fn: (prev: number) => number) => void;
  zoomIn: () => void;
  zoomOut: () => void;
}

export const useZoomStore = create<ZoomState>((set) => ({
  zoom: loadZoom(),
  setZoom: (fn) => set((state) => {
    const next = Math.round(Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, fn(state.zoom))) * 100) / 100;
    localStorage.setItem(STORAGE_KEY, String(next));
    return { zoom: next };
  }),
  zoomIn: () => set((state) => {
    const next = Math.round(Math.min(MAX_ZOOM, state.zoom + STEP) * 100) / 100;
    localStorage.setItem(STORAGE_KEY, String(next));
    return { zoom: next };
  }),
  zoomOut: () => set((state) => {
    const next = Math.round(Math.max(MIN_ZOOM, state.zoom - STEP) * 100) / 100;
    localStorage.setItem(STORAGE_KEY, String(next));
    return { zoom: next };
  }),
}));
