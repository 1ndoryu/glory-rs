/* [204A-1] Input UI base para el frontend SPA.
 * Centraliza el uso de `<input>` para respetar la regla local de UI atómica. */

import { forwardRef, type InputHTMLAttributes } from 'react';

export type InputProps = InputHTMLAttributes<HTMLInputElement>;

const Input = forwardRef<HTMLInputElement, InputProps>(function Input(props, ref) {
  return <input {...props} ref={ref} />;
});

export default Input;
