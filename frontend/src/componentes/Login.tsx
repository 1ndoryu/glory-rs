/* 253A-7: Login — formulario de autenticación
   253A-10: hook useLoginForm + componentes UI atómicos */

import useLoginForm from '../hooks/useLoginForm';
import { Input, Boton } from './ui';
import '../estilos/Login.css';

function Login() {
  const { credenciales, cambiarCredencial, modoRegistro, alternarModo, error, manejarEnvio, cargando } = useLoginForm();

  return (
    <div className="contenedorLogin">
      <div className="tarjetaLogin">
        <h1 className="tituloLogin">Gestión Restaurante</h1>
        <p className="subtituloLogin">
          {modoRegistro ? 'Crea tu cuenta' : 'Inicia sesión para continuar'}
        </p>

        {error && <div className="errorLogin">{error}</div>}

        <form className="formularioLogin" onSubmit={manejarEnvio}>
          <div className="grupoInput">
            <label className="etiqueta" htmlFor="email">Email</label>
            <Input
              id="email"
              type="email"
              value={credenciales.email}
              onChange={(e) => cambiarCredencial('email', e.target.value)}
              placeholder="correo@ejemplo.com"
              autoComplete="email"
            />
          </div>

          <div className="grupoInput">
            <label className="etiqueta" htmlFor="password">Contraseña</label>
            <Input
              id="password"
              type="password"
              value={credenciales.password}
              onChange={(e) => cambiarCredencial('password', e.target.value)}
              placeholder="••••••••"
              autoComplete={modoRegistro ? 'new-password' : 'current-password'}
            />
          </div>

          <Boton className="botonLogin" type="submit" disabled={cargando}>
            {cargando ? 'Cargando...' : modoRegistro ? 'Crear cuenta' : 'Entrar'}
          </Boton>
        </form>

        <p className="enlaceRegistro">
          {modoRegistro ? '¿Ya tienes cuenta? ' : '¿No tienes cuenta? '}
          <span onClick={alternarModo}>
            {modoRegistro ? 'Inicia sesión' : 'Regístrate'}
          </span>
        </p>
      </div>
    </div>
  );
}

export default Login;
