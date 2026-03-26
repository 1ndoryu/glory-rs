/* 263A-1: Formulario para crear/editar un cliente del CRM */

import { Cliente } from '../api/generated';
import useFormularioCliente from '../hooks/useFormularioCliente';
import { Input, Textarea, Boton } from '@glory/componentes/ui';
import '../estilos/Formularios.css';

interface Props {
  onExito?: () => void;
  cliente?: Cliente;
}

function FormularioCliente({ onExito, cliente }: Props) {
  const { campos, cambiarCampo, error, manejarEnvio, cargando, esEdicion } = useFormularioCliente(onExito, cliente);

  return (
    <div>
      {error && <div className="errorFormulario">{error}</div>}

      <form className="formulario" onSubmit={manejarEnvio}>
        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="nombre">Nombre *</label>
            <Input id="nombre" type="text" value={campos.nombre} onChange={(e) => cambiarCampo('nombre', e.target.value)} placeholder="Nombre" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="apellidos">Apellidos</label>
            <Input id="apellidos" type="text" value={campos.apellidos} onChange={(e) => cambiarCampo('apellidos', e.target.value)} placeholder="Apellidos" />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario" style={{ flex: '0 0 80px' }}>
            <label className="etiquetaFormulario" htmlFor="prefijo">Prefijo</label>
            <Input id="prefijo" type="text" value={campos.prefijoTelefono} onChange={(e) => cambiarCampo('prefijoTelefono', e.target.value)} placeholder="+34" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="telefono">Teléfono</label>
            <Input id="telefono" type="tel" value={campos.telefono} onChange={(e) => cambiarCampo('telefono', e.target.value)} placeholder="Teléfono" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="email">Email</label>
            <Input id="email" type="email" value={campos.email} onChange={(e) => cambiarCampo('email', e.target.value)} placeholder="correo@ejemplo.com" />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="empresa">Empresa</label>
            <Input id="empresa" type="text" value={campos.empresa} onChange={(e) => cambiarCampo('empresa', e.target.value)} placeholder="Empresa (opcional)" />
          </div>
        </div>

        <div className="grupoFormulario ancho">
          <label className="etiquetaFormulario" htmlFor="notas">Notas</label>
          <Textarea id="notas" rows={2} value={campos.notas} onChange={(e) => cambiarCampo('notas', e.target.value)} placeholder="Notas sobre el cliente" />
        </div>

        <div className="grupoFormulario ancho">
          <label className="etiquetaFormulario" htmlFor="alergias">Alergias</label>
          <Input id="alergias" type="text" value={campos.alergias} onChange={(e) => cambiarCampo('alergias', e.target.value)} placeholder="Alergias conocidas" />
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="preferenciasBebida">Preferencias de bebida</label>
            <Input id="preferenciasBebida" type="text" value={campos.preferenciasBebida} onChange={(e) => cambiarCampo('preferenciasBebida', e.target.value)} placeholder="Ej: vino tinto" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="preferenciasUbicacion">Preferencias de ubicación</label>
            <Input id="preferenciasUbicacion" type="text" value={campos.preferenciasUbicacion} onChange={(e) => cambiarCampo('preferenciasUbicacion', e.target.value)} placeholder="Ej: terraza" />
          </div>
        </div>

        <div className="filaFormulario">
          <label className="checkboxFormulario">
            <Input type="checkbox" checked={campos.consentimientoEmail} onChange={(e) => cambiarCampo('consentimientoEmail', e.target.checked)} />
            Consentimiento email comercial
          </label>
          <label className="checkboxFormulario">
            <Input type="checkbox" checked={campos.consentimientoSms} onChange={(e) => cambiarCampo('consentimientoSms', e.target.checked)} />
            Consentimiento SMS comercial
          </label>
          <label className="checkboxFormulario">
            <Input type="checkbox" checked={campos.enviarEncuestas} onChange={(e) => cambiarCampo('enviarEncuestas', e.target.checked)} />
            Enviar encuestas
          </label>
        </div>

        <Boton variante="primario" ancho type="submit" cargando={cargando}>
          {esEdicion ? 'Guardar Cambios' : 'Crear Cliente'}
        </Boton>
      </form>
    </div>
  );
}

export default FormularioCliente;
