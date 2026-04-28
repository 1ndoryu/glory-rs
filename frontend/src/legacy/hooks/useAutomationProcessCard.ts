/* [284A-2] Hook de estado para la tarjeta de automatizacion admin.
 * Por que: la tarjeta solo debe renderizar JSX; la logica de edicion,
 * calculo de proximas corridas y validacion local queda aislada aqui. */

import { type FormEvent, useEffect, useState } from 'react';
import type { AutomatizacionConfigProceso, LoteResumen, TipoProceso } from '../services/apiAutomatizacion';

interface UseAutomationProcessCardParams {
    tipo: TipoProceso;
    activo: boolean;
    limiteLote: number;
    intervaloSegundos: number;
    ultimoLote: LoteResumen | null;
    onGuardarConfig: (config: Required<AutomatizacionConfigProceso>) => Promise<unknown>;
}

const formatearFecha = (fecha: string | null): string => {
    if (!fecha) return '-';
    const d = new Date(fecha);
    return d.toLocaleDateString('es', { day: '2-digit', month: 'short' }) +
        ' ' + d.toLocaleTimeString('es', { hour: '2-digit', minute: '2-digit' });
};

const formatearIntervalo = (segundos: number): string => {
    if (segundos >= 3600 && segundos % 3600 === 0) {
        const horas = segundos / 3600;
        return horas === 1 ? 'Cada hora' : `Cada ${horas} h`;
    }
    if (segundos >= 60 && segundos % 60 === 0) {
        const minutos = segundos / 60;
        return minutos === 1 ? 'Cada minuto' : `Cada ${minutos} min`;
    }
    return `Cada ${segundos} s`;
};

const formatearProximaEjecucion = (
    activo: boolean,
    intervaloSegundos: number,
    ultimoLote: LoteResumen | null
): string => {
    if (!activo) return 'Automatización pausada';
    if (!ultimoLote) return 'Pendiente de primer lote';
    if (ultimoLote.estado === 'ejecutando') return 'Al terminar el lote actual';

    const marcaBase = ultimoLote.completado_at ?? ultimoLote.iniciado_at;
    if (!marcaBase) return 'Pendiente de próximo ciclo';

    const proximaFecha = new Date(new Date(marcaBase).getTime() + (intervaloSegundos * 1000));
    if (Number.isNaN(proximaFecha.getTime())) return 'Pendiente de próximo ciclo';
    if (proximaFecha.getTime() <= Date.now()) return 'Disponible para ejecutar ahora';
    return formatearFecha(proximaFecha.toISOString());
};

export function useAutomationProcessCard({
    tipo,
    activo,
    limiteLote,
    intervaloSegundos,
    ultimoLote,
    onGuardarConfig,
}: UseAutomationProcessCardParams) {
    const [enabled, setEnabled] = useState(activo);
    const [lote, setLote] = useState(String(limiteLote));
    const [intervalo, setIntervalo] = useState(String(intervaloSegundos));

    useEffect(() => {
        setEnabled(activo);
        setLote(String(limiteLote));
        setIntervalo(String(intervaloSegundos));
    }, [activo, limiteLote, intervaloSegundos]);

    const limiteMaximo = tipo === 'extraccion' ? 500 : 200;
    const intervaloMinimo = tipo === 'extraccion' ? 5 : 30;
    const loteActual = Number.parseInt(lote, 10);
    const intervaloActual = Number.parseInt(intervalo, 10);
    const configInvalida = Number.isNaN(loteActual) || Number.isNaN(intervaloActual);
    const hayCambios = enabled !== activo || loteActual !== limiteLote || intervaloActual !== intervaloSegundos;

    const manejarSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        await onGuardarConfig({
            enabled,
            lote_size: loteActual,
            intervalo_seg: intervaloActual,
        });
    };

    return {
        enabled,
        lote,
        intervalo,
        limiteMaximo,
        intervaloMinimo,
        loteActual,
        intervaloActual,
        configInvalida,
        hayCambios,
        intervaloLegible: formatearIntervalo(intervaloSegundos),
        ultimoLoteLegible: formatearFecha(ultimoLote?.iniciado_at ?? null),
        proximaEjecucionLegible: formatearProximaEjecucion(enabled, intervaloActual, ultimoLote),
        setEnabled,
        setLote,
        setIntervalo,
        manejarSubmit,
    };
}