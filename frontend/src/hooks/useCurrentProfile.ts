/* [054A-3] Hook compartido para el perfil autenticado.
 * Centraliza la query del perfil para que header, sidebar y ajustes compartan cache.
 * La key incluye userId para evitar fugas de cache entre sesiones distintas. */
import {useQuery} from '@tanstack/react-query';
import {obtenerPerfil, type PerfilResponse} from '../api/profile';
import {useAuthStore} from '../stores/authStore';

export const DEFAULT_PROFILE_AVATAR = 'https://i.pravatar.cc/100?u=default';

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