/* 253A-10: Componente atómico Boton — reemplaza <button> nativo (regla html-nativo-en-vez-de-componente) */

import { ButtonHTMLAttributes } from 'react';

function Boton(props: ButtonHTMLAttributes<HTMLButtonElement>) {
  return <button {...props} />;
}

export default Boton;
