/* [174A-109b-fase2] Compat wrapper para `@app/components/ui/BotonBase`. */

import type { ButtonHTMLAttributes, ReactNode } from 'react';
import '../../styles/legacyUi.css';

type VarianteBoton = 'primario' | 'secundario' | 'ghost' | 'peligro';
type TamanoBoton = 'sm' | 'md' | 'ninguno';

interface BotonBaseProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variante?: VarianteBoton;
  tamano?: TamanoBoton;
  cargando?: boolean;
  soloIcono?: boolean;
  anchoCompleto?: boolean;
  children?: ReactNode;
}

const mapaVariante: Record<VarianteBoton, string> = {
  primario: 'variantePrimario',
  secundario: 'varianteSecundario',
  ghost: 'varianteGhost',
  peligro: 'variantePeligro',
};

const mapaTamano: Record<TamanoBoton, string> = {
  sm: 'tamanoSm',
  md: 'tamanoMd',
  ninguno: '',
};

export const BotonBase = ({
  variante = 'primario',
  tamano = 'md',
  cargando = false,
  soloIcono = false,
  anchoCompleto = false,
  children,
  className = '',
  disabled,
  ...props
}: BotonBaseProps): JSX.Element => {
  const classes = [
    'botonBase',
    mapaVariante[variante],
    mapaTamano[tamano],
    soloIcono ? 'soloIcono' : '',
    anchoCompleto ? 'anchoCompleto' : '',
    cargando ? 'cargando' : '',
    className,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <button
      {...props}
      className={classes}
      disabled={disabled || cargando}
    >
      {children}
    </button>
  );
};

export default BotonBase;
