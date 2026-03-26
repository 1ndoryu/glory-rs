/* [263A-16] Pantalla "Olvidé mi contraseña" — reescrita con shadcn.
 * Siempre muestra éxito para no revelar si el email existe (seguridad). */

import { useState, FormEvent } from 'react';
import { Link } from 'react-router-dom';
import { useForgotPassword } from '../api/generated';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';

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
    <div className="flex min-h-screen items-center justify-center bg-background p-4">
      <Card className="w-full max-w-sm">
        <CardHeader className="text-center">
          <CardTitle className="text-2xl">Recuperar contraseña</CardTitle>
          <CardDescription>
            {enviado
              ? 'Si el email existe, recibirás un enlace para restablecer tu contraseña.'
              : 'Introduce tu email y te enviaremos un enlace para restablecer tu contraseña.'}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {error && (
            <div className="mb-4 rounded-md bg-destructive/10 p-3 text-sm text-destructive">
              {error}
            </div>
          )}

          {!enviado && (
            <form className="flex flex-col gap-4" onSubmit={manejarEnvio}>
              <div className="flex flex-col gap-2">
                <Label htmlFor="email">Email</Label>
                <Input
                  id="email"
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder="correo@ejemplo.com"
                  autoComplete="email"
                />
              </div>

              <Button type="submit" className="w-full" disabled={mutation.isPending}>
                {mutation.isPending ? 'Enviando...' : 'Enviar enlace'}
              </Button>
            </form>
          )}
        </CardContent>
        <CardFooter className="justify-center">
          <p className="text-sm text-muted-foreground">
            <Link to="/login" className="underline underline-offset-4 hover:text-primary">
              Volver al inicio de sesión
            </Link>
          </p>
        </CardFooter>
      </Card>
    </div>
  );
}

export default ForgotPassword;
