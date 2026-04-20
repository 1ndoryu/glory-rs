/* [204A-1] Wrapper de ruta para mantener el router lazy sin duplicar la UI de auth. */

import AuthPage from './AuthPage';

export default function LoginPage() {
  return <AuthPage mode="login" />;
}
