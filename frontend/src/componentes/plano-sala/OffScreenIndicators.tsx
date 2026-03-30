/* [303A-12] Indicadores de mesas fuera de vista — flechas en los bordes del canvas
 * cuando hay mesas más allá del área visible del viewport. */

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
}

function OffScreenIndicators({
  mesas,
  viewportWidth,
  viewportHeight,
  scrollLeft,
  scrollTop,
}: OffScreenIndicatorsProps) {
  let arriba = false;
  let abajo = false;
  let izquierda = false;
  let derecha = false;

  for (const m of mesas) {
    const mesaRight = m.x + m.ancho;
    const mesaBottom = m.y + m.alto;
    const viewRight = scrollLeft + viewportWidth;
    const viewBottom = scrollTop + viewportHeight;

    if (mesaBottom < scrollTop) arriba = true;
    if (m.y > viewBottom) abajo = true;
    if (mesaRight < scrollLeft) izquierda = true;
    if (m.x > viewRight) derecha = true;
  }

  if (!arriba && !abajo && !izquierda && !derecha) return null;

  return (
    <>
      {arriba && <div className="planoIndicador arriba" title="Hay mesas arriba">▲</div>}
      {abajo && <div className="planoIndicador abajo" title="Hay mesas abajo">▼</div>}
      {izquierda && <div className="planoIndicador izquierda" title="Hay mesas a la izquierda">◀</div>}
      {derecha && <div className="planoIndicador derecha" title="Hay mesas a la derecha">▶</div>}
    </>
  );
}

export default OffScreenIndicators;
