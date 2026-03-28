/* [283A-17] Draggable "Mesa" template — arrastra al canvas para crear mesa en posición.
 * Se usa en el toolbar del PlanoSala como fuente drag-and-drop. */

import { useDraggable } from '@dnd-kit/core';
import { GripVertical } from 'lucide-react';

function MesaTemplate({ disabled }: { disabled: boolean }) {
  const { attributes, listeners, setNodeRef, isDragging } = useDraggable({ id: 'new-mesa-template', disabled });
  return (
    <div
      ref={setNodeRef}
      {...listeners}
      {...attributes}
      className={`inline-flex items-center gap-1 px-2.5 py-1.5 text-sm font-medium border border-dashed rounded-md select-none ${disabled ? 'opacity-40 cursor-not-allowed' : 'cursor-grab'} ${isDragging ? 'opacity-50' : ''}`}
    >
      <GripVertical className="size-4" /> Mesa
    </div>
  );
}

export default MesaTemplate;
