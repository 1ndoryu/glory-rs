/* [263A-15] Pantalla "Restablecer contraseña" — formulario para nueva contraseña.
 * Recibe token via query param (?token=...). Hook useResetPasswordForm maneja lógica.
 * Redirige al login tras éxito. */

import { Link } from 'react-router-dom';
import { Input, Boton } from '@glory/componentes/ui';
import useResetPasswordForm from '../hooks/useResetPasswordForm';
import '../estilos/Login.css';

function ResetPassword() {
  const { form, set, manejarEnvio, cargando } = useResetPasswordForm();

  return (
    <div className="contenedorLogin">
      <div className="tarjetaLogin">
        <h1 className="tituloLogin">Nueva contraseña</h1>
        <p className="subtituloLogin">
          {form.exito
            ? 'Contraseña actualizada. Redirigiendo al login...'
            : 'Introduce tu nueva contraseña.'}
        </p>

        {form.error && <div className="errorLogin">{form.error}</div>}

        {!form.exito && (
          <form className="formularioLogin" onSubmit={manejarEnvio}>
            <div className="grupoInput">
              <label className="etiqueta" htmlFor="password">Nueva contraseña</label>
              <Input
                id="password"
                type="password"
                value={form.password}
                onChange={(e) => set({ password: e.target.value })}
                placeholder="Mínimo 8 caracteres"
                autoComplete="new-password"
              />
            </div>

            <div className="grupoInput">
              <label className="etiqueta" htmlFor="confirmar">Confirmar contraseña</label>
              <Input
                id="confirmar"
                type="password"
                value={form.confirmar}
                onChange={(e) => set({ confirmar: e.target.value })}
                placeholder="Repite la contraseña"
                autoComplete="new-password"
              />
            </div>

            <Boton variante="primario" ancho type="submit" cargando={cargando}>
              Restablecer contraseña
            </Boton>
          </form>
        )}

        <p className="enlaceRegistro">
          <Link to="/login">Volver al inicio de sesión</Link>
        </p>
      </div>
    </div>
  );
}

export default ResetPassword;
