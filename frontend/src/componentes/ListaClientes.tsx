/* [263A-16] Lista de clientes — reescrita con shadcn Table + Dialog + Input.
 * [263A-26] Agregado: seleccionar 2 clientes y fusionarlos (merge).
 * CRM con búsqueda, paginación, modal crear/editar, merge duplicados. */

import { useState } from 'react';
import useListaClientes from '../hooks/useListaClientes';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter, DialogDescription } from '@/components/ui/dialog';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Checkbox } from '@/components/ui/checkbox';
import { Trash2, Pencil, Merge } from 'lucide-react';
import FormularioCliente from './FormularioCliente';

function ListaClientes() {
  const {
    pagina,
    setPagina,
    busqueda,
    buscar,
    modalCrear,
    setModalCrear,
    clienteEditar,
    setClienteEditar,
    porPagina,
    clientes,
    isLoading,
    eliminarMut,
    cerrarModalYRefrescar,
    seleccionados,
    toggleSeleccion,
    modalMerge,
    setModalMerge,
    mergeMut,
  } = useListaClientes();

  /* [263A-26] En el diálogo de merge el usuario elige quién sobrevive (destino) */
  const [destinoId, setDestinoId] = useState<string | null>(null);

  const clientesSeleccionados = clientes?.items.filter((c) => seleccionados.includes(c.id)) ?? [];

  const ejecutarMerge = () => {
    if (seleccionados.length !== 2 || !destinoId) return;
    const origenId = seleccionados.find((id) => id !== destinoId);
    if (!origenId) return;
    mergeMut.mutate({ data: { origen_id: origenId, destino_id: destinoId } });
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <p className="text-sm text-muted-foreground">{clientes ? `${clientes.total} registros` : ''}</p>
          {seleccionados.length === 2 && (
            <Button
              variant="outline"
              size="sm"
              onClick={() => { setDestinoId(null); setModalMerge(true); }}
            >
              <Merge className="size-4 mr-1" /> Fusionar seleccionados
            </Button>
          )}
          {seleccionados.length > 0 && seleccionados.length < 2 && (
            <span className="text-xs text-muted-foreground">Selecciona otro cliente para fusionar</span>
          )}
        </div>
        <Button onClick={() => setModalCrear(true)}>+ Nuevo Cliente</Button>
      </div>

      <Input
        type="search"
        placeholder="Buscar por nombre, apellidos, teléfono o email..."
        value={busqueda}
        onChange={(e) => buscar(e.target.value)}
        className="max-w-md"
      />

      <Dialog open={modalCrear} onOpenChange={setModalCrear}>
        <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nuevo Cliente</DialogTitle>
          </DialogHeader>
          <FormularioCliente onExito={cerrarModalYRefrescar} />
        </DialogContent>
      </Dialog>

      <Dialog open={!!clienteEditar} onOpenChange={(open) => { if (!open) setClienteEditar(null); }}>
        <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Editar Cliente</DialogTitle>
          </DialogHeader>
          {clienteEditar && <FormularioCliente onExito={cerrarModalYRefrescar} cliente={clienteEditar} />}
        </DialogContent>
      </Dialog>

      {isLoading ? (
        <p className="text-sm text-muted-foreground">Cargando...</p>
      ) : clientes && clientes.items.length > 0 ? (
        <>
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="w-10"></TableHead>
                  <TableHead>Nombre</TableHead>
                  <TableHead>Teléfono</TableHead>
                  <TableHead>Email</TableHead>
                  <TableHead>Empresa</TableHead>
                  <TableHead>Notas</TableHead>
                  <TableHead className="w-20"></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {clientes.items.map((c) => (
                  <TableRow key={c.id} className={seleccionados.includes(c.id) ? 'bg-muted/50' : ''}>
                    <TableCell>
                      <Checkbox
                        checked={seleccionados.includes(c.id)}
                        onCheckedChange={() => toggleSeleccion(c.id)}
                        disabled={!seleccionados.includes(c.id) && seleccionados.length >= 2}
                      />
                    </TableCell>
                    <TableCell>{c.nombre} {c.apellidos}</TableCell>
                    <TableCell>{c.telefono ? `${c.prefijo_telefono} ${c.telefono}` : '—'}</TableCell>
                    <TableCell>{c.email || '—'}</TableCell>
                    <TableCell>{c.empresa || '—'}</TableCell>
                    <TableCell className="max-w-32 truncate">{c.notas || '—'}</TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" onClick={() => setClienteEditar(c)}>
                          <Pencil className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => eliminarMut.mutate({ id: c.id })}
                          disabled={eliminarMut.isPending}
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
            <Button variant="outline" size="sm" disabled={pagina <= 1} onClick={() => setPagina(pagina - 1)}>Anterior</Button>
            <span className="text-sm text-muted-foreground">Página {pagina} de {Math.ceil(clientes.total / porPagina)}</span>
            <Button variant="outline" size="sm" disabled={pagina * porPagina >= clientes.total} onClick={() => setPagina(pagina + 1)}>Siguiente</Button>
          </div>
        </>
      ) : (
        <p className="text-sm text-muted-foreground">No hay clientes registrados</p>
      )}

      {/* [263A-26] Diálogo de merge: el usuario elige cuál de los 2 sobrevive */}
      <Dialog open={modalMerge} onOpenChange={(open) => { if (!open) { setModalMerge(false); } }}>
        <DialogContent className="sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>Fusionar clientes</DialogTitle>
            <DialogDescription>
              Selecciona cuál de los dos clientes debe sobrevivir. El otro será absorbido: sus reservas, etiquetas y campañas se migrarán, y sus campos vacíos se completarán.
            </DialogDescription>
          </DialogHeader>
          <div className="flex flex-col gap-3">
            {clientesSeleccionados.map((c) => (
              <button
                key={c.id}
                type="button"
                onClick={() => setDestinoId(c.id)}
                className={`flex items-center gap-3 rounded-md border p-3 text-left transition-colors ${
                  destinoId === c.id ? 'border-primary bg-primary/10' : 'border-border hover:bg-muted/50'
                }`}
              >
                <div className="flex-1">
                  <p className="font-medium">{c.nombre} {c.apellidos}</p>
                  <p className="text-sm text-muted-foreground">
                    {[c.telefono, c.email, c.empresa].filter(Boolean).join(' · ') || 'Sin datos adicionales'}
                  </p>
                </div>
                {destinoId === c.id && (
                  <span className="text-xs font-semibold text-primary">SOBREVIVE</span>
                )}
                {destinoId && destinoId !== c.id && (
                  <span className="text-xs text-destructive">SE ELIMINA</span>
                )}
              </button>
            ))}
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setModalMerge(false)}>Cancelar</Button>
            <Button
              disabled={!destinoId || mergeMut.isPending}
              onClick={ejecutarMerge}
            >
              {mergeMut.isPending ? 'Fusionando...' : 'Confirmar fusión'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}

export default ListaClientes;
