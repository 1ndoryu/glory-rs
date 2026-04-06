/* [263A-16] Lista de ventas — reescrita con shadcn Table + Dialog + Button.
 * Iconos para eliminar en vez de texto (tarea 17).
 * [283A-22] Botón de edición con Dialog reutilizando FormularioVenta.
 * [283A-28] Filtros de fecha (desde/hasta) extraídos a useListaVentas hook.
 * [034A-5] Columna "Cliente" con nombre_cliente + botón para ver reserva asociada.
 * [044A-8+9] Buscador + cabeceras de columna clicables para ordenar (sort_by/sort_order).
 * [064A-3] Filtros por columna en Turno, Canal y Método de pago con ColumnFilterHeader. */

import { useState } from 'react';
import useListaVentas from '../hooks/useListaVentas';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Trash2, Pencil, Eye } from 'lucide-react';
import { Badge } from '@/components/ui/badge';
import FormularioVenta from './FormularioVenta';
import FormularioReserva from './FormularioReserva';
import ColumnFilterHeader from '@/components/column-filter-header';

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
    toggleSort,
    cambiarFiltroColumna,
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
    reservaIdViewer,
    setReservaIdViewer,
    reservaDetalle,
    reservaCargando,
  } = useListaVentas();

  /* [044A-11] Estado para alternar entre vista y edición en el dialog de Reserva Asociada */
  const [editandoReserva, setEditandoReserva] = useState(false);

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-muted-foreground">{ventas ? `${ventas.total} registros` : ''}</p>
        </div>
        <Button onClick={() => setModalAbierto(true)}>+ Nueva Venta</Button>
      </div>

      {/* [044A-9] Buscador + filtros de fecha */}
      <div className="flex flex-wrap gap-3 items-center">
        <Input
          type="search"
          placeholder="Buscar por descripción, cliente, turno, canal..."
          value={filtros.busqueda}
          onChange={(e) => cambiarFiltro('busqueda', e.target.value)}
          className="max-w-xs"
        />
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

      {/* [034A-5] Diálogo de detalle de la reserva asociada a una venta
         [044A-11] Con botón para editar la reserva directamente desde aquí */}
      <Dialog open={!!reservaIdViewer} onOpenChange={(open) => { if (!open) { setReservaIdViewer(null); setEditandoReserva(false); } }}>
        <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{editandoReserva ? 'Editar Reserva' : 'Reserva Asociada'}</DialogTitle>
          </DialogHeader>
          {editandoReserva && reservaDetalle ? (
            <FormularioReserva
              reserva={reservaDetalle}
              onExito={() => { setEditandoReserva(false); setReservaIdViewer(null); }}
            />
          ) : reservaCargando ? (
            <p className="text-sm text-muted-foreground">Cargando reserva...</p>
          ) : reservaDetalle ? (
            <div className="flex flex-col gap-3 text-sm">
              <div className="grid grid-cols-2 gap-2">
                <div>
                  <span className="text-muted-foreground">Cliente</span>
                  <p className="font-medium">{reservaDetalle.nombre_cliente} {reservaDetalle.apellidos_cliente}</p>
                </div>
                <div>
                  <span className="text-muted-foreground">Teléfono</span>
                  <p className="font-medium">{reservaDetalle.telefono || '—'}</p>
                </div>
              </div>
              <div className="grid grid-cols-3 gap-2">
                <div>
                  <span className="text-muted-foreground">Fecha</span>
                  <p className="font-medium">{reservaDetalle.fecha}</p>
                </div>
                <div>
                  <span className="text-muted-foreground">Hora</span>
                  <p className="font-medium">{reservaDetalle.hora}</p>
                </div>
                <div>
                  <span className="text-muted-foreground">Personas</span>
                  <p className="font-medium">{reservaDetalle.num_personas}</p>
                </div>
              </div>
              <div className="grid grid-cols-2 gap-2">
                <div>
                  <span className="text-muted-foreground">Estado</span>
                  <p><Badge variant="outline">{reservaDetalle.estado}</Badge></p>
                </div>
                <div>
                  <span className="text-muted-foreground">Mesa</span>
                  <p className="font-medium">{reservaDetalle.num_mesa ?? '—'}</p>
                </div>
              </div>
              {reservaDetalle.notas && (
                <div>
                  <span className="text-muted-foreground">Notas</span>
                  <p className="font-medium">{reservaDetalle.notas}</p>
                </div>
              )}
              <Button variant="outline" size="sm" className="self-end mt-2" onClick={() => setEditandoReserva(true)}>
                <Pencil className="size-3.5 mr-1.5" /> Editar reserva
              </Button>
            </div>
          ) : (
            <p className="text-sm text-muted-foreground">No se encontró la reserva</p>
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
                  <TableHead className="cursor-pointer select-none" onClick={() => toggleSort('fecha')}>
                    Fecha {filtros.sortBy === 'fecha' && (filtros.sortOrder === 'asc' ? '↑' : '↓')}
                  </TableHead>
                  <TableHead className="cursor-pointer select-none" onClick={() => toggleSort('nombre_cliente')}>
                    Cliente {filtros.sortBy === 'nombre_cliente' && (filtros.sortOrder === 'asc' ? '↑' : '↓')}
                  </TableHead>
                  <TableHead>
                    <ColumnFilterHeader
                      title="Turno"
                      sortKey="turno"
                      currentSortBy={filtros.sortBy}
                      currentSortOrder={filtros.sortOrder}
                      onSort={() => toggleSort('turno')}
                      options={[
                        { value: 'manana', label: 'Mañana' },
                        { value: 'mediodia', label: 'Mediodía' },
                        { value: 'noche', label: 'Noche' },
                      ]}
                      selectedValues={filtros.turno}
                      onFilterChange={(v) => cambiarFiltroColumna('turno', v)}
                    />
                  </TableHead>
                  <TableHead>
                    <ColumnFilterHeader
                      title="Canal"
                      sortKey="canal"
                      currentSortBy={filtros.sortBy}
                      currentSortOrder={filtros.sortOrder}
                      onSort={() => toggleSort('canal')}
                      options={[
                        { value: 'comedor', label: 'Comedor' },
                        { value: 'barra', label: 'Barra' },
                        { value: 'terraza', label: 'Terraza' },
                        { value: 'delivery', label: 'Delivery' },
                        { value: 'just_eat', label: 'Just Eat' },
                        { value: 'eventos', label: 'Eventos' },
                      ]}
                      selectedValues={filtros.canal}
                      onFilterChange={(v) => cambiarFiltroColumna('canal', v)}
                    />
                  </TableHead>
                  <TableHead>
                    <ColumnFilterHeader
                      title="Método"
                      sortKey="metodo_pago"
                      currentSortBy={filtros.sortBy}
                      currentSortOrder={filtros.sortOrder}
                      onSort={() => toggleSort('metodo_pago')}
                      options={[
                        { value: 'efectivo', label: 'Efectivo' },
                        { value: 'tarjeta', label: 'Tarjeta' },
                        { value: 'transferencia', label: 'Transferencia' },
                      ]}
                      selectedValues={filtros.metodoPago}
                      onFilterChange={(v) => cambiarFiltroColumna('metodoPago', v)}
                    />
                  </TableHead>
                  <TableHead className="text-right cursor-pointer select-none" onClick={() => toggleSort('importe_base')}>
                    Base {filtros.sortBy === 'importe_base' && (filtros.sortOrder === 'asc' ? '↑' : '↓')}
                  </TableHead>
                  <TableHead className="text-right">IVA</TableHead>
                  <TableHead className="text-right">Total</TableHead>
                  <TableHead className="w-28"></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {ventas.items.map((v) => (
                  <TableRow key={v.id}>
                    <TableCell>{v.fecha}</TableCell>
                    <TableCell>{v.nombre_cliente ?? '—'}</TableCell>
                    <TableCell>{ETIQUETAS_TURNO[v.turno] ?? v.turno}</TableCell>
                    <TableCell className="capitalize">{v.canal}</TableCell>
                    <TableCell className="capitalize">{v.metodo_pago}</TableCell>
                    <TableCell className="text-right">{formatearMoneda(v.importe_base)}</TableCell>
                    <TableCell className="text-right">{formatearMoneda(v.importe_iva)}</TableCell>
                    <TableCell className="text-right font-medium">{formatearMoneda((parseFloat(v.importe_base) + parseFloat(v.importe_iva)).toFixed(2))}</TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        {/* [034A-5] Botón para ver la reserva de origen (solo si existe) */}
                        {v.reserva_id && (
                          <Button
                            variant="ghost"
                            size="icon"
                            onClick={() => setReservaIdViewer(v.reserva_id!)}
                            title="Ver reserva"
                          >
                            <Eye className="size-4" />
                          </Button>
                        )}
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
