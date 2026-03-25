/* 253A-7: Lista paginada de gastos */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useListarGastos, useEliminarGasto } from '../api/generated';
import '../estilos/Formularios.css';

function formatearMoneda(valor: string): string {
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(valor));
}

function ListaGastos() {
  const navigate = useNavigate();
  const [pagina, setPagina] = useState(1);
  const porPagina = 15;

  const { data, isLoading, refetch } = useListarGastos({ page: pagina, per_page: porPagina });
  const eliminarMutation = useEliminarGasto({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const gastos = data?.status === 200 ? data.data : null;

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Gastos</h1>
          <p className="subtituloPagina">{gastos ? `${gastos.total} registros` : ''}</p>
        </div>
        <button className="botonNuevo" onClick={() => navigate('/gastos/nuevo')}>+ Nuevo Gasto</button>
      </div>

      <div className="contenedorLista">
        {isLoading ? (
          <p className="sinDatos">Cargando...</p>
        ) : gastos && gastos.items.length > 0 ? (
          <>
            <table className="tabla">
              <thead>
                <tr>
                  <th>Fecha</th>
                  <th>Proveedor</th>
                  <th>Tipo doc.</th>
                  <th>Método</th>
                  <th>Base</th>
                  <th>IVA</th>
                  <th>Total</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {gastos.items.map((g) => (
                  <tr key={g.id}>
                    <td>{g.fecha}</td>
                    <td>{g.proveedor || '—'}</td>
                    <td style={{ textTransform: 'capitalize' }}>{g.tipo_documento}</td>
                    <td style={{ textTransform: 'capitalize' }}>{g.metodo_pago}</td>
                    <td>{formatearMoneda(g.importe_base)}</td>
                    <td>{formatearMoneda(g.importe_iva)}</td>
                    <td><strong>{formatearMoneda((parseFloat(g.importe_base) + parseFloat(g.importe_iva)).toFixed(2))}</strong></td>
                    <td>
                      <button
                        className="botonEliminar"
                        onClick={() => eliminarMutation.mutate({ id: g.id })}
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
              <span className="infoPagina">Página {pagina} de {Math.ceil(gastos.total / porPagina)}</span>
              <button className="botonPagina" disabled={pagina * porPagina >= gastos.total} onClick={() => setPagina(pagina + 1)}>Siguiente →</button>
            </div>
          </>
        ) : (
          <p className="sinDatos">No hay gastos registrados</p>
        )}
      </div>
    </div>
  );
}

export default ListaGastos;
