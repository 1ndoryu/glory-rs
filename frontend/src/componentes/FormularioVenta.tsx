/* 253A-7: Formulario para registrar una nueva venta */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearVenta, Turno, CanalVenta, MetodoPago } from '../api/generated';
import '../estilos/Formularios.css';

function FormularioVenta() {
  const navigate = useNavigate();
  const [error, setError] = useState('');

  const hoy = new Date().toISOString().split('T')[0];
  const [fecha, setFecha] = useState(hoy);
  const [comensales, setComensales] = useState('');
  const [descripcion, setDescripcion] = useState('');
  const [ivaPorcentaje, setIvaPorcentaje] = useState('10');
  const [turno, setTurno] = useState<Turno>(Turno.mediodia);
  const [canal, setCanal] = useState<CanalVenta>(CanalVenta.comedor);
  const [metodoPago, setMetodoPago] = useState<MetodoPago>(MetodoPago.efectivo);
  const [importeBase, setImporteBase] = useState('');
  const [importeIva, setImporteIva] = useState('');

  /* Calcula IVA automáticamente al cambiar importe_base o porcentaje */
  const calcularIva = (base: string, porcentaje: string) => {
    const b = parseFloat(base);
    const p = parseFloat(porcentaje);
    if (!isNaN(b) && !isNaN(p)) {
      setImporteIva((b * p / 100).toFixed(2));
    }
  };

  const mutation = useCrearVenta({
    mutation: {
      onSuccess: (res) => {
        if (res.status === 201) {
          navigate('/ventas');
        }
      },
      onError: () => {
        setError('Error al crear la venta');
      },
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');

    if (!fecha || !importeBase || !importeIva) {
      setError('Completa los campos obligatorios');
      return;
    }

    mutation.mutate({
      data: {
        fecha,
        comensales: comensales ? parseInt(comensales, 10) : null,
        descripcion: descripcion || null,
        iva_porcentaje: ivaPorcentaje,
        turno,
        canal,
        metodo_pago: metodoPago,
        importe_base: importeBase,
        importe_iva: importeIva,
      },
    });
  };

  return (
    <div className="formularioPagina">
      <div className="cabeceraPagina">
        <h1 className="tituloPagina">Nueva Venta</h1>
        <p className="subtituloPagina">Registrar una venta del restaurante</p>
      </div>

      {error && <div className="errorFormulario">{error}</div>}

      <form className="formulario" onSubmit={manejarEnvio}>
        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="fecha">Fecha</label>
            <input id="fecha" type="date" value={fecha} onChange={(e) => setFecha(e.target.value)} />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="comensales">Comensales</label>
            <input id="comensales" type="number" min="1" value={comensales} onChange={(e) => setComensales(e.target.value)} placeholder="Opcional" />
          </div>
        </div>

        <div className="grupoFormulario ancho">
          <label className="etiquetaFormulario" htmlFor="descripcion">Descripción</label>
          <input id="descripcion" type="text" value={descripcion} onChange={(e) => setDescripcion(e.target.value)} placeholder="Opcional" />
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="turno">Turno</label>
            <select id="turno" value={turno} onChange={(e) => setTurno(e.target.value as Turno)}>
              <option value={Turno.manana}>Mañana</option>
              <option value={Turno.mediodia}>Mediodía</option>
              <option value={Turno.noche}>Noche</option>
            </select>
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="canal">Canal</label>
            <select id="canal" value={canal} onChange={(e) => setCanal(e.target.value as CanalVenta)}>
              <option value={CanalVenta.comedor}>Comedor</option>
              <option value={CanalVenta.barra}>Barra</option>
              <option value={CanalVenta.terraza}>Terraza</option>
              <option value={CanalVenta.delivery}>Delivery</option>
              <option value={CanalVenta.just_eat}>Just Eat</option>
              <option value={CanalVenta.eventos}>Eventos</option>
            </select>
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="metodoPago">Método de pago</label>
            <select id="metodoPago" value={metodoPago} onChange={(e) => setMetodoPago(e.target.value as MetodoPago)}>
              <option value={MetodoPago.efectivo}>Efectivo</option>
              <option value={MetodoPago.tarjeta}>Tarjeta</option>
              <option value={MetodoPago.transferencia}>Transferencia</option>
            </select>
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="ivaPorcentaje">IVA %</label>
            <input id="ivaPorcentaje" type="number" step="0.01" value={ivaPorcentaje} onChange={(e) => { setIvaPorcentaje(e.target.value); calcularIva(importeBase, e.target.value); }} />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="importeBase">Importe base (€)</label>
            <input id="importeBase" type="number" step="0.01" min="0" value={importeBase} onChange={(e) => { setImporteBase(e.target.value); calcularIva(e.target.value, ivaPorcentaje); }} placeholder="0.00" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="importeIva">Importe IVA (€)</label>
            <input id="importeIva" type="number" step="0.01" min="0" value={importeIva} onChange={(e) => setImporteIva(e.target.value)} placeholder="Auto-calculado" />
          </div>
        </div>

        <button className="botonEnviar" type="submit" disabled={mutation.isPending}>
          {mutation.isPending ? 'Guardando...' : 'Registrar Venta'}
        </button>
      </form>
    </div>
  );
}

export default FormularioVenta;
