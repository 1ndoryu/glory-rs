/* 253A-7: Lista paginada de ventas */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useListarVentas, useEliminarVenta } from '../api/generated';
import '../estilos/Formularios.css';

function formatearMoneda(valor: string): string {
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(valor));
}

function ListaVentas() {
  const navigate = useNavigate();
  const [pagina, setPagina] = useState(1);
  const porPagina = 15;

  const { data, isLoading, refetch } = useListarVentas({ page: pagina, per_page: porPagina });
  const eliminarMutation = useEliminarVenta({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const ventas = data?.status === 200 ? data.data : null;

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Ventas</h1>
          <p className="subtituloPagina">{ventas ? `${ventas.total} registros` : ''}</p>
        </div>
        <button className="botonNuevo" onClick={() => navigate('/ventas/nueva')}>+ Nueva Venta</button>
      </div>

      <div className="contenedorLista">
        {isLoading ? (
          <p className="sinDatos">Cargando...</p>
        ) : ventas && ventas.items.length > 0 ? (
          <>
            <table className="tabla">
              <thead>
                <tr>
                  <th>Fecha</th>
                  <th>Turno</th>
                  <th>Canal</th>
                  <th>Método</th>
                  <th>Base</th>
                  <th>IVA</th>
                  <th>Total</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {ventas.items.map((v) => (
                  <tr key={v.id}>
                    <td>{v.fecha}</td>
                    <td className="textoCapitalizado">{v.turno}</td>
                    <td className="textoCapitalizado">{v.canal}</td>
                    <td className="textoCapitalizado">{v.metodo_pago}</td>
                    <td>{formatearMoneda(v.importe_base)}</td>
                    <td>{formatearMoneda(v.importe_iva)}</td>
                    <td><strong>{formatearMoneda((parseFloat(v.importe_base) + parseFloat(v.importe_iva)).toFixed(2))}</strong></td>
                    <td>
                      <button
                        className="botonEliminar"
                        onClick={() => eliminarMutation.mutate({ id: v.id })}
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
              <span className="infoPagina">Página {pagina} de {Math.ceil(ventas.total / porPagina)}</span>
              <button className="botonPagina" disabled={pagina * porPagina >= ventas.total} onClick={() => setPagina(pagina + 1)}>Siguiente →</button>
            </div>
          </>
        ) : (
          <p className="sinDatos">No hay ventas registradas</p>
        )}
      </div>
    </div>
  );
}

export default ListaVentas;
