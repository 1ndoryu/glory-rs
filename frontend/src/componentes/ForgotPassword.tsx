/* [263A-15] Pantalla "Olvidé mi contraseña" — solicita email para enviar enlace de reset.
 * Usa useForgotPassword de Orval. Siempre muestra éxito para no revelar si el email existe. */

import { useState, FormEvent } from 'react';
import { Link } from 'react-router-dom';
import { Input, Boton } from '@glory/componentes/ui';
import { useForgotPassword } from '../api/generated';
import '../estilos/Login.css';

function ForgotPassword() {
  const [email, setEmail] = useState('');
  const [enviado, setEnviado] = useState(false);
  const [error, setError] = useState('');

  const mutation = useForgotPassword({
    mutation: {
      onSuccess: () => setEnviado(true),
      onError: () => setError('Error al enviar la solicitud. Inténtalo de nuevo.'),
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');
    if (!email) {
      setError('Introduce tu email');
      return;
    }
    mutation.mutate({ data: { email } });
  };

  return (
    <div className="contenedorLogin">
      <div className="tarjetaLogin">
        <h1 className="tituloLogin">Recuperar contraseña</h1>
        <p className="subtituloLogin">
          {enviado
            ? 'Si el email existe, recibirás un enlace para restablecer tu contraseña.'
            : 'Introduce tu email y te enviaremos un enlace para restablecer tu contraseña.'}
        </p>

        {error && <div className="errorLogin">{error}</div>}

        {!enviado && (
          <form className="formularioLogin" onSubmit={manejarEnvio}>
            <div className="grupoInput">
              <label className="etiqueta" htmlFor="email">Email</label>
              <Input
                id="email"
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="correo@ejemplo.com"
                autoComplete="email"
              />
            </div>

            <Boton variante="primario" ancho type="submit" cargando={mutation.isPending}>
              Enviar enlace
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

export default ForgotPassword;
