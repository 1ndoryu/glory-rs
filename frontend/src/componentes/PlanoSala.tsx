/* [263A-16] PlanoSala — Constructor visual de plano de sala con drag-and-drop.
 * [263A-28] Diálogos nativos reemplazados por shadcn Dialog + toast.
 * [303A-11] Canvas con altura fija del store, overflow:auto.
 * [303A-12] Pan (shift+drag), minimap, indicadores off-screen.
 * [303A-13] Resize handle arrastrable en borde inferior.
 * Lógica en usePlanoSala, mesa arrastrable en MesaDraggable, config en PanelConfigMesa. */

import { useRef, useState, useCallback } from 'react';
import { DndContext, DragOverlay, PointerSensor, useSensor, useSensors } from '@dnd-kit/core';
import type { DragEndEvent, DragStartEvent } from '@dnd-kit/core';
import { Button } from '@/components/ui/button';
import { Pencil, Trash2, Download, Upload, Combine, ZoomIn, ZoomOut } from 'lucide-react';
import MesaDraggable from './plano-sala/MesaDraggable';
import MesaTemplate from './plano-sala/MesaTemplate';
import PanelConfigMesa from './plano-sala/PanelConfigMesa';
import PlanoDialogs from './plano-sala/PlanoDialogs';
import CanvasMinimap from './plano-sala/CanvasMinimap';
import OffScreenIndicators from './plano-sala/OffScreenIndicators';
import { usePlanoSala } from './plano-sala/usePlanoSala';
import { useCanvasResize } from '../hooks/useCanvasResize';
import { useCanvasPan } from '../hooks/useCanvasPan';
import { useZoomStore } from '../stores/zoomStore';
import '../estilos/PlanoSala.css';

