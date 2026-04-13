/* [134A-3] Página de gestión de reseñas — review gating.
 * Lista admin de reseñas recibidas + botón para solicitar nueva reseña.
 * Las reseñas con 4-5 estrellas se marcan como "redirigidas a Google"
 * si el restaurante tiene configurada la URL de Google Reviews. */

import { useState } from 'react';
import { useResenas } from '../hooks/useResenas';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Star, Send, ChevronLeft, ChevronRight } from 'lucide-react';

function Estrellas({ puntuacion }: { puntuacion: number | null | undefined }) {
  if (!puntuacion) return <span className="text-muted-foreground text-sm">Sin respuesta</span>;
  return (
    <div className="flex gap-0.5">
      {[1, 2, 3, 4, 5].map((n) => (
        <Star
          key={n}
          className={`size-4 ${n <= puntuacion ? 'fill-yellow-400 text-yellow-400' : 'text-muted-foreground'}`}
        />
      ))}
    </div>
  );
}

export default function ListaResenas() {
  const {
    resenas,
    total,
    page,
    totalPages,
    isLoading,
    setPage,
    solicitarResena,
    solicitando,
  } = useResenas();

  const [modalSolicitar, setModalSolicitar] = useState(false);

  const handleSolicitar = () => {
    solicitarResena();
    setModalSolicitar(false);
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <p className="text-sm text-muted-foreground">{total} reseña{total !== 1 ? 's' : ''}</p>
        <Button onClick={() => setModalSolicitar(true)}>
          <Send className="size-4 mr-1" /> Solicitar Reseña
        </Button>
      </div>

      <Dialog open={modalSolicitar} onOpenChange={setModalSolicitar}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Solicitar Reseña</DialogTitle>
          </DialogHeader>
          <div className="flex flex-col gap-3">
            <p className="text-sm text-muted-foreground">
              Genera un enlace único para que el cliente deje su valoración.
              Si puntúa 4 o 5 estrellas, será redirigido a Google Reviews.
            </p>
            <Button onClick={handleSolicitar} disabled={solicitando}>
              {solicitando ? 'Generando...' : 'Generar enlace'}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {isLoading ? (
        <p className="text-sm text-muted-foreground">Cargando...</p>
      ) : resenas.length > 0 ? (
        <>
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Cliente</TableHead>
                  <TableHead>Puntuación</TableHead>
                  <TableHead>Comentario</TableHead>
                  <TableHead>Google</TableHead>
                  <TableHead>Fecha</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {resenas.map((r) => (
                  <TableRow key={r.id}>
                    <TableCell className="font-medium">{r.cliente_nombre || 'Anónimo'}</TableCell>
                    <TableCell>
                      <Estrellas puntuacion={r.puntuacion} />
                    </TableCell>
                    <TableCell className="max-w-xs truncate">
                      {r.comentario || '—'}
                    </TableCell>
                    <TableCell>
                      {r.redirigido_google ? (
                        <Badge variant="default">Redirigido</Badge>
                      ) : r.respondida_at ? (
                        <Badge variant="secondary">Local</Badge>
                      ) : (
                        <Badge variant="outline">Pendiente</Badge>
                      )}
                    </TableCell>
                    <TableCell className="text-sm text-muted-foreground">
                      {new Date(r.created_at).toLocaleDateString('es-ES')}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
          {totalPages > 1 && (
            <div className="flex items-center justify-center gap-2">
              <Button variant="outline" size="icon" disabled={page <= 1} onClick={() => setPage(page - 1)}>
                <ChevronLeft className="size-4" />
              </Button>
              <span className="text-sm">{page} / {totalPages}</span>
              <Button variant="outline" size="icon" disabled={page >= totalPages} onClick={() => setPage(page + 1)}>
                <ChevronRight className="size-4" />
              </Button>
            </div>
          )}
        </>
      ) : (
        <div className="text-center py-12 text-muted-foreground">
          <Star className="size-12 mx-auto mb-3 opacity-30" />
          <p>No hay reseñas todavía</p>
          <p className="text-sm mt-1">Solicita reseñas a tus clientes para mejorar tu reputación online</p>
        </div>
      )}
    </div>
  );
}
