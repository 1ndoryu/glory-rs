/* [044A-43] Hook de perfil: carga perfil desde backend, sube avatar.
 * Reemplaza la versión stub que usaba obtenerUsuarioActual (siempre null). */
import {useState, useEffect} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {subirAvatar} from '../api/profile';
import type {PerfilResponse} from '../api/profile';
import {currentProfileKey, useCurrentProfile} from './useCurrentProfile';
import {useAuthStore} from '../stores/authStore';

interface EstadoPerfil {
    nombre: string;
    descripcion: string;
    linkedin: string;
    twitter: string;
    website: string;
}

interface RetornoUsePerfil {
    estado: EstadoPerfil;
    guardado: boolean;
    cargando: boolean;
    perfil: PerfilResponse | null;
    avatarUrl: string;
    subiendoAvatar: boolean;
    actualizarCampo: (campo: keyof EstadoPerfil, valor: string) => void;
    handleGuardar: (e: React.FormEvent) => void;
    handleSubirAvatar: (archivo: File) => Promise<void>;
}

export const usePerfil = (): RetornoUsePerfil => {
    const queryClient = useQueryClient();
    const userId = useAuthStore(s => s.user?.userId);
    const {perfil, cargando: cargandoPerfil, avatarUrl} = useCurrentProfile();
    const [subiendoAvatar, setSubiendoAvatar] = useState(false);

    const [estado, setEstado] = useState<EstadoPerfil>({
        nombre: '',
        descripcion: '',
        linkedin: '',
        twitter: '',
        website: ''
    });

    const [guardado, setGuardado] = useState(false);

    /* Sincroniza los datos editables cuando el backend devuelve el perfil actual */
    useEffect(() => {
        if (!perfil) return;
        setEstado(prev => ({
            ...prev,
            nombre: prev.nombre || perfil.display_name || ''
        }));
    }, [perfil]);

    const actualizarCampo = (campo: keyof EstadoPerfil, valor: string) => {
        setEstado(prev => ({...prev, [campo]: valor}));
    };

    const handleGuardar = (e: React.FormEvent) => {
        e.preventDefault();
        /* TO-DO: Endpoint de actualización de perfil completo (display_name, bio, redes) */
        setGuardado(true);
        setTimeout(() => setGuardado(false), 3000);
    };

    const handleSubirAvatar = async (archivo: File) => {
        setSubiendoAvatar(true);
        try {
            const resp = await subirAvatar(archivo);
            queryClient.setQueryData<PerfilResponse | undefined>(
                currentProfileKey(userId),
                (prev) => prev ? {...prev, avatar_url: resp.avatar_url} : prev,
            );
        } finally {
            setSubiendoAvatar(false);
        }
    };

    return {
        estado, guardado, cargando: cargandoPerfil, perfil, avatarUrl,
        subiendoAvatar, actualizarCampo, handleGuardar, handleSubirAvatar
    };
};
