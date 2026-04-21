/* [174A-109b-fase2] Compatibilidad mínima para `@app/stores/panelLateralStore`.
 * Mezclador solo necesita cerrar el panel y alternar expansión; el resto del shell
 * lateral del legado se reintroducirá cuando exista su equivalente SPA real. */

import { create } from 'zustand';

type ModoPanelLateral = 'mezclador' | null;

interface PanelLateralState {
  modo: ModoPanelLateral;
  habilitado: boolean;
  expandido: boolean;
  abrirMezclador: () => void;
  toggleExpandido: () => void;
  cerrar: () => void;
}

export const usePanelLateralStore = create<PanelLateralState>((set) => ({
  modo: 'mezclador',
  habilitado: true,
  expandido: true,
  abrirMezclador: () => set({ modo: 'mezclador', habilitado: true }),
  toggleExpandido: () => set((state) => ({ expandido: !state.expandido })),
  cerrar: () => set({ modo: null, expandido: false }),
}));
