/* 263A-7: Vista de reservas por mes — cuadrícula calendario.
   Muestra personas + reservas por día, totales mensuales.
   Click en día → navega a vista día con fecha seleccionada. */

import { useState, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { useResumenMensual } from '../api/generated';
import { Boton } from '@glory/componentes/ui';
import '../estilos/Calendario.css';

const NOMBRES_MES = [
  'Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio',
  'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre',
];

const DIAS_SEMANA = ['Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb', 'Dom'];

function CalendarioReservas() {
  const hoy = new Date();
  const [anio, setAnio] = useState(hoy.getFullYear());
  const [mes, setMes] = useState(hoy.getMonth() + 1);
  const navigate = useNavigate();

  const { data, isLoading } = useResumenMensual({ anio, mes });
  const resumen = data?.status === 200 ? data.data : [];

  /* Totales del mes */
  const totales = useMemo(() => {
    let reservas = 0;
    let personas = 0;
    for (const d of resumen) {
      reservas += d.total_reservas;
      personas += d.total_personas;
    }
    return { reservas, personas };
  }, [resumen]);

  /* Mapa fecha→datos para lookup rápido */
  const mapaFechas = useMemo(() => {
    const m = new Map<string, { reservas: number; personas: number }>();
    for (const d of resumen) {
      m.set(d.fecha, { reservas: d.total_reservas, personas: d.total_personas });
    }
    return m;
  }, [resumen]);

  /* Generar grid del calendario (lunes = inicio de semana) */
  const diasGrid = useMemo(() => {
    const primerDia = new Date(anio, mes - 1, 1);
    const ultimoDia = new Date(anio, mes, 0).getDate();
    /* getDay: 0=dom..6=sab → convertir a lunes=0 */
    const offsetInicio = (primerDia.getDay() + 6) % 7;
    const celdas: (number | null)[] = [];
    for (let i = 0; i < offsetInicio; i++) celdas.push(null);
    for (let d = 1; d <= ultimoDia; d++) celdas.push(d);
    /* Rellenar hasta completar semana */
    while (celdas.length % 7 !== 0) celdas.push(null);
    return celdas;
  }, [anio, mes]);

  const mesAnterior = () => {
    if (mes === 1) { setMes(12); setAnio(anio - 1); }
    else setMes(mes - 1);
  };

  const mesSiguiente = () => {
    if (mes === 12) { setMes(1); setAnio(anio + 1); }
    else setMes(mes + 1);
  };

  const irADia = (dia: number) => {
    const fechaStr = `${anio}-${String(mes).padStart(2, '0')}-${String(dia).padStart(2, '0')}`;
    navigate(`/reservas?fecha=${fechaStr}`);
  };

  const esHoy = (dia: number) =>
    anio === hoy.getFullYear() && mes === hoy.getMonth() + 1 && dia === hoy.getDate();

  return (
    <div>
      <div className="cabeceraLista">
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Calendario de Reservas</h1>
          <p className="subtituloPagina">
            {totales.reservas} reservas · {totales.personas} personas en {NOMBRES_MES[mes - 1]}
          </p>
        </div>
      </div>

      <div className="navegacionCalendario">
        <Boton variante="fantasma" tamano="sm" onClick={mesAnterior}>← Anterior</Boton>
        <span className="mesActual">{NOMBRES_MES[mes - 1]} {anio}</span>
        <Boton variante="fantasma" tamano="sm" onClick={mesSiguiente}>Siguiente →</Boton>
      </div>

      {isLoading ? (
        <p className="sinDatos">Cargando...</p>
      ) : (
        <div className="gridCalendario">
          {DIAS_SEMANA.map((d) => (
            <div key={d} className="cabeceraDiaSemana">{d}</div>
          ))}
          {diasGrid.map((dia, i) => {
            if (dia === null) return <div key={`v-${i}`} className="celdaDiaVacia" />;
            const fechaKey = `${anio}-${String(mes).padStart(2, '0')}-${String(dia).padStart(2, '0')}`;
            const datos = mapaFechas.get(fechaKey);
            return (
              <div
                key={dia}
                className={`celdaDia ${esHoy(dia) ? 'celdaDiaHoy' : ''} ${datos ? 'celdaDiaConDatos' : ''}`}
                onClick={() => irADia(dia)}
                role="button"
                tabIndex={0}
                onKeyDown={(e) => e.key === 'Enter' && irADia(dia)}
              >
                <span className="numeroDia">{dia}</span>
                {datos && (
                  <div className="datosDia">
                    <span className="reservasDia">{datos.reservas} res.</span>
                    <span className="personasDia">{datos.personas} pers.</span>
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

export default CalendarioReservas;
