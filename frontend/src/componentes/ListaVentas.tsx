/* [263A-16] Lista de ventas — reescrita con shadcn Table + Dialog + Button.
 * Iconos para eliminar en vez de texto (tarea 17). */

import { useState } from 'react';
import { useListarVentas, useEliminarVenta } from '../api/generated';
import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Trash2 } from 'lucide-react';
import FormularioVenta from './FormularioVenta';

function formatearMoneda(valor: string): string {
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(valor));
}

function ListaVentas() {
  const [pagina, setPagina] = useState(1);
  const [modalAbierto, setModalAbierto] = useState(false);
  const porPagina = 15;

  const { data, isLoading, refetch } = useListarVentas({ page: pagina, per_page: porPagina });
  const eliminarMutation = useEliminarVenta({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const ventas = data?.status === 200 ? data.data : null;

  const cerrarModalYRefrescar = () => {
    setModalAbierto(false);
    refetch();
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-muted-foreground">{ventas ? `${ventas.total} registros` : ''}</p>
        </div>
        <Button onClick={() => setModalAbierto(true)}>+ Nueva Venta</Button>
      </div>

      <Dialog open={modalAbierto} onOpenChange={setModalAbierto}>
        <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nueva Venta</DialogTitle>
          </DialogHeader>
          <FormularioVenta onExito={cerrarModalYRefrescar} />
        </DialogContent>
      </Dialog>

      {isLoading ? (
        <p className="text-sm text-muted-foreground">Cargando...</p>
      ) : ventas && ventas.items.length > 0 ? (
        <>
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Fecha</TableHead>
                  <TableHead>Turno</TableHead>
                  <TableHead>Canal</TableHead>
                  <TableHead>Método</TableHead>
                  <TableHead className="text-right">Base</TableHead>
                  <TableHead className="text-right">IVA</TableHead>
                  <TableHead className="text-right">Total</TableHead>
                  <TableHead className="w-10"></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {ventas.items.map((v) => (
                  <TableRow key={v.id}>
                    <TableCell>{v.fecha}</TableCell>
                    <TableCell className="capitalize">{v.turno}</TableCell>
                    <TableCell className="capitalize">{v.canal}</TableCell>
                    <TableCell className="capitalize">{v.metodo_pago}</TableCell>
                    <TableCell className="text-right">{formatearMoneda(v.importe_base)}</TableCell>
                    <TableCell className="text-right">{formatearMoneda(v.importe_iva)}</TableCell>
                    <TableCell className="text-right font-medium">{formatearMoneda((parseFloat(v.importe_base) + parseFloat(v.importe_iva)).toFixed(2))}</TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => eliminarMutation.mutate({ id: v.id })}
                        disabled={eliminarMutation.isPending}
                      >
                        <Trash2 className="size-4 text-destructive" />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>

          <div className="flex items-center justify-between">
            <Button variant="outline" size="sm" disabled={pagina <= 1} onClick={() => setPagina(pagina - 1)}>Anterior</Button>
            <span className="text-sm text-muted-foreground">Página {pagina} de {Math.ceil(ventas.total / porPagina)}</span>
            <Button variant="outline" size="sm" disabled={pagina * porPagina >= ventas.total} onClick={() => setPagina(pagina + 1)}>Siguiente</Button>
          </div>
        </>
      ) : (
        <p className="text-sm text-muted-foreground">No hay ventas registradas</p>
      )}
    </div>
  );
}

export default ListaVentas;
