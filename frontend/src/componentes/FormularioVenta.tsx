/* 253A-7: Formulario para registrar una nueva venta
   253A-10: hook useFormularioVenta + componentes UI atomicos
   253A-14: acepta onExito para uso en modales, omite cabecera si se pasa */

import {Turno, CanalVenta, MetodoPago} from '../api/generated';
import useFormularioVenta from '../hooks/useFormularioVenta';
import {Input, Select, Boton} from './ui';
import '../estilos/Formularios.css';

interface Props {
    onExito?: () => void;
}

function FormularioVenta({ onExito }: Props) {
    const {campos, cambiarCampo, error, manejarEnvio, cargando} = useFormularioVenta(onExito);

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
                        <label className="etiquetaFormulario" htmlFor="fecha">
                            Fecha
                        </label>
                        <Input id="fecha" type="date" value={campos.fecha} onChange={e => cambiarCampo('fecha', e.target.value)} />
                    </div>
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="comensales">
                            Comensales
                        </label>
                        <Input id="comensales" type="number" min="1" value={campos.comensales} onChange={e => cambiarCampo('comensales', e.target.value)} placeholder="Opcional" />
                    </div>
                </div>

                <div className="grupoFormulario ancho">
                    <label className="etiquetaFormulario" htmlFor="descripcion">
                        Descripción
                    </label>
                    <Input id="descripcion" type="text" value={campos.descripcion} onChange={e => cambiarCampo('descripcion', e.target.value)} placeholder="Opcional" />
                </div>

                <div className="filaFormulario">
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="turno">
                            Turno
                        </label>
                        <Select id="turno" value={campos.turno} onChange={e => cambiarCampo('turno', e.target.value as Turno)}>
                            <option value={Turno.manana}>Mañana</option>
                            <option value={Turno.mediodia}>Mediodía</option>
                            <option value={Turno.noche}>Noche</option>
                        </Select>
                    </div>
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="canal">
                            Canal
                        </label>
                        <Select id="canal" value={campos.canal} onChange={e => cambiarCampo('canal', e.target.value as CanalVenta)}>
                            <option value={CanalVenta.comedor}>Comedor</option>
                            <option value={CanalVenta.barra}>Barra</option>
                            <option value={CanalVenta.terraza}>Terraza</option>
                            <option value={CanalVenta.delivery}>Delivery</option>
                            <option value={CanalVenta.just_eat}>Just Eat</option>
                            <option value={CanalVenta.eventos}>Eventos</option>
                        </Select>
                    </div>
                </div>

                <div className="filaFormulario">
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="metodoPago">
                            Método de pago
                        </label>
                        <Select id="metodoPago" value={campos.metodoPago} onChange={e => cambiarCampo('metodoPago', e.target.value as MetodoPago)}>
                            <option value={MetodoPago.efectivo}>Efectivo</option>
                            <option value={MetodoPago.tarjeta}>Tarjeta</option>
                            <option value={MetodoPago.transferencia}>Transferencia</option>
                        </Select>
                    </div>
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="ivaPorcentaje">
                            IVA %
                        </label>
                        <Input id="ivaPorcentaje" type="number" step="0.01" value={campos.ivaPorcentaje} onChange={e => cambiarCampo('ivaPorcentaje', e.target.value)} />
                    </div>
                </div>

                <div className="filaFormulario">
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="importeBase">
                            Importe base (€)
                        </label>
                        <Input id="importeBase" type="number" step="0.01" min="0" value={campos.importeBase} onChange={e => cambiarCampo('importeBase', e.target.value)} placeholder="0.00" />
                    </div>
                    <div className="grupoFormulario">
                        <label className="etiquetaFormulario" htmlFor="importeIva">
                            Importe IVA (€)
                        </label>
                        <Input id="importeIva" type="number" step="0.01" min="0" value={campos.importeIva} onChange={e => cambiarCampo('importeIva', e.target.value)} placeholder="Auto-calculado" />
                    </div>
                </div>

                <Boton variante="primario" ancho type="submit" cargando={cargando}>
                    Registrar Venta
                </Boton>
            </form>
        </div>
    );
}

export default FormularioVenta;
