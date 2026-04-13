/* [134A-3] Pared arrastrable — pointer events nativos con setPointerCapture.
 * [134A-6] Fix ancho/alto con rotación: cuando la pared está rotada ~90°/270°,
 * el handle E cambia alto y el S cambia ancho (swap de ejes). Handles más finos.
 * Mínimo igual para ancho y alto (MIN_DIM). */

import { useRef, useState, useEffect } from 'react';
import { RotateCw } from 'lucide-react';
import type { ParedSala } from '../../api/generated/gestionRestauranteAPI.schemas';
import type { ActualizarParedRequest } from '../../api/generated';

type DragMode = 'none' | 'move' | 'rotate' | 'resize-e' | 'resize-s';
const MIN_DIM = 10;

/* Determina si la rotación está más cerca de 90°/270° (ejes visuamente invertidos) */
function isSwapped(deg: number): boolean {
  const norm = ((deg % 360) + 360) % 360;
  return (norm > 45 && norm < 135) || (norm > 225 && norm < 315);
}

interface Props {
  pared: ParedSala;       /* display coords (× zoom) */
  canonical: ParedSala;   /* canonical coords (sin escala) — usado en callbacks */
  zoom: number;
  zonaAncho: number;      /* canónico */
  zonaAlto: number;       /* canónico */
  seleccionada: boolean;
  onClick: () => void;
  onMoveEnd: (id: string, x: number, y: number) => void;
  onRotateEnd: (id: string, deg: number) => void;
  onResizeEnd: (id: string, req: ActualizarParedRequest) => void;
}

