/* [263A-16] Vista de ocupación del plano de sala — solo lectura.
 * Muestra mesas coloreadas por estado: libre (gris), ocupada (verde), no-show (rojo), inactiva.
 * Se integra dentro de ListaReservas (vista día). Click en mesa muestra reservas en tooltip. */

import { useState } from 'react';
import { Boton } from '@glory/componentes/ui';
import { useObtenerOcupacion, MesaOcupacion, ZonaOcupacion } from '../api/generated';
import '../estilos/PlanoOcupacion.css';

interface Props {
  fecha: string;
  turno: string;
}

/* [263A-16] Color de mesa según reservas asociadas */
function estadoMesa(mesa: MesaOcupacion): 'libre' | 'ocupada' | 'no_show' | 'inactiva' {
  if (!mesa.activa) return 'inactiva';
  if (mesa.reservas.length === 0) return 'libre';
  if (mesa.reservas.some((r) => r.estado === 'no_show')) return 'no_show';
  return 'ocupada';
}

function PlanoOcupacion({ fecha, turno }: Props) {
  const { data, isLoading } = useObtenerOcupacion(
    { fecha, turno: turno || undefined },
    { query: { enabled: !!fecha } }
  );

  const plano = data?.status === 200 ? data.data : null;
  const [zonaActiva, setZonaActiva] = useState<string | null>(null);
  const [mesaHover, setMesaHover] = useState<string | null>(null);

  /* Auto-seleccionar primera zona */
  if (plano && plano.zonas.length > 0 && !zonaActiva) {
    setZonaActiva(plano.zonas[0].id);
  }

  const zonaData = plano?.zonas.find((z: ZonaOcupacion) => z.id === zonaActiva);

  if (isLoading) return <p className="sinDatos">Cargando plano...</p>;
  if (!plano || plano.zonas.length === 0) return null;

  return (
    <div className="planoOcupacion">
      <h3 className="planoOcupacionTitulo">Plano de sala</h3>

      {/* Tabs de zonas */}
      <div className="planoOcupacionZonas">
        {plano.zonas.map((z: ZonaOcupacion) => (
          <Boton
            key={z.id}
            variante="fantasma"
            tamano="sm"
            className={`planoOcupacionZonaTab ${z.id === zonaActiva ? 'activa' : ''}`}
            onClick={() => { setZonaActiva(z.id); setMesaHover(null); }}
          >
            {z.nombre}
          </Boton>
        ))}
      </div>

      {/* Leyenda */}
      <div className="planoOcupacionLeyenda">
        <span className="planoOcupacionLeyendaItem"><span className="indicador libre" /> Libre</span>
        <span className="planoOcupacionLeyendaItem"><span className="indicador ocupada" /> Ocupada</span>
        <span className="planoOcupacionLeyendaItem"><span className="indicador no_show" /> No Show</span>
        <span className="planoOcupacionLeyendaItem"><span className="indicador inactiva" /> Inactiva</span>
      </div>

      {/* Canvas */}
      {zonaData && (
        <div
          className="planoOcupacionCanvas"
          style={{ width: zonaData.ancho, height: zonaData.alto }}
        >
          {zonaData.mesas.map((mesa: MesaOcupacion) => {
            const estado = estadoMesa(mesa);
            const esHover = mesaHover === mesa.id;

            return (
              <div
                key={mesa.id}
                className={`mesaOcupacion ${estado} ${mesa.forma} ${esHover ? 'hover' : ''}`}
                style={{
                  left: mesa.pos_x,
                  top: mesa.pos_y,
                  width: mesa.ancho,
                  height: mesa.alto,
                }}
                onMouseEnter={() => setMesaHover(mesa.id)}
                onMouseLeave={() => setMesaHover(null)}
              >
                <span className="mesaOcupacionNumero">{mesa.numero}</span>

                {/* Tooltip con reservas */}
                {esHover && mesa.reservas.length > 0 && (
                  <div className="mesaOcupacionTooltip">
                    {mesa.reservas.map((r) => (
                      <div key={r.reserva_id} className="mesaOcupacionReserva">
                        <strong>{r.hora.slice(0, 5)}</strong>{' '}
                        {r.nombre_cliente} {r.apellidos_cliente}
                        <span className="mesaOcupacionPersonas">{r.num_personas} pers.</span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}

export default PlanoOcupacion;
