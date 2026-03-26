/* [263A-16] Configuración — reescrita con shadcn Card + Switch + Input.
 * Campos obligatorios al reservar, IVA por defecto, nombre del restaurante. */

import { useConfiguracion } from '../hooks/useConfiguracion';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

function Configuracion() {
  const { config, cambiarCampo, guardar, mensaje, cargando, guardando } = useConfiguracion();

  if (cargando) return <p className="text-sm text-muted-foreground">Cargando configuración...</p>;

  return (
    <div className="flex flex-col gap-6 max-w-2xl">
      <Card>
        <CardHeader>
          <CardTitle>Datos del restaurante</CardTitle>
          <CardDescription>Información general del establecimiento</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-2">
            <Label htmlFor="nombre">Nombre del restaurante</Label>
            <Input
              id="nombre"
              value={config.nombre_restaurante}
              onChange={(e) => cambiarCampo('nombre_restaurante', e.target.value)}
              placeholder="Ej: Restaurante La Gloria"
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Campos obligatorios al reservar</CardTitle>
          <CardDescription>Define qué datos se requieren para crear una reserva</CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <div className="flex items-center justify-between">
            <Label htmlFor="nombre-obligatorio">Nombre</Label>
            <Switch
              id="nombre-obligatorio"
              checked={config.reserva_nombre_obligatorio}
              onCheckedChange={(checked) => cambiarCampo('reserva_nombre_obligatorio', checked)}
            />
          </div>
          <div className="flex items-center justify-between">
            <Label htmlFor="apellidos-obligatorio">Apellidos</Label>
            <Switch
              id="apellidos-obligatorio"
              checked={config.reserva_apellidos_obligatorio}
              onCheckedChange={(checked) => cambiarCampo('reserva_apellidos_obligatorio', checked)}
            />
          </div>
          <div className="flex items-center justify-between">
            <Label htmlFor="email-obligatorio">Email</Label>
            <Switch
              id="email-obligatorio"
              checked={config.reserva_email_obligatorio}
              onCheckedChange={(checked) => cambiarCampo('reserva_email_obligatorio', checked)}
            />
          </div>
          <div className="flex items-center justify-between">
            <Label htmlFor="telefono-obligatorio">Teléfono</Label>
            <Switch
              id="telefono-obligatorio"
              checked={config.reserva_telefono_obligatorio}
              onCheckedChange={(checked) => cambiarCampo('reserva_telefono_obligatorio', checked)}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Impuestos</CardTitle>
          <CardDescription>Configuración fiscal por defecto</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-2">
            <Label htmlFor="iva">IVA por defecto (%)</Label>
            <Input
              id="iva"
              type="number"
              min={0}
              max={100}
              step={0.01}
              value={config.iva_por_defecto}
              onChange={(e) => cambiarCampo('iva_por_defecto', Number(e.target.value))}
              className="max-w-32"
            />
          </div>
        </CardContent>
      </Card>

      <div className="flex items-center gap-4">
        <Button onClick={guardar} disabled={guardando}>
          {guardando ? 'Guardando...' : 'Guardar configuración'}
        </Button>
        {mensaje && (
          <span className={`text-sm ${mensaje.includes('Error') ? 'text-destructive' : 'text-green-600'}`}>
            {mensaje}
          </span>
        )}
      </div>
    </div>
  );
}

export default Configuracion;
