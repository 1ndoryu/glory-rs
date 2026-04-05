/* [044A-43] Hook de perfil: carga perfil desde backend, sube avatar.
 * Reemplaza la versión stub que usaba obtenerUsuarioActual (siempre null). */
import {useState, useEffect, useRef, useCallback} from 'react';
import {obtenerPerfil, subirAvatar} from '../api/profile';
import type {PerfilResponse} from '../api/profile';

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
    const [perfil, setPerfil] = useState<PerfilResponse | null>(null);
    const [cargando, setCargando] = useState(true);
    const [subiendoAvatar, setSubiendoAvatar] = useState(false);
    const [avatarUrl, setAvatarUrl] = useState('https://i.pravatar.cc/100?u=default');

    const [estado, setEstado] = useState<EstadoPerfil>({
        nombre: '',
        descripcion: '',
        linkedin: '',
        twitter: '',
        website: ''
    });

    const [guardado, setGuardado] = useState(false);
    const abortRef = useRef<AbortController | null>(null);

    /* Cargar perfil del backend al montar */
    useEffect(() => {
        const controller = new AbortController();
        abortRef.current = controller;

        obtenerPerfil()
            .then((data) => {
                if (!controller.signal.aborted) {
                    setPerfil(data);
                    setAvatarUrl(data.avatar_url || 'https://i.pravatar.cc/100?u=default');
                    setEstado(prev => ({
                        ...prev,
                        nombre: data.display_name || ''
                    }));
                    setCargando(false);
                }
            })
            .catch(() => {
                if (!controller.signal.aborted) setCargando(false);
            });

        return () => { controller.abort(); };
    }, []);

    const actualizarCampo = (campo: keyof EstadoPerfil, valor: string) => {
        setEstado(prev => ({...prev, [campo]: valor}));
    };

    const handleGuardar = (e: React.FormEvent) => {
        e.preventDefault();
        /* TO-DO: Endpoint de actualización de perfil completo (display_name, bio, redes) */
        setGuardado(true);
        setTimeout(() => setGuardado(false), 3000);
    };

    const handleSubirAvatar = useCallback(async (archivo: File) => {
        setSubiendoAvatar(true);
        try {
            const resp = await subirAvatar(archivo);
            setAvatarUrl(resp.avatar_url);
            setPerfil(prev => prev ? {...prev, avatar_url: resp.avatar_url} : prev);
        } finally {
            setSubiendoAvatar(false);
        }
    }, []);

    return {
        estado, guardado, cargando, perfil, avatarUrl,
        subiendoAvatar, actualizarCampo, handleGuardar, handleSubirAvatar
    };
};
