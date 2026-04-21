/* [174A-109b-fase2] Compat wrapper para `@app/components/ui/Input`. */

import { forwardRef, type InputHTMLAttributes } from 'react';
import '../../styles/legacyUi.css';

export interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  error?: string;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(function Input(
  { className = '', error, ...props },
  ref,
) {
  const errorClass = error ? 'inputError' : '';
  return <input {...props} ref={ref} className={`campTextoInput ${errorClass} ${className}`.trim()} />;
});

export default Input;
