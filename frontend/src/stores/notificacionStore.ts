/* [283A-20] Store de notificaciones en tiempo real (Zustand).
 * Mantiene la lista de notificaciones recientes y el conteo de no leídas.
 * Se actualiza via SSE y al cargar la página. */

import { create } from 'zustand';

export interface Notificacion {
    id: string;
    user_id: string;
    tipo: string;
    titulo: string;
    mensaje: string;
    leida: boolean;
    created_at: string;
}

interface NotificacionState {
    items: Notificacion[];
    noLeidas: number;
    setItems: (items: Notificacion[]) => void;
    setNoLeidas: (count: number) => void;
    agregarNotificacion: (notif: Notificacion) => void;
    marcarLeida: (id: string) => void;
    marcarTodasLeidas: () => void;
}

export const useNotificacionStore = create<NotificacionState>((set) => ({
    items: [],
    noLeidas: 0,

    setItems: (items) => set({ items }),
    setNoLeidas: (count) => set({ noLeidas: count }),

    agregarNotificacion: (notif) =>
        set((state) => ({
            items: [notif, ...state.items].slice(0, 50),
            noLeidas: state.noLeidas + 1,
        })),

    marcarLeida: (id) =>
        set((state) => ({
            items: state.items.map((n) => (n.id === id ? { ...n, leida: true } : n)),
            noLeidas: Math.max(0, state.noLeidas - 1),
        })),

    marcarTodasLeidas: () =>
        set((state) => ({
            items: state.items.map((n) => ({ ...n, leida: true })),
            noLeidas: 0,
        })),
}));
