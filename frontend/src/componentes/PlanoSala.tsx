/* [263A-16] PlanoSala — Constructor visual de plano de sala con drag-and-drop.
 * [263A-28] Diálogos nativos reemplazados por shadcn Dialog + toast.
 * Lógica en usePlanoSala, mesa arrastrable en MesaDraggable, config en PanelConfigMesa. */

import { useRef, useState, useEffect } from 'react';
import { DndContext, PointerSensor, useSensor, useSensors } from '@dnd-kit/core';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter, DialogDescription } from '@/components/ui/dialog';
import { Plus, Pencil, Trash2, Download, Upload, Combine } from 'lucide-react';
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
    dialogoEntrada, setDialogoEntrada,
    dialogoConfirmar, setDialogoConfirmar,
    dialogoCombinacion, setDialogoCombinacion,
  } = usePlanoSala();

  /* Estado local para inputs de los diálogos */
  const [entradaValor, setEntradaValor] = useState('');
  const [combNombre, setCombNombre] = useState('');
  const [combMaxP, setCombMaxP] = useState('');

  /* Sincronizar valor inicial al abrir diálogo de entrada */
  useEffect(() => {
    if (dialogoEntrada) setEntradaValor(dialogoEntrada.valorInicial);
  }, [dialogoEntrada]);

  const canvasRef = useRef<HTMLDivElement>(null);
  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 5 } })
  );

  return (
    <div className="flex flex-col gap-4">
      {/* Barra de herramientas */}
      <div className="flex items-center gap-2 flex-wrap">
        <Button size="sm" onClick={handleCrearMesa} disabled={!zonaActiva}><Plus className="size-4 mr-1" />Mesa</Button>
        {zonaActiva && (
          <>
            <Button size="sm" variant="ghost" onClick={handleEditarZona}><Pencil className="size-4 mr-1" />Renombrar</Button>
            <Button size="sm" variant="destructive" onClick={handleEliminarZona}><Trash2 className="size-4 mr-1" />Zona</Button>
          </>
        )}
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
                <Trash2 className="size-4" />
              </Button>
            </div>
          ))}
        </div>
      )}
      <Button size="sm" variant="ghost" onClick={handleCrearCombinacion}><Combine className="size-4 mr-1" />Combinación</Button>

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

      {/* Diálogo para crear combinación (2 campos) */}
      <Dialog
        open={!!dialogoCombinacion}
        onOpenChange={(open) => { if (!open) { setDialogoCombinacion(null); setCombNombre(''); setCombMaxP(''); } }}
      >
        <DialogContent className="max-w-sm">
          <DialogHeader>
            <DialogTitle>Nueva combinación de mesas</DialogTitle>
          </DialogHeader>
          <div className="flex flex-col gap-3">
            <div className="flex flex-col gap-2">
              <Label>Nombre</Label>
              <Input value={combNombre} onChange={e => setCombNombre(e.target.value)} autoFocus />
            </div>
            <div className="flex flex-col gap-2">
              <Label>Máx personas</Label>
              <Input type="number" value={combMaxP} onChange={e => setCombMaxP(e.target.value)} />
            </div>
          </div>
          <DialogFooter>
            <Button variant="ghost" onClick={() => { setDialogoCombinacion(null); setCombNombre(''); setCombMaxP(''); }}>Cancelar</Button>
            <Button onClick={() => {
              if (combNombre.trim() && combMaxP) {
                dialogoCombinacion?.onConfirmar(combNombre.trim(), Number(combMaxP));
                setDialogoCombinacion(null);
                setCombNombre('');
                setCombMaxP('');
              }
            }}>Crear</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}

export default PlanoSala;
