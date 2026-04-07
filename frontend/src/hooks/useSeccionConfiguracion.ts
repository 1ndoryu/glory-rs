/* [084A-11] Hook para SeccionConfiguracion — extraído para cumplir max 3 useState.
 * Gestiona estado de carga, resultado, error y confirmación de acciones seed. */
import { useState, useCallback } from 'react';
import { apiRecreateSeed, apiDeleteSeed } from '../api/admin-seed';

type AccionSeed = 'recrear' | 'borrar' | null;

export function useSeccionConfiguracion() {
    const [cargando, setCargando] = useState(false);
    const [resultado, setResultado] = useState<string | null>(null);
    const [errorMsg, setErrorMsg] = useState<string | null>(null);
    const [confirmando, setConfirmando] = useState<AccionSeed>(null);

    const ejecutarSeed = useCallback(async (accion: AccionSeed) => {
        if (!accion) return;
        setConfirmando(null);
        setCargando(true);
        setResultado(null);
        setErrorMsg(null);
        try {
            const resp = accion === 'recrear'
                ? await apiRecreateSeed()
                : await apiDeleteSeed();
            setResultado(resp.message);
        } catch (err: unknown) {
            const msg = err instanceof Error ? err.message : 'Error desconocido';
            setErrorMsg(msg);
        } finally {
            setCargando(false);
        }
    }, []);

    return { cargando, resultado, errorMsg, confirmando, setConfirmando, ejecutarSeed };
}
