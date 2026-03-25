/* 253A-7: Lista paginada de reservas */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useListarReservas, useEliminarReserva } from '../api/generated';
import '../estilos/Formularios.css';

function ListaReservas() {
  const navigate = useNavigate();
  const [pagina, setPagina] = useState(1);
  const porPagina = 15;

  const { data, isLoading, refetch } = useListarReservas({ page: pagina, per_page: porPagina });
  const eliminarMutation = useEliminarReserva({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const reservas = data?.status === 200 ? data.data : null;

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Reservas</h1>
          <p className="subtituloPagina">{reservas ? `${reservas.total} registros` : ''}</p>
        </div>
        <button className="botonNuevo" onClick={() => navigate('/reservas/nueva')}>+ Nueva Reserva</button>
      </div>

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
                      <button
                        className="botonEliminar"
                        onClick={() => eliminarMutation.mutate({ id: r.id })}
                        disabled={eliminarMutation.isPending}
                      >
                        Eliminar
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            <div className="paginacion">
              <button className="botonPagina" disabled={pagina <= 1} onClick={() => setPagina(pagina - 1)}>← Anterior</button>
              <span className="infoPagina">Página {pagina} de {Math.ceil(reservas.total / porPagina)}</span>
              <button className="botonPagina" disabled={pagina * porPagina >= reservas.total} onClick={() => setPagina(pagina + 1)}>Siguiente →</button>
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
