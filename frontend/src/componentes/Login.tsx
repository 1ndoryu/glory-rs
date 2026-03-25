/* 253A-7: Login — formulario de autenticación */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useLogin, useRegister } from '../api/generated';
import { useAuthStore } from '../stores/authStore';
import '../estilos/Login.css';

function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [modoRegistro, setModoRegistro] = useState(false);
  const [error, setError] = useState('');
  const navigate = useNavigate();
  const iniciarSesion = useAuthStore((s) => s.iniciarSesion);

  const loginMutation = useLogin({
    mutation: {
      onSuccess: (respuesta) => {
        if (respuesta.status === 200) {
          iniciarSesion(respuesta.data.token);
          navigate('/');
        }
      },
      onError: () => {
        setError('Credenciales incorrectas');
      },
    },
  });

  const registerMutation = useRegister({
    mutation: {
      onSuccess: (respuesta) => {
        if (respuesta.status === 201) {
          iniciarSesion(respuesta.data.token);
          navigate('/');
        }
      },
      onError: () => {
        setError('No se pudo crear la cuenta. Verifica los datos.');
      },
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');

    if (!email || !password) {
      setError('Completa todos los campos');
      return;
    }

    if (modoRegistro) {
      registerMutation.mutate({ data: { email, password } });
    } else {
      loginMutation.mutate({ data: { email, password } });
    }
  };

  const cargando = loginMutation.isPending || registerMutation.isPending;

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
            <input
              id="email"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="correo@ejemplo.com"
              autoComplete="email"
            />
          </div>

          <div className="grupoInput">
            <label className="etiqueta" htmlFor="password">Contraseña</label>
            <input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="••••••••"
              autoComplete={modoRegistro ? 'new-password' : 'current-password'}
            />
          </div>

          <button className="botonLogin" type="submit" disabled={cargando}>
            {cargando ? 'Cargando...' : modoRegistro ? 'Crear cuenta' : 'Entrar'}
          </button>
        </form>

        <p className="enlaceRegistro">
          {modoRegistro ? '¿Ya tienes cuenta? ' : '¿No tienes cuenta? '}
          <span onClick={() => { setModoRegistro(!modoRegistro); setError(''); }}>
            {modoRegistro ? 'Inicia sesión' : 'Regístrate'}
          </span>
        </p>
      </div>
    </div>
  );
}

export default Login;
