/* [263A-16] Lista de clientes — reescrita con shadcn Table + Dialog + Input.
 * CRM con búsqueda, paginación, modal crear/editar. Iconos para acciones. */

import useListaClientes from '../hooks/useListaClientes';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Trash2, Pencil } from 'lucide-react';
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
  } = useListaClientes();

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <p className="text-sm text-muted-foreground">{clientes ? `${clientes.total} registros` : ''}</p>
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
        <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nuevo Cliente</DialogTitle>
          </DialogHeader>
          <FormularioCliente onExito={cerrarModalYRefrescar} />
        </DialogContent>
      </Dialog>

      <Dialog open={!!clienteEditar} onOpenChange={(open) => { if (!open) setClienteEditar(null); }}>
        <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
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
                  <TableRow key={c.id}>
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
    </div>
  );
}

export default ListaClientes;
