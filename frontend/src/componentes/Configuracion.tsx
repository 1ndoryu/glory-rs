/* [263A-17] Página de configuración del restaurante.
 * Campos obligatorios al reservar, IVA por defecto, nombre del restaurante. */

import { useConfiguracion } from '../hooks/useConfiguracion';
import { Input, Boton } from '@glory/componentes/ui';
import '../estilos/Configuracion.css';

function Configuracion() {
  const { config, cambiarCampo, guardar, mensaje, cargando, guardando } = useConfiguracion();

  if (cargando) return <p className="cargando">Cargando configuración...</p>;

  return (
    <div className="paginaConfiguracion">
      <h1>Configuración</h1>

      <section className="seccionConfig">
        <h2>Datos del restaurante</h2>
        <Input
          etiqueta="Nombre del restaurante"
          value={config.nombre_restaurante}
          onChange={(e) => cambiarCampo('nombre_restaurante', e.target.value)}
          placeholder="Ej: Restaurante La Gloria"
        />
      </section>

      <section className="seccionConfig">
        <h2>Campos obligatorios al reservar</h2>
        <div className="grupoToggles">
          <label className="toggleConfig">
            <Input
              type="checkbox"
              checked={config.reserva_nombre_obligatorio}
              onChange={(e) => cambiarCampo('reserva_nombre_obligatorio', e.target.checked)}
            />
            <span>Nombre</span>
          </label>
          <label className="toggleConfig">
            <Input
              type="checkbox"
              checked={config.reserva_apellidos_obligatorio}
              onChange={(e) => cambiarCampo('reserva_apellidos_obligatorio', e.target.checked)}
            />
            <span>Apellidos</span>
          </label>
          <label className="toggleConfig">
            <Input
              type="checkbox"
              checked={config.reserva_email_obligatorio}
              onChange={(e) => cambiarCampo('reserva_email_obligatorio', e.target.checked)}
            />
            <span>Email</span>
          </label>
          <label className="toggleConfig">
            <Input
              type="checkbox"
              checked={config.reserva_telefono_obligatorio}
              onChange={(e) => cambiarCampo('reserva_telefono_obligatorio', e.target.checked)}
            />
            <span>Teléfono</span>
          </label>
        </div>
      </section>

      <section className="seccionConfig">
        <h2>Impuestos</h2>
        <Input
          etiqueta="IVA por defecto (%)"
          type="number"
          min={0}
          max={100}
          step={0.01}
          value={config.iva_por_defecto}
          onChange={(e) => cambiarCampo('iva_por_defecto', Number(e.target.value))}
        />
      </section>

      <div className="accionesConfig">
        <Boton onClick={guardar} disabled={guardando}>
          {guardando ? 'Guardando...' : 'Guardar configuración'}
        </Boton>
        {mensaje && <span className={`mensajeConfig ${mensaje.includes('Error') ? 'error' : 'exito'}`}>{mensaje}</span>}
      </div>
    </div>
  );
}

export default Configuracion;
