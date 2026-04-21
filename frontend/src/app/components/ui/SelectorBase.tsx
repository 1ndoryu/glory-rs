/* [174A-109b-fase2] Compat wrapper para `@app/components/ui/SelectorBase`. */

import { forwardRef, type ReactNode, type SelectHTMLAttributes } from 'react';
import '../../styles/legacyUi.css';

interface SelectorBaseProps extends SelectHTMLAttributes<HTMLSelectElement> {
  etiqueta?: string;
  error?: string;
  children: ReactNode;
}

export const SelectorBase = forwardRef<HTMLSelectElement, SelectorBaseProps>(function SelectorBase(
  { etiqueta, error, className = '', children, ...props },
  ref,
) {
  const errorClass = error ? 'inputError' : '';
  return (
    <div className={`contenedorCampoTexto ${className}`.trim()}>
      {etiqueta && <label className="etiquetaCampoTexto">{etiqueta}</label>}
      <select {...props} ref={ref} className={`campTextoInput ${errorClass}`.trim()}>
        {children}
      </select>
      {error && <span className="errorCampoTexto">{error}</span>}
    </div>
  );
});

export default SelectorBase;
