/* 253A-7: Formulario para crear una nueva reserva */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearReserva, EstadoReserva } from '../api/generated';
import '../estilos/Formularios.css';

function FormularioReserva() {
  const navigate = useNavigate();
  const [error, setError] = useState('');

  const hoy = new Date().toISOString().split('T')[0];
  const [fecha, setFecha] = useState(hoy);
  const [hora, setHora] = useState('20:00');
  const [nombreCliente, setNombreCliente] = useState('');
  const [numPersonas, setNumPersonas] = useState('2');
  const [telefono, setTelefono] = useState('');
  const [notas, setNotas] = useState('');
  const [estado, setEstado] = useState<EstadoReserva>(EstadoReserva.pendiente);

  const mutation = useCrearReserva({
    mutation: {
      onSuccess: (res) => {
        if (res.status === 201) {
          navigate('/reservas');
        }
      },
      onError: () => {
        setError('Error al crear la reserva');
      },
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');

    if (!fecha || !hora || !nombreCliente || !numPersonas) {
      setError('Completa los campos obligatorios');
      return;
    }

    mutation.mutate({
      data: {
        fecha,
        hora,
        nombre_cliente: nombreCliente,
        num_personas: parseInt(numPersonas, 10),
        telefono: telefono || null,
        notas: notas || null,
        estado,
      },
    });
  };

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
            <input id="fecha" type="date" value={fecha} onChange={(e) => setFecha(e.target.value)} />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="hora">Hora</label>
            <input id="hora" type="time" value={hora} onChange={(e) => setHora(e.target.value)} />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="nombreCliente">Nombre del cliente</label>
            <input id="nombreCliente" type="text" value={nombreCliente} onChange={(e) => setNombreCliente(e.target.value)} placeholder="Nombre completo" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="numPersonas">Personas</label>
            <input id="numPersonas" type="number" min="1" value={numPersonas} onChange={(e) => setNumPersonas(e.target.value)} />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="telefono">Teléfono</label>
            <input id="telefono" type="tel" value={telefono} onChange={(e) => setTelefono(e.target.value)} placeholder="Opcional" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="estado">Estado</label>
            <select id="estado" value={estado} onChange={(e) => setEstado(e.target.value as EstadoReserva)}>
              <option value={EstadoReserva.pendiente}>Pendiente</option>
              <option value={EstadoReserva.confirmada}>Confirmada</option>
              <option value={EstadoReserva.cancelada}>Cancelada</option>
            </select>
          </div>
        </div>

        <div className="grupoFormulario ancho">
          <label className="etiquetaFormulario" htmlFor="notas">Notas</label>
          <textarea id="notas" rows={3} value={notas} onChange={(e) => setNotas(e.target.value)} placeholder="Alergias, preferencias, etc." />
        </div>

        <button className="botonEnviar" type="submit" disabled={mutation.isPending}>
          {mutation.isPending ? 'Guardando...' : 'Crear Reserva'}
        </button>
      </form>
    </div>
  );
}

export default FormularioReserva;
