/* 253A-7: Lista paginada de reservas
   263A-6: Vista de reservas por día con filtros turno/estado/fecha.
   Muestra: nº mesa, hora, nombre, apellidos, personas, estado, teléfono. */

import { Boton, Modal, Input, Select } from '@glory/componentes/ui';
import FormularioReserva from './FormularioReserva';
import useVistaReservas from '../hooks/useVistaReservas';
import '../estilos/Formularios.css';

/* 263A-6: Etiquetas descriptivas para cada estado */
const ETIQUETA_ESTADO: Record<string, string> = {
  pendiente: 'Pendiente',
  confirmada: 'Confirmada',
  cancelada: 'Cancelada',
  completada: 'Completada',
  no_show: 'No Show',
  lista_espera: 'Lista de espera',
};

function ListaReservas() {
  const {
    filtros,
    cambiarFiltro,
    modalAbierto,
    setModalAbierto,
    reservas,
    isLoading,
    eliminarMutation,
    cerrarModalYRefrescar,
    porPagina,
  } = useVistaReservas();

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Reservas</h1>
          <p className="subtituloPagina">{reservas ? `${reservas.total} reservas` : ''}</p>
        </div>
        <Boton variante="exito" onClick={() => setModalAbierto(true)}>+ Nueva Reserva</Boton>
      </div>

      {/* 263A-6: Barra de filtros — fecha, turno, estado */}
      <div className="filtroBusqueda" style={{ display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
        <Input
          type="date"
          value={filtros.fecha}
          onChange={(e) => cambiarFiltro('fecha', e.target.value)}
          style={{ maxWidth: '10rem' }}
        />
        <Select
          value={filtros.turno}
          onChange={(e) => cambiarFiltro('turno', e.target.value)}
          style={{ maxWidth: '10rem' }}
        >
          <option value="">Día completo</option>
          <option value="desayuno">Desayuno</option>
          <option value="comida">Comida</option>
          <option value="cena">Cena</option>
        </Select>
        <Select
          value={filtros.estado}
          onChange={(e) => cambiarFiltro('estado', e.target.value)}
          style={{ maxWidth: '10rem' }}
        >
          <option value="">Todos los estados</option>
          <option value="confirmada">Confirmadas</option>
          <option value="pendiente">Pendientes</option>
          <option value="lista_espera">Lista de espera</option>
          <option value="completada">Completadas</option>
          <option value="no_show">No Show</option>
          <option value="cancelada">Canceladas</option>
        </Select>
      </div>

      <Modal abierto={modalAbierto} onCerrar={() => setModalAbierto(false)} titulo="Nueva Reserva">
        <FormularioReserva onExito={cerrarModalYRefrescar} />
      </Modal>

      <div className="contenedorLista">
        {isLoading ? (
          <p className="sinDatos">Cargando...</p>
        ) : reservas && reservas.items.length > 0 ? (
          <>
            <table className="tabla">
              <thead>
                <tr>
                  <th>Mesa</th>
                  <th>Hora</th>
                  <th>Nombre</th>
                  <th>Apellidos</th>
                  <th>Personas</th>
                  <th>Estado</th>
                  <th>Teléfono</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {reservas.items.map((r) => (
                  <tr key={r.id}>
                    <td>{r.num_mesa ?? '—'}</td>
                    <td>{r.hora}</td>
                    <td>{r.nombre_cliente}</td>
                    <td className="textoTruncado">{r.apellidos_cliente || '—'}</td>
                    <td>{r.num_personas}</td>
                    <td>
                      <span className={`badgeEstado ${r.estado}`}>
                        {ETIQUETA_ESTADO[r.estado] ?? r.estado}
                      </span>
                    </td>
                    <td>{r.telefono || '—'}</td>
                    <td className="accionesTabla">
                      <Boton
                        variante="peligro"
                        tamano="sm"
                        onClick={() => eliminarMutation.mutate({ id: r.id })}
                        disabled={eliminarMutation.isPending}
                      >
                        Eliminar
                      </Boton>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            <div className="paginacion">
              <Boton variante="fantasma" tamano="sm" disabled={filtros.pagina <= 1} onClick={() => cambiarFiltro('pagina', filtros.pagina - 1)}>Anterior</Boton>
              <span className="infoPagina">Página {filtros.pagina} de {Math.ceil(reservas.total / porPagina)}</span>
              <Boton variante="fantasma" tamano="sm" disabled={filtros.pagina * porPagina >= reservas.total} onClick={() => cambiarFiltro('pagina', filtros.pagina + 1)}>Siguiente</Boton>
            </div>
          </>
        ) : (
          <p className="sinDatos">No hay reservas para esta fecha</p>
        )}
      </div>
    </div>
  );
}

export default ListaReservas;
