/* [134A-15] Toolbar flotante tipo Illustrator para el canvas del plano.
 * 2 columnas de iconos, esquina superior derecha del canvas.
 * Cada herramienta activa un modo de interacción distinto en el canvas. */

import { MousePointer2, Square, Circle, BrickWall, Eraser, Hand, Combine } from 'lucide-react';

export type CanvasTool = 'select' | 'mesa-cuadrada' | 'mesa-redonda' | 'pared' | 'delete' | 'pan' | 'combine';

const tools: { id: CanvasTool; icon: typeof MousePointer2; label: string }[] = [
  { id: 'select', icon: MousePointer2, label: 'Seleccionar' },
  { id: 'mesa-cuadrada', icon: Square, label: 'Mesa cuadrada' },
  { id: 'mesa-redonda', icon: Circle, label: 'Mesa redonda' },
  { id: 'pared', icon: BrickWall, label: 'Dibujar pared' },
  { id: 'delete', icon: Eraser, label: 'Borrar elemento' },
  { id: 'pan', icon: Hand, label: 'Mover plano' },
  { id: 'combine', icon: Combine, label: 'Combinar mesas' },
];

interface Props {
  activeTool: CanvasTool;
  onToolChange: (tool: CanvasTool) => void;
  disabled?: boolean;
}

export default function CanvasToolbar({ activeTool, onToolChange, disabled }: Props) {
  return (
    <div className="absolute top-2 left-2 z-50 bg-card border border-border rounded-lg shadow-md p-1.5 grid grid-cols-2 gap-1">
      {tools.map(tool => {
        const Icon = tool.icon;
        const isActive = activeTool === tool.id;
        return (
          <button
            key={tool.id}
            type="button"
            disabled={disabled}
            className={`flex items-center justify-center w-8 h-8 rounded transition-colors ${
              isActive
                ? 'bg-primary text-primary-foreground'
                : 'hover:bg-muted text-muted-foreground hover:text-foreground'
            } ${disabled ? 'opacity-40 cursor-not-allowed' : 'cursor-pointer'}`}
            onClick={() => onToolChange(tool.id)}
            title={tool.label}
          >
            <Icon className="size-4" />
          </button>
        );
      })}
    </div>
  );
}
