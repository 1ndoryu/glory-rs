/* [263A-16] Lista de reservas — reescrita con shadcn Table + Dialog + Badge.
 * Filtros con shadcn Input y select nativo. Plano de ocupación integrado. */

import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Badge } from '@/components/ui/badge';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Trash2, Pencil } from 'lucide-react';
import FormularioReserva from './FormularioReserva';
import PlanoOcupacion from './PlanoOcupacion';
import useVistaReservas from '../hooks/useVistaReservas';

const ETIQUETA_ESTADO: Record<string, string> = {
  pendiente: 'Pendiente',
  confirmada: 'Confirmada',
  cancelada: 'Cancelada',
  completada: 'Completada',
  no_show: 'No Show',
  lista_espera: 'Lista de espera',
};

const VARIANTE_ESTADO: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  confirmada: 'default',
  pendiente: 'secondary',
  completada: 'outline',
  cancelada: 'destructive',
  no_show: 'destructive',
  lista_espera: 'secondary',
};

function ListaReservas() {
  const {
    filtros,
    cambiarFiltro,
    modalAbierto,
    setModalAbierto,
    reservas,
    isLoading,
    eliminarMutation,
    actualizarMutation,
    cerrarModalYRefrescar,
    porPagina,
    reservaEditando,
    abrirEdicion,
  } = useVistaReservas();

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <p className="text-sm text-muted-foreground">{reservas ? `${reservas.total} reservas` : ''}</p>
        <Button onClick={() => setModalAbierto(true)}>+ Nueva Reserva</Button>
      </div>

      {/* Filtros */}
      <div className="flex flex-wrap gap-3">
        <Input
          type="search"
          placeholder="Buscar por nombre, apellidos o teléfono..."
          value={filtros.busqueda}
          onChange={(e) => cambiarFiltro('busqueda', e.target.value)}
          className="max-w-xs"
        />
        <Input
          type="date"
          value={filtros.fecha}
          onChange={(e) => cambiarFiltro('fecha', e.target.value)}
          className="max-w-40"
          title="Fecha desde"
        />
        <Input
          type="date"
          value={filtros.fechaHasta}
          onChange={(e) => cambiarFiltro('fechaHasta', e.target.value)}
          className="max-w-40"
          title="Fecha hasta (dejar vacío para un solo día)"
          placeholder="Hasta"
        />
        <Select value={filtros.turno || '__all__'} onValueChange={v => cambiarFiltro('turno', v === '__all__' ? '' : v)}>
          <SelectTrigger className="max-w-40"><SelectValue /></SelectTrigger>
          <SelectContent>
            <SelectItem value="__all__">Día completo</SelectItem>
            <SelectItem value="desayuno">Desayuno</SelectItem>
            <SelectItem value="comida">Comida</SelectItem>
            <SelectItem value="cena">Cena</SelectItem>
          </SelectContent>
        </Select>
        <Select value={filtros.estado || '__all__'} onValueChange={v => cambiarFiltro('estado', v === '__all__' ? '' : v)}>
          <SelectTrigger className="max-w-44"><SelectValue /></SelectTrigger>
          <SelectContent>
            <SelectItem value="__all__">Todos los estados</SelectItem>
            <SelectItem value="confirmada">Confirmadas</SelectItem>
            <SelectItem value="pendiente">Pendientes</SelectItem>
            <SelectItem value="lista_espera">Lista de espera</SelectItem>
            <SelectItem value="completada">Completadas</SelectItem>
            <SelectItem value="no_show">No Show</SelectItem>
            <SelectItem value="cancelada">Canceladas</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <Dialog open={modalAbierto} onOpenChange={(open) => {
        setModalAbierto(open);
        if (!open) cerrarModalYRefrescar();
      }}>
        <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{reservaEditando ? 'Editar Reserva' : 'Nueva Reserva'}</DialogTitle>
          </DialogHeader>
          {/* [024A-5] key fuerza remount al cambiar de reserva o al crear nueva */}
          <FormularioReserva
            key={reservaEditando?.id ?? 'nueva'}
            onExito={cerrarModalYRefrescar}
            reserva={reservaEditando ?? undefined}
          />
        </DialogContent>
      </Dialog>

      {isLoading ? (
        <p className="text-sm text-muted-foreground">Cargando...</p>
      ) : reservas && reservas.items.length > 0 ? (
        <>
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Mesa</TableHead>
                  {filtros.fechaHasta && <TableHead>Fecha</TableHead>}
                  <TableHead>Hora</TableHead>
                  <TableHead>Nombre</TableHead>
                  <TableHead>Apellidos</TableHead>
                  <TableHead>Personas</TableHead>
                  <TableHead>Estado</TableHead>
                  <TableHead>Teléfono</TableHead>
                  <TableHead className="w-10"></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {reservas.items.map((r) => (
                  <TableRow key={r.id}>
                    <TableCell>{r.num_mesa ?? '—'}</TableCell>
                    {filtros.fechaHasta && <TableCell>{r.fecha}</TableCell>}
                    <TableCell>{r.hora}</TableCell>
                    <TableCell>{r.nombre_cliente}</TableCell>
                    <TableCell className="max-w-32 truncate">{r.apellidos_cliente || '—'}</TableCell>
                    <TableCell>{r.num_personas}</TableCell>
                    {/* [024A-2] Select inline para cambiar estado directamente desde la lista */}
                    <TableCell>
                      <Select
                        value={r.estado}
                        onValueChange={(nuevoEstado) => {
                          if (nuevoEstado !== r.estado) {
                            actualizarMutation.mutate({ id: r.id, data: { estado: nuevoEstado as 'pendiente' | 'confirmada' | 'cancelada' | 'completada' | 'no_show' | 'lista_espera' } });
                          }
                        }}
                      >
                        <SelectTrigger className="h-7 w-auto min-w-28 border-0 bg-transparent px-1">
                          <Badge variant={VARIANTE_ESTADO[r.estado] ?? 'outline'}>
                            {ETIQUETA_ESTADO[r.estado] ?? r.estado}
                          </Badge>
                        </SelectTrigger>
                        <SelectContent>
                          {Object.entries(ETIQUETA_ESTADO).map(([valor, etiqueta]) => (
                            <SelectItem key={valor} value={valor}>{etiqueta}</SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </TableCell>
                    <TableCell>{r.telefono || '—'}</TableCell>
                    <TableCell>
                      <div className="flex items-center gap-1">
                        {/* [024A-5] Botón para editar todos los campos de la reserva */}
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => abrirEdicion(r)}
                          title="Editar reserva"
                        >
                          <Pencil className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => eliminarMutation.mutate({ id: r.id })}
                          disabled={eliminarMutation.isPending}
                          title="Eliminar reserva"
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
            <span className="text-sm text-muted-foreground">Página {filtros.pagina} de {Math.ceil(reservas.total / porPagina)}</span>
            <Button variant="outline" size="sm" disabled={filtros.pagina * porPagina >= reservas.total} onClick={() => cambiarFiltro('pagina', filtros.pagina + 1)}>Siguiente</Button>
          </div>
        </>
      ) : (
        <p className="text-sm text-muted-foreground">No hay reservas para {filtros.fechaHasta ? 'este rango de fechas' : 'esta fecha'}</p>
      )}

      <PlanoOcupacion fecha={filtros.fecha} turno={filtros.turno} />
    </div>
  );
}

export default ListaReservas;
