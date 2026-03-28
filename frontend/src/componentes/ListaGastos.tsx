/* [263A-16] Lista de gastos — reescrita con shadcn Table + Dialog + Button.
 * Iconos para eliminar (tarea 17). Columnas en flex (tarea 18).
 * [283A-22] Botón de edición con Dialog reutilizando FormularioGasto.
 * [283A-34] Filtros de fecha (desde/hasta) via hook useListaGastos. */

import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Trash2, Pencil } from 'lucide-react';
import FormularioGasto from './FormularioGasto';
import useListaGastos from '@/hooks/useListaGastos';

function formatearMoneda(valor: string): string {
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(valor));
}

function ListaGastos() {
  const {
    filtros, cambiarFiltro,
    modalAbierto, setModalAbierto,
    gastoEditando, setGastoEditando,
    porPagina, gastos, isLoading,
    eliminarMutation,
    cerrarModalYRefrescar,
    cerrarEdicionYRefrescar,
  } = useListaGastos();

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-wrap items-end justify-between gap-4">
        <div className="flex flex-wrap items-end gap-3">
          <div>
            <Label htmlFor="gasto-desde" className="text-xs mb-1">Desde</Label>
            <Input id="gasto-desde" type="date" className="w-40" value={filtros.desde} onChange={e => cambiarFiltro('desde', e.target.value)} />
          </div>
          <div>
            <Label htmlFor="gasto-hasta" className="text-xs mb-1">Hasta</Label>
            <Input id="gasto-hasta" type="date" className="w-40" value={filtros.hasta} onChange={e => cambiarFiltro('hasta', e.target.value)} />
          </div>
          <p className="text-sm text-muted-foreground pb-1">{gastos ? `${gastos.total} registros` : ''}</p>
        </div>
        <Button onClick={() => setModalAbierto(true)}>+ Nuevo Gasto</Button>
      </div>

      <Dialog open={modalAbierto} onOpenChange={setModalAbierto}>
        {/* [283A-30] sm:max-w-lg = 512px (tarea 18) */}
        <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nuevo Gasto</DialogTitle>
          </DialogHeader>
          <FormularioGasto onExito={cerrarModalYRefrescar} />
        </DialogContent>
      </Dialog>

      {/* [283A-22] Dialog de edición — reutiliza FormularioGasto con gasto pre-cargado */}
      <Dialog open={!!gastoEditando} onOpenChange={(open) => { if (!open) setGastoEditando(null); }}>
        <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Editar Gasto</DialogTitle>
          </DialogHeader>
          {gastoEditando && (
            <FormularioGasto onExito={cerrarEdicionYRefrescar} gasto={gastoEditando} />
          )}
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
                  <TableHead className="w-20"></TableHead>
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
                      <div className="flex gap-1">
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => setGastoEditando(g)}
                          title="Editar"
                        >
                          <Pencil className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => eliminarMutation.mutate({ id: g.id })}
                          disabled={eliminarMutation.isPending}
                          title="Eliminar"
                        >
                          <Trash2 className="size-4 text-destructive" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>

          <div className="flex items-center justify-between">
            <Button variant="outline" size="sm" disabled={filtros.pagina <= 1} onClick={() => cambiarFiltro('pagina', filtros.pagina - 1)}>Anterior</Button>
            <span className="text-sm text-muted-foreground">Página {filtros.pagina} de {Math.ceil(gastos.total / porPagina)}</span>
            <Button variant="outline" size="sm" disabled={filtros.pagina * porPagina >= gastos.total} onClick={() => cambiarFiltro('pagina', filtros.pagina + 1)}>Siguiente</Button>
          </div>
        </>
      ) : (
        <p className="text-sm text-muted-foreground">No hay gastos registrados</p>
      )}
    </div>
  );
}

export default ListaGastos;
