/* [283A-23] Sección de integraciones de marketing en Configuración.
 * Formularios para SMTP, Twilio y Meta WhatsApp credentials.
 * Los passwords nunca se devuelven — se muestran como "Configurado ✓" si ya hay valor. */

import { useIntegraciones } from '../hooks/useIntegraciones';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

function IntegracionesMarketing() {
  const {
    form,
    cambiarCampo,
    guardar,
    mensaje,
    cargando,
    guardando,
    smtpConfigurado,
    twilioConfigurado,
    metaConfigurado,
  } = useIntegraciones();

  if (cargando) return <p className="text-sm text-muted-foreground">Cargando integraciones...</p>;

  return (
    <div className="flex flex-col gap-6 max-w-2xl">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            Email (SMTP)
            {smtpConfigurado && <Badge variant="secondary">Configurado</Badge>}
          </CardTitle>
          <CardDescription>Configura tu servidor SMTP para enviar campañas y recordatorios por email</CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <div className="grid grid-cols-2 gap-3">
            <div className="flex flex-col gap-1">
              <Label htmlFor="smtp-host">Host SMTP</Label>
              <Input
                id="smtp-host"
                value={form.smtp_host}
                onChange={(e) => cambiarCampo('smtp_host', e.target.value)}
                placeholder="smtp.gmail.com"
              />
            </div>
            <div className="flex flex-col gap-1">
              <Label htmlFor="smtp-port">Puerto</Label>
              <Input
                id="smtp-port"
                type="number"
                value={form.smtp_port}
                onChange={(e) => cambiarCampo('smtp_port', Number(e.target.value))}
                className="max-w-24"
              />
            </div>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div className="flex flex-col gap-1">
              <Label htmlFor="smtp-user">Usuario</Label>
              <Input
                id="smtp-user"
                value={form.smtp_user}
                onChange={(e) => cambiarCampo('smtp_user', e.target.value)}
                placeholder="tu@email.com"
              />
            </div>
            <div className="flex flex-col gap-1">
              <Label htmlFor="smtp-password">Contraseña</Label>
              <Input
                id="smtp-password"
                type="password"
                value={form.smtp_password}
                onChange={(e) => cambiarCampo('smtp_password', e.target.value)}
                placeholder={smtpConfigurado ? '••••••••' : 'Contraseña SMTP'}
              />
            </div>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div className="flex flex-col gap-1">
              <Label htmlFor="smtp-from-email">Email remitente</Label>
              <Input
                id="smtp-from-email"
                value={form.smtp_from_email}
                onChange={(e) => cambiarCampo('smtp_from_email', e.target.value)}
                placeholder="noreply@turestaurante.com"
              />
            </div>
            <div className="flex flex-col gap-1">
              <Label htmlFor="smtp-from-name">Nombre remitente</Label>
              <Input
                id="smtp-from-name"
                value={form.smtp_from_name}
                onChange={(e) => cambiarCampo('smtp_from_name', e.target.value)}
                placeholder="Mi Restaurante"
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            SMS (Twilio)
            {twilioConfigurado && <Badge variant="secondary">Configurado</Badge>}
          </CardTitle>
          <CardDescription>Configura Twilio para enviar SMS en campañas y recordatorios</CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <div className="flex flex-col gap-1">
            <Label htmlFor="twilio-sid">Account SID</Label>
            <Input
              id="twilio-sid"
              value={form.twilio_account_sid}
              onChange={(e) => cambiarCampo('twilio_account_sid', e.target.value)}
              placeholder="AC..."
            />
          </div>
          <div className="flex flex-col gap-1">
            <Label htmlFor="twilio-token">Auth Token</Label>
            <Input
              id="twilio-token"
              type="password"
              value={form.twilio_auth_token}
              onChange={(e) => cambiarCampo('twilio_auth_token', e.target.value)}
              placeholder={twilioConfigurado ? '••••••••' : 'Auth Token de Twilio'}
            />
          </div>
          <div className="flex flex-col gap-1">
            <Label htmlFor="twilio-from">Número remitente</Label>
            <Input
              id="twilio-from"
              value={form.twilio_from_number}
              onChange={(e) => cambiarCampo('twilio_from_number', e.target.value)}
              placeholder="+34600000000"
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            WhatsApp (Meta)
            {metaConfigurado && <Badge variant="secondary">Configurado</Badge>}
          </CardTitle>
          <CardDescription>Configura Meta Cloud API para enviar mensajes por WhatsApp</CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <div className="flex flex-col gap-1">
            <Label htmlFor="meta-waba">WABA ID</Label>
            <Input
              id="meta-waba"
              value={form.meta_waba_id}
              onChange={(e) => cambiarCampo('meta_waba_id', e.target.value)}
              placeholder="ID de WhatsApp Business Account"
            />
          </div>
          <div className="flex flex-col gap-1">
            <Label htmlFor="meta-app">Business App ID</Label>
            <Input
              id="meta-app"
              value={form.meta_business_app_id}
              onChange={(e) => cambiarCampo('meta_business_app_id', e.target.value)}
              placeholder="ID de la app en Meta"
            />
          </div>
          <div className="flex flex-col gap-1">
            <Label htmlFor="meta-token">Access Token</Label>
            <Input
              id="meta-token"
              type="password"
              value={form.meta_access_token}
              onChange={(e) => cambiarCampo('meta_access_token', e.target.value)}
              placeholder={metaConfigurado ? '••••••••' : 'Token de acceso'}
            />
          </div>
        </CardContent>
      </Card>

      <div className="flex items-center gap-4">
        <Button onClick={guardar} disabled={guardando}>
          {guardando ? 'Guardando...' : 'Guardar integraciones'}
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

export default IntegracionesMarketing;
