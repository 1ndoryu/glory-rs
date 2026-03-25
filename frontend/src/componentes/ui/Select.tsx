/* 253A-10: Componente atómico Select — reemplaza <select> nativo (regla html-nativo-en-vez-de-componente) */

import { SelectHTMLAttributes } from 'react';

function Select(props: SelectHTMLAttributes<HTMLSelectElement>) {
  return <select {...props} />;
}

export default Select;
