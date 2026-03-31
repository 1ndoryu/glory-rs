export interface MesaGeometryInput {
  ancho: number;
  alto: number;
  forma?: string;
}

export const MIN_LADO_MESA = 72;
export const RATIO_RECTANGULAR = 1.8;

export function normalizarDimensionesMesa<T extends MesaGeometryInput>(mesa: T): T {
  const ladoBase = Math.max(mesa.ancho, mesa.alto, MIN_LADO_MESA);

  if (mesa.forma === 'rectangular') {
    const alto = Math.max(MIN_LADO_MESA, mesa.alto);
    const ancho = Math.max(mesa.ancho, Math.round(alto * RATIO_RECTANGULAR));
    return { ...mesa, ancho, alto };
  }

  if (mesa.forma === 'redonda' || mesa.forma === 'cuadrada') {
    return { ...mesa, ancho: ladoBase, alto: ladoBase };
  }

  return mesa;
}