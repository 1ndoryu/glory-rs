/* [064A-9] Diálogo de detalle/edición de reserva asociada a una venta.
 * Extraído de ListaVentas para reducir líneas del componente principal.
 * [034A-5] Visor de reserva con datos read-only.
 * [044A-11] Botón para editar reserva directamente desde aquí. */

import { useState } from 'react';
import { Pencil } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import type { Reserva } from '../api/generated/gestionRestauranteAPI.schemas';
import FormularioReserva from './FormularioReserva';

interface Props {
  reservaId: string | null;
  onClose: () => void;
  reserva: Reserva | null;
  cargando: boolean;
}

function ReservaViewer({ reservaId, onClose, reserva, cargando }: Props) {
  const [editando, setEditando] = useState(false);

  const cerrar = () => {
    setEditando(false);
    onClose();
  };

  return (
    <Dialog open={!!reservaId} onOpenChange={(open) => { if (!open) cerrar(); }}>
      <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>{editando ? 'Editar Reserva' : 'Reserva Asociada'}</DialogTitle>
        </DialogHeader>
        {editando && reserva ? (
          <FormularioReserva reserva={reserva} onExito={cerrar} />
        ) : cargando ? (
          <p className="text-sm text-muted-foreground">Cargando reserva...</p>
        ) : reserva ? (
          <div className="flex flex-col gap-3 text-sm">
            <div className="grid grid-cols-2 gap-2">
              <div>
                <span className="text-muted-foreground">Cliente</span>
                <p className="font-medium">{reserva.nombre_cliente} {reserva.apellidos_cliente}</p>
              </div>
              <div>
                <span className="text-muted-foreground">Telefono</span>
                <p className="font-medium">{reserva.telefono || '\u2014'}</p>
              </div>
            </div>
            <div className="grid grid-cols-3 gap-2">
              <div>
                <span className="text-muted-foreground">Fecha</span>
                <p className="font-medium">{reserva.fecha}</p>
              </div>
              <div>
                <span className="text-muted-foreground">Hora</span>
                <p className="font-medium">{reserva.hora}</p>
              </div>
              <div>
                <span className="text-muted-foreground">Personas</span>
                <p className="font-medium">{reserva.num_personas}</p>
              </div>
            </div>
            <div className="grid grid-cols-2 gap-2">
              <div>
                <span className="text-muted-foreground">Estado</span>
                <p><Badge variant="outline">{reserva.estado}</Badge></p>
              </div>
              <div>
                <span className="text-muted-foreground">Mesa</span>
                <p className="font-medium">{reserva.num_mesa ?? '\u2014'}</p>
              </div>
            </div>
            {reserva.notas && (
              <div>
                <span className="text-muted-foreground">Notas</span>
                <p className="font-medium">{reserva.notas}</p>
              </div>
            )}
            <Button variant="outline" size="sm" className="self-end mt-2" onClick={() => setEditando(true)}>
              <Pencil className="size-3.5 mr-1.5" /> Editar reserva
            </Button>
          </div>
        ) : (
          <p className="text-sm text-muted-foreground">No se encontro la reserva</p>
        )}
      </DialogContent>
    </Dialog>
  );
}

export default ReservaViewer;
