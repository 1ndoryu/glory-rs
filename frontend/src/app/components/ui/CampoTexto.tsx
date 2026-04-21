/* [174A-109b-fase2] Compat wrapper para `@app/components/ui/CampoTexto`. */

import { forwardRef, type InputHTMLAttributes, type TextareaHTMLAttributes } from 'react';
import '../../styles/legacyUi.css';

type VarianteCampo = 'minimal' | 'bordado' | 'desnudo';

interface CampoTextoBaseProps {
  etiqueta?: string;
  error?: string;
  className?: string;
  variante?: VarianteCampo;
}

type CampoInputProps = CampoTextoBaseProps & InputHTMLAttributes<HTMLInputElement> & { multilínea?: false };
type CampoAreaProps = CampoTextoBaseProps & TextareaHTMLAttributes<HTMLTextAreaElement> & { multilínea: true };
type CampoTextoProps = CampoInputProps | CampoAreaProps;

export const CampoTexto = forwardRef<HTMLInputElement | HTMLTextAreaElement, CampoTextoProps>(
  function CampoTexto(props, ref) {
    const { etiqueta, error, className = '', multilínea, variante = 'minimal', ...rest } = props;
    const errorClass = error ? 'inputError' : '';
    const naked = variante === 'desnudo';

    return (
      <div className={`contenedorCampoTexto ${className}`.trim()}>
        {etiqueta && <label className="etiquetaCampoTexto">{etiqueta}</label>}
        {multilínea ? (
          <textarea
            {...(rest as TextareaHTMLAttributes<HTMLTextAreaElement>)}
            ref={ref as React.Ref<HTMLTextAreaElement>}
            className={naked ? errorClass : `campoTextoArea ${errorClass}`.trim()}
          />
        ) : (
          <input
            {...(rest as InputHTMLAttributes<HTMLInputElement>)}
            ref={ref as React.Ref<HTMLInputElement>}
            className={naked ? errorClass : `campTextoInput ${errorClass}`.trim()}
          />
        )}
        {error && <span className="errorCampoTexto">{error}</span>}
      </div>
    );
  },
);

export default CampoTexto;
