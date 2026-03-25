/* 253A-7: Store de autenticación JWT — Zustand para estado de cliente */

import { create } from 'zustand';

interface AuthState {
  token: string | null;
  iniciarSesion: (token: string) => void;
  cerrarSesion: () => void;
  estaAutenticado: () => boolean;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  token: localStorage.getItem('token'),

  iniciarSesion: (token: string) => {
    localStorage.setItem('token', token);
    set({ token });
  },

  cerrarSesion: () => {
    localStorage.removeItem('token');
    set({ token: null });
  },

  estaAutenticado: () => get().token !== null,
}));