function PlanoSala() {
  const canvasRef = useRef<HTMLDivElement>(null);
  const viewportRef = useRef<HTMLDivElement>(null);
  const setCanvasHeight = useZoomStore(s => s.setCanvasHeight);

  /* [303A-12] Estado de scroll para minimap e indicadores off-screen */
  const [canvasViewState, setCanvasViewState] = useState({
    scrollLeft: 0, scrollTop: 0, w: 0, h: 0,
  });

  const onScroll = useCallback(() => {
    const el = viewportRef.current;
    if (!el) return;
    setCanvasViewState({
      scrollLeft: el.scrollLeft,
      scrollTop: el.scrollTop,
      w: el.clientWidth,
      h: el.clientHeight,
    });
  }, []);

  const {
    plano, zonaActiva, zonaData, mesasZona, mesaSeleccionada, arrastrando,
    posicionesLocales, setMesaSeleccionada, cambiarZona, zoom, setZoom,
    canvasHeight,
    handleCrearZona, handleEliminarZona, handleEditarZona,
    handleCrearMesa, handleGuardarMesa, handleEliminarMesa,
    handleDragStart, handleDragEnd,
    handleExportar, handleImportar,
    handleCrearCombinacion, handleEliminarCombinacion,
    dialogoEntrada, setDialogoEntrada,
    dialogoConfirmar, setDialogoConfirmar,
    dialogoCombinacion, setDialogoCombinacion,
  } = usePlanoSala(canvasRef);

  /* [303A-13] Resize del canvas arrastrando el borde inferior */
  const { resizing, onResizeStart } = useCanvasResize({ canvasHeight, setCanvasHeight });

  /* [303A-12] Pan del canvas shift+drag o middle-click */
  const { panning, onPanMouseDown } = useCanvasPan(viewportRef);

  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 5 } })
  );

  /* [283A-17] Handler unificado de drag: distingue mesa template vs mesa existente.
   * Siempre llama handleDragStart para que 'arrastrando' (del hook) se setee. */
  const onDragStart = (event: DragStartEvent) => {
    handleDragStart(event);
  };

  const onDragEnd = (event: DragEndEvent) => {
    const id = String(event.active.id);
    if (id === 'new-mesa-template') {
      if (!canvasRef.current || !zonaActiva || !zonaData) return;
      const rect = canvasRef.current.getBoundingClientRect();
      const pe = event.activatorEvent as PointerEvent;
      /* [283A-25] Coordenadas de pantalla → canónicas dividiendo por zoom.
       * Bounds usan ancho real del canvas / zoom. */
      const rawX = Math.round((pe.clientX + event.delta.x - rect.left) / zoom);
      const rawY = Math.round((pe.clientY + event.delta.y - rect.top) / zoom);
      const canvasWidth = canvasRef.current.clientWidth;
      const x = Math.min(canvasWidth / zoom - 80, Math.max(0, rawX));
      const y = Math.min(zonaData.alto - 80, Math.max(0, rawY));
      handleCrearMesa({ x, y });
    } else {
      handleDragEnd(event);
    }
  };

  return (
    <DndContext sensors={sensors} onDragStart={onDragStart} onDragEnd={onDragEnd}>
    <div className="flex flex-col gap-4">
      {/* [283A-17] Toolbar: Mesa draggable + acciones zona + Combinación + Export/Import */}
      <div className="flex items-center gap-2 flex-wrap">
        <MesaTemplate disabled={!zonaActiva} />
        {zonaActiva && (
          <>
            <Button size="sm" variant="ghost" onClick={handleEditarZona}><Pencil className="size-4 mr-1" />Renombrar</Button>
            <Button size="sm" variant="destructive" onClick={handleEliminarZona}><Trash2 className="size-4 mr-1" />Zona</Button>
          </>
        )}
        <Button size="sm" variant="ghost" onClick={handleCrearCombinacion} disabled={!zonaActiva || mesasZona.length < 2}>
          <Combine className="size-4 mr-1" />Combinación
        </Button>
        <div className="ml-auto flex gap-2">
          <Button size="sm" variant="ghost" onClick={handleExportar}><Download className="size-4 mr-1" />Exportar</Button>
          <Button size="sm" variant="ghost" onClick={handleImportar}><Upload className="size-4 mr-1" />Importar</Button>
        </div>
        {/* [283A-25] Controles de zoom para escalar el canvas visualmente */}
        <div className="flex items-center gap-1">
          <Button size="sm" variant="outline" onClick={() => setZoom(z => Math.max(0.5, +(z - 0.1).toFixed(2)))}><ZoomOut className="size-4" /></Button>
          <span className="text-xs w-12 text-center tabular-nums">{Math.round(zoom * 100)}%</span>
          <Button size="sm" variant="outline" onClick={() => setZoom(z => Math.min(2, +(z + 0.1).toFixed(2)))}><ZoomIn className="size-4" /></Button>
        </div>
      </div>

      {/* Zonas (tabs) */}
      <div className="flex gap-1 flex-wrap items-center border-b border-border">
        {plano?.zonas.map(z => (
          <button
            key={z.id}
            type="button"
            className={`relative inline-flex items-center justify-center gap-1.5 px-3 py-1.5 text-sm font-medium whitespace-nowrap transition-colors rounded-t-md ${z.id === zonaActiva ? 'text-foreground after:absolute after:inset-x-0 after:bottom-[-1px] after:h-0.5 after:bg-foreground' : 'text-muted-foreground hover:text-foreground'}`}
            onClick={() => cambiarZona(z.id)}
          >
            {z.nombre} ({z.mesas.length})
          </button>
        ))}
        <button
          type="button"
          className="inline-flex items-center gap-1 px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors rounded-t-md"
          onClick={handleCrearZona}
        >
          + Zona
        </button>
      </div>

      {/* [283A-17] Canvas 100% ancho + Panel lateral derecho */}
      {/* [303A-11] Canvas viewport con altura fija del store, overflow:auto.
         * Inner div con minHeight = zonaData.alto * zoom para que el scroll
         * funcione cuando el contenido zoom-in excede el viewport. */}
      <div className="flex gap-4">
        <div className="flex-1 min-w-0">
          {zonaData ? (
            <div style={{ position: 'relative' }}>
              <div
                ref={viewportRef}
                className={`planoCanvas ${panning ? 'planoPanning' : ''}`}
                style={{ height: canvasHeight }}
                onClick={() => setMesaSeleccionada(null)}
                onScroll={onScroll}
                onMouseDown={onPanMouseDown}
              >
                <div
                  ref={canvasRef}
                  className="planoCanvasContent"
                  style={{ minHeight: Math.max(canvasHeight, zonaData.alto * zoom) }}
                >
                  {mesasZona.map(mesa => {
                    const pos = posicionesLocales[mesa.id];
                    const base = pos ? { ...mesa, pos_x: pos.x, pos_y: pos.y } : mesa;
                    const mesaZoom = zoom === 1 ? base : {
                      ...base,
                      pos_x: base.pos_x * zoom,
                      pos_y: base.pos_y * zoom,
                      ancho: base.ancho * zoom,
                      alto: base.alto * zoom,
                    };
                    return (
                      <MesaDraggable
                        key={mesa.id}
                        mesa={mesaZoom}
                        seleccionada={mesaSeleccionada?.id === mesa.id}
                        arrastrando={arrastrando === mesa.id}
                        onClick={() => setMesaSeleccionada(mesa)}
                      />
                    );
                  })}
                </div>
              </div>
              {/* [303A-12] Minimap + indicadores off-screen */}
              <CanvasMinimap
                mesas={mesasZona.map(m => {
                  const p = posicionesLocales[m.id];
                  return {
                    x: (p?.x ?? m.pos_x) * zoom,
                    y: (p?.y ?? m.pos_y) * zoom,
                    ancho: m.ancho * zoom,
                    alto: m.alto * zoom,
                  };
                })}
                contentWidth={canvasRef.current?.scrollWidth ?? 0}
                contentHeight={canvasRef.current?.scrollHeight ?? 0}
                viewportWidth={canvasViewState.w || (viewportRef.current?.clientWidth ?? 0)}
                viewportHeight={canvasViewState.h || (viewportRef.current?.clientHeight ?? 0)}
                scrollLeft={canvasViewState.scrollLeft}
                scrollTop={canvasViewState.scrollTop}
                onNavigate={(sl, st) => {
                  if (viewportRef.current) {
                    viewportRef.current.scrollLeft = sl;
                    viewportRef.current.scrollTop = st;
                  }
                }}
              />
              <OffScreenIndicators
                mesas={mesasZona.map(m => {
                  const p = posicionesLocales[m.id];
                  return {
                    x: (p?.x ?? m.pos_x) * zoom,
                    y: (p?.y ?? m.pos_y) * zoom,
                    ancho: m.ancho * zoom,
                    alto: m.alto * zoom,
                  };
                })}
                viewportWidth={canvasViewState.w || (viewportRef.current?.clientWidth ?? 0)}
                viewportHeight={canvasViewState.h || (viewportRef.current?.clientHeight ?? 0)}
                scrollLeft={canvasViewState.scrollLeft}
                scrollTop={canvasViewState.scrollTop}
              />
            </div>
          ) : (
            <div className="planoCanvas planoCanvasVacio" style={{ height: canvasHeight }}>
              {plano && plano.zonas.length === 0
                ? 'Crea tu primera zona para empezar a diseñar el plano'
                : 'Selecciona una zona'}
            </div>
          )}
          {/* [303A-13] Handle de resize — barra arrastrable debajo del canvas */}
          <div
            className={`planoResizeHandle ${resizing ? 'activo' : ''}`}
            onMouseDown={onResizeStart}
            title="Arrastrar para cambiar altura del plano"
          />
        </div>
        {/* [283A-17] Panel lateral derecho: config de mesa seleccionada */}
        {/* [303A-9] key={mesa.id} fuerza remount al cambiar de mesa, reseteando
         * el useState interno para que min/max personas se actualicen. */}
        {mesaSeleccionada && (
          <div className="w-72 shrink-0">
            <PanelConfigMesa
              key={mesaSeleccionada.id}
              mesa={mesaSeleccionada}
              onGuardar={handleGuardarMesa}
              onEliminar={handleEliminarMesa}
              onCerrar={() => setMesaSeleccionada(null)}
            />
          </div>
        )}
      </div>

      {/* Combinaciones */}
      {plano && plano.combinaciones.length > 0 && (
        <div className="flex flex-col gap-2">
          <h3 className="font-semibold text-sm">Combinaciones de mesas</h3>
          {plano.combinaciones.map(c => (
            <div key={c.id} className="flex items-center justify-between rounded-md border p-3">
              <div>
                <strong className="text-sm">{c.nombre}</strong>
                <p className="text-xs text-muted-foreground">
                  {c.min_personas}-{c.max_personas} personas · {c.mesas.map(m => `Mesa ${m.numero}`).join(', ')}
                </p>
              </div>
              <Button size="sm" variant="destructive" onClick={() => handleEliminarCombinacion(c.id)}>
                <Trash2 className="size-4" />
              </Button>
            </div>
          ))}
        </div>
      )}
      {/* [283A-25] Diálogos extraídos a PlanoDialogs para mantener < 300 líneas */}
      <PlanoDialogs
        dialogoEntrada={dialogoEntrada} setDialogoEntrada={setDialogoEntrada}
        dialogoConfirmar={dialogoConfirmar} setDialogoConfirmar={setDialogoConfirmar}
        dialogoCombinacion={dialogoCombinacion} setDialogoCombinacion={setDialogoCombinacion}
        mesasZona={mesasZona}
      />
    </div>
    {/* [283A-17] DragOverlay: preview visual mientras se arrastra la mesa template */}
    <DragOverlay>
      {arrastrando === 'new-mesa-template' && (
        <div className="planoMesa" style={{ width: 80, height: 80, position: 'relative' }}>
          <span className="planoMesaNumero">?</span>
        </div>
      )}
    </DragOverlay>
    </DndContext>
  );
}

export default PlanoSala;
