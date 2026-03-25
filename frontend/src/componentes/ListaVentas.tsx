/* 253A-7: Lista paginada de ventas
   253A-10: componentes UI atomicos
   253A-14: modal para crear venta en vez de ruta separada */

import { useState } from 'react';
import { useListarVentas, useEliminarVenta } from '../api/generated';
import { Boton, Modal } from './ui';
import FormularioVenta from './FormularioVenta';
import '../estilos/Formularios.css';

function formatearMoneda(valor: string): string {
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(valor));
}

function ListaVentas() {
  const [pagina, setPagina] = useState(1);
  const [modalAbierto, setModalAbierto] = useState(false);
  const porPagina = 15;

  const { data, isLoading, refetch } = useListarVentas({ page: pagina, per_page: porPagina });
  const eliminarMutation = useEliminarVenta({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const ventas = data?.status === 200 ? data.data : null;

  const cerrarModalYRefrescar = () => {
    setModalAbierto(false);
    refetch();
  };

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Ventas</h1>
          <p className="subtituloPagina">{ventas ? `${ventas.total} registros` : ''}</p>
        </div>
        <Boton variante="exito" onClick={() => setModalAbierto(true)}>+ Nueva Venta</Boton>
      </div>

      <Modal abierto={modalAbierto} onCerrar={() => setModalAbierto(false)} titulo="Nueva Venta">
        <FormularioVenta onExito={cerrarModalYRefrescar} />
      </Modal>

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
                      <Boton
                        variante="peligro"
                        tamano="sm"
                        onClick={() => eliminarMutation.mutate({ id: v.id })}
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
              <span className="infoPagina">Pagina {pagina} de {Math.ceil(ventas.total / porPagina)}</span>
              <Boton variante="fantasma" tamano="sm" disabled={pagina * porPagina >= ventas.total} onClick={() => setPagina(pagina + 1)}>Siguiente</Boton>
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
