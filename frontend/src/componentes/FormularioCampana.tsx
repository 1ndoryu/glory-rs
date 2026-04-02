/* [263A-23] Formulario de creación de campañas de marketing.
 * Replica la funcionalidad de Cover Manager: nombre, canales, segmento,
 * cuerpo SMS con contador, opt-out, preview de destinatarios. */

import { useFormularioCampana } from '../hooks/useFormularioCampana';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Users, Mail, Phone, MessageSquare } from 'lucide-react';

/* [024A-1] WhatsApp re-habilitado con selector de plantilla aprobada */
const ETIQUETAS_SEGMENTO: Record<string, string> = {
  habitual: 'Habituales',
  sin_1m: 'Sin venir 1 mes',
  sin_3m: 'Sin venir 3 meses',
  sin_6m: 'Sin venir 6 meses',
  sin_9m: 'Sin venir 9 meses',
  sin_1a: 'Sin venir 1 año',
  sin_mas_1a: 'Sin venir +1 año',
  todos: 'Todos los clientes',
};

/* [024A-1] WhatsApp añadido con icono MessageSquare */
const ETIQUETAS_CANAL: Record<string, { label: string; icon: React.ReactNode }> = {
  sms: { label: 'SMS', icon: <Phone className="size-4" /> },
  email: { label: 'Email', icon: <Mail className="size-4" /> },
  whatsapp: { label: 'WhatsApp', icon: <MessageSquare className="size-4" /> },
};

