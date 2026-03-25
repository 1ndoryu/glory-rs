/* 253A-7: Formulario para crear una nueva reserva
   253A-10: hook useFormularioReserva + componentes UI atómicos */

import { EstadoReserva } from '../api/generated';
import useFormularioReserva from '../hooks/useFormularioReserva';
import { Input, Select, Textarea, Boton } from './ui';
import '../estilos/Formularios.css';

function FormularioReserva() {
  const { campos, cambiarCampo, error, manejarEnvio, cargando } = useFormularioReserva();

  return (
    <div className="formularioPagina">
      <div className="cabeceraPagina">
        <h1 className="tituloPagina">Nueva Reserva</h1>
        <p className="subtituloPagina">Registrar una reserva</p>
      </div>

      {error && <div className="errorFormulario">{error}</div>}

      <form className="formulario" onSubmit={manejarEnvio}>
        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="fecha">Fecha</label>
            <Input id="fecha" type="date" value={campos.fecha} onChange={(e) => cambiarCampo('fecha', e.target.value)} />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="hora">Hora</label>
            <Input id="hora" type="time" value={campos.hora} onChange={(e) => cambiarCampo('hora', e.target.value)} />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="nombreCliente">Nombre del cliente</label>
            <Input id="nombreCliente" type="text" value={campos.nombreCliente} onChange={(e) => cambiarCampo('nombreCliente', e.target.value)} placeholder="Nombre completo" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="numPersonas">Personas</label>
            <Input id="numPersonas" type="number" min="1" value={campos.numPersonas} onChange={(e) => cambiarCampo('numPersonas', e.target.value)} />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="telefono">Teléfono</label>
            <Input id="telefono" type="tel" value={campos.telefono} onChange={(e) => cambiarCampo('telefono', e.target.value)} placeholder="Opcional" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="estado">Estado</label>
            <Select id="estado" value={campos.estado} onChange={(e) => cambiarCampo('estado', e.target.value as EstadoReserva)}>
              <option value={EstadoReserva.pendiente}>Pendiente</option>
              <option value={EstadoReserva.confirmada}>Confirmada</option>
              <option value={EstadoReserva.cancelada}>Cancelada</option>
            </Select>
          </div>
        </div>

        <div className="grupoFormulario ancho">
          <label className="etiquetaFormulario" htmlFor="notas">Notas</label>
          <Textarea id="notas" rows={3} value={campos.notas} onChange={(e) => cambiarCampo('notas', e.target.value)} placeholder="Alergias, preferencias, etc." />
        </div>

        <Boton className="botonEnviar" type="submit" disabled={cargando}>
          {cargando ? 'Guardando...' : 'Crear Reserva'}
        </Boton>
      </form>
    </div>
  );
}

export default FormularioReserva;
