/* 253A-7: Formulario para registrar una nueva venta
   253A-10: hook useFormularioVenta + componentes UI atómicos
   253A-14: acepta onExito para uso en modales, omite cabecera si se pasa
   253A-19: Refactorizado para turnos multi-select y detalles por turno.
   Cada turno seleccionado genera una venta independiente (el backend acepta 1 turno/venta). */

import { Turno, CanalVenta } from '../api/generated';
import useFormularioVenta, { calcularIva } from '../hooks/useFormularioVenta';
import { Input, Select, Boton } from '@glory/componentes/ui';
import '../estilos/Formularios.css';

const ETIQUETAS_TURNO: Record<Turno, string> = {
    [Turno.manana]: 'Mañana',
    [Turno.mediodia]: 'Mediodía',
    [Turno.noche]: 'Noche',
};

interface Props {
    onExito?: () => void;
}

function FormularioVenta({ onExito }: Props) {
    const { campos, cambiarCampo, toggleTurno, cambiarDetalle, error, manejarEnvio, cargando } = useFormularioVenta(onExito);

    return (
        <div className={onExito ? '' : 'formularioPagina'}>
            {!onExito && (
                <div className="cabeceraPagina">
                    <h1 className="tituloPagina">Nueva Venta</h1>
                    <p className="subtituloPagina">Registrar una venta del restaurante</p>
                </div>
            )}

            {error && <div className="errorFormulario">{error}</div>}

            <form className="formulario" onSubmit={manejarEnvio}>
                <div className="filaFormulario">
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="fecha">Fecha</label>
                        <Input id="fecha" type="date" value={campos.fecha} onChange={e => cambiarCampo('fecha', e.target.value)} />
                    </div>
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="comensales">Comensales</label>
                        <Input id="comensales" type="number" min="1" value={campos.comensales} onChange={e => cambiarCampo('comensales', e.target.value)} placeholder="Opcional" />
                    </div>
                </div>

                <div className="grupoFormulario ancho">
                    <label className="etiquetaFormulario" htmlFor="descripcion">Descripción</label>
                    <Input id="descripcion" type="text" value={campos.descripcion} onChange={e => cambiarCampo('descripcion', e.target.value)} placeholder="Opcional" />
                </div>

                <div className="grupoFormulario">
                    <label className="etiquetaFormulario">Turno(s)</label>
                    <div className="selectorTurnos">
                        {(Object.values(Turno) as Turno[]).map(t => (
                            <Boton
                                key={t}
                                type="button"
                                variante="fantasma"
                                tamano="sm"
                                className={`chipTurno${campos.turnos.includes(t) ? ' activo' : ''}`}
                                onClick={() => toggleTurno(t)}
                                aria-pressed={campos.turnos.includes(t)}
                            >
                                {ETIQUETAS_TURNO[t]}
                            </Boton>
                        ))}
                    </div>
                </div>

                <div className="grupoFormulario">
                    <label className="etiquetaFormulario" htmlFor="canal">Canal</label>
                    <Select id="canal" value={campos.canal} onChange={e => cambiarCampo('canal', e.target.value as CanalVenta)}>
                        <option value={CanalVenta.comedor}>Comedor</option>
                        <option value={CanalVenta.barra}>Barra</option>
                        <option value={CanalVenta.terraza}>Terraza</option>
                        <option value={CanalVenta.delivery}>Delivery</option>
                        <option value={CanalVenta.just_eat}>Just Eat</option>
                        <option value={CanalVenta.eventos}>Eventos</option>
                    </Select>
                </div>

                {campos.turnos.map(t => {
                    const d = campos.detalles[t];
                    const iva = calcularIva(d.importeBase, campos.ivaPorcentaje);
                    const total = d.importeBase ? (parseFloat(d.importeBase) + parseFloat(iva)).toFixed(2) : '';
                    return (
                        <div key={t} className="detalleTurno">
                            <span className="etiquetaDetalleTurno">{ETIQUETAS_TURNO[t]}</span>
                            <div className="filaFormulario">
                                <div className="grupoFormulario">
                                    <label className="etiquetaFormulario" htmlFor={`importe-${t}`}>Importe base (€)</label>
                                    <Input id={`importe-${t}`} type="number" step="0.01" min="0" value={d.importeBase} onChange={e => cambiarDetalle(t, 'importeBase', e.target.value)} placeholder="0.00" />
                                </div>
                                <div className="grupoFormulario">
                                    <label className="etiquetaFormulario">IVA + Total</label>
                                    <div className="totalCalculado">{d.importeBase ? `+${iva}€ IVA = ${total}€` : '—'}</div>
                                </div>
                            </div>
                            <div className="grupoFormulario">
                                <label className="etiquetaFormulario" htmlFor={`pago-${t}`}>Método de pago</label>
                                <Select id={`pago-${t}`} value={d.metodoPago} onChange={e => cambiarDetalle(t, 'metodoPago', e.target.value)}>
                                    <option value="efectivo">Efectivo</option>
                                    <option value="tarjeta">Tarjeta</option>
                                    <option value="transferencia">Transferencia</option>
                                </Select>
                            </div>
                        </div>
                    );
                })}

                <details className="opcionesAvanzadas">
                    <summary className="resumenAvanzado">Opciones avanzadas</summary>
                    <div className="contenidoAvanzado">
                        <div className="filaFormulario">
                            <div className="grupoFormulario">
                                <label className="etiquetaFormulario" htmlFor="ivaPorcentaje">IVA %</label>
                                <Input id="ivaPorcentaje" type="number" step="0.01" value={campos.ivaPorcentaje} onChange={e => cambiarCampo('ivaPorcentaje', e.target.value)} />
                            </div>
                            <div className="grupoFormulario checkboxAvanzado">
                                <Input id="duplicados" type="checkbox" checked={campos.permitirDuplicados} onChange={e => cambiarCampo('permitirDuplicados', e.target.checked)} />
                                <label htmlFor="duplicados" className="etiquetaFormulario">Permitir duplicados</label>
                            </div>
                        </div>
                    </div>
                </details>

                <Boton variante="primario" ancho type="submit" cargando={cargando}>
                    Registrar {campos.turnos.length > 1 ? `${campos.turnos.length} ventas` : 'venta'}
                </Boton>
            </form>
        </div>
    );
}

export default FormularioVenta;

