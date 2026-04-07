/* [054A-3] Hook compartido para el perfil autenticado.
 * Centraliza la query del perfil para que header, sidebar y ajustes compartan cache.
 * La key incluye userId para evitar fugas de cache entre sesiones distintas. */
import {useQuery} from '@tanstack/react-query';
import {obtenerPerfil, type PerfilResponse} from '../api/profile';
import {useAuthStore} from '../stores/authStore';

/* [064A-53] Avatar default: SVG inline con fondo taupe (#c4b5a5) y silueta clara (#f5f3f1).
 * Mas contraste y aspecto moderno que la version anterior. Sin dependencia externa. */
export const DEFAULT_PROFILE_AVATAR = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Ccircle cx='50' cy='50' r='50' fill='%23c4b5a5'/%3E%3Ccircle cx='50' cy='38' r='14' fill='%23f5f3f1'/%3E%3Cpath d='M24 85c0-15 12-26 26-26s26 11 26 26' fill='%23f5f3f1'/%3E%3C/svg%3E";

export const currentProfileKey = (userId?: string) =>
    ['current-profile', userId ?? 'anonymous'] as const;

export function resolveProfileAvatar(avatarUrl?: string | null): string {
    return avatarUrl || DEFAULT_PROFILE_AVATAR;
}

export function useCurrentProfile() {
    const token = useAuthStore(s => s.token);
    const userId = useAuthStore(s => s.user?.userId);

    const query = useQuery<PerfilResponse>({
        queryKey: currentProfileKey(userId),
        queryFn: obtenerPerfil,
        enabled: Boolean(token && userId),
    });

    return {
        ...query,
        cargando: query.isLoading,
        perfil: query.data ?? null,
        avatarUrl: resolveProfileAvatar(query.data?.avatar_url),
    };
}