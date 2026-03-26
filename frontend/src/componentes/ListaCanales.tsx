/* [263A-11] ListaCanales — CRUD canales de reserva (WhatsApp, Instagram, teléfono, web, etc.)
 * Los canales se usan para clasificar por dónde entran las reservas y generar estadísticas. */

import { useState } from 'react';
import { useListarCanales, useCrearCanal, useEliminarCanal, getListarCanalesQueryKey } from '../api/generated';
import { Boton, Input, Modal } from '@glory/componentes/ui';
import { useQueryClient } from '@tanstack/react-query';
import '../estilos/Formularios.css';

function ListaCanales() {
  const [modalCrear, setModalCrear] = useState(false);
  const [nombre, setNombre] = useState('');
  const queryClient = useQueryClient();

  const { data: canalesResp, isLoading } = useListarCanales();
  const canales = canalesResp?.status === 200 ? canalesResp.data : [];

  const crearMut = useCrearCanal({
    mutation: {
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: getListarCanalesQueryKey() });
        setNombre('');
        setModalCrear(false);
      },
    },
  });

  const eliminarMut = useEliminarCanal({
    mutation: {
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: getListarCanalesQueryKey() });
      },
    },
  });

  const handleCrear = () => {
    const nombreLimpio = nombre.trim();
    if (!nombreLimpio) return;
    crearMut.mutate({ data: { nombre: nombreLimpio } });
  };

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Canales de Reserva</h1>
          <p className="subtituloPagina">{canales.length} canales configurados</p>
        </div>
        <Boton variante="exito" onClick={() => setModalCrear(true)}>+ Nuevo Canal</Boton>
      </div>

      <Modal abierto={modalCrear} onCerrar={() => { setModalCrear(false); setNombre(''); }} titulo="Nuevo Canal">
        <form onSubmit={(e) => { e.preventDefault(); handleCrear(); }} className="formulario">
          <div className="campoFormulario">
            <label>Nombre del canal</label>
            <Input
              placeholder="Ej: WhatsApp, Instagram, Teléfono, Web..."
              value={nombre}
              onChange={(e) => setNombre(e.target.value)}
              autoFocus
            />
          </div>
          <div className="accionesFormulario">
            <Boton type="submit" variante="exito" disabled={crearMut.isPending || !nombre.trim()}>
              {crearMut.isPending ? 'Creando...' : 'Crear Canal'}
            </Boton>
          </div>
        </form>
      </Modal>

      <div className="contenedorLista">
        {isLoading ? (
          <p className="sinDatos">Cargando...</p>
        ) : canales.length > 0 ? (
          <table className="tabla">
            <thead>
              <tr>
                <th>Canal</th>
                <th>Fecha de creación</th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              {canales.map((c) => (
                <tr key={c.id}>
                  <td>{c.nombre}</td>
                  <td>{new Date(c.created_at).toLocaleDateString('es-ES')}</td>
                  <td className="accionesTabla">
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
        ) : (
          <p className="sinDatos">No hay canales configurados. Crea uno para clasificar por dónde llegan las reservas.</p>
        )}
      </div>
    </div>
  );
}

export default ListaCanales;
