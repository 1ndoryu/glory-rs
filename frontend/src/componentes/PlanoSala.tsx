/* [263A-16] PlanoSala — Constructor visual de plano de sala con drag-and-drop.
 * [263A-28] Diálogos nativos reemplazados por shadcn Dialog + toast.
 * Lógica en usePlanoSala, mesa arrastrable en MesaDraggable, config en PanelConfigMesa. */

import { useRef, useState, useEffect } from 'react';
import { DndContext, DragOverlay, PointerSensor, useSensor, useSensors } from '@dnd-kit/core';
import type { DragEndEvent, DragStartEvent } from '@dnd-kit/core';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter, DialogDescription } from '@/components/ui/dialog';
import { Pencil, Trash2, Download, Upload, Combine } from 'lucide-react';
import MesaDraggable from './plano-sala/MesaDraggable';
import MesaTemplate from './plano-sala/MesaTemplate';
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
    dialogoEntrada, setDialogoEntrada,
    dialogoConfirmar, setDialogoConfirmar,
    dialogoCombinacion, setDialogoCombinacion,
  } = usePlanoSala();

  /* Estado local para inputs de los diálogos — combForm agrupa nombre, maxP y mesas
   * para mantener max 3 useState (regla SRP). dragActivo se reutiliza via arrastrando del hook. */
  const [entradaValor, setEntradaValor] = useState('');
  const [combForm, setCombForm] = useState({ nombre: '', maxP: '', mesas: new Set<string>() });

  /* Sincronizar valor inicial al abrir diálogo de entrada */
  useEffect(() => {
    if (dialogoEntrada) setEntradaValor(dialogoEntrada.valorInicial);
  }, [dialogoEntrada]);

  useEffect(() => {
    if (dialogoCombinacion) setCombForm({ nombre: '', maxP: '', mesas: new Set() });
  }, [dialogoCombinacion]);

  const canvasRef = useRef<HTMLDivElement>(null);
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
      /* [283A-24] Clamp para que la mesa nueva no caiga fuera del canvas */
      const rawX = Math.round(pe.clientX + event.delta.x - rect.left);
      const rawY = Math.round(pe.clientY + event.delta.y - rect.top);
      const x = Math.min(zonaData.ancho - 80, Math.max(0, rawX));
      const y = Math.min(zonaData.alto - 80, Math.max(0, rawY));
      handleCrearMesa({ x, y });
    } else {
      handleDragEnd(event);
    }
  };

  const toggleCombMesa = (id: string) => {
    setCombForm(prev => {
      const next = new Set(prev.mesas);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return { ...prev, mesas: next };
    });
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
      <div className="flex gap-4">
        <div className="flex-1 min-w-0">
          {zonaData ? (
            <div
              ref={canvasRef}
              className="planoCanvas"
              style={{ height: zonaData.alto }}
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
          ) : (
            <div className="planoCanvas planoCanvasVacio">
              {plano && plano.zonas.length === 0
                ? 'Crea tu primera zona para empezar a diseñar el plano'
                : 'Selecciona una zona'}
            </div>
          )}
        </div>
        {/* [283A-17] Panel lateral derecho: config de mesa seleccionada */}
        {mesaSeleccionada && (
          <div className="w-72 shrink-0">
            <PanelConfigMesa
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
      {/* Diálogo de entrada (reemplaza prompt nativo) */}
      <Dialog
        open={!!dialogoEntrada}
        onOpenChange={(open) => { if (!open) setDialogoEntrada(null); }}
      >
        <DialogContent className="max-w-sm">
          <DialogHeader>
            <DialogTitle>{dialogoEntrada?.titulo}</DialogTitle>
          </DialogHeader>
          <div className="flex flex-col gap-2">
            <Label>{dialogoEntrada?.label}</Label>
            <Input
              type={dialogoEntrada?.tipo ?? 'text'}
              value={entradaValor}
              onChange={e => setEntradaValor(e.target.value)}
              onKeyDown={e => {
                if (e.key === 'Enter' && entradaValor.trim()) {
                  dialogoEntrada?.onConfirmar(entradaValor.trim());
                  setDialogoEntrada(null);
                  setEntradaValor('');
                }
              }}
              autoFocus
            />
          </div>
          <DialogFooter>
            <Button variant="ghost" onClick={() => { setDialogoEntrada(null); setEntradaValor(''); }}>Cancelar</Button>
            <Button onClick={() => {
              if (entradaValor.trim()) {
                dialogoEntrada?.onConfirmar(entradaValor.trim());
                setDialogoEntrada(null);
                setEntradaValor('');
              }
            }}>Aceptar</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Diálogo de confirmación (reemplaza confirm nativo) */}
      <Dialog
        open={!!dialogoConfirmar}
        onOpenChange={(open) => { if (!open) setDialogoConfirmar(null); }}
      >
        <DialogContent className="max-w-sm">
          <DialogHeader>
            <DialogTitle>{dialogoConfirmar?.titulo}</DialogTitle>
            <DialogDescription>{dialogoConfirmar?.mensaje}</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="ghost" onClick={() => setDialogoConfirmar(null)}>Cancelar</Button>
            <Button variant="destructive" onClick={() => {
              dialogoConfirmar?.onConfirmar();
              setDialogoConfirmar(null);
            }}>Confirmar</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* [283A-17] Diálogo de combinación con selección visual de mesas */}
      <Dialog
        open={!!dialogoCombinacion}
        onOpenChange={(open) => { if (!open) { setDialogoCombinacion(null); setCombForm({ nombre: '', maxP: '', mesas: new Set() }); } }}
      >
        <DialogContent className="max-w-sm">
          <DialogHeader>
            <DialogTitle>Nueva combinación de mesas</DialogTitle>
          </DialogHeader>
          <div className="flex flex-col gap-3">
            <div className="flex flex-col gap-2">
              <Label>Nombre</Label>
              <Input value={combForm.nombre} onChange={e => setCombForm(p => ({ ...p, nombre: e.target.value }))} autoFocus />
            </div>
            <div className="flex flex-col gap-2">
              <Label>Máx personas</Label>
              <Input type="number" value={combForm.maxP} onChange={e => setCombForm(p => ({ ...p, maxP: e.target.value }))} />
            </div>
            <div className="flex flex-col gap-2">
              <Label>Mesas a combinar</Label>
              <div className="flex flex-wrap gap-1.5">
                {mesasZona.map(m => (
                  <Button
                    key={m.id}
                    size="sm"
                    variant={combForm.mesas.has(m.id) ? 'default' : 'outline'}
                    onClick={() => toggleCombMesa(m.id)}
                  >
                    Mesa {m.numero}
                  </Button>
                ))}
              </div>
            </div>
          </div>
          <DialogFooter>
            <Button variant="ghost" onClick={() => { setDialogoCombinacion(null); setCombForm({ nombre: '', maxP: '', mesas: new Set() }); }}>Cancelar</Button>
            <Button
              disabled={combForm.mesas.size < 2 || !combForm.nombre.trim() || !combForm.maxP}
              onClick={() => {
                if (combForm.nombre.trim() && combForm.maxP && combForm.mesas.size >= 2) {
                  dialogoCombinacion?.onConfirmar(combForm.nombre.trim(), Number(combForm.maxP), Array.from(combForm.mesas));
                  setDialogoCombinacion(null);
                  setCombForm({ nombre: '', maxP: '', mesas: new Set() });
                }
              }}
            >Crear</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
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
