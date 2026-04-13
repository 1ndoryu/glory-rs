/* [134A-3] Panel de configuración de paredes en el plano de sala.
 * Permite editar dimensiones, color y rotación de una pared seleccionada. */

import { useState, type FormEvent } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Trash2, X } from 'lucide-react';
import type { ParedSala, ActualizarParedRequest } from '../../api/generated/gestionRestauranteAPI.schemas';

interface Props {
  pared: ParedSala;
  onGuardar: (id: string, req: ActualizarParedRequest) => void;
  onEliminar: (id: string) => void;
  onCerrar: () => void;
}

export default function PanelConfigPared({ pared, onGuardar, onEliminar, onCerrar }: Props) {
  const [ancho, setAncho] = useState(pared.ancho);
  const [alto, setAlto] = useState(pared.alto);
  const [color, setColor] = useState(pared.color || '#6b7280');
  const [rotacion, setRotacion] = useState(pared.rotacion);
  const [posX, setPosX] = useState(pared.pos_x);
  const [posY, setPosY] = useState(pared.pos_y);

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    onGuardar(pared.id, { ancho, alto, color, rotacion, pos_x: posX, pos_y: posY });
  };

  return (
    <div className="rounded-md border p-4 space-y-3">
      <div className="flex items-center justify-between">
        <h3 className="font-semibold text-sm">Configurar pared</h3>
        <Button size="icon" variant="ghost" onClick={onCerrar}>
          <X className="size-4" />
        </Button>
      </div>
      <form onSubmit={handleSubmit} className="space-y-3">
        <div className="grid grid-cols-2 gap-2">
          <div>
            <Label htmlFor="pared-ancho">Ancho</Label>
            <Input id="pared-ancho" type="number" min={10} value={ancho} onChange={e => setAncho(+e.target.value)} />
          </div>
          <div>
            <Label htmlFor="pared-alto">Alto</Label>
            <Input id="pared-alto" type="number" min={5} value={alto} onChange={e => setAlto(+e.target.value)} />
          </div>
        </div>
        <div className="grid grid-cols-2 gap-2">
          <div>
            <Label htmlFor="pared-x">Pos X</Label>
            <Input id="pared-x" type="number" min={0} value={posX} onChange={e => setPosX(+e.target.value)} />
          </div>
          <div>
            <Label htmlFor="pared-y">Pos Y</Label>
            <Input id="pared-y" type="number" min={0} value={posY} onChange={e => setPosY(+e.target.value)} />
          </div>
        </div>
        <div>
          <Label htmlFor="pared-color">Color</Label>
          <div className="flex gap-2 items-center">
            <input
              id="pared-color"
              type="color"
              value={color}
              onChange={e => setColor(e.target.value)}
              className="h-8 w-10 rounded border cursor-pointer"
            />
            <Input value={color} onChange={e => setColor(e.target.value)} className="flex-1" />
          </div>
        </div>
        <div>
          <Label htmlFor="pared-rotacion">Rotación (°)</Label>
          <Input id="pared-rotacion" type="number" min={0} max={360} value={rotacion} onChange={e => setRotacion(+e.target.value)} />
        </div>
        <div className="flex gap-2">
          <Button type="submit" size="sm" className="flex-1">Guardar</Button>
          <Button type="button" size="sm" variant="destructive" onClick={() => onEliminar(pared.id)}>
            <Trash2 className="size-4" />
          </Button>
        </div>
      </form>
    </div>
  );
}
