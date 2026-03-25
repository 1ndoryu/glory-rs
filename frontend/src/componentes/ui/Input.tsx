/* 253A-10: Componente atómico Input — reemplaza <input> nativo (regla html-nativo-en-vez-de-componente) */

import { InputHTMLAttributes } from 'react';

function Input(props: InputHTMLAttributes<HTMLInputElement>) {
  return <input {...props} />;
}

export default Input;
