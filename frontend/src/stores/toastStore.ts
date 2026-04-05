/* [054A-5] Store de toasts con Zustand.
 * Sistema de notificaciones temporales (auto-dismiss).
 * Cada toast tiene tipo, mensaje y duración configurable. */
import { create } from 'zustand';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface Toast {
    id: string;
    type: ToastType;
    message: string;
    duration: number;
}

interface ToastState {
    toasts: Toast[];
    addToast: (type: ToastType, message: string, duration?: number) => void;
    removeToast: (id: string) => void;
}

let toastCounter = 0;

export const useToastStore = create<ToastState>((set) => ({
    toasts: [],

    addToast: (type, message, duration = 4000) => {
        const id = `toast-${Date.now()}-${++toastCounter}`;
        set((s) => ({ toasts: [...s.toasts, { id, type, message, duration }] }));

        /* Auto-remove después de duration */
        setTimeout(() => {
            set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) }));
        }, duration);
    },

    removeToast: (id) => {
        set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) }));
    },
}));

/* Helpers para uso directo sin necesidad de acceder al store */
export const toast = {
    success: (message: string, duration?: number) =>
        useToastStore.getState().addToast('success', message, duration),
    error: (message: string, duration?: number) =>
        useToastStore.getState().addToast('error', message, duration),
    info: (message: string, duration?: number) =>
        useToastStore.getState().addToast('info', message, duration),
    warning: (message: string, duration?: number) =>
        useToastStore.getState().addToast('warning', message, duration),
};
