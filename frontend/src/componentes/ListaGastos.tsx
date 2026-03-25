/* 253A-7: Lista paginada de gastos
   253A-10: componentes UI atomicos
   253A-14: modal para crear gasto en vez de ruta separada */

import { useState } from 'react';
import { useListarGastos, useEliminarGasto } from '../api/generated';
import { Boton, Modal } from './ui';
import FormularioGasto from './FormularioGasto';
import '../estilos/Formularios.css';

function formatearMoneda(valor: string): string {
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(valor));
}

function ListaGastos() {
  const [pagina, setPagina] = useState(1);
  const [modalAbierto, setModalAbierto] = useState(false);
  const porPagina = 15;

  const { data, isLoading, refetch } = useListarGastos({ page: pagina, per_page: porPagina });
  const eliminarMutation = useEliminarGasto({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const gastos = data?.status === 200 ? data.data : null;

  const cerrarModalYRefrescar = () => {
    setModalAbierto(false);
    refetch();
  };

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Gastos</h1>
          <p className="subtituloPagina">{gastos ? `${gastos.total} registros` : ''}</p>
        </div>
        <Boton variante="exito" onClick={() => setModalAbierto(true)}>+ Nuevo Gasto</Boton>
      </div>

      <Modal abierto={modalAbierto} onCerrar={() => setModalAbierto(false)} titulo="Nuevo Gasto">
        <FormularioGasto onExito={cerrarModalYRefrescar} />
      </Modal>

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
                    <td className="textoCapitalizado">{g.tipo_documento}</td>
                    <td className="textoCapitalizado">{g.metodo_pago}</td>
                    <td>{formatearMoneda(g.importe_base)}</td>
                    <td>{formatearMoneda(g.importe_iva)}</td>
                    <td><strong>{formatearMoneda((parseFloat(g.importe_base) + parseFloat(g.importe_iva)).toFixed(2))}</strong></td>
                    <td>
                      <Boton
                        variante="peligro"
                        tamano="sm"
                        onClick={() => eliminarMutation.mutate({ id: g.id })}
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
              <span className="infoPagina">Pagina {pagina} de {Math.ceil(gastos.total / porPagina)}</span>
              <Boton variante="fantasma" tamano="sm" disabled={pagina * porPagina >= gastos.total} onClick={() => setPagina(pagina + 1)}>Siguiente</Boton>
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
