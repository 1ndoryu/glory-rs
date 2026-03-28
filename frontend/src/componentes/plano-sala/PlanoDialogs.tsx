/* [283A-25] Diálogos del plano de sala extraídos de PlanoSala.tsx
 * para mantener el componente principal bajo 300 líneas.
 * entradaValor y combForm son estado local de este componente. */

import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter, DialogDescription } from '@/components/ui/dialog';
import type { Mesa } from '../../api/generated';
import type { DialogoEntrada, DialogoConfirmar, DialogoCombinacion } from './usePlanoSala';

interface Props {
  dialogoEntrada: DialogoEntrada | null;
  setDialogoEntrada: (d: DialogoEntrada | null) => void;
  dialogoConfirmar: DialogoConfirmar | null;
  setDialogoConfirmar: (d: DialogoConfirmar | null) => void;
  dialogoCombinacion: DialogoCombinacion | null;
  setDialogoCombinacion: (d: DialogoCombinacion | null) => void;
  mesasZona: Mesa[];
}

function PlanoDialogs({
  dialogoEntrada, setDialogoEntrada,
  dialogoConfirmar, setDialogoConfirmar,
  dialogoCombinacion, setDialogoCombinacion,
  mesasZona,
}: Props) {
  const [entradaValor, setEntradaValor] = useState('');
  const [combForm, setCombForm] = useState({ nombre: '', maxP: '', mesas: new Set<string>() });

  useEffect(() => {
    if (dialogoEntrada) setEntradaValor(dialogoEntrada.valorInicial);
  }, [dialogoEntrada]);

  useEffect(() => {
    if (dialogoCombinacion) setCombForm({ nombre: '', maxP: '', mesas: new Set() });
  }, [dialogoCombinacion]);

  const toggleCombMesa = (id: string) => {
    setCombForm(prev => {
      const next = new Set(prev.mesas);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return { ...prev, mesas: next };
    });
  };

  return (
    <>
      {/* Diálogo de entrada (reemplaza prompt nativo) */}
      <Dialog open={!!dialogoEntrada} onOpenChange={(open) => { if (!open) setDialogoEntrada(null); }}>
        <DialogContent className="max-w-sm">
          <DialogHeader><DialogTitle>{dialogoEntrada?.titulo}</DialogTitle></DialogHeader>
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
      <Dialog open={!!dialogoConfirmar} onOpenChange={(open) => { if (!open) setDialogoConfirmar(null); }}>
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

      {/* Diálogo de combinación con selección visual de mesas */}
      <Dialog
        open={!!dialogoCombinacion}
        onOpenChange={(open) => { if (!open) { setDialogoCombinacion(null); setCombForm({ nombre: '', maxP: '', mesas: new Set() }); } }}
      >
        <DialogContent className="max-w-sm">
          <DialogHeader><DialogTitle>Nueva combinación de mesas</DialogTitle></DialogHeader>
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
                  >Mesa {m.numero}</Button>
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
    </>
  );
}

export default PlanoDialogs;
