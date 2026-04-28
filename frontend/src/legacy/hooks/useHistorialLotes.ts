/* sentinel-disable-file directory-size — hook legacy existente; 284A-1 agrega guardado de config sin mover el arbol completo de hooks. */
/*
 * Hook: useHistorialLotes — [223A-3] Estado y gestión del historial de lotes automáticos.
 * Carga estado de automatización + historial paginado + reactivación.
 */

import { useState, useEffect, useCallback } from 'react';
import {
    actualizarConfigProceso,
    obtenerEstadoAutomatizacion,
    obtenerHistorialLotes,
    reactivarProceso,
} from '../services/apiAutomatizacion';
import type {
    AutomatizacionConfigProceso,
    EstadoAutomatizacion,
    LoteResumen,
    TipoProceso,
} from '../services/apiAutomatizacion';
import { toast } from '../stores/toastStore';

export function useHistorialLotes() {
    const [estado, setEstado] = useState<EstadoAutomatizacion | null>(null);
    const [lotes, setLotes] = useState<LoteResumen[]>([]);
    const [total, setTotal] = useState(0);
    const [pagina, setPagina] = useState(1);
    const [filtroTipo, setFiltroTipo] = useState<TipoProceso | ''>('');
    const [cargando, setCargando] = useState(false);
    const [guardandoConfig, setGuardandoConfig] = useState<TipoProceso | null>(null);
    const [error, setError] = useState('');

    const cargarEstado = useCallback(async () => {
        try {
            const resp = await obtenerEstadoAutomatizacion();
            if (resp.ok && resp.data?.estado) {
                setEstado(resp.data.estado);
            }
        } catch {
            /* silencioso — estado inicial null */
        }
    }, []);

    const cargarHistorial = useCallback(async () => {
        setCargando(true);
        setError('');
        try {
            const tipo = filtroTipo || undefined;
            const resp = await obtenerHistorialLotes(tipo, pagina);
            if (resp.ok && resp.data) {
                setLotes(resp.data.items);
                setTotal(resp.data.total);
            } else {
                setError(resp.error ?? 'Error cargando historial');
            }
        } catch {
            setError('Error de conexión');
        } finally {
            setCargando(false);
        }
    }, [filtroTipo, pagina]);

    const manejarReactivar = useCallback(async (tipo: TipoProceso) => {
        try {
            const resp = await reactivarProceso(tipo);
            if (resp.ok) {
                await cargarEstado();
                toast.exito(resp.data?.mensaje ?? 'Proceso reactivado');
            } else {
                toast.error(resp.error ?? 'No se pudo reactivar el proceso');
            }
            return resp;
        } catch {
            toast.error('Error de conexión');
            return { ok: false, error: 'Error de conexión' };
        }
    }, [cargarEstado]);

    const guardarConfig = useCallback(async (
        tipo: TipoProceso,
        config: Required<AutomatizacionConfigProceso>
    ) => {
        setGuardandoConfig(tipo);
        try {
            const resp = await actualizarConfigProceso(tipo, config);
            if (resp.ok) {
                await cargarEstado();
                toast.exito('Configuración actualizada');
            } else {
                toast.error(resp.error ?? 'No se pudo actualizar la configuración');
            }
            return resp;
        } catch {
            toast.error('Error de conexión');
            return { ok: false, error: 'Error de conexión' };
        } finally {
            setGuardandoConfig(null);
        }
    }, [cargarEstado]);

    useEffect(() => {
        cargarEstado();
        cargarHistorial();
    }, [cargarEstado, cargarHistorial]);

    return {
        estado,
        lotes,
        total,
        pagina,
        setPagina,
        filtroTipo,
        setFiltroTipo,
        cargando,
        guardandoConfig,
        error,
        refrescar: cargarHistorial,
        refrescarEstado: cargarEstado,
        reactivar: manejarReactivar,
        guardarConfig,
    };
}
