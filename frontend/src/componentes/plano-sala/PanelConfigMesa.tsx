/* [263A-16] Panel lateral para editar propiedades de una mesa seleccionada — shadcn */

import { useState } from 'react';
import type { ActualizarMesaRequest, Mesa } from '../../api/generated';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { MIN_LADO_MESA, RATIO_RECTANGULAR } from './mesaGeometry';

interface PanelConfigMesaProps {
  mesa: Mesa;
  onGuardar: (id: string, data: ActualizarMesaRequest) => void;
  onEliminar: (id: string) => void;
  onCerrar: () => void;
}

function PanelConfigMesa({ mesa, onGuardar, onEliminar, onCerrar }: PanelConfigMesaProps) {
  const [form, setForm] = useState({
    numero: mesa.numero,
    minP: mesa.min_personas,
    maxP: mesa.max_personas,
    forma: mesa.forma,
    ancho: mesa.ancho,
    alto: mesa.alto,
    activa: mesa.activa,
  });

  /* [313A-1] La forma debe producir un cambio visual real.
   * Se fuerzan proporciones claras para evitar que rectangular quede casi
   * igual a cuadrada y para que redonda/cuadrada no hereden deformaciones. */
  const set = (campo: string, valor: number | string | boolean) =>
    setForm((prev) => {
      const next = { ...prev, [campo]: valor };
      const ladoBase = Math.max(next.ancho, next.alto, MIN_LADO_MESA);

      if (campo === 'forma') {
        if (valor === 'cuadrada') {
          next.ancho = ladoBase;
          next.alto = ladoBase;
        } else if (valor === 'rectangular') {
          next.alto = Math.max(MIN_LADO_MESA, Math.round(ladoBase * 0.9));
          next.ancho = Math.round(next.alto * RATIO_RECTANGULAR);
        } else if (valor === 'redonda') {
          next.ancho = ladoBase;
          next.alto = ladoBase;
        }
      } else if ((campo === 'ancho' || campo === 'alto') && (next.forma === 'cuadrada' || next.forma === 'redonda')) {
        const lado = Math.max(next.ancho, next.alto, MIN_LADO_MESA);
        next.ancho = lado;
        next.alto = lado;
      } else if ((campo === 'ancho' || campo === 'alto') && next.forma === 'rectangular') {
        next.alto = Math.max(MIN_LADO_MESA, next.alto);
        next.ancho = Math.max(next.ancho, Math.round(next.alto * RATIO_RECTANGULAR));
      }
      return next;
    });

  const guardar = () => {
    onGuardar(mesa.id, {
      numero: form.numero,
      min_personas: form.minP,
      max_personas: form.maxP,
      forma: form.forma,
      ancho: form.ancho,
      alto: form.alto,
      activa: form.activa,
    });
  };

  return (
    <div className="rounded-lg border bg-card p-4 flex flex-col gap-3 sticky top-4">
      <h3 className="font-semibold">Mesa {mesa.numero}</h3>
      <div className="flex flex-col gap-2">
        <Label htmlFor="cfg-numero">Número</Label>
        <Input id="cfg-numero" type="number" value={form.numero} onChange={e => set('numero', Number(e.target.value))} />
      </div>
      <div className="grid grid-cols-2 gap-3">
        <div className="flex flex-col gap-2">
          <Label htmlFor="cfg-minP">Mín personas</Label>
          <Input id="cfg-minP" type="number" value={form.minP} onChange={e => set('minP', Number(e.target.value))} />
        </div>
        <div className="flex flex-col gap-2">
          <Label htmlFor="cfg-maxP">Máx personas</Label>
          <Input id="cfg-maxP" type="number" value={form.maxP} onChange={e => set('maxP', Number(e.target.value))} />
        </div>
      </div>
      <div className="flex flex-col gap-2">
        <Label htmlFor="cfg-forma">Forma</Label>
        <Select value={form.forma} onValueChange={v => set('forma', v)}>
          <SelectTrigger id="cfg-forma"><SelectValue /></SelectTrigger>
          <SelectContent>
            <SelectItem value="cuadrada">Cuadrada</SelectItem>
            <SelectItem value="redonda">Redonda</SelectItem>
            <SelectItem value="rectangular">Rectangular</SelectItem>
          </SelectContent>
        </Select>
      </div>
      <div className="grid grid-cols-2 gap-3">
        <div className="flex flex-col gap-2">
          <Label htmlFor="cfg-ancho">Ancho (px)</Label>
          <Input id="cfg-ancho" type="number" value={form.ancho} onChange={e => set('ancho', Number(e.target.value))} />
        </div>
        <div className="flex flex-col gap-2">
          <Label htmlFor="cfg-alto">Alto (px)</Label>
          <Input id="cfg-alto" type="number" value={form.alto} onChange={e => set('alto', Number(e.target.value))} />
        </div>
      </div>
      <div className="flex items-center gap-2">
        <Switch id="cfg-activa" checked={form.activa} onCheckedChange={checked => set('activa', checked)} />
        <Label htmlFor="cfg-activa">Activa</Label>
      </div>
      <div className="flex gap-2 pt-2">
        <Button size="sm" onClick={guardar}>Guardar</Button>
        <Button size="sm" variant="destructive" onClick={() => onEliminar(mesa.id)}>Eliminar</Button>
        <Button size="sm" variant="ghost" onClick={onCerrar}>Cerrar</Button>
      </div>
    </div>
  );
}

export default PanelConfigMesa;
