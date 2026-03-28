/* [263A-16] Lista de ventas — reescrita con shadcn Table + Dialog + Button.
 * Iconos para eliminar en vez de texto (tarea 17).
 * [283A-22] Botón de edición con Dialog reutilizando FormularioVenta.
 * [283A-28] Filtros de fecha (desde/hasta) extraídos a useListaVentas hook. */

import useListaVentas from '../hooks/useListaVentas';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Trash2, Pencil } from 'lucide-react';
import FormularioVenta from './FormularioVenta';

/* [283A-47] Mapa de etiquetas para turnos — el enum backend usa ascii ("manana")
 * pero la UI debe mostrar tildes ("Mañana"). */
const ETIQUETAS_TURNO: Record<string, string> = {
  manana: 'Mañana',
  mediodia: 'Mediodía',
  noche: 'Noche',
};

function formatearMoneda(valor: string): string {
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(valor));
}

function ListaVentas() {
  const {
    filtros,
    cambiarFiltro,
    modalAbierto,
    setModalAbierto,
    ventaEditando,
    setVentaEditando,
    porPagina,
    ventas,
    isLoading,
    eliminarMutation,
    cerrarModalYRefrescar,
    cerrarEdicionYRefrescar,
  } = useListaVentas();

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-muted-foreground">{ventas ? `${ventas.total} registros` : ''}</p>
        </div>
        <Button onClick={() => setModalAbierto(true)}>+ Nueva Venta</Button>
      </div>

      {/* Filtros de fecha */}
      <div className="flex flex-wrap gap-3 items-center">
        <label className="text-sm text-muted-foreground">Desde</label>
        <Input
          type="date"
          value={filtros.desde}
          onChange={(e) => cambiarFiltro('desde', e.target.value)}
          className="max-w-40"
        />
        <label className="text-sm text-muted-foreground">Hasta</label>
        <Input
          type="date"
          value={filtros.hasta}
          onChange={(e) => cambiarFiltro('hasta', e.target.value)}
          className="max-w-40"
        />
      </div>

      <Dialog open={modalAbierto} onOpenChange={setModalAbierto}>
        <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nueva Venta</DialogTitle>
          </DialogHeader>
          <FormularioVenta onExito={cerrarModalYRefrescar} />
        </DialogContent>
      </Dialog>

      <Dialog open={!!ventaEditando} onOpenChange={(open) => { if (!open) setVentaEditando(null); }}>
        <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Editar Venta</DialogTitle>
          </DialogHeader>
          {ventaEditando && (
            <FormularioVenta onExito={cerrarEdicionYRefrescar} venta={ventaEditando} />
          )}
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
                  <TableHead className="w-20"></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {ventas.items.map((v) => (
                  <TableRow key={v.id}>
                    <TableCell>{v.fecha}</TableCell>
                    <TableCell>{ETIQUETAS_TURNO[v.turno] ?? v.turno}</TableCell>
                    <TableCell className="capitalize">{v.canal}</TableCell>
                    <TableCell className="capitalize">{v.metodo_pago}</TableCell>
                    <TableCell className="text-right">{formatearMoneda(v.importe_base)}</TableCell>
                    <TableCell className="text-right">{formatearMoneda(v.importe_iva)}</TableCell>
                    <TableCell className="text-right font-medium">{formatearMoneda((parseFloat(v.importe_base) + parseFloat(v.importe_iva)).toFixed(2))}</TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => setVentaEditando(v)}
                          title="Editar"
                        >
                          <Pencil className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => eliminarMutation.mutate({ id: v.id })}
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
            <span className="text-sm text-muted-foreground">Página {filtros.pagina} de {Math.ceil(ventas.total / porPagina)}</span>
            <Button variant="outline" size="sm" disabled={filtros.pagina * porPagina >= ventas.total} onClick={() => cambiarFiltro('pagina', filtros.pagina + 1)}>Siguiente</Button>
          </div>
        </>
      ) : (
        <p className="text-sm text-muted-foreground">No hay ventas registradas</p>
      )}
    </div>
  );
}

export default ListaVentas;
