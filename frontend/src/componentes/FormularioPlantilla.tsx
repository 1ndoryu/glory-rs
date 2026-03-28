/* [263A-24] Formulario de creación de plantillas WhatsApp.
 * Permite definir nombre, categoría, idioma, cabecera, cuerpo, pie y media.
 * Al crear, la plantilla queda en estado "borrador" lista para enviar a Meta. */

import { useFormularioPlantilla } from '../hooks/useFormularioPlantilla';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Label } from '@/components/ui/label';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

export default function FormularioPlantilla() {
  const f = useFormularioPlantilla();

  return (
    <div className="mx-auto max-w-3xl w-full space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Nueva Plantilla WhatsApp</CardTitle>
          <CardDescription>
            Crea una plantilla para enviarla a Meta Business API para su aprobación
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="nombre">Nombre de la plantilla *</Label>
            <Input
              id="nombre"
              value={f.nombre}
              onChange={e => f.setNombre(e.target.value)}
              placeholder="Ej: promocion_verano"
              maxLength={255}
            />
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>Categoría</Label>
              <Select value={f.categoria} onValueChange={f.setCategoria}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {f.CATEGORIAS.map(cat => (
                    <SelectItem key={cat} value={cat}>
                      {cat === 'MARKETING' ? 'Marketing' : cat === 'UTILITY' ? 'Utilidad' : 'Autenticación'}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Idioma</Label>
              <Select value={f.idioma} onValueChange={f.setIdioma}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {f.IDIOMAS.map(i => (
                    <SelectItem key={i.value} value={i.value}>
                      {i.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Contenido del mensaje</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="cabecera">Cabecera (opcional)</Label>
            <Input
              id="cabecera"
              value={f.cabeceraTexto}
              onChange={e => f.setCabeceraTexto(e.target.value)}
              placeholder="Texto de cabecera"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="cuerpo">Cuerpo del mensaje *</Label>
            <Textarea
              id="cuerpo"
              value={f.cuerpoMensaje}
              onChange={e => f.setCuerpoMensaje(e.target.value)}
              placeholder="Escribe el contenido de la plantilla..."
              rows={6}
            />
            <p className="text-xs text-muted-foreground">
              {f.cuerpoMensaje.length} caracteres
            </p>
          </div>

          <div className="space-y-2">
            <Label htmlFor="pie">Pie de mensaje (opcional, máx 60 caracteres)</Label>
            <Input
              id="pie"
              value={f.pieTexto}
              onChange={e => f.setPieTexto(e.target.value)}
              placeholder="Texto de pie"
              maxLength={60}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Media (opcional)</CardTitle>
          <CardDescription>
            Adjunta una imagen, vídeo o documento a la cabecera
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="mediaUrl">URL del archivo</Label>
            <Input
              id="mediaUrl"
              value={f.cabeceraMediaUrl}
              onChange={e => f.setCabeceraMediaUrl(e.target.value)}
              placeholder="https://ejemplo.com/imagen.jpg"
              type="url"
            />
          </div>
          <div className="space-y-2">
            <Label>Tipo de media</Label>
            <Select
              value={f.cabeceraMediaTipo || '__none__'}
              onValueChange={v => f.setCabeceraMediaTipo(v === '__none__' ? '' : v)}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">Sin media</SelectItem>
                <SelectItem value="image">Imagen</SelectItem>
                <SelectItem value="video">Vídeo</SelectItem>
                <SelectItem value="document">Documento</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <div className="flex justify-end gap-3">
        <Button variant="outline" onClick={f.cancelar}>
          Cancelar
        </Button>
        <Button
          onClick={f.crearPlantilla}
          disabled={!f.formValido || f.enviando}
        >
          {f.enviando ? 'Creando...' : 'Crear Plantilla'}
        </Button>
      </div>
    </div>
  );
}