export default function ParedDraggable({
  pared, canonical, zoom, zonaAncho, zonaAlto, seleccionada, onClick, onMoveEnd, onRotateEnd, onResizeEnd,
}: Props) {
  const divRef = useRef<HTMLDivElement>(null);
  const [, forceUpdate] = useState(0);

  /* Modo activo del drag */
  const mode = useRef<DragMode>('none');
  const startMouse = useRef({ x: 0, y: 0 });
  const startRect = useRef({ x: 0, y: 0, w: 0, h: 0 });

  /* Valores de preview (display coords) */
  const previewX = useRef(pared.pos_x);
  const previewY = useRef(pared.pos_y);
  const previewW = useRef(pared.ancho);
  const previewH = useRef(pared.alto);
  const previewRot = useRef(canonical.rotacion);

  /* Refs para callbacks y valores que cambian sin reinstalar handlers */
  const canonicalRef = useRef(canonical);
  const zoomRef = useRef(zoom);
  const zonaRef = useRef({ ancho: zonaAncho, alto: zonaAlto });
  const onMoveEndRef = useRef(onMoveEnd);
  const onRotateEndRef = useRef(onRotateEnd);
  const onResizeEndRef = useRef(onResizeEnd);

  useEffect(() => { canonicalRef.current = canonical; }, [canonical]);
  useEffect(() => { zoomRef.current = zoom; }, [zoom]);
  useEffect(() => { zonaRef.current = { ancho: zonaAncho, alto: zonaAlto }; }, [zonaAncho, zonaAlto]);
  useEffect(() => { onMoveEndRef.current = onMoveEnd; }, [onMoveEnd]);
  useEffect(() => { onRotateEndRef.current = onRotateEnd; }, [onRotateEnd]);
  useEffect(() => { onResizeEndRef.current = onResizeEnd; }, [onResizeEnd]);

  /* Sincronizar preview cuando llega dato nuevo del servidor (post-guardar) */
  useEffect(() => {
    if (mode.current === 'none') {
      previewX.current = pared.pos_x;
      previewY.current = pared.pos_y;
      previewW.current = pared.ancho;
      previewH.current = pared.alto;
      previewRot.current = canonical.rotacion;
      forceUpdate(n => n + 1);
    }
  }, [pared.pos_x, pared.pos_y, pared.ancho, pared.alto, canonical.rotacion]);

  /* Todos los modos capturan el pointer en el div padre para recibir pointermove/up
   * incluso si el cursor sale del elemento. */
  const captureOnParent = (e: React.PointerEvent) => {
    divRef.current?.setPointerCapture(e.pointerId);
  };

  const onPointerDownBody = (e: React.PointerEvent) => {
    if (e.shiftKey) return;  /* shift reservado para pan del canvas */
    e.stopPropagation();     /* evita que dnd-kit vea el evento */
    e.preventDefault();
    onClick();
    mode.current = 'move';
    startMouse.current = { x: e.clientX, y: e.clientY };
    startRect.current = { x: previewX.current, y: previewY.current, w: previewW.current, h: previewH.current };
    captureOnParent(e);
  };

  const onPointerDownRotate = (e: React.PointerEvent) => {
    e.stopPropagation();
    e.preventDefault();
    mode.current = 'rotate';
    captureOnParent(e);
  };

  const onPointerDownResizeE = (e: React.PointerEvent) => {
    e.stopPropagation();
    e.preventDefault();
    mode.current = 'resize-e';
    startMouse.current = { x: e.clientX, y: e.clientY };
    startRect.current = { x: previewX.current, y: previewY.current, w: previewW.current, h: previewH.current };
    captureOnParent(e);
  };

  const onPointerDownResizeS = (e: React.PointerEvent) => {
    e.stopPropagation();
    e.preventDefault();
    mode.current = 'resize-s';
    startMouse.current = { x: e.clientX, y: e.clientY };
    startRect.current = { x: previewX.current, y: previewY.current, w: previewW.current, h: previewH.current };
    captureOnParent(e);
  };

  const onPointerMove = (e: React.PointerEvent) => {
    if (mode.current === 'none') return;
    const dx = e.clientX - startMouse.current.x;
    const dy = e.clientY - startMouse.current.y;
    const z = zoomRef.current;
    const zona = zonaRef.current;
    const c = canonicalRef.current;

    if (mode.current === 'move') {
      const maxX = (zona.ancho - c.ancho) * z;
      const maxY = (zona.alto - c.alto) * z;
      previewX.current = Math.min(Math.max(maxX, 0), Math.max(0, startRect.current.x + dx));
      previewY.current = Math.min(Math.max(maxY, 0), Math.max(0, startRect.current.y + dy));
    } else if (mode.current === 'rotate' && divRef.current) {
      const rect = divRef.current.getBoundingClientRect();
      const cx = rect.left + rect.width / 2;
      const cy = rect.top + rect.height / 2;
      let angle = Math.atan2(e.clientY - cy, e.clientX - cx) * (180 / Math.PI) + 90;
      angle = Math.round(angle / 15) * 15;
      previewRot.current = ((angle % 360) + 360) % 360;
    } else if (mode.current === 'resize-e') {
      /* [134A-6] Si la pared está rotada ~90°/270°, el handle E (visual: derecha)
       * controla el alto canónico en vez del ancho. */
      const swapped = isSwapped(previewRot.current);
      if (swapped) {
        previewH.current = Math.max(MIN_DIM * z, startRect.current.h + dx);
      } else {
        previewW.current = Math.max(MIN_DIM * z, startRect.current.w + dx);
      }
    } else if (mode.current === 'resize-s') {
      const swapped = isSwapped(previewRot.current);
      if (swapped) {
        previewW.current = Math.max(MIN_DIM * z, startRect.current.w + dy);
      } else {
        previewH.current = Math.max(MIN_DIM * z, startRect.current.h + dy);
      }
    }
    forceUpdate(n => n + 1);
  };

  const onPointerUp = () => {
    if (mode.current === 'none') return;
    const z = zoomRef.current;
    const c = canonicalRef.current;
    const m = mode.current;
    mode.current = 'none';

    if (m === 'move') {
      onMoveEndRef.current(c.id, Math.round(previewX.current / z), Math.round(previewY.current / z));
    } else if (m === 'rotate') {
      onRotateEndRef.current(c.id, previewRot.current);
    } else if (m === 'resize-e' || m === 'resize-s') {
      onResizeEndRef.current(c.id, {
        pos_x: Math.round(previewX.current / z),
        pos_y: Math.round(previewY.current / z),
        ancho: Math.round(previewW.current / z),
        alto: Math.round(previewH.current / z),
        color: c.color,
        rotacion: previewRot.current,
      });
    }
    forceUpdate(n => n + 1);
  };

  return (
    <div
      ref={divRef}
      className="absolute rounded-sm border border-border/50 select-none"
      style={{
        left: previewX.current,
        top: previewY.current,
        width: previewW.current,
        height: previewH.current,
        backgroundColor: canonical.color || '#6b7280',
        transform: previewRot.current ? `rotate(${previewRot.current}deg)` : undefined,
        transformOrigin: 'center center',
        boxShadow: seleccionada ? '0 0 0 2px hsl(var(--primary))' : undefined,
        cursor: mode.current === 'move' ? 'grabbing' : 'grab',
        zIndex: 1,
        touchAction: 'none',
      }}
      onPointerDown={onPointerDownBody}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onClick={e => e.stopPropagation()}
    >
      {seleccionada && (
        <>
          {/* Handle de rotación — encima del centro */}
          <div
            className="absolute -top-7 left-1/2 -translate-x-1/2 w-5 h-5 rounded-full bg-primary flex items-center justify-center shadow-md hover:scale-110 transition-transform"
            style={{ zIndex: 3, touchAction: 'none', cursor: 'grab' }}
            onPointerDown={onPointerDownRotate}
            title="Arrastrar para rotar (snap 15°)"
          >
            <RotateCw className="size-3 text-primary-foreground pointer-events-none" />
          </div>
          {/* Handle resize E (borde derecho) — más fino */}
          <div
            className="absolute top-1/2 -right-1 -translate-y-1/2 w-2 h-5 rounded-sm bg-primary/80 hover:bg-primary"
            style={{ zIndex: 3, touchAction: 'none', cursor: 'ew-resize' }}
            onPointerDown={onPointerDownResizeE}
            title="Arrastrar para cambiar dimensión"
          />
          {/* Handle resize S (borde inferior) — más fino */}
          <div
            className="absolute left-1/2 -bottom-1 -translate-x-1/2 w-5 h-2 rounded-sm bg-primary/80 hover:bg-primary"
            style={{ zIndex: 3, touchAction: 'none', cursor: 'ns-resize' }}
            onPointerDown={onPointerDownResizeS}
            title="Arrastrar para cambiar dimensión"
          />
        </>
      )}
    </div>
  );
}



