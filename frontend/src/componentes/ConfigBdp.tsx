/* [065A-2] Configuracion de BDP/WebLink REST API.
 * Mantiene credenciales fuera de respuestas publicas y ofrece diagnostico
 * Health + Login + GetVersion para la sesion remota con el PC del restaurante. */

import { useState } from 'react';
import { Activity, Loader2 } from 'lucide-react';
import axios from '@/api/axios-instance';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { toast } from 'sonner';
import type { EstadoConfiguracion } from '../hooks/useConfiguracion';
import type { BdpDiagnosticoResponse } from '../api/generated/gestionRestauranteAPI.schemas';

interface ConfigBdpProps {
  config: EstadoConfiguracion;
  cambiarCampo: <K extends keyof EstadoConfiguracion>(campo: K, valor: EstadoConfiguracion[K]) => void;
}

function ConfigBdp({ config, cambiarCampo }: ConfigBdpProps) {
  const [diagnostico, setDiagnostico] = useState<BdpDiagnosticoResponse | null>(null);
  const [diagnosticando, setDiagnosticando] = useState(false);

  async function diagnosticar() {
    setDiagnosticando(true);
    try {
      const resp = await axios.get<BdpDiagnosticoResponse>('/api/configuracion/bdp/diagnostico');
      setDiagnostico(resp.data);
      if (resp.data.health_ok && resp.data.login_ok) {
        toast.success('BDP conectado', { description: resp.data.mensaje });
      } else {
        toast.warning('BDP pendiente', { description: resp.data.mensaje });
      }
    } catch (err: unknown) {
      const msg = (err as { response?: { data?: { message?: string } } })?.response?.data?.message ?? 'No se pudo diagnosticar BDP';
      toast.error('Error BDP', { description: msg });
    } finally {
      setDiagnosticando(false);
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>BDP WebLink REST API</CardTitle>
        <CardDescription>Conexión al TPV/BDP del restaurante para validar servicio, sesión y versión</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <div className="flex flex-col gap-2">
          <Label htmlFor="bdp-base-url">URL pública BDP</Label>
          <Input
            id="bdp-base-url"
            type="url"
            value={config.bdp_base_url}
            onChange={(e) => cambiarCampo('bdp_base_url', e.target.value)}
            placeholder="https://ip-o-dominio:8080"
          />
        </div>
        <div className="grid gap-4 md:grid-cols-3">
          <div className="flex flex-col gap-2">
            <Label htmlFor="bdp-login">Login</Label>
            <Input
              id="bdp-login"
              value={config.bdp_login}
              onChange={(e) => cambiarCampo('bdp_login', e.target.value)}
              autoComplete="off"
            />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="bdp-password">Password</Label>
            <Input
              id="bdp-password"
              type="password"
              value={config.bdp_password}
              onChange={(e) => cambiarCampo('bdp_password', e.target.value)}
              autoComplete="new-password"
            />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="bdp-integrator-code">Código integrador</Label>
            <Input
              id="bdp-integrator-code"
              type="password"
              value={config.bdp_integrator_code}
              onChange={(e) => cambiarCampo('bdp_integrator_code', e.target.value)}
              autoComplete="off"
            />
          </div>
        </div>
        <div className="grid gap-4 md:grid-cols-3">
          <div className="flex flex-col gap-2">
            <Label htmlFor="bdp-pos-id">Terminal POS</Label>
            <Input
              id="bdp-pos-id"
              type="number"
              min={1}
              value={config.bdp_pos_id}
              onChange={(e) => cambiarCampo('bdp_pos_id', Number(e.target.value))}
            />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="bdp-employee-id">Empleado</Label>
            <Input
              id="bdp-employee-id"
              type="number"
              min={1}
              value={config.bdp_employee_id}
              onChange={(e) => cambiarCampo('bdp_employee_id', Number(e.target.value))}
            />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="bdp-items-profile-id">Perfil artículos</Label>
            <Input
              id="bdp-items-profile-id"
              type="number"
              min={1}
              value={config.bdp_items_profile_id}
              onChange={(e) => cambiarCampo('bdp_items_profile_id', Number(e.target.value))}
            />
          </div>
        </div>
        <div className="flex items-center justify-between">
          <Label htmlFor="bdp-sync-enabled">Sincronización BDP activa</Label>
          <Switch
            id="bdp-sync-enabled"
            checked={config.bdp_sync_enabled}
            onCheckedChange={(checked: boolean) => cambiarCampo('bdp_sync_enabled', checked)}
          />
        </div>
        <div className="flex flex-wrap items-center gap-3">
          <Button type="button" variant="outline" onClick={diagnosticar} disabled={diagnosticando}>
            {diagnosticando ? <Loader2 className="size-4 animate-spin" /> : <Activity className="size-4" />}
            Probar conexión
          </Button>
          {diagnostico && (
            <span className={diagnostico.health_ok && diagnostico.login_ok ? 'text-sm text-green-600' : 'text-sm text-destructive'}>
              {diagnostico.mensaje}
            </span>
          )}
        </div>
        {diagnostico?.version && (
          <div className="grid gap-2 rounded-md border p-3 text-sm md:grid-cols-2">
            <span>Versión: {diagnostico.version}.{diagnostico.sub_version ?? 0}</span>
            <span>Aplicación: {diagnostico.application_description || diagnostico.application}</span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

export default ConfigBdp;