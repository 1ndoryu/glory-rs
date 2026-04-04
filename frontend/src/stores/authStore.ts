/* [044A-13] Store de autenticación con Zustand.
 * Maneja token JWT, estado de sesión y persistencia en localStorage.
 * El token se guarda en localStorage para que axios-instance lo inyecte automáticamente. */
import {create} from 'zustand';

interface AuthUser {
    userId: string;
    email: string;
}

interface AuthState {
    token: string | null;
    user: AuthUser | null;
    logueado: boolean;
    login: (token: string, userId: string, email: string) => void;
    logout: () => void;
    inicializar: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
    token: null,
    user: null,
    logueado: false,

    login: (token, userId, email) => {
        localStorage.setItem('token', token);
        localStorage.setItem('auth_user', JSON.stringify({userId, email}));
        set({token, user: {userId, email}, logueado: true});
    },

    logout: () => {
        localStorage.removeItem('token');
        localStorage.removeItem('auth_user');
        set({token: null, user: null, logueado: false});
    },

    /* Restaura sesión desde localStorage al cargar la app */
    inicializar: () => {
        const token = localStorage.getItem('token');
        const userData = localStorage.getItem('auth_user');
        if (token && userData) {
            try {
                const user = JSON.parse(userData) as AuthUser;
                set({token, user, logueado: true});
            } catch {
                localStorage.removeItem('token');
                localStorage.removeItem('auth_user');
            }
        }
    },
}));
