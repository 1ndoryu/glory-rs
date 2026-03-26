/* [263A-16] Formulario de gasto — reescrito con shadcn Button + Input + Label + Switch.
 * Menú 3 opciones (manual / digitalizar / por correo).
 * "Por correo" descartado por el cliente — deshabilitado.
 * "Digitalizar" es placeholder hasta soporte OCR. */

import { useState } from 'react';
import { MetodoPago, TipoDocumento } from '../api/generated';
import useFormularioGasto from '../hooks/useFormularioGasto';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Card, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

type ModoGasto = 'menu' | 'manual' | 'digitalizar';

interface Props {
  onExito?: () => void;
}

function FormularioGasto({ onExito }: Props) {
  const [modo, setModo] = useState<ModoGasto>('menu');
  const { campos, cambiarCampo, error, manejarEnvio, cargando, categorias } = useFormularioGasto(onExito);

  if (modo === 'menu') {
    return (
      <div className="grid gap-4 sm:grid-cols-3">
        <Card className="cursor-pointer hover:border-primary transition-colors" onClick={() => setModo('manual')}>
          <CardHeader className="items-center text-center">
            <span className="text-3xl">📋</span>
            <CardTitle className="text-base">Gasto manual</CardTitle>
            <CardDescription>Introduce los datos a mano</CardDescription>
          </CardHeader>
        </Card>
        <Card className="cursor-pointer hover:border-primary transition-colors" onClick={() => setModo('digitalizar')}>
          <CardHeader className="items-center text-center">
            <span className="text-3xl">📷</span>
            <CardTitle className="text-base">Digitalizar archivos</CardTitle>
            <CardDescription>Sube una foto del documento</CardDescription>
          </CardHeader>
        </Card>
        <Card className="opacity-50 cursor-not-allowed" title="Funcionalidad no disponible en esta versión">
          <CardHeader className="items-center text-center">
            <span className="text-3xl">✉️</span>
            <CardTitle className="text-base">Por correo</CardTitle>
            <CardDescription>Próximamente</CardDescription>
          </CardHeader>
        </Card>
      </div>
    );
  }

  if (modo === 'digitalizar') {
    return (
      <div className="flex flex-col items-center gap-6 py-8">
        <span className="text-5xl">📷</span>
        <p className="text-muted-foreground text-center">Funcionalidad de digitalización próximamente disponible</p>
        <p className="text-sm text-muted-foreground text-center">Pronto podrás subir una foto de tu factura o albarán y los datos se extraerán automáticamente.</p>
        <Button variant="outline" onClick={() => setModo('menu')}>Volver</Button>
      </div>
    );
  }

  return (
    <div>
      {error && (
        <div className="mb-4 rounded-md bg-destructive/10 p-3 text-sm text-destructive">{error}</div>
      )}

      <form className="flex flex-col gap-4" onSubmit={manejarEnvio}>
        <div className="grid grid-cols-2 gap-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="fecha">Fecha</Label>
            <Input id="fecha" type="date" value={campos.fecha} onChange={e => cambiarCampo('fecha', e.target.value)} />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="proveedor">Proveedor</Label>
            <Input id="proveedor" type="text" value={campos.proveedor} onChange={e => cambiarCampo('proveedor', e.target.value)} placeholder="Opcional" />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="categoria">Categoría</Label>
            <Select value={campos.categoriaId || '__none__'} onValueChange={v => cambiarCampo('categoriaId', v === '__none__' ? '' : v)}>
              <SelectTrigger id="categoria"><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">Sin categoría</SelectItem>
                {categorias.map(cat => (
                  <SelectItem key={cat.id} value={cat.id}>{cat.nombre}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="tipoDocumento">Tipo documento</Label>
            <Select value={campos.tipoDocumento} onValueChange={v => cambiarCampo('tipoDocumento', v as TipoDocumento)}>
              <SelectTrigger id="tipoDocumento"><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value={TipoDocumento.factura}>Factura</SelectItem>
                <SelectItem value={TipoDocumento.albaran}>Albarán</SelectItem>
                <SelectItem value={TipoDocumento.ticket}>Ticket</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="metodoPago">Método de pago <span className="text-muted-foreground">(Opcional)</span></Label>
            <Select value={campos.metodoPago || '__none__'} onValueChange={v => cambiarCampo('metodoPago', v === '__none__' ? '' : v as MetodoPago | '')}>
              <SelectTrigger id="metodoPago"><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">— sin especificar —</SelectItem>
                <SelectItem value={MetodoPago.efectivo}>Efectivo</SelectItem>
                <SelectItem value={MetodoPago.tarjeta}>Tarjeta</SelectItem>
                <SelectItem value={MetodoPago.transferencia}>Transferencia</SelectItem>
                <SelectItem value={MetodoPago.otros}>Otros</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="numeroDocumento">Nº documento <span className="text-muted-foreground">(Opcional)</span></Label>
            <Input id="numeroDocumento" type="text" value={campos.numeroDocumento} onChange={e => cambiarCampo('numeroDocumento', e.target.value)} placeholder="Para programación automática futura" />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="importeBase">Importe base (€)</Label>
            <Input id="importeBase" type="number" step="0.01" min="0" value={campos.importeBase} onChange={e => cambiarCampo('importeBase', e.target.value)} placeholder="0.00" />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="importeIva">Importe IVA (€)</Label>
            <Input id="importeIva" type="number" step="0.01" min="0" value={campos.importeIva} onChange={e => cambiarCampo('importeIva', e.target.value)} placeholder="0.00" />
          </div>
        </div>

        <div className="flex items-center gap-2">
          <Switch id="recurrente" checked={campos.recurrente} onCheckedChange={checked => cambiarCampo('recurrente', checked)} />
          <Label htmlFor="recurrente">Gasto recurrente</Label>
        </div>

        <div className="flex gap-3">
          <Button type="button" variant="outline" onClick={() => setModo('menu')}>Volver</Button>
          <Button type="submit" disabled={cargando}>{cargando ? 'Registrando...' : 'Registrar Gasto'}</Button>
        </div>
      </form>
    </div>
  );
}

export default FormularioGasto;

