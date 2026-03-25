/* 253A-7: Formulario para registrar un nuevo gasto
   253A-10: hook useFormularioGasto + componentes UI atomicos
   253A-14: acepta onExito para uso en modales */

import { MetodoPago, TipoDocumento } from '../api/generated';
import useFormularioGasto from '../hooks/useFormularioGasto';
import { Input, Select, Boton } from './ui';
import '../estilos/Formularios.css';

interface Props {
  onExito?: () => void;
}

function FormularioGasto({ onExito }: Props) {
  const { campos, cambiarCampo, error, manejarEnvio, cargando, categorias } = useFormularioGasto(onExito);

  return (
    <div className={onExito ? '' : 'formularioPagina'}>
      {!onExito && (
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Nuevo Gasto</h1>
          <p className="subtituloPagina">Registrar un gasto del restaurante</p>
        </div>
      )}

      {error && <div className="errorFormulario">{error}</div>}

      <form className="formulario" onSubmit={manejarEnvio}>
        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="fecha">Fecha</label>
            <Input id="fecha" type="date" value={campos.fecha} onChange={(e) => cambiarCampo('fecha', e.target.value)} />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="proveedor">Proveedor</label>
            <Input id="proveedor" type="text" value={campos.proveedor} onChange={(e) => cambiarCampo('proveedor', e.target.value)} placeholder="Opcional" />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="categoria">Categoría</label>
            <Select id="categoria" value={campos.categoriaId} onChange={(e) => cambiarCampo('categoriaId', e.target.value)}>
              <option value="">Sin categoría</option>
              {categorias.map((cat) => (
                <option key={cat.id} value={cat.id}>{cat.nombre}</option>
              ))}
            </Select>
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="tipoDocumento">Tipo documento</label>
            <Select id="tipoDocumento" value={campos.tipoDocumento} onChange={(e) => cambiarCampo('tipoDocumento', e.target.value as TipoDocumento)}>
              <option value={TipoDocumento.factura}>Factura</option>
              <option value={TipoDocumento.albaran}>Albarán</option>
              <option value={TipoDocumento.ticket}>Ticket</option>
            </Select>
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="metodoPago">Método de pago</label>
            <Select id="metodoPago" value={campos.metodoPago} onChange={(e) => cambiarCampo('metodoPago', e.target.value as MetodoPago)}>
              <option value={MetodoPago.efectivo}>Efectivo</option>
              <option value={MetodoPago.tarjeta}>Tarjeta</option>
              <option value={MetodoPago.transferencia}>Transferencia</option>
            </Select>
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="numeroDocumento">Nº documento</label>
            <Input id="numeroDocumento" type="text" value={campos.numeroDocumento} onChange={(e) => cambiarCampo('numeroDocumento', e.target.value)} placeholder="Opcional" />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="importeBase">Importe base (€)</label>
            <Input id="importeBase" type="number" step="0.01" min="0" value={campos.importeBase} onChange={(e) => cambiarCampo('importeBase', e.target.value)} placeholder="0.00" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="importeIva">Importe IVA (€)</label>
            <Input id="importeIva" type="number" step="0.01" min="0" value={campos.importeIva} onChange={(e) => cambiarCampo('importeIva', e.target.value)} placeholder="0.00" />
          </div>
        </div>

        <div className="checkboxFormulario">
          <Input id="recurrente" type="checkbox" checked={campos.recurrente} onChange={(e) => cambiarCampo('recurrente', e.target.checked)} />
          <label htmlFor="recurrente">Gasto recurrente</label>
        </div>

        <Boton variante="primario" ancho type="submit" cargando={cargando}>
          Registrar Gasto
        </Boton>
      </form>
    </div>
  );
}

export default FormularioGasto;
