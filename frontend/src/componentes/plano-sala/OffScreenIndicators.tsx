/* [303A-12] Indicadores de mesas fuera de vista — flechas en los bordes del canvas
 * cuando hay mesas más allá del área visible del viewport.
 * [303A-18] Click en indicador navega hacia las mesas ocultas (75% del viewport). */

interface MesaPos {
  x: number;
  y: number;
  ancho: number;
  alto: number;
}

interface OffScreenIndicatorsProps {
  mesas: MesaPos[];
  viewportWidth: number;
  viewportHeight: number;
  scrollLeft: number;
  scrollTop: number;
  onNavigate: (scrollLeft: number, scrollTop: number) => void;
}

function OffScreenIndicators({
  mesas,
  viewportWidth,
  viewportHeight,
  scrollLeft,
  scrollTop,
  onNavigate,
}: OffScreenIndicatorsProps) {
  let arriba = false;
  let abajo = false;
  let izquierda = false;
  let derecha = false;

  const viewRight = scrollLeft + viewportWidth;
  const viewBottom = scrollTop + viewportHeight;

  for (const m of mesas) {
    const mesaRight = m.x + m.ancho;
    const mesaBottom = m.y + m.alto;

    if (mesaBottom < scrollTop) arriba = true;
    if (m.y > viewBottom) abajo = true;
    if (mesaRight < scrollLeft) izquierda = true;
    if (m.x > viewRight) derecha = true;
  }

  if (!arriba && !abajo && !izquierda && !derecha) return null;

  const stepX = viewportWidth * 0.75;
  const stepY = viewportHeight * 0.75;

  return (
    <>
      {arriba && (
        <div className="planoIndicador arriba" title="Hay mesas arriba"
          onClick={() => onNavigate(scrollLeft, Math.max(0, scrollTop - stepY))}>▲</div>
      )}
      {abajo && (
        <div className="planoIndicador abajo" title="Hay mesas abajo"
          onClick={() => onNavigate(scrollLeft, scrollTop + stepY)}>▼</div>
      )}
      {izquierda && (
        <div className="planoIndicador izquierda" title="Hay mesas a la izquierda"
          onClick={() => onNavigate(Math.max(0, scrollLeft - stepX), scrollTop)}>◀</div>
      )}
      {derecha && (
        <div className="planoIndicador derecha" title="Hay mesas a la derecha"
          onClick={() => onNavigate(scrollLeft + stepX, scrollTop)}>▶</div>
      )}
    </>
  );
}

export default OffScreenIndicators;
