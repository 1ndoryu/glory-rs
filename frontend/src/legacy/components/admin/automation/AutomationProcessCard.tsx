/* [284A-1] Tarjeta editable de automatizacion admin.
 * Por que: el estado "activo" solo refleja app_config; el admin necesita
 * mutar enabled/lote/intervalo sin ir a herramientas externas. */

import { type FormEvent, useEffect, useState } from 'react';
import { AlertTriangle, CheckCircle, Loader2, PauseCircle, Play, Save } from 'lucide-react';
import { Badge } from '../../ui/Badge';
import { BotonBase } from '../../ui/BotonBase';
import { Input } from '../../ui/Input';
import type { AutomatizacionConfigProceso, TipoProceso } from '../../../services/apiAutomatizacion';

interface AutomationProcessCardProps {
    titulo: string;
    tipo: TipoProceso;
    activo: boolean;
    limiteLote: number;
    intervaloSegundos: number;
    ultimoLote: { exitosos?: number; fallidos?: number; iniciado_at?: string } | null;
    fallosConsecutivos?: number;
    reactivando: boolean;
    guardando: boolean;
    onReactivar: () => void;
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

export const AutomationProcessCard = ({
    titulo,
    tipo,
    activo,
    limiteLote,
    intervaloSegundos,
    ultimoLote,
    fallosConsecutivos,
    reactivando,
    guardando,
    onReactivar,
    onGuardarConfig,
}: AutomationProcessCardProps): JSX.Element => {
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

    const manejarSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        await onGuardarConfig({
            enabled,
            lote_size: Number.parseInt(lote, 10),
            intervalo_seg: Number.parseInt(intervalo, 10),
        });
    };

    return (
        <form className="tarjetaEstadoProceso" onSubmit={manejarSubmit}>
            <div className="tarjetaEstadoEncabezado">
                {activo
                    ? <CheckCircle size={14} className="iconoEstadoProceso iconoEstadoProcesoExito" />
                    : <PauseCircle size={14} className="iconoEstadoProceso iconoEstadoProcesoError" />}
                <strong>{titulo}</strong>
                <Badge variante={activo ? 'exito' : 'error'}>{activo ? 'Habilitado' : 'Deshabilitado'}</Badge>
            </div>

            <div className="tarjetaEstadoInfo">
                {limiteLote} items/lote · {formatearIntervalo(intervaloSegundos)}
                {fallosConsecutivos !== undefined && fallosConsecutivos > 0 && (
                    <span> · <AlertTriangle size={12} className="iconoAlertaLote" /> {fallosConsecutivos} fallos consecutivos</span>
                )}
            </div>

            {ultimoLote ? (
                <div className="tarjetaEstadoUltimo">
                    Último: {ultimoLote.exitosos ?? 0} ok / {ultimoLote.fallidos ?? 0} err - {formatearFecha(ultimoLote.iniciado_at ?? null)}
                </div>
            ) : (
                <div className="tarjetaEstadoUltimo">Último: sin registros</div>
            )}

            <div className="tarjetaEstadoControles">
                <BotonBase
                    type="button"
                    variante="ghost"
                    tamano="ninguno"
                    className={`interruptorProceso ${enabled ? 'interruptorProcesoActivo' : ''}`}
                    role="switch"
                    aria-checked={enabled}
                    onClick={() => setEnabled(v => !v)}
                    disabled={guardando}
                >
                    <span className="interruptorProcesoPunto" />
                    <span>{enabled ? 'Habilitado' : 'Deshabilitado'}</span>
                </BotonBase>
                <label className="campoConfigProceso">
                    <span>Items/lote</span>
                    <Input
                        type="number"
                        min={1}
                        max={limiteMaximo}
                        value={lote}
                        onChange={(event) => setLote(event.target.value)}
                        required
                    />
                </label>
                <label className="campoConfigProceso">
                    <span>Intervalo (s)</span>
                    <Input
                        type="number"
                        min={intervaloMinimo}
                        max={86400}
                        value={intervalo}
                        onChange={(event) => setIntervalo(event.target.value)}
                        required
                    />
                </label>
                <BotonBase
                    variante="secundario"
                    tamano="sm"
                    type="submit"
                    disabled={guardando}
                    className="tarjetaEstadoGuardar"
                >
                    {guardando ? <Loader2 size={14} className="animacionGiro" /> : <Save size={14} />}
                    Guardar
                </BotonBase>
            </div>

            {!activo && (
                <BotonBase
                    variante="primario"
                    tamano="sm"
                    type="button"
                    onClick={onReactivar}
                    disabled={reactivando || guardando}
                    className="tarjetaEstadoReactivar"
                >
                    {reactivando ? <Loader2 size={14} className="animacionGiro" /> : <Play size={14} />}
                    Reactivar
                </BotonBase>
            )}
        </form>
    );
};