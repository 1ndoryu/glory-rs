/* 263A-1: ListaClientes — CRM con búsqueda, paginación y modal crear/editar */

import useListaClientes from '../hooks/useListaClientes';
import { Boton, Input, Modal } from '@glory/componentes/ui';
import FormularioCliente from './FormularioCliente';
import '../estilos/Formularios.css';

function ListaClientes() {
  const {
    pagina,
    setPagina,
    busqueda,
    buscar,
    modalCrear,
    setModalCrear,
    clienteEditar,
    setClienteEditar,
    porPagina,
    clientes,
    isLoading,
    eliminarMut,
    cerrarModalYRefrescar,
  } = useListaClientes();

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Clientes</h1>
          <p className="subtituloPagina">{clientes ? `${clientes.total} registros` : ''}</p>
        </div>
        <Boton variante="exito" onClick={() => setModalCrear(true)}>+ Nuevo Cliente</Boton>
      </div>

      <div className="filtroBusqueda">
        <Input
          type="search"
          placeholder="Buscar por nombre, apellidos, teléfono o email..."
          value={busqueda}
          onChange={(e) => buscar(e.target.value)}
        />
      </div>

      {/* Modal crear */}
      <Modal abierto={modalCrear} onCerrar={() => setModalCrear(false)} titulo="Nuevo Cliente">
        <FormularioCliente onExito={cerrarModalYRefrescar} />
      </Modal>

      {/* Modal editar */}
      <Modal abierto={!!clienteEditar} onCerrar={() => setClienteEditar(null)} titulo="Editar Cliente">
        {clienteEditar && <FormularioCliente onExito={cerrarModalYRefrescar} cliente={clienteEditar} />}
      </Modal>

      <div className="contenedorLista">
        {isLoading ? (
          <p className="sinDatos">Cargando...</p>
        ) : clientes && clientes.items.length > 0 ? (
          <>
            <table className="tabla">
              <thead>
                <tr>
                  <th>Nombre</th>
                  <th>Teléfono</th>
                  <th>Email</th>
                  <th>Empresa</th>
                  <th>Notas</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {clientes.items.map((c) => (
                  <tr key={c.id}>
                    <td>{c.nombre} {c.apellidos}</td>
                    <td>{c.telefono ? `${c.prefijo_telefono} ${c.telefono}` : '—'}</td>
                    <td>{c.email || '—'}</td>
                    <td>{c.empresa || '—'}</td>
                    <td className="textoTruncado">{c.notas || '—'}</td>
                    <td className="accionesTabla">
                      <Boton variante="fantasma" tamano="sm" onClick={() => setClienteEditar(c)}>Editar</Boton>
                      <Boton
                        variante="peligro"
                        tamano="sm"
                        onClick={() => eliminarMut.mutate({ id: c.id })}
                        disabled={eliminarMut.isPending}
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
              <span className="infoPagina">Página {pagina} de {Math.ceil(clientes.total / porPagina)}</span>
              <Boton variante="fantasma" tamano="sm" disabled={pagina * porPagina >= clientes.total} onClick={() => setPagina(pagina + 1)}>Siguiente</Boton>
            </div>
          </>
        ) : (
          <p className="sinDatos">No hay clientes registrados</p>
        )}
      </div>
    </div>
  );
}

export default ListaClientes;
