/* [263A-16] Pantalla "Restablecer contraseña" — reescrita con shadcn.
 * Token via query param (?token=...). Hook useResetPasswordForm maneja lógica. */

import { Link } from 'react-router-dom';
import useResetPasswordForm from '../hooks/useResetPasswordForm';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';

function ResetPassword() {
  const { form, set, manejarEnvio, cargando } = useResetPasswordForm();

  return (
    <div className="flex min-h-screen items-center justify-center bg-background p-4">
      <Card className="w-full max-w-sm">
        <CardHeader className="text-center">
          <CardTitle className="text-2xl">Nueva contraseña</CardTitle>
          <CardDescription>
            {form.exito
              ? 'Contraseña actualizada. Redirigiendo al login...'
              : 'Introduce tu nueva contraseña.'}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {form.error && (
            <div className="mb-4 rounded-md bg-destructive/10 p-3 text-sm text-destructive">
              {form.error}
            </div>
          )}

          {!form.exito && (
            <form className="flex flex-col gap-4" onSubmit={manejarEnvio}>
              <div className="flex flex-col gap-2">
                <Label htmlFor="password">Nueva contraseña</Label>
                <Input
                  id="password"
                  type="password"
                  value={form.password}
                  onChange={(e) => set({ password: e.target.value })}
                  placeholder="Mínimo 8 caracteres"
                  autoComplete="new-password"
                />
              </div>

              <div className="flex flex-col gap-2">
                <Label htmlFor="confirmar">Confirmar contraseña</Label>
                <Input
                  id="confirmar"
                  type="password"
                  value={form.confirmar}
                  onChange={(e) => set({ confirmar: e.target.value })}
                  placeholder="Repite la contraseña"
                  autoComplete="new-password"
                />
              </div>

              <Button type="submit" className="w-full" disabled={cargando}>
                {cargando ? 'Restableciendo...' : 'Restablecer contraseña'}
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

export default ResetPassword;
