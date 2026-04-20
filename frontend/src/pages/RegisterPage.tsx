/* [204A-1] Wrapper de ruta para separar el registro dentro de la SPA real. */

import AuthPage from './AuthPage';

export default function RegisterPage() {
  return <AuthPage mode="register" />;
}
