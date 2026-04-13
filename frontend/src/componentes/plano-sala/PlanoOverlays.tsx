/* [134A-15] Overlays del canvas: minimap + indicadores off-screen.
 * Extraído de PlanoSala para reducir líneas. Centraliza el cálculo
 * de posiciones escaladas (zoom) que minimap e indicators comparten. */

import { useMemo } from 'react';
import CanvasMinimap from './CanvasMinimap';
import OffScreenIndicators from './OffScreenIndicators';
import type { Mesa, ParedSala } from '../../api/generated';

interface Props {
  mesasZona: Mesa[];
  paredesZona: ParedSala[];
  posicionesLocales: Record<string, { x: number; y: number }>;
  dimensionesLocales: Record<string, { ancho: number; alto: number }>;
  zoom: number;
  contentBounds: { w: number; h: number };
  viewportSize: { w: number; h: number };
  panOffset: { x: number; y: number };
  setPanOffset: (offset: { x: number; y: number }) => void;
}

export default function PlanoOverlays(props: Props) {
  const {
    mesasZona, paredesZona, posicionesLocales, dimensionesLocales,
    zoom, contentBounds, viewportSize, panOffset, setPanOffset,
  } = props;

  const mesasScaled = useMemo(() => mesasZona.map(m => {
    const p = posicionesLocales[m.id];
    const d = dimensionesLocales[m.id];
    return {
      x: (p?.x ?? m.pos_x) * zoom,
      y: (p?.y ?? m.pos_y) * zoom,
      ancho: (d?.ancho ?? m.ancho) * zoom,
      alto: (d?.alto ?? m.alto) * zoom,
    };
  }), [mesasZona, posicionesLocales, dimensionesLocales, zoom]);

  const paredesScaled = useMemo(() => paredesZona.map(p => ({
    x: p.pos_x * zoom,
    y: p.pos_y * zoom,
    ancho: p.ancho * zoom,
    alto: p.alto * zoom,
    rotacion: p.rotacion,
  })), [paredesZona, zoom]);

  const navigate = (x: number, y: number) => setPanOffset({ x, y });

  return (
    <>
      <CanvasMinimap
        mesas={mesasScaled}
        paredes={paredesScaled}
        contentWidth={contentBounds.w}
        contentHeight={contentBounds.h}
        viewportWidth={viewportSize.w}
        viewportHeight={viewportSize.h}
        scrollLeft={panOffset.x}
        scrollTop={panOffset.y}
        onNavigate={navigate}
      />
      <OffScreenIndicators
        mesas={mesasScaled}
        viewportWidth={viewportSize.w}
        viewportHeight={viewportSize.h}
        scrollLeft={panOffset.x}
        scrollTop={panOffset.y}
        onNavigate={navigate}
      />
    </>
  );
}
