/* [283A-27] Sección de API Keys del chatbot dentro de Configuración.
 * CRUD de claves API: crear (mostrando key una sola vez), listar con
 * prefijo y fecha, revocar con confirmación. Incluye enlace a Swagger UI. */

import { useState } from 'react';
import { toast } from 'sonner';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Copy, Trash2, Plus, ExternalLink } from 'lucide-react';
import {
  useListarApiKeys,
  useCrearApiKey,
  useRevocarApiKey,
} from '../api/generated';

function ConfigChatbot() {
  const { data, refetch } = useListarApiKeys();
  const keys = data?.status === 200 ? data.data : [];
  const crearMut = useCrearApiKey();
  const revocarMut = useRevocarApiKey();

  const [nombre, setNombre] = useState('');
  const [keyCreada, setKeyCreada] = useState<string | null>(null);

  const crearKey = async () => {
    if (!nombre.trim()) return;
    try {
      const resp = await crearMut.mutateAsync({ data: { nombre: nombre.trim() } });
      if (resp.status === 201) {
        setKeyCreada(resp.data.key);
        setNombre('');
        refetch();
        toast.success('API key creada');
      }
    } catch {
      toast.error('Error al crear API key');
    }
  };

  const revocar = async (id: string) => {
    try {
      await revocarMut.mutateAsync({ id });
      refetch();
      toast.success('API key revocada');
    } catch {
      toast.error('Error al revocar');
    }
  };

  const copiar = (text: string) => {
    navigator.clipboard.writeText(text);
    toast.success('Copiado al portapapeles');
  };

  return (
    <div className="flex flex-col gap-6">
      <Card>
        <CardHeader>
          <CardTitle>API del Chatbot</CardTitle>
          <CardDescription>
            Crea claves API para conectar tu chatbot con la plataforma.
            Las claves permiten crear y consultar reservas de forma segura.
          </CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <div className="flex items-center gap-2">
            <a
              href="/swagger-ui/"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-1 text-sm text-primary underline"
            >
              <ExternalLink className="size-3.5" />
              Documentación API (Swagger)
            </a>
          </div>

          <div className="flex gap-2">
            <div className="flex-1">
              <Label htmlFor="key-nombre" className="sr-only">Nombre de la clave</Label>
              <Input
                id="key-nombre"
                placeholder="Nombre de la clave (ej: Chatbot WhatsApp)"
                value={nombre}
                onChange={e => setNombre(e.target.value)}
                onKeyDown={e => { if (e.key === 'Enter') crearKey(); }}
              />
            </div>
            <Button onClick={crearKey} disabled={!nombre.trim()}>
              <Plus className="size-4 mr-1" />Crear clave
            </Button>
          </div>

          {keyCreada && (
            <div className="rounded-md border border-green-500/40 bg-green-50 dark:bg-green-950/20 p-3 flex flex-col gap-2">
              <p className="text-sm font-medium text-green-700 dark:text-green-400">
                Clave creada — cópiala ahora, no podrás verla de nuevo.
              </p>
              <div className="flex items-center gap-2">
                <code className="flex-1 text-xs bg-background p-2 rounded border font-mono break-all">{keyCreada}</code>
                <Button size="sm" variant="outline" onClick={() => copiar(keyCreada)}>
                  <Copy className="size-4" />
                </Button>
              </div>
              <Button size="sm" variant="ghost" className="self-end" onClick={() => setKeyCreada(null)}>
                Cerrar
              </Button>
            </div>
          )}
        </CardContent>
      </Card>

      {keys.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Claves existentes</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            {keys.map(k => (
              <div key={k.id} className="flex items-center justify-between rounded-md border p-3">
                <div className="flex flex-col gap-0.5">
                  <span className="text-sm font-medium">{k.nombre}</span>
                  <span className="text-xs text-muted-foreground font-mono">{k.key_prefix}•••</span>
                  <span className="text-xs text-muted-foreground">
                    Creada: {new Date(k.created_at).toLocaleDateString('es-ES')}
                    {k.last_used_at && ` · Último uso: ${new Date(k.last_used_at).toLocaleDateString('es-ES')}`}
                  </span>
                </div>
                <div className="flex items-center gap-2">
                  <Badge variant={k.activa ? 'default' : 'secondary'}>
                    {k.activa ? 'Activa' : 'Revocada'}
                  </Badge>
                  {k.activa && (
                    <Button size="sm" variant="destructive" onClick={() => revocar(k.id)}>
                      <Trash2 className="size-4" />
                    </Button>
                  )}
                </div>
              </div>
            ))}
          </CardContent>
        </Card>
      )}
    </div>
  );
}

export default ConfigChatbot;
