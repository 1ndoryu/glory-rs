/* [263A-16] PlanoSala — Constructor visual de plano de sala con drag-and-drop.
 * Reescrito imports a shadcn Button. Mantiene PlanoSala.css para canvas/mesas.
 * Lógica en usePlanoSala, mesa arrastrable en MesaDraggable, config en PanelConfigMesa. */

import { useRef } from 'react';
import { DndContext, PointerSensor, useSensor, useSensors } from '@dnd-kit/core';
import { Button } from '@/components/ui/button';
import MesaDraggable from './plano-sala/MesaDraggable';
import PanelConfigMesa from './plano-sala/PanelConfigMesa';
import { usePlanoSala } from './plano-sala/usePlanoSala';
import '../estilos/PlanoSala.css';

function PlanoSala() {
  const {
    plano, zonaActiva, zonaData, mesasZona, mesaSeleccionada, arrastrando,
    posicionesLocales, setMesaSeleccionada, cambiarZona,
    handleCrearZona, handleEliminarZona, handleEditarZona,
    handleCrearMesa, handleGuardarMesa, handleEliminarMesa,
    handleDragStart, handleDragEnd,
    handleExportar, handleImportar,
    handleCrearCombinacion, handleEliminarCombinacion,
  } = usePlanoSala();

  const canvasRef = useRef<HTMLDivElement>(null);
  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 5 } })
  );

  return (
    <div className="flex flex-col gap-4">
      {/* Barra de herramientas */}
      <div className="flex items-center gap-2 flex-wrap">
        <Button size="sm" onClick={handleCrearMesa} disabled={!zonaActiva}>+ Mesa</Button>
        {zonaActiva && (
          <>
            <Button size="sm" variant="ghost" onClick={handleEditarZona}>Renombrar zona</Button>
            <Button size="sm" variant="destructive" onClick={handleEliminarZona}>Eliminar zona</Button>
          </>
        )}
        <div className="ml-auto flex gap-2">
          <Button size="sm" variant="ghost" onClick={handleExportar}>Exportar</Button>
          <Button size="sm" variant="ghost" onClick={handleImportar}>Importar</Button>
        </div>
      </div>

      {/* Zonas (tabs) */}
      <div className="flex gap-1 flex-wrap border-b">
        {plano?.zonas.map(z => (
          <Button
            key={z.id}
            variant="ghost"
            size="sm"
            className={`rounded-b-none ${z.id === zonaActiva ? 'border-b-2 border-primary font-semibold' : ''}`}
            onClick={() => cambiarZona(z.id)}
          >
            {z.nombre} ({z.mesas.length})
          </Button>
        ))}
        <Button variant="ghost" size="sm" className="rounded-b-none text-muted-foreground" onClick={handleCrearZona}>
          + Zona
        </Button>
      </div>

      {/* Info de zona */}
      {zonaData && (
        <p className="text-sm text-muted-foreground">
          {zonaData.nombre} — {mesasZona.length} mesas — {zonaData.ancho}&times;{zonaData.alto}px
        </p>
      )}

      {/* Canvas con mesas */}
      {zonaData ? (
        <DndContext sensors={sensors} onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
          <div
            ref={canvasRef}
            className="planoCanvas"
            style={{ width: zonaData.ancho, height: zonaData.alto }}
            onClick={() => setMesaSeleccionada(null)}
          >
            {mesasZona.map(mesa => {
              const pos = posicionesLocales[mesa.id];
              const mesaConPos = pos ? { ...mesa, pos_x: pos.x, pos_y: pos.y } : mesa;
              return (
                <MesaDraggable
                  key={mesa.id}
                  mesa={mesaConPos}
                  seleccionada={mesaSeleccionada?.id === mesa.id}
                  arrastrando={arrastrando === mesa.id}
                  onClick={() => setMesaSeleccionada(mesa)}
                />
              );
            })}
          </div>
        </DndContext>
      ) : (
        <div className="planoCanvas planoCanvasVacio">
          {plano && plano.zonas.length === 0
            ? 'Crea tu primera zona para empezar a diseñar el plano'
            : 'Selecciona una zona'}
        </div>
      )}

      {/* Panel de configuración de mesa seleccionada */}
      {mesaSeleccionada && (
        <PanelConfigMesa
          mesa={mesaSeleccionada}
          onGuardar={handleGuardarMesa}
          onEliminar={handleEliminarMesa}
          onCerrar={() => setMesaSeleccionada(null)}
        />
      )}

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
                &times;
              </Button>
            </div>
          ))}
        </div>
      )}
      <Button size="sm" variant="ghost" onClick={handleCrearCombinacion}>+ Combinación de mesas</Button>
    </div>
  );
}

export default PlanoSala;