export default function FormularioCampana() {
  const f = useFormularioCampana();

  return (
    <div className="mx-auto max-w-3xl w-full space-y-6">
      {/* Nombre y descripción */}
      <Card>
        <CardHeader>
          <CardTitle>Nueva Campaña</CardTitle>
          <CardDescription>
            Crea una campaña de marketing para tus clientes
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="nombre">Nombre de la campaña *</Label>
            <Input
              id="nombre"
              value={f.nombre}
              onChange={e => f.setNombre(e.target.value)}
              placeholder="Ej: Promoción verano 2026"
              maxLength={255}
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="descripcion">Descripción interna</Label>
            <Textarea
              id="descripcion"
              value={f.descripcionInterna}
              onChange={e => f.setDescripcionInterna(e.target.value)}
              placeholder="Notas internas sobre esta campaña (no se envía al cliente)"
              rows={2}
            />
          </div>
        </CardContent>
      </Card>

      {/* Canales */}
      <Card>
        <CardHeader>
          <CardTitle>Canales de envío *</CardTitle>
          <CardDescription>
            Selecciona por qué canales enviar la campaña
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex gap-3">
            {f.CANALES_VALIDOS.map(canal => {
              const activo = f.canales.has(canal);
              const info = ETIQUETAS_CANAL[canal];
              return (
                <Button
                  key={canal}
                  type="button"
                  variant={activo ? 'default' : 'outline'}
                  onClick={() => f.toggleCanal(canal)}
                  className="gap-2"
                >
                  {info.icon}
                  {info.label}
                </Button>
              );
            })}
          </div>
          {/* [024A-1] Selector de plantilla WhatsApp aprobada */}
          {f.tieneWhatsapp && (
            <div className="mt-4 space-y-2">
              <Label>Plantilla WhatsApp aprobada *</Label>
              {f.plantillasAprobadas.length === 0 ? (
                <p className="text-destructive text-sm">
                  No hay plantillas aprobadas. Crea y envía una plantilla a Meta primero.
                </p>
              ) : (
                <Select
                  value={f.plantillaWhatsappId ?? '__none__'}
                  onValueChange={v => f.setPlantillaWhatsappId(v === '__none__' ? null : v)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Seleccionar plantilla" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="__none__">Seleccionar plantilla</SelectItem>
                    {f.plantillasAprobadas.map(p => (
                      <SelectItem key={p.id} value={p.id}>
                        {p.nombre} ({p.idioma})
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              )}
              <p className="text-muted-foreground text-xs">
                WhatsApp solo permite enviar mensajes usando plantillas aprobadas por Meta.
              </p>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Segmento */}
      <Card>
        <CardHeader>
          <CardTitle>Segmento de clientes</CardTitle>
          <CardDescription>
            Filtra a qué clientes se enviará la campaña según su actividad
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <Select
            value={f.segmento || '__none__'}
            onValueChange={v => f.setSegmento(v === '__none__' ? '' : v)}
          >
            <SelectTrigger>
              <SelectValue placeholder="Seleccionar segmento" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="__none__">Sin filtro</SelectItem>
              {f.SEGMENTOS_VALIDOS.map(seg => (
                <SelectItem key={seg} value={seg}>
                  {ETIQUETAS_SEGMENTO[seg] || seg}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          {/* Preview de segmento */}
          {f.previewHabilitado && (
            <div className="bg-muted rounded-md p-4">
              {f.previewLoading ? (
                <p className="text-muted-foreground text-sm">Calculando destinatarios...</p>
              ) : f.preview ? (
                <div className="grid grid-cols-2 gap-3 text-sm">
                  <div className="flex items-center gap-2">
                    <Users className="text-muted-foreground size-4" />
                    <span>Total clientes: <strong>{f.preview.total_clientes}</strong></span>
                  </div>
                  <div className="flex items-center gap-2">
                    <Mail className="text-muted-foreground size-4" />
                    <span>Con email: <strong>{f.preview.con_email}</strong></span>
                  </div>
                  <div className="flex items-center gap-2">
                    <Phone className="text-muted-foreground size-4" />
                    <span>Con teléfono: <strong>{f.preview.con_telefono}</strong></span>
                  </div>
                  <div>
                    <span>Consentimiento SMS: <strong>{f.preview.con_consentimiento_sms}</strong></span>
                  </div>
                  <div>
                    <span>Consentimiento email: <strong>{f.preview.con_consentimiento_email}</strong></span>
                  </div>
                </div>
              ) : null}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Mensaje */}
      <Card>
        <CardHeader>
          <CardTitle>Contenido del mensaje</CardTitle>
          <CardDescription>
            Escribe el mensaje que recibirán tus clientes
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="mensaje">Mensaje</Label>
            <Textarea
              id="mensaje"
              value={f.cuerpoMensaje}
              onChange={e => f.setCuerpoMensaje(e.target.value)}
              placeholder="Escribe tu mensaje aquí..."
              rows={4}
            />
            {f.tieneSms && (
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">
                  {f.incluirBaja && '(17 caracteres reservados para texto de baja)'}
                </span>
                <span className={f.caracteresSms > f.maxSms ? 'text-destructive font-medium' : 'text-muted-foreground'}>
                  {f.caracteresSms} / {f.maxSms} caracteres
                </span>
              </div>
            )}
            {f.tieneSms && f.caracteresSms > f.maxSms && (
              <p className="text-destructive text-sm">
                El mensaje excede el límite de caracteres para SMS. Se enviará en varios SMS con coste adicional.
              </p>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Opt-out / Baja */}
      <Card>
        <CardHeader>
          <CardTitle>Opción de baja</CardTitle>
          <CardDescription>
            Incluir número de teléfono para que el cliente pueda darse de baja
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center gap-3">
            <Switch
              id="incluir-baja"
              checked={f.incluirBaja}
              onCheckedChange={f.setIncluirBaja}
            />
            <Label htmlFor="incluir-baja">
              Incluir opción de baja en el mensaje
            </Label>
          </div>
          {f.incluirBaja && (
            <div className="space-y-2">
              <Label htmlFor="telefono-baja">Teléfono para baja</Label>
              <Input
                id="telefono-baja"
                value={f.telefonoBaja}
                onChange={e => f.setTelefonoBaja(e.target.value)}
                placeholder="Ej: 612345678"
                type="tel"
              />
              <p className="text-muted-foreground text-xs">
                Se añadirá al final del SMS: &quot;No: {f.telefonoBaja || '___'}&quot;
              </p>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Aviso coste SMS */}
      {f.tieneSms && (
        <div className="bg-muted rounded-md p-3 text-sm">
          <Badge variant="outline" className="mb-1">Información</Badge>
          <p className="text-muted-foreground">
            Se aplicarán las tarifas de SMS de tu proveedor por cada mensaje enviado.
            Los caracteres especiales (€, |, ^, etc.) ocupan el doble de espacio.
          </p>
        </div>
      )}

      {/* Acciones */}
      <div className="flex justify-end gap-3">
        <Button variant="outline" onClick={f.cancelar}>
          Cancelar
        </Button>
        <Button
          onClick={f.crearCampana}
          disabled={!f.formValido || f.enviando}
        >
          {f.enviando ? 'Creando...' : 'Crear Campaña'}
        </Button>
      </div>
    </div>
  );
}
