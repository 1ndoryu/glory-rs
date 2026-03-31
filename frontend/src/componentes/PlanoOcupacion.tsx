/* [263A-16] Vista de ocupación del plano de sala — solo lectura.
 * Reescrito imports a shadcn Button. Mantiene PlanoOcupacion.css para canvas/mesas.
 * Mesas coloreadas: libre (gris), ocupada (verde), no-show (rojo), inactiva.
 * Se integra dentro de ListaReservas. Click en mesa muestra reservas en tooltip.
 * [283A-36] Zoom sincronizado con PlanoSala via zoomStore.
 * [303A-12] Pan, minimap, indicadores off-screen. */

import { useState, useRef, useMemo, useLayoutEffect } from 'react';
import { Button } from '@/components/ui/button';
import { ZoomIn, ZoomOut } from 'lucide-react';
import { useObtenerOcupacion, MesaOcupacion, ZonaOcupacion } from '../api/generated';
import { useZoomStore } from '../stores/zoomStore';
import { useCanvasResize } from '../hooks/useCanvasResize';
import { useCanvasPan } from '../hooks/useCanvasPan';
import CanvasMinimap from './plano-sala/CanvasMinimap';
import OffScreenIndicators from './plano-sala/OffScreenIndicators';
import { normalizarDimensionesMesa, obtenerEstiloVisualMesa } from './plano-sala/mesaGeometry';
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
  const viewportRef = useRef<HTMLDivElement>(null);
  const zoom = useZoomStore(s => s.zoom);
  const zoomIn = useZoomStore(s => s.zoomIn);
  const zoomOut = useZoomStore(s => s.zoomOut);
  const canvasHeight = useZoomStore(s => s.canvasHeight);
  const setCanvasHeight = useZoomStore(s => s.setCanvasHeight);

  /* [303A-13] Resize del canvas arrastrando el borde inferior — sincronizado via store */
  const { resizing, onResizeStart } = useCanvasResize({ canvasHeight, setCanvasHeight });

  /* [303A-20] Medir viewport via ResizeObserver */
  const [viewportSize, setViewportSize] = useState({ w: 0, h: 0 });
  useLayoutEffect(() => {
    const el = viewportRef.current;
    if (!el) return;
    setViewportSize({ w: el.clientWidth, h: el.clientHeight });
    const ro = new ResizeObserver(() => {
      setViewportSize({ w: el.clientWidth, h: el.clientHeight });
    });
    ro.observe(el);
    return () => ro.disconnect();
  }, []);

  if (plano && plano.zonas.length > 0 && !zonaActiva) {
    setZonaActiva(plano.zonas[0].id);
  }

  const zonaData = plano?.zonas.find((z: ZonaOcupacion) => z.id === zonaActiva);

  const mesasRender = useMemo(() => {
    if (!zonaData) return [];
    return zonaData.mesas.map((mesa) => {
      const normalizada = normalizarDimensionesMesa(mesa);
      return {
        ...normalizada,
        pos_x: normalizada.pos_x * zoom,
        pos_y: normalizada.pos_y * zoom,
        ancho: normalizada.ancho * zoom,
        alto: normalizada.alto * zoom,
      };
    });
  }, [zonaData, zoom]);

  /* [313A-1] El tamaño real del plano sale de la zona y de la mesa más lejana. */
  const contentBounds = useMemo(() => {
    if (!zonaData) return { w: 0, h: 0 };
    let maxX = zonaData.ancho * zoom;
    let maxY = Math.max(canvasHeight, zonaData.alto * zoom);
    for (const m of mesasRender) {
      const x = m.pos_x + m.ancho;
      const y = m.pos_y + m.alto;
      if (x > maxX) maxX = x;
      if (y > maxY) maxY = y;
    }
    return { w: maxX, h: maxY };
  }, [canvasHeight, mesasRender, zonaData, zoom]);

  const maxPanOffset = useMemo(() => ({
    x: Math.max(0, contentBounds.w - viewportSize.w),
    y: Math.max(0, contentBounds.h - viewportSize.h),
  }), [contentBounds.h, contentBounds.w, viewportSize.h, viewportSize.w]);

  /* [303A-12] Pan del canvas
   * [303A-20] Transform-based + clamp al área real del plano. */
  const { panning, panOffset, setPanOffset, onPanMouseDown } = useCanvasPan(maxPanOffset);

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

      {/* [303A-20] Canvas con overflow:hidden + transform para pan. */}
      {zonaData && (
        <div style={{ position: 'relative' }}>
          <div
            ref={viewportRef}
            className={`planoOcupacionCanvas ${panning ? 'planoPanning' : ''}`}
            style={{ height: canvasHeight }}
            onMouseDown={onPanMouseDown}
          >
            <div
              className="planoOcupacionContent"
              style={{
                width: contentBounds.w,
                height: contentBounds.h,
                transform: `translate(${-panOffset.x}px, ${-panOffset.y}px)`,
              }}
            >
              {mesasRender.map((mesa: MesaOcupacion) => {
                const estado = estadoMesa(mesa);
                const esHover = mesaHover === mesa.id;

                return (
                  <div
                    key={mesa.id}
                    className={`mesaOcupacion ${estado} ${mesa.forma} ${esHover ? 'hover' : ''}`}
                    style={{
                      left: mesa.pos_x * zoom,
                      top: mesa.pos_y * zoom,
                      width: mesa.ancho * zoom,
                      height: mesa.alto * zoom,
                      ...obtenerEstiloVisualMesa(mesa.forma),
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
          {/* [303A-12] Minimap + indicadores off-screen */}
          <CanvasMinimap
            mesas={mesasRender.map((m: MesaOcupacion) => ({
              x: m.pos_x,
              y: m.pos_y,
              ancho: m.ancho,
              alto: m.alto,
            }))}
            contentWidth={contentBounds.w}
            contentHeight={Math.max(canvasHeight, contentBounds.h)}
            viewportWidth={viewportSize.w}
            viewportHeight={viewportSize.h}
            scrollLeft={panOffset.x}
            scrollTop={panOffset.y}
            onNavigate={(x, y) => setPanOffset({ x, y })}
          />
          <OffScreenIndicators
            mesas={mesasRender.map((m: MesaOcupacion) => ({
              x: m.pos_x,
              y: m.pos_y,
              ancho: m.ancho,
              alto: m.alto,
            }))}
            viewportWidth={viewportSize.w}
            viewportHeight={viewportSize.h}
            scrollLeft={panOffset.x}
            scrollTop={panOffset.y}
            onNavigate={(x, y) => setPanOffset({ x, y })}
          />
        </div>
      )}
      {/* [303A-13] Handle de resize — misma clase CSS que PlanoSala */}
      {zonaData && (
        <div
          className={`planoResizeHandle ${resizing ? 'activo' : ''}`}
          onMouseDown={onResizeStart}
          title="Arrastrar para cambiar altura del plano"
        />
      )}
    </div>
  );
}

export default PlanoOcupacion;
