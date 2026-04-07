/* [074A-13] Hook CRUD para miembros del equipo en el panel admin.
 * Patrón idéntico a useContenidoProyectos. */

import {useState, useEffect, useCallback} from 'react';
import {
    AdminTeamMember,
    CreateTeamMemberBody,
    UpdateTeamMemberBody,
    apiListAdminTeamMembers,
    apiCreateTeamMember,
    apiUpdateTeamMember,
    apiArchiveTeamMember
} from '../api/admin-team';

export function useContenidoEquipo() {
    const [miembros, setMiembros] = useState<AdminTeamMember[]>([]);
    const [cargando, setCargando] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [guardando, setGuardando] = useState(false);

    const cargar = useCallback(async () => {
        setCargando(true);
        setError(null);
        try {
            const data = await apiListAdminTeamMembers();
            setMiembros(data);
        } catch (e) {
            setError(e instanceof Error ? e.message : 'Error al cargar miembros');
        } finally {
            setCargando(false);
        }
    }, []);

    useEffect(() => {
        const controller = new AbortController();
        cargar();
        return () => controller.abort();
    }, [cargar]);

    const crear = useCallback(async (body: CreateTeamMemberBody) => {
        setGuardando(true);
        try {
            await apiCreateTeamMember(body);
            await cargar();
        } finally {
            setGuardando(false);
        }
    }, [cargar]);

    const actualizar = useCallback(async (id: string, body: UpdateTeamMemberBody) => {
        setGuardando(true);
        try {
            await apiUpdateTeamMember(id, body);
            await cargar();
        } finally {
            setGuardando(false);
        }
    }, [cargar]);

    const archivar = useCallback(async (id: string) => {
        setGuardando(true);
        try {
            await apiArchiveTeamMember(id);
            await cargar();
        } finally {
            setGuardando(false);
        }
    }, [cargar]);

    return {miembros, cargando, error, guardando, crear, actualizar, archivar, recargar: cargar};
}
