/* [263A-16] Configuración — reescrita con shadcn Card + Switch + Input.
 * Campos obligatorios al reservar, IVA por defecto, nombre del restaurante.
 * [283A-8] Sección de API key de Groq para digitalización de documentos.
 * [283A-23] Pestañas: General + Integraciones Marketing.
 * [283A-27] Pestaña Chatbot con gestión de API Keys.
 * [283A-39] Card de datos de prueba: botón seed y botón reset. */

import { useState } from 'react';
import { useConfiguracion } from '../hooks/useConfiguracion';
import IntegracionesMarketing from './IntegracionesMarketing';
import ConfigChatbot from './ConfigChatbot';
import axios from '@/api/axios-instance';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { toast } from 'sonner';

function Configuracion() {
  const { config, cambiarCampo, guardar, mensaje, cargando, guardando } = useConfiguracion();
  const [operandoSeed, setOperandoSeed] = useState(false);

  /* [303A-2] Migrado de raw fetch a axios para usar interceptors JWT/401 */
  async function ejecutarOperacion(endpoint: string, descripcion: string) {
    setOperandoSeed(true);
    try {
      const resp = await axios.post(`/api/${endpoint}`);
      toast.success(descripcion, { description: resp.data.mensaje });
    } catch (err: unknown) {
      const msg = (err as { response?: { data?: { mensaje?: string } } })?.response?.data?.mensaje ?? 'No se pudo conectar con el servidor';
      toast.error('Error', { description: msg });
    } finally {
      setOperandoSeed(false);
    }
  }

  if (cargando) return <p className="text-sm text-muted-foreground">Cargando configuración...</p>;

  return (
    <Tabs defaultValue="general" className="max-w-2xl">
      <TabsList>
        <TabsTrigger value="general">General</TabsTrigger>
        <TabsTrigger value="integraciones">Integraciones</TabsTrigger>
        <TabsTrigger value="chatbot">Chatbot</TabsTrigger>
      </TabsList>

      <TabsContent value="general" className="flex flex-col gap-6 mt-4">
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

      <Card>
        <CardHeader>
          <CardTitle>Digitalización con IA</CardTitle>
          <CardDescription>Configura tu API key de Groq para digitalizar facturas, albaranes y tickets automáticamente</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-2">
            <Label htmlFor="groq-api-key">API Key de Groq</Label>
            <Input
              id="groq-api-key"
              type="password"
              value={config.groq_api_key}
              onChange={(e) => cambiarCampo('groq_api_key', e.target.value)}
              placeholder="gsk_..."
            />
            <p className="text-xs text-muted-foreground">
              Obtén tu API key en <a href="https://console.groq.com/keys" target="_blank" rel="noopener noreferrer" className="underline">console.groq.com/keys</a>. Es gratuita.
            </p>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Datos de prueba</CardTitle>
          <CardDescription>Cargar o eliminar los datos de demostración del sistema</CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <div className="flex gap-3 flex-wrap">
            <Button
              variant="outline"
              disabled={operandoSeed}
              onClick={() => ejecutarOperacion('admin/seed', 'Datos de prueba cargados')}
            >
              {operandoSeed ? 'Procesando...' : 'Recargar datos de prueba'}
            </Button>
            <Button
              variant="destructive"
              disabled={operandoSeed}
              onClick={() => ejecutarOperacion('admin/reset', 'Datos eliminados')}
            >
              Eliminar datos de prueba
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            "Recargar" restablece todos los datos demo. "Eliminar" borra todos los datos pero mantiene la cuenta.
          </p>
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
      </TabsContent>

      <TabsContent value="integraciones" className="mt-4">
        <IntegracionesMarketing />
      </TabsContent>

      <TabsContent value="chatbot" className="mt-4">
        <ConfigChatbot />
      </TabsContent>
    </Tabs>
  );
}

export default Configuracion;
