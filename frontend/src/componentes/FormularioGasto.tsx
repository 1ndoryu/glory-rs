/* 253A-7: Formulario para registrar un nuevo gasto */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearGasto, useListarCategorias, MetodoPago, TipoDocumento } from '../api/generated';
import '../estilos/Formularios.css';

function FormularioGasto() {
  const navigate = useNavigate();
  const [error, setError] = useState('');

  const hoy = new Date().toISOString().split('T')[0];
  const [fecha, setFecha] = useState(hoy);
  const [proveedor, setProveedor] = useState('');
  const [categoriaId, setCategoriaId] = useState('');
  const [tipoDocumento, setTipoDocumento] = useState<TipoDocumento>(TipoDocumento.factura);
  const [metodoPago, setMetodoPago] = useState<MetodoPago>(MetodoPago.efectivo);
  const [numeroDocumento, setNumeroDocumento] = useState('');
  const [recurrente, setRecurrente] = useState(false);
  const [importeBase, setImporteBase] = useState('');
  const [importeIva, setImporteIva] = useState('');

  const { data: categoriasResp } = useListarCategorias();
  const categorias = categoriasResp?.status === 200 ? categoriasResp.data : [];

  const mutation = useCrearGasto({
    mutation: {
      onSuccess: (res) => {
        if (res.status === 201) {
          navigate('/gastos');
        }
      },
      onError: () => {
        setError('Error al crear el gasto');
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
        proveedor: proveedor || null,
        categoria_id: categoriaId || null,
        tipo_documento: tipoDocumento,
        metodo_pago: metodoPago,
        numero_documento: numeroDocumento || null,
        recurrente: recurrente || null,
        importe_base: importeBase,
        importe_iva: importeIva,
      },
    });
  };

  return (
    <div className="formularioPagina">
      <div className="cabeceraPagina">
        <h1 className="tituloPagina">Nuevo Gasto</h1>
        <p className="subtituloPagina">Registrar un gasto del restaurante</p>
      </div>

      {error && <div className="errorFormulario">{error}</div>}

      <form className="formulario" onSubmit={manejarEnvio}>
        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="fecha">Fecha</label>
            <input id="fecha" type="date" value={fecha} onChange={(e) => setFecha(e.target.value)} />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="proveedor">Proveedor</label>
            <input id="proveedor" type="text" value={proveedor} onChange={(e) => setProveedor(e.target.value)} placeholder="Opcional" />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="categoria">Categoría</label>
            <select id="categoria" value={categoriaId} onChange={(e) => setCategoriaId(e.target.value)}>
              <option value="">Sin categoría</option>
              {categorias.map((cat) => (
                <option key={cat.id} value={cat.id}>{cat.nombre}</option>
              ))}
            </select>
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="tipoDocumento">Tipo documento</label>
            <select id="tipoDocumento" value={tipoDocumento} onChange={(e) => setTipoDocumento(e.target.value as TipoDocumento)}>
              <option value={TipoDocumento.factura}>Factura</option>
              <option value={TipoDocumento.albaran}>Albarán</option>
              <option value={TipoDocumento.ticket}>Ticket</option>
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
            <label className="etiquetaFormulario" htmlFor="numeroDocumento">Nº documento</label>
            <input id="numeroDocumento" type="text" value={numeroDocumento} onChange={(e) => setNumeroDocumento(e.target.value)} placeholder="Opcional" />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="importeBase">Importe base (€)</label>
            <input id="importeBase" type="number" step="0.01" min="0" value={importeBase} onChange={(e) => setImporteBase(e.target.value)} placeholder="0.00" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="importeIva">Importe IVA (€)</label>
            <input id="importeIva" type="number" step="0.01" min="0" value={importeIva} onChange={(e) => setImporteIva(e.target.value)} placeholder="0.00" />
          </div>
        </div>

        <div className="checkboxFormulario">
          <input id="recurrente" type="checkbox" checked={recurrente} onChange={(e) => setRecurrente(e.target.checked)} />
          <label htmlFor="recurrente">Gasto recurrente</label>
        </div>

        <button className="botonEnviar" type="submit" disabled={mutation.isPending}>
          {mutation.isPending ? 'Guardando...' : 'Registrar Gasto'}
        </button>
      </form>
    </div>
  );
}

export default FormularioGasto;
