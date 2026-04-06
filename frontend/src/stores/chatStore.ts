/* [064A-5] Store global para controlar la visibilidad del ChatWidget.
 * Permite abrir el chat desde cualquier componente (CTAs, header, etc.)
 * sin prop drilling ni custom events. */
import { create } from 'zustand';

interface ChatState {
    abierto: boolean;
    abrir: () => void;
    cerrar: () => void;
}

export const useChatStore = create<ChatState>((set) => ({
    abierto: false,
    abrir: () => set({ abierto: true }),
    cerrar: () => set({ abierto: false }),
}));
