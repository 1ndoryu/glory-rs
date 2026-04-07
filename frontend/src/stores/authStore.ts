/* [044A-13] Store de autenticación con Zustand.
 * Maneja token JWT, estado de sesión y persistencia en localStorage.
 * El token se guarda en localStorage para que axios-instance lo inyecte automáticamente.
 * [044A-38 Fase 1] Añadido role/effectiveRole para panel dinámico por rol.
 * [084A-1] Añadido impersonating + userId dinámico para impersonación real. */
import {create} from 'zustand';
import type {UserRole} from '../api/auth';

interface AuthUser {
    userId: string;
    email: string;
    role: UserRole;
    effectiveRole: UserRole;
    impersonating: boolean;
}

interface AuthState {
    token: string | null;
    user: AuthUser | null;
    logueado: boolean;
    login: (token: string, userId: string, email: string, role: UserRole, effectiveRole: UserRole) => void;
    logout: () => void;
    inicializar: () => void;
    /* [084A-1] Actualiza token, roles, userId y estado de impersonación tras switch-role */
    actualizarRol: (token: string, userId: string, role: UserRole, effectiveRole: UserRole, impersonating: boolean) => void;
}

export const useAuthStore = create<AuthState>((set, get) => ({
    token: null,
    user: null,
    logueado: false,

    login: (token, userId, email, role, effectiveRole) => {
        localStorage.setItem('token', token);
        localStorage.setItem('auth_user', JSON.stringify({userId, email, role, effectiveRole, impersonating: false}));
        set({token, user: {userId, email, role, effectiveRole, impersonating: false}, logueado: true});
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
                /* [044A-38 Fase 1] Backward compat: si no tiene role, asumir client */
                if (!user.role) {
                    user.role = 'client';
                    user.effectiveRole = 'client';
                }
                if (user.impersonating === undefined) {
                    user.impersonating = false;
                }
                set({token, user, logueado: true});
            } catch {
                localStorage.removeItem('token');
                localStorage.removeItem('auth_user');
            }
        }
    },

    /* [084A-1] Switch role: actualiza token, userId, roles e impersonación */
    actualizarRol: (token, userId, role, effectiveRole, impersonating) => {
        const currentUser = get().user;
        if (!currentUser) return;
        const updatedUser = {...currentUser, userId, role, effectiveRole, impersonating};
        localStorage.setItem('token', token);
        localStorage.setItem('auth_user', JSON.stringify(updatedUser));
        set({token, user: updatedUser});
    },
}));
