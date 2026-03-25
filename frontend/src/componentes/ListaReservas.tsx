/* 253A-7: Lista paginada de reservas
   253A-10: componentes UI atomicos
   253A-14: modal para crear reserva en vez de ruta separada */

import { useState } from 'react';
import { useListarReservas, useEliminarReserva } from '../api/generated';
import { Boton, Modal } from '@glory/componentes/ui';
import FormularioReserva from './FormularioReserva';
import '../estilos/Formularios.css';

function ListaReservas() {
  const [pagina, setPagina] = useState(1);
  const [modalAbierto, setModalAbierto] = useState(false);
  const porPagina = 15;

  const { data, isLoading, refetch } = useListarReservas({ page: pagina, per_page: porPagina });
  const eliminarMutation = useEliminarReserva({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const reservas = data?.status === 200 ? data.data : null;

  const cerrarModalYRefrescar = () => {
    setModalAbierto(false);
    refetch();
  };

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Reservas</h1>
          <p className="subtituloPagina">{reservas ? `${reservas.total} registros` : ''}</p>
        </div>
        <Boton variante="exito" onClick={() => setModalAbierto(true)}>+ Nueva Reserva</Boton>
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
                  <th>Fecha</th>
                  <th>Hora</th>
                  <th>Cliente</th>
                  <th>Personas</th>
                  <th>Teléfono</th>
                  <th>Estado</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {reservas.items.map((r) => (
                  <tr key={r.id}>
                    <td>{r.fecha}</td>
                    <td>{r.hora}</td>
                    <td>{r.nombre_cliente}</td>
                    <td>{r.num_personas}</td>
                    <td>{r.telefono || '—'}</td>
                    <td>
                      <span className={`badgeEstado ${r.estado}`}>{r.estado}</span>
                    </td>
                    <td>
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
              <Boton variante="fantasma" tamano="sm" disabled={pagina <= 1} onClick={() => setPagina(pagina - 1)}>Anterior</Boton>
              <span className="infoPagina">Pagina {pagina} de {Math.ceil(reservas.total / porPagina)}</span>
              <Boton variante="fantasma" tamano="sm" disabled={pagina * porPagina >= reservas.total} onClick={() => setPagina(pagina + 1)}>Siguiente</Boton>
            </div>
          </>
        ) : (
          <p className="sinDatos">No hay reservas registradas</p>
        )}
      </div>
    </div>
  );
}

export default ListaReservas;
