/* [263A-16] Inicio — reescrito con shadcn Card y Dialog.
 * Combina resumen económico (Ventas/Gastos/Margen) + conteo reservas.
 * Modales para crear venta/gasto/reserva usan shadcn Dialog. */

import { useState } from 'react';
import { useResumen, useConteoReservas } from '../api/generated';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { DollarSign, TrendingDown, TrendingUp, CalendarDays } from 'lucide-react';
import FormularioVenta from './FormularioVenta';
import FormularioGasto from './FormularioGasto';
import FormularioReserva from './FormularioReserva';

function formatearMoneda(valor: string): string {
  const num = parseFloat(valor);
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(num);
}

function Inicio() {
  const [modalVenta, setModalVenta] = useState(false);
  const [modalGasto, setModalGasto] = useState(false);
  const [modalReserva, setModalReserva] = useState(false);
  const ahora = new Date();
  const year = ahora.getFullYear();
  const month = ahora.getMonth() + 1;

  const { data: resumen, isLoading: cargandoResumen, refetch: refetchResumen } = useResumen({ year, month });
  const { data: conteo, isLoading: cargandoConteo, refetch: refetchConteo } = useConteoReservas();

  const datosResumen = resumen?.status === 200 ? resumen.data : null;
  const datosConteo = conteo?.status === 200 ? conteo.data : null;

  return (
    <div className="flex flex-col gap-6">
      {/* Acciones rápidas */}
      <div className="flex flex-wrap gap-3">
        <Button onClick={() => setModalVenta(true)} className="bg-green-600 hover:bg-green-700 text-white">
          + Nueva Venta
        </Button>
        <Button onClick={() => setModalGasto(true)} variant="default">
          + Nuevo Gasto
        </Button>
        <Button onClick={() => setModalReserva(true)} variant="secondary">
          + Nueva Reserva
        </Button>
      </div>

      {/* Modales */}
      <Dialog open={modalVenta} onOpenChange={setModalVenta}>
        <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nueva Venta</DialogTitle>
          </DialogHeader>
          <FormularioVenta onExito={() => { setModalVenta(false); refetchResumen(); }} />
        </DialogContent>
      </Dialog>
      <Dialog open={modalGasto} onOpenChange={setModalGasto}>
        <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nuevo Gasto</DialogTitle>
          </DialogHeader>
          <FormularioGasto onExito={() => { setModalGasto(false); refetchResumen(); }} />
        </DialogContent>
      </Dialog>
      <Dialog open={modalReserva} onOpenChange={setModalReserva}>
        <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Nueva Reserva</DialogTitle>
          </DialogHeader>
          <FormularioReserva onExito={() => { setModalReserva(false); refetchConteo(); }} />
        </DialogContent>
      </Dialog>

      {/* Resumen económico */}
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Ventas</CardTitle>
            <TrendingUp className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600">
              {cargandoResumen ? '...' : datosResumen ? formatearMoneda(datosResumen.total_ventas) : '—'}
            </div>
            <p className="text-xs text-muted-foreground">Total del mes</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Gastos</CardTitle>
            <TrendingDown className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">
              {cargandoResumen ? '...' : datosResumen ? formatearMoneda(datosResumen.total_gastos) : '—'}
            </div>
            <p className="text-xs text-muted-foreground">Total del mes</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Margen</CardTitle>
            <DollarSign className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className={`text-2xl font-bold ${datosResumen && parseFloat(datosResumen.margen) >= 0 ? 'text-green-600' : 'text-red-600'}`}>
              {cargandoResumen ? '...' : datosResumen ? formatearMoneda(datosResumen.margen) : '—'}
            </div>
            <p className="text-xs text-muted-foreground">Ventas − Gastos</p>
          </CardContent>
        </Card>
      </div>

      {/* Reservas */}
      <div className="grid gap-4 sm:grid-cols-2">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Reservas hoy</CardTitle>
            <CalendarDays className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {cargandoConteo ? '...' : datosConteo ? datosConteo.total_hoy : 0}
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Reservas este mes</CardTitle>
            <CalendarDays className="size-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {cargandoConteo ? '...' : datosConteo ? datosConteo.total_mes : 0}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

export default Inicio;
