/* [263A-16] Vista de ocupación del plano de sala — solo lectura.
 * Reescrito imports a shadcn Button. Mantiene PlanoOcupacion.css para canvas/mesas.
 * Mesas coloreadas: libre (gris), ocupada (verde), no-show (rojo), inactiva.
 * Se integra dentro de ListaReservas. Click en mesa muestra reservas en tooltip.
 * [283A-36] Zoom sincronizado con PlanoSala via zoomStore. */

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { ZoomIn, ZoomOut } from 'lucide-react';
import { useObtenerOcupacion, MesaOcupacion, ZonaOcupacion } from '../api/generated';
import { useZoomStore } from '../stores/zoomStore';
import '../estilos/PlanoOcupacion.css';

interface Props {
  fecha: string;
  turno: string;
}

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
  const { zoom, zoomIn, zoomOut } = useZoomStore();

  if (plano && plano.zonas.length > 0 && !zonaActiva) {
    setZonaActiva(plano.zonas[0].id);
  }

  const zonaData = plano?.zonas.find((z: ZonaOcupacion) => z.id === zonaActiva);

  if (isLoading) return <p className="text-sm text-muted-foreground text-center py-4">Cargando plano...</p>;
  if (!plano || plano.zonas.length === 0) return null;

  return (
    <div className="flex flex-col gap-3">
      <div className="flex items-center justify-between">
        <h3 className="font-semibold text-sm">Plano de sala</h3>
        <div className="flex items-center gap-1">
          <Button size="sm" variant="outline" onClick={zoomOut}><ZoomOut className="size-4" /></Button>
          <span className="text-xs w-12 text-center tabular-nums">{Math.round(zoom * 100)}%</span>
          <Button size="sm" variant="outline" onClick={zoomIn}><ZoomIn className="size-4" /></Button>
        </div>
      </div>

      {/* Tabs de zonas */}
      <div className="flex gap-1 flex-wrap">
        {plano.zonas.map((z: ZonaOcupacion) => (
          <Button
            key={z.id}
            variant={z.id === zonaActiva ? 'default' : 'outline'}
            size="sm"
            onClick={() => { setZonaActiva(z.id); setMesaHover(null); }}
          >
            {z.nombre}
          </Button>
        ))}
      </div>

      {/* Leyenda */}
      <div className="flex gap-4 text-xs text-muted-foreground">
        <span className="flex items-center gap-1"><span className="inline-block w-3 h-3 rounded-sm bg-muted" /> Libre</span>
        <span className="flex items-center gap-1"><span className="inline-block w-3 h-3 rounded-sm bg-green-500" /> Ocupada</span>
        <span className="flex items-center gap-1"><span className="inline-block w-3 h-3 rounded-sm bg-destructive" /> No Show</span>
        <span className="flex items-center gap-1"><span className="inline-block w-3 h-3 rounded-sm bg-muted-foreground/30" /> Inactiva</span>
      </div>

      {/* [283A-25] Canvas 100% ancho con aspect-ratio proporcional a la zona.
         * [283A-39] Zoom: el canvas se expande a zoom*100% de ancho y el contenedor
         * permite scroll horizontal. Los % de posición/tamaño de mesas escalan solos.
         * Sin transform: scale() — era solo visual y no cambiaba el tamaño real de los cuadros. */}
      {zonaData && (
        <div style={{ overflow: 'auto', width: '100%' }}>
          <div
            className="planoOcupacionCanvas"
            style={{
              aspectRatio: `${zonaData.ancho} / ${zonaData.alto}`,
              width: `${zoom * 100}%`,
              overflow: 'visible',
            }}
          >
          {zonaData.mesas.map((mesa: MesaOcupacion) => {
            const estado = estadoMesa(mesa);
            const esHover = mesaHover === mesa.id;

            return (
              <div
                key={mesa.id}
                className={`mesaOcupacion ${estado} ${mesa.forma} ${esHover ? 'hover' : ''}`}
                style={{
                  left: `${(mesa.pos_x / zonaData.ancho) * 100}%`,
                  top: `${(mesa.pos_y / zonaData.alto) * 100}%`,
                  width: `${(mesa.ancho / zonaData.ancho) * 100}%`,
                  height: `${(mesa.alto / zonaData.alto) * 100}%`,
                }}
                onMouseEnter={() => setMesaHover(mesa.id)}
                onMouseLeave={() => setMesaHover(null)}
              >
                <span className="mesaOcupacionNumero">{mesa.numero}</span>

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
        </div>
      )}
    </div>
  );
}

export default PlanoOcupacion;
