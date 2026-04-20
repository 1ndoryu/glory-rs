/* [204A-1] AuthPage reescribe la entrada de sesión del legado dentro de la SPA Vite.
 * Se mantiene el contrato 1:1 con Orval/Auth del backend, pero la UI se rehace
 * como página SPA tipada porque las islands importadas siguen dependiendo del árbol legacy. */

import { useState, type ChangeEvent, type FormEvent } from 'react';
import axios from 'axios';
import { Link, Navigate, useNavigate } from 'react-router-dom';
import Boton from '../components/ui/Boton';
import Input from '../components/ui/Input';
import { useAuth } from '../hooks/useAuth';

type AuthMode = 'login' | 'register';

type AuthPageProps = {
  mode: AuthMode;
};

type AuthFormState = {
  email: string;
  identifier: string;
  nombreVisible: string;
  password: string;
  username: string;
};

const initialFormState: AuthFormState = {
  email: '',
  identifier: '',
  nombreVisible: '',
  password: '',
  username: '',
};

function getErrorMessage(error: unknown): string {
  if (axios.isAxiosError(error)) {
    const apiMessage = (error.response?.data as { message?: string } | undefined)?.message;
    return apiMessage ?? error.message;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return 'No se pudo completar la operación.';
}

export default function AuthPage({ mode }: AuthPageProps) {
  const auth = useAuth();
  const navigate = useNavigate();
  const [formState, setFormState] = useState<AuthFormState>(initialFormState);
  const [submitError, setSubmitError] = useState<string | null>(null);

  const isLogin = mode === 'login';
  const title = isLogin ? 'Inicia sesión en Kamples' : 'Crea tu cuenta en Kamples';
  const description = isLogin
    ? 'Accede a tu feed, tus descargas y el panel del creador desde la SPA real.'
    : 'Regístrate para guardar favoritos, seguir creadores y activar tu panel mínimo.';

  if (auth.isAuthenticated) {
    return <Navigate replace to="/dashboard" />;
  }

  const handleChange = (field: keyof AuthFormState) => (event: ChangeEvent<HTMLInputElement>) => {
    const { value } = event.target;
    setFormState((current) => ({ ...current, [field]: value }));
  };

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSubmitError(null);

    try {
      if (isLogin) {
        await auth.login({
          identifier: formState.identifier.trim(),
          password: formState.password,
        });
      } else {
        await auth.register({
          email: formState.email.trim(),
          nombre_visible: formState.nombreVisible.trim() || undefined,
          password: formState.password,
          username: formState.username.trim(),
        });
      }

      navigate('/dashboard', { replace: true });
    } catch (error) {
      setSubmitError(getErrorMessage(error));
    }
  };

  if (auth.isLoading) {
    return (
      <section className="authPagina">
        <div className="authTarjeta authTarjetaCentrada">
          <p className="estadoNeutral">Validando tu sesión…</p>
        </div>
      </section>
    );
  }

  return (
    <section className="authPagina">
      <article className="authTarjeta">
        <div className="authCabecera">
          <p className="heroEyebrow">204A-1 · SPA real</p>
          <h1 className="authTitulo">{title}</h1>
          <p className="authDescripcion">{description}</p>
        </div>

        <form className="authFormulario" onSubmit={handleSubmit}>
          {!isLogin && (
            <>
              <label className="authCampo">
                <span>Nombre visible</span>
                <Input
                  autoComplete="name"
                  onChange={handleChange('nombreVisible')}
                  placeholder="Tu alias o nombre artístico"
                  value={formState.nombreVisible}
                />
              </label>

              <label className="authCampo">
                <span>Username</span>
                <Input
                  autoComplete="username"
                  onChange={handleChange('username')}
                  placeholder="productor_01"
                  required
                  value={formState.username}
                />
              </label>

              <label className="authCampo">
                <span>Email</span>
                <Input
                  autoComplete="email"
                  onChange={handleChange('email')}
                  placeholder="tu@email.com"
                  required
                  type="email"
                  value={formState.email}
                />
              </label>
            </>
          )}

          {isLogin && (
            <label className="authCampo">
              <span>Email o username</span>
              <Input
                autoComplete="username"
                onChange={handleChange('identifier')}
                placeholder="tu@email.com o productor_01"
                required
                value={formState.identifier}
              />
            </label>
          )}

          <label className="authCampo">
            <span>Contraseña</span>
            <Input
              autoComplete={isLogin ? 'current-password' : 'new-password'}
              minLength={8}
              onChange={handleChange('password')}
              placeholder="Mínimo 8 caracteres"
              required
              type="password"
              value={formState.password}
            />
          </label>

          {submitError && <p className="mensajeError">{submitError}</p>}

          <div className="authAcciones">
            <Boton className="accionPrimaria" disabled={auth.isSubmitting} type="submit">
              {auth.isSubmitting
                ? (isLogin ? 'Entrando…' : 'Creando cuenta…')
                : (isLogin ? 'Entrar' : 'Crear cuenta')}
            </Boton>
            <Link className="accionSecundaria" to="/">
              Volver al feed
            </Link>
          </div>
        </form>
      </article>

      <aside className="authPanelSecundario">
        <p className="heroEyebrow">Migración frontend</p>
        <h2>Qué se conserva en esta fase</h2>
        <ul className="listaSimple">
          <li>Auth real contra Orval y JWT del backend Rust.</li>
          <li>Navegación SPA con rutas estables para login, registro y dashboard.</li>
          <li>Entrada limpia sin depender de imports `@app/*` del legado.</li>
        </ul>

        <p className="authAlternativa">
          {isLogin ? '¿Todavía no tienes cuenta?' : '¿Ya tienes una cuenta?'}{' '}
          <Link to={isLogin ? '/auth/registro' : '/auth/login'}>
            {isLogin ? 'Regístrate aquí' : 'Entra aquí'}
          </Link>
        </p>
      </aside>
    </section>
  );
}
