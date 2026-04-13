/* [134A-15] Lista de combinaciones de mesas extraída de PlanoSala.
 * Muestra las combinaciones activas con botón de eliminar. */

import { Button } from '@/components/ui/button';
import { Trash2 } from 'lucide-react';
import type { CombinacionConMesas } from '../../api/generated';

interface Props {
  combinaciones: CombinacionConMesas[];
  onEliminar: (id: string) => void;
}

export default function CombinacionesList({ combinaciones, onEliminar }: Props) {
  if (combinaciones.length === 0) return null;
  return (
    <div className="flex flex-col gap-2">
      <h3 className="font-semibold text-sm">Combinaciones de mesas</h3>
      {combinaciones.map(c => (
        <div key={c.id} className="flex items-center justify-between rounded-md border p-3">
          <div>
            <strong className="text-sm">{c.nombre}</strong>
            <p className="text-xs text-muted-foreground">
              {c.min_personas}-{c.max_personas} personas · {c.mesas.map(m => `Mesa ${m.numero}`).join(', ')}
            </p>
          </div>
          <Button size="sm" variant="destructive" onClick={() => onEliminar(c.id)}>
            <Trash2 className="size-4" />
          </Button>
        </div>
      ))}
    </div>
  );
}
