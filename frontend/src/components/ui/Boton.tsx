/* [204A-1] Boton UI base para el frontend SPA.
 * Cumple la convención local de no usar `<button>` nativo directamente en pantallas. */

import type { ButtonHTMLAttributes, ReactNode } from 'react';

type BotonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  children: ReactNode;
};

export default function Boton({ children, className, type = 'button', ...props }: BotonProps) {
  return (
    <button
      {...props}
      className={className}
      type={type}
    >
      {children}
    </button>
  );
}
