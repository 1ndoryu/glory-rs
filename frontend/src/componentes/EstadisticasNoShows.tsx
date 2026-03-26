/* [263A-11] EstadisticasNoShows — Panel de estadísticas de no-shows con filtro de fechas
 * y desglose por canal de reserva. Usa el endpoint GET /api/reservas/no-shows. */

import { useState } from 'react';
import { useNoShowStats } from '../api/generated';
import { Boton, Input } from '@glory/componentes/ui';
import '../estilos/Formularios.css';
import '../estilos/NoShows.css';

function EstadisticasNoShows() {
  const [fechaDesde, setFechaDesde] = useState('');
  const [fechaHasta, setFechaHasta] = useState('');

  const params = {
    ...(fechaDesde ? { fecha_desde: fechaDesde } : {}),
    ...(fechaHasta ? { fecha_hasta: fechaHasta } : {}),
  };

  const { data: resp, isLoading } = useNoShowStats(params);
  const stats = resp?.status === 200 ? resp.data : null;

  const limpiarFiltros = () => {
    setFechaDesde('');
    setFechaHasta('');
  };

  return (
    <div>
      <div className="cabeceraPagina">
        <h1 className="tituloPagina">No-Shows</h1>
        <p className="subtituloPagina">Reservas no presentadas — ratio y desglose por canal</p>
      </div>

      <div className="filtrosNoShows">
        <div className="filtroFecha">
          <label>Desde</label>
          <Input type="date" value={fechaDesde} onChange={(e) => setFechaDesde(e.target.value)} />
        </div>
        <div className="filtroFecha">
          <label>Hasta</label>
          <Input type="date" value={fechaHasta} onChange={(e) => setFechaHasta(e.target.value)} />
        </div>
        {(fechaDesde || fechaHasta) && (
          <Boton variante="fantasma" tamano="sm" onClick={limpiarFiltros}>Limpiar</Boton>
        )}
      </div>

      {isLoading ? (
        <p className="sinDatos">Cargando estadísticas...</p>
      ) : stats ? (
        <>
          <div className="panelResumenNoShows">
            <div className="tarjetaNoShow">
              <span className="tarjetaNoShowValor">{stats.total_reservas}</span>
              <span className="tarjetaNoShowLabel">Total reservas</span>
            </div>
            <div className="tarjetaNoShow tarjetaNoShowPeligro">
              <span className="tarjetaNoShowValor">{stats.total_no_shows}</span>
              <span className="tarjetaNoShowLabel">No-shows</span>
            </div>
            <div className="tarjetaNoShow">
              <span className="tarjetaNoShowValor">{stats.ratio_porcentaje.toFixed(1)}%</span>
              <span className="tarjetaNoShowLabel">Ratio no-show</span>
            </div>
          </div>

          {stats.por_canal.length > 0 ? (
            <div className="contenedorLista">
              <h2 className="subtituloSeccion">Desglose por canal</h2>
              <table className="tabla">
                <thead>
                  <tr>
                    <th>Canal</th>
                    <th>Reservas</th>
                    <th>No-shows</th>
                    <th>Ratio</th>
                  </tr>
                </thead>
                <tbody>
                  {stats.por_canal.map((canal, i) => (
                    <tr key={i}>
                      <td>{canal.canal_nombre || 'Sin canal'}</td>
                      <td>{canal.total_reservas}</td>
                      <td>{canal.no_shows}</td>
                      <td>
                        <span className={canal.ratio_porcentaje > 20 ? 'ratioAlto' : canal.ratio_porcentaje > 10 ? 'ratioMedio' : 'ratioBajo'}>
                          {canal.ratio_porcentaje.toFixed(1)}%
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          ) : (
            <p className="sinDatos">No hay datos de canales para el período seleccionado</p>
          )}
        </>
      ) : (
        <p className="sinDatos">No hay datos de reservas</p>
      )}
    </div>
  );
}

export default EstadisticasNoShows;
