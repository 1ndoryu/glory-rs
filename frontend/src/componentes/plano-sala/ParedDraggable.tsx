/* [134A-3] Pared arrastrable del plano de sala.
 * Drag con mouse para mover; handle circular encima del centro para rotar en múltiplos de 15°.
 * Usa refs para los event listeners globales — sin reinstalar en cada render.
 * Patrón de previewX/Y en refs para evitar state stale dentro de los handlers. */

import { useRef, useState, useEffect } from 'react';
import { RotateCw } from 'lucide-react';
import type { ParedSala } from '../../api/generated/gestionRestauranteAPI.schemas';

interface Props {
  /* Pared en coords de display (pos × zoom, ancho × zoom) */
  pared: ParedSala;
  /* Pared en coords canónicas (sin escala) — para onMoveEnd y onRotateEnd */
  canonical: ParedSala;
  zoom: number;
  zonaAncho: number; /* canónico */
  zonaAlto: number;  /* canónico */
  seleccionada: boolean;
  onClick: () => void;
  onMoveEnd: (id: string, x: number, y: number) => void;
  onRotateEnd: (id: string, deg: number) => void;
}

export default function ParedDraggable({
  pared, canonical, zoom, zonaAncho, zonaAlto, seleccionada, onClick, onMoveEnd, onRotateEnd,
}: Props) {
  const divRef = useRef<HTMLDivElement>(null);
  const [, forceUpdate] = useState(0);

  /* Refs de estado mutable — evitan reinstalar listeners en cada cambio */
  const previewXRef = useRef(pared.pos_x);
  const previewYRef = useRef(pared.pos_y);
  const previewRotRef = useRef(canonical.rotacion);

  const dragging = useRef(false);
  const rotating = useRef(false);
  const startMouse = useRef({ x: 0, y: 0 });
  const startPos = useRef({ x: 0, y: 0 });

  /* Refs sincronizados con props para que los handlers globales accedan al valor actual */
  const zoomRef = useRef(zoom);
  const maxXRef = useRef(zonaAncho * zoom - pared.ancho);
  const maxYRef = useRef(zonaAlto * zoom - pared.alto);
  const onMoveEndRef = useRef(onMoveEnd);
  const onRotateEndRef = useRef(onRotateEnd);
  const idRef = useRef(canonical.id);

  useEffect(() => { zoomRef.current = zoom; }, [zoom]);
  useEffect(() => { onMoveEndRef.current = onMoveEnd; }, [onMoveEnd]);
  useEffect(() => { onRotateEndRef.current = onRotateEnd; }, [onRotateEnd]);
  useEffect(() => {
    maxXRef.current = zonaAncho * zoom - pared.ancho;
    maxYRef.current = zonaAlto * zoom - pared.alto;
  }, [zonaAncho, zonaAlto, zoom, pared.ancho, pared.alto]);

  /* Sincronizar preview cuando llega dato nuevo del servidor (post-guardar) */
  useEffect(() => {
    if (!dragging.current && !rotating.current) {
      previewXRef.current = pared.pos_x;
      previewYRef.current = pared.pos_y;
      previewRotRef.current = canonical.rotacion;
      forceUpdate(n => n + 1);
    }
  }, [pared.pos_x, pared.pos_y, canonical.rotacion]);

  /* Listeners globales — instalados una sola vez, usan refs para valores variables */
  useEffect(() => {
    const onMouseMove = (e: MouseEvent) => {
      if (dragging.current) {
        const dx = e.clientX - startMouse.current.x;
        const dy = e.clientY - startMouse.current.y;
        previewXRef.current = Math.min(maxXRef.current, Math.max(0, startPos.current.x + dx));
        previewYRef.current = Math.min(maxYRef.current, Math.max(0, startPos.current.y + dy));
        forceUpdate(n => n + 1);
      }
      if (rotating.current && divRef.current) {
        const rect = divRef.current.getBoundingClientRect();
        const cx = rect.left + rect.width / 2;
        const cy = rect.top + rect.height / 2;
        let angle = Math.atan2(e.clientY - cy, e.clientX - cx) * (180 / Math.PI) + 90;
        angle = Math.round(angle / 15) * 15;
        previewRotRef.current = ((angle % 360) + 360) % 360;
        forceUpdate(n => n + 1);
      }
    };

    const onMouseUp = () => {
      if (dragging.current) {
        dragging.current = false;
        onMoveEndRef.current(
          idRef.current,
          Math.round(previewXRef.current / zoomRef.current),
          Math.round(previewYRef.current / zoomRef.current),
        );
      }
      if (rotating.current) {
        rotating.current = false;
        onRotateEndRef.current(idRef.current, previewRotRef.current);
      }
    };

    document.addEventListener('mousemove', onMouseMove);
    document.addEventListener('mouseup', onMouseUp);
    return () => {
      document.removeEventListener('mousemove', onMouseMove);
      document.removeEventListener('mouseup', onMouseUp);
    };
  }, []); /* sin deps — refs garantizan valores actuales */

  const handleMouseDown = (e: React.MouseEvent) => {
    if (e.shiftKey) return; /* shift reservado para pan del canvas */
    e.stopPropagation();
    e.preventDefault();
    onClick();
    dragging.current = true;
    startMouse.current = { x: e.clientX, y: e.clientY };
    startPos.current = { x: previewXRef.current, y: previewYRef.current };
  };

  const handleRotateMouseDown = (e: React.MouseEvent) => {
    e.stopPropagation();
    e.preventDefault();
    rotating.current = true;
  };

  return (
    <div
      ref={divRef}
      className="absolute rounded-sm border border-border/50 select-none transition-shadow"
      style={{
        left: previewXRef.current,
        top: previewYRef.current,
        width: pared.ancho,
        height: pared.alto,
        backgroundColor: canonical.color || '#6b7280',
        transform: previewRotRef.current ? `rotate(${previewRotRef.current}deg)` : undefined,
        transformOrigin: 'center center',
        boxShadow: seleccionada ? '0 0 0 2px hsl(var(--primary))' : undefined,
        cursor: dragging.current ? 'grabbing' : 'grab',
        zIndex: 1,
      }}
      onMouseDown={handleMouseDown}
      onClick={e => e.stopPropagation()}
    >
      {seleccionada && (
        <div
          className="absolute -top-7 left-1/2 -translate-x-1/2 w-5 h-5 rounded-full bg-primary flex items-center justify-center cursor-pointer shadow-md hover:scale-110 transition-transform"
          style={{ zIndex: 2 }}
          onMouseDown={handleRotateMouseDown}
          title="Arrastrar para rotar (snap 15°)"
        >
          <RotateCw className="size-3 text-primary-foreground pointer-events-none" />
        </div>
      )}
    </div>
  );
}
