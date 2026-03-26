/* [263A-13] DashboardReservas — Dashboard de reservas Fase 2.
 * 3 paneles con tabs: Resumen, Ocupación, Análisis.
 * Usa recharts para gráficos y el endpoint GET /api/dashboard/reservas. */

import { useState } from 'react';
import { useDashboardReservas, ResumenReservas, OcupacionReservas, AnalisisReservas, AgrupacionCanal } from '../api/generated';
import { Boton, Select } from '@glory/componentes/ui';
import {
  BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer,
  PieChart, Pie, Cell, Legend
} from 'recharts';
import '../estilos/DashboardReservas.css';

const COLORES = ['#4A90D9', '#50E3C2', '#F5A623', '#E91E63', '#7B61FF', '#4CAF50', '#FF5722', '#9C27B0'];

function DashboardReservas() {
  const ahora = new Date();
  const [anio, setAnio] = useState(ahora.getFullYear());
  const [mes, setMes] = useState(ahora.getMonth() + 1);
  const [pestana, setPestana] = useState<'resumen' | 'ocupacion' | 'analisis'>('resumen');

  const { data, isLoading } = useDashboardReservas({ year: anio, month: mes });
  const dashboard = data?.status === 200 ? data.data : null;

  const meses = [
    'Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio',
    'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre',
  ];

  return (
    <div className="dashboardReservas">
      <div className="cabeceraPagina">
        <h1 className="tituloPagina">Dashboard Reservas</h1>
        <p className="subtituloPagina">Análisis detallado del mes</p>
      </div>

      <div className="selectorMesDashboard">
        <Select tamano="sm" value={mes} onChange={e => setMes(Number(e.target.value))}>
          {meses.map((nombre, i) => (
            <option key={i + 1} value={i + 1}>{nombre}</option>
          ))}
        </Select>
        <Select tamano="sm" value={anio} onChange={e => setAnio(Number(e.target.value))}>
          {[anio - 1, anio, anio + 1].map(a => (
            <option key={a} value={a}>{a}</option>
          ))}
        </Select>
      </div>

      <div className="pestanasDashboard">
        <Boton
          variante="fantasma"
          tamano="sm"
          className={`pestanaDashboard ${pestana === 'resumen' ? 'activa' : ''}`}
          onClick={() => setPestana('resumen')}
        >Resumen</Boton>
        <Boton
          variante="fantasma"
          tamano="sm"
          className={`pestanaDashboard ${pestana === 'ocupacion' ? 'activa' : ''}`}
          onClick={() => setPestana('ocupacion')}
        >Ocupación</Boton>
        <Boton
          variante="fantasma"
          tamano="sm"
          className={`pestanaDashboard ${pestana === 'analisis' ? 'activa' : ''}`}
          onClick={() => setPestana('analisis')}
        >Análisis</Boton>
      </div>

      {isLoading && <p className="cargando">Cargando dashboard...</p>}

      {dashboard && pestana === 'resumen' && <PanelResumen data={dashboard.resumen} />}
      {dashboard && pestana === 'ocupacion' && <PanelOcupacion data={dashboard.ocupacion} />}
      {dashboard && pestana === 'analisis' && <PanelAnalisis data={dashboard.analisis} />}
    </div>
  );
}

/* ========== Panel Resumen ========== */
function PanelResumen({ data }: { data: ResumenReservas }) {
  const signoVariacion = data.variacion_porcentaje >= 0 ? '+' : '';

  return (
    <>
      <div className="kpisGrid">
        <div className="tarjetaKpi">
          <div className="kpiEtiqueta">Total reservas</div>
          <div className="kpiValor">{data.total_reservas}</div>
          <div className={`kpiDetalle ${data.variacion_porcentaje >= 0 ? 'kpiPositivo' : 'kpiNegativo'}`}>
            {signoVariacion}{data.variacion_porcentaje}% vs mes anterior ({data.total_mes_anterior})
          </div>
        </div>
        <div className="tarjetaKpi">
          <div className="kpiEtiqueta">Clientes nuevos</div>
          <div className="kpiValor">{data.clientes_nuevos}</div>
        </div>
      </div>

      <div className="graficosGrid">
        <div className="panelGrafico">
          <h3>Reservas por día</h3>
          <ResponsiveContainer width="100%" height={250}>
            <BarChart data={data.por_dia.map((d: { fecha: string; total: number }) => ({ ...d, fecha: d.fecha.slice(5) }))}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="fecha" fontSize={11} />
              <YAxis allowDecimals={false} />
              <Tooltip />
              <Bar dataKey="total" fill="#4A90D9" name="Reservas" />
            </BarChart>
          </ResponsiveContainer>
        </div>

        <div className="panelGrafico">
          <h3>Por día de semana</h3>
          <ResponsiveContainer width="100%" height={250}>
            <BarChart data={data.por_dia_semana}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="dia" fontSize={11} />
              <YAxis allowDecimals={false} />
              <Tooltip />
              <Bar dataKey="total" fill="#50E3C2" name="Reservas" />
            </BarChart>
          </ResponsiveContainer>
        </div>

        <div className="panelGrafico">
          <h3>Distribución por canal</h3>
          {data.por_canal.length > 0 ? (
            <ResponsiveContainer width="100%" height={250}>
              <PieChart>
                <Pie data={data.por_canal} dataKey="total" nameKey="canal" cx="50%" cy="50%" outerRadius={90} label={({ name, value }: { name?: string; value?: number }) => `${name ?? ''} (${value ?? 0})`}>
                  {data.por_canal.map((_: AgrupacionCanal, i: number) => (
                    <Cell key={i} fill={COLORES[i % COLORES.length]} />
                  ))}
                </Pie>
                <Tooltip />
                <Legend />
              </PieChart>
            </ResponsiveContainer>
          ) : (
            <p className="cargando">Sin datos de canales</p>
          )}
        </div>
      </div>
    </>
  );
}

