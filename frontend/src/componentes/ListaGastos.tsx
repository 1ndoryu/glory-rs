/* [263A-16] Lista de gastos — reescrita con shadcn Table + Dialog + Button.
 * Iconos para eliminar (tarea 17). Columnas en flex (tarea 18). */

import { useState } from 'react';
import { useListarGastos, useEliminarGasto } from '../api/generated';
import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Trash2 } from 'lucide-react';
import FormularioGasto from './FormularioGasto';

function formatearMoneda(valor: string): string {
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(valor));
}

function ListaGastos() {
  const [pagina, setPagina] = useState(1);
  const [modalAbierto, setModalAbierto] = useState(false);
  const porPagina = 15;

  const { data, isLoading, refetch } = useListarGastos({ page: pagina, per_page: porPagina });
  const eliminarMutation = useEliminarGasto({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const gastos = data?.status === 200 ? data.data : null;

  const cerrarModalYRefrescar = () => {
    setModalAbierto(false);
    refetch();
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-muted-foreground">{gastos ? `${gastos.total} registros` : ''}</p>
        </div>
        <Button onClick={() => setModalAbierto(true)}>+ Nuevo Gasto</Button>
      </div>

      <Dialog open={modalAbierto} onOpenChange={setModalAbierto}>
        <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nuevo Gasto</DialogTitle>
          </DialogHeader>
          <FormularioGasto onExito={cerrarModalYRefrescar} />
        </DialogContent>
      </Dialog>

      {isLoading ? (
        <p className="text-sm text-muted-foreground">Cargando...</p>
      ) : gastos && gastos.items.length > 0 ? (
        <>
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Fecha</TableHead>
                  <TableHead>Proveedor</TableHead>
                  <TableHead>Tipo doc.</TableHead>
                  <TableHead>Método</TableHead>
                  <TableHead className="text-right">Base</TableHead>
                  <TableHead className="text-right">IVA</TableHead>
                  <TableHead className="text-right">Total</TableHead>
                  <TableHead className="w-10"></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {gastos.items.map((g) => (
                  <TableRow key={g.id}>
                    <TableCell>{g.fecha}</TableCell>
                    <TableCell>{g.proveedor || '—'}</TableCell>
                    <TableCell className="capitalize">{g.tipo_documento}</TableCell>
                    <TableCell className="capitalize">{g.metodo_pago}</TableCell>
                    <TableCell className="text-right">{formatearMoneda(g.importe_base)}</TableCell>
                    <TableCell className="text-right">{formatearMoneda(g.importe_iva)}</TableCell>
                    <TableCell className="text-right font-medium">{formatearMoneda((parseFloat(g.importe_base) + parseFloat(g.importe_iva)).toFixed(2))}</TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => eliminarMutation.mutate({ id: g.id })}
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
            <span className="text-sm text-muted-foreground">Página {pagina} de {Math.ceil(gastos.total / porPagina)}</span>
            <Button variant="outline" size="sm" disabled={pagina * porPagina >= gastos.total} onClick={() => setPagina(pagina + 1)}>Siguiente</Button>
          </div>
        </>
      ) : (
        <p className="text-sm text-muted-foreground">No hay gastos registrados</p>
      )}
    </div>
  );
}

export default ListaGastos;
