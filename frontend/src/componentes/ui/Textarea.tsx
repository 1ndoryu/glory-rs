/* 253A-10: Componente atómico Textarea — reemplaza <textarea> nativo (regla html-nativo-en-vez-de-componente) */

import { TextareaHTMLAttributes } from 'react';

function Textarea(props: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <textarea {...props} />;
}

export default Textarea;