/* ========== Panel Ocupación ========== */
function PanelOcupacion({ data }: { data: OcupacionReservas }) {
  return (
    <>
      <div className="kpisGrid">
        <div className="tarjetaKpi">
          <div className="kpiEtiqueta">Media personas/reserva</div>
          <div className="kpiValor">{data.media_personas}</div>
        </div>
        <div className="tarjetaKpi">
          <div className="kpiEtiqueta">Media reservas/día</div>
          <div className="kpiValor">{data.media_reservas_dia}</div>
        </div>
        <div className="tarjetaKpi">
          <div className="kpiEtiqueta">Total reservas</div>
          <div className="kpiValor">{data.total_reservas}</div>
        </div>
        <div className="tarjetaKpi">
          <div className="kpiEtiqueta">Antelación media</div>
          <div className="kpiValor">{data.antelacion_media_dias} días</div>
        </div>
      </div>

      <div className="graficosGrid">
        <div className="panelGrafico">
          <h3>Distribución por hora</h3>
          <ResponsiveContainer width="100%" height={250}>
            <BarChart data={data.por_hora}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="hora" fontSize={11} />
              <YAxis allowDecimals={false} />
              <Tooltip />
              <Bar dataKey="total" fill="#7B61FF" name="Reservas" />
            </BarChart>
          </ResponsiveContainer>
        </div>

        <div className="panelGrafico">
          <h3>Por turno</h3>
          <ResponsiveContainer width="100%" height={250}>
            <BarChart data={data.por_turno}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="turno" fontSize={11} />
              <YAxis allowDecimals={false} />
              <Tooltip />
              <Bar dataKey="total" fill="#F5A623" name="Reservas" />
              <Bar dataKey="personas" fill="#4CAF50" name="Personas" />
            </BarChart>
          </ResponsiveContainer>
        </div>

        <div className="panelGrafico">
          <h3>Procedencia (canal)</h3>
          <table className="tablaDistribucion">
            <thead>
              <tr><th>Canal</th><th>Reservas</th><th>%</th><th></th></tr>
            </thead>
            <tbody>
              {data.por_procedencia.map((c: AgrupacionCanal) => (
                <tr key={c.canal}>
                  <td>{c.canal}</td>
                  <td>{c.total}</td>
                  <td>{c.porcentaje}%</td>
                  <td>
                    <div className="barraCanal">
                      <div className="barraProgreso" style={{ width: `${Math.min(c.porcentaje, 100)}%`, maxWidth: '100px' }} />
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </>
  );
}

/* ========== Panel Análisis ========== */
function PanelAnalisis({ data }: { data: AnalisisReservas }) {
  const formatMoneda = (v: string | null | undefined) => {
    if (!v) return '—';
    return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(v));
  };

  return (
    <div className="kpisGrid">
      <div className="tarjetaKpi">
        <div className="kpiEtiqueta">Reservas efectivas</div>
        <div className="kpiValor">{data.reservas_efectivas}</div>
        <div className="kpiDetalle">Sin cancelaciones ni no-shows</div>
      </div>
      <div className="tarjetaKpi">
        <div className="kpiEtiqueta">Total comensales</div>
        <div className="kpiValor">{data.total_comensales}</div>
      </div>
      <div className="tarjetaKpi">
        <div className="kpiEtiqueta">Comensales/reserva</div>
        <div className="kpiValor">{data.comensales_por_reserva}</div>
      </div>
      <div className="tarjetaKpi">
        <div className="kpiEtiqueta">Ticket medio/reserva</div>
        <div className="kpiValor">{formatMoneda(data.ticket_medio_reserva)}</div>
        <div className="kpiDetalle">Ventas ÷ reservas efectivas</div>
      </div>
      <div className="tarjetaKpi">
        <div className="kpiEtiqueta">Ticket medio/persona</div>
        <div className="kpiValor">{formatMoneda(data.ticket_medio_persona)}</div>
        <div className="kpiDetalle">Ventas ÷ total comensales</div>
      </div>
    </div>
  );
}

export default DashboardReservas;
