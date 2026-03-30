/* [263A-14] Mesa arrastrable con @dnd-kit para el canvas del plano de sala */

import { useDraggable } from '@dnd-kit/core';
import type { Mesa } from '../../api/generated';

interface MesaDraggableProps {
  mesa: Mesa;
  seleccionada: boolean;
  arrastrando: boolean;
  onClick: () => void;
}

function MesaDraggable({ mesa, seleccionada, arrastrando, onClick }: MesaDraggableProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: mesa.id,
  });

  const style: React.CSSProperties = {
    left: mesa.pos_x,
    top: mesa.pos_y,
    width: mesa.ancho,
    height: mesa.alto,
    transform: transform
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
    </div>
  );
}

export default MesaDraggable;
