/* [263A-14] Mesa arrastrable con @dnd-kit para el canvas del plano de sala */

import { useDraggable } from '@dnd-kit/core';
import type { ActualizarMesaRequest, Mesa } from '../../api/generated';
import { Button } from '@/components/ui/button';
import { useMesaResize } from '../../hooks/useMesaResize';

interface MesaDraggableProps {
  mesa: Mesa;
  seleccionada: boolean;
  arrastrando: boolean;
  zoom: number;
  zonaAncho: number;
  zonaAlto: number;
  onResize: (id: string, data: ActualizarMesaRequest) => void | Promise<void>;
  onClick: () => void;
}

function MesaDraggable({
  mesa,
  seleccionada,
  arrastrando,
  zoom,
  zonaAncho,
  zonaAlto,
  onResize,
  onClick,
}: MesaDraggableProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: mesa.id,
  });

  const { previewRect, resizing, onResizeStart } = useMesaResize({
    rect: {
      x: mesa.pos_x,
      y: mesa.pos_y,
      width: mesa.ancho,
      height: mesa.alto,
    },
    bounds: {
      width: zonaAncho * zoom,
      height: zonaAlto * zoom,
    },
    zoom,
    onCommit: (rect) => onResize(mesa.id, {
      pos_x: Math.round(rect.x / zoom),
      pos_y: Math.round(rect.y / zoom),
      ancho: Math.round(rect.width / zoom),
      alto: Math.round(rect.height / zoom),
    }),
  });

  const style: React.CSSProperties = {
    left: previewRect.x,
    top: previewRect.y,
    width: previewRect.width,
    height: previewRect.height,
    transform: !resizing && transform
      ? `translate(${transform.x}px, ${transform.y}px)`
      : undefined,
  };

  /* [303A-10] Aplica la clase de forma directamente (cuadrada/redonda/rectangular)
   * para que CSS diferencie cada tipo. Antes solo aplicaba 'redonda'. */
  const clases = [
    'planoMesa',
    mesa.forma,
    !mesa.activa ? 'inactiva' : '',
    seleccionada ? 'seleccionada' : '',
    arrastrando ? 'arrastrando' : '',
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div
      ref={setNodeRef}
      className={clases}
      style={style}
      onClick={(e) => {
        e.stopPropagation();
        onClick();
      }}
      {...listeners}
      {...attributes}
    >
      <span className="planoMesaNumero">{mesa.numero}</span>
      <span className="planoMesaCapacidad">
        {mesa.min_personas}-{mesa.max_personas}
      </span>
      {seleccionada && (
        <>
          <Button type="button" variant="ghost" size="icon-xs" className="planoMesaResizeHandle norte" onMouseDown={onResizeStart('n')} aria-label="Redimensionar alto desde arriba" />
          <Button type="button" variant="ghost" size="icon-xs" className="planoMesaResizeHandle sur" onMouseDown={onResizeStart('s')} aria-label="Redimensionar alto desde abajo" />
          <Button type="button" variant="ghost" size="icon-xs" className="planoMesaResizeHandle este" onMouseDown={onResizeStart('e')} aria-label="Redimensionar ancho desde la derecha" />
          <Button type="button" variant="ghost" size="icon-xs" className="planoMesaResizeHandle oeste" onMouseDown={onResizeStart('w')} aria-label="Redimensionar ancho desde la izquierda" />
        </>
      )}
    </div>
  );
}

export default MesaDraggable;
