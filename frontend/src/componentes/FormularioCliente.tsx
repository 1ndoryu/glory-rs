/* [263A-16] Formulario cliente CRM — reescrito con shadcn Input + Button + Label + Textarea + Switch. */

import { Cliente } from '../api/generated';
import useFormularioCliente from '../hooks/useFormularioCliente';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Switch } from '@/components/ui/switch';

interface Props {
  onExito?: () => void;
  cliente?: Cliente;
}

function FormularioCliente({ onExito, cliente }: Props) {
  const { campos, cambiarCampo, error, manejarEnvio, cargando, esEdicion } = useFormularioCliente(onExito, cliente);

  return (
    <div>
      {error && (
        <div className="mb-4 rounded-md bg-destructive/10 p-3 text-sm text-destructive">{error}</div>
      )}

      <form className="flex flex-col gap-4" onSubmit={manejarEnvio}>
        <div className="grid grid-cols-2 gap-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="nombre">Nombre *</Label>
            <Input id="nombre" type="text" value={campos.nombre} onChange={e => cambiarCampo('nombre', e.target.value)} placeholder="Nombre" />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="apellidos">Apellidos</Label>
            <Input id="apellidos" type="text" value={campos.apellidos} onChange={e => cambiarCampo('apellidos', e.target.value)} placeholder="Apellidos" />
          </div>
        </div>

        <div className="grid grid-cols-[80px_1fr_1fr] gap-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="prefijo">Prefijo</Label>
            <Input id="prefijo" type="text" value={campos.prefijoTelefono} onChange={e => cambiarCampo('prefijoTelefono', e.target.value)} placeholder="+34" />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="telefono">Teléfono</Label>
            <Input id="telefono" type="tel" value={campos.telefono} onChange={e => cambiarCampo('telefono', e.target.value)} placeholder="Teléfono" />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="email">Email</Label>
            <Input id="email" type="email" value={campos.email} onChange={e => cambiarCampo('email', e.target.value)} placeholder="correo@ejemplo.com" />
          </div>
        </div>

        <div className="flex flex-col gap-2">
          <Label htmlFor="empresa">Empresa</Label>
          <Input id="empresa" type="text" value={campos.empresa} onChange={e => cambiarCampo('empresa', e.target.value)} placeholder="Empresa (opcional)" />
        </div>

        <div className="flex flex-col gap-2">
          <Label htmlFor="notas">Notas</Label>
          <Textarea id="notas" rows={2} value={campos.notas} onChange={e => cambiarCampo('notas', e.target.value)} placeholder="Notas sobre el cliente" />
        </div>

        <div className="flex flex-col gap-2">
          <Label htmlFor="alergias">Alergias</Label>
          <Input id="alergias" type="text" value={campos.alergias} onChange={e => cambiarCampo('alergias', e.target.value)} placeholder="Alergias conocidas" />
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="preferenciasBebida">Preferencias de bebida</Label>
            <Input id="preferenciasBebida" type="text" value={campos.preferenciasBebida} onChange={e => cambiarCampo('preferenciasBebida', e.target.value)} placeholder="Ej: vino tinto" />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="preferenciasUbicacion">Preferencias de ubicación</Label>
            <Input id="preferenciasUbicacion" type="text" value={campos.preferenciasUbicacion} onChange={e => cambiarCampo('preferenciasUbicacion', e.target.value)} placeholder="Ej: terraza" />
          </div>
        </div>

        <div className="flex flex-wrap gap-6">
          <div className="flex items-center gap-2">
            <Switch id="consentimientoEmail" checked={campos.consentimientoEmail} onCheckedChange={checked => cambiarCampo('consentimientoEmail', checked)} />
            <Label htmlFor="consentimientoEmail">Consentimiento email comercial</Label>
          </div>
          <div className="flex items-center gap-2">
            <Switch id="consentimientoSms" checked={campos.consentimientoSms} onCheckedChange={checked => cambiarCampo('consentimientoSms', checked)} />
            <Label htmlFor="consentimientoSms">Consentimiento SMS comercial</Label>
          </div>
          <div className="flex items-center gap-2">
            <Switch id="enviarEncuestas" checked={campos.enviarEncuestas} onCheckedChange={checked => cambiarCampo('enviarEncuestas', checked)} />
            <Label htmlFor="enviarEncuestas">Enviar encuestas</Label>
          </div>
        </div>

        <Button type="submit" className="w-full" disabled={cargando}>
          {cargando ? 'Guardando...' : (esEdicion ? 'Guardar Cambios' : 'Crear Cliente')}
        </Button>
      </form>
    </div>
  );
}

export default FormularioCliente;
