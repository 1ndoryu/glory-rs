import en from './en';
import es from './es';
import ja from './ja';
import { useIdiomaStore, type Idioma } from './idiomaStore';

type Libro = Record<string, string>;
type Params = Record<string, string | number>;

const libros: Record<Idioma, Libro> = {
  es,
  en,
  ja,
};

function resolver(libro: Libro, clave: string, fallback: Libro): string {
  return libro[clave] ?? fallback[clave] ?? clave;
}

function interpolar(texto: string, params: Params): string {
  return texto.replace(/\{(\w+)\}/g, (_, clave) => String(params[clave] ?? `{${clave}}`));
}

export interface UseTResult {
  t: (clave: string, params?: Params) => string;
  idioma: Idioma;
}

export function useT(): UseTResult {
  const idioma = useIdiomaStore((state) => state.idioma);

  const t = (clave: string, params?: Params): string => {
    const texto = resolver(libros[idioma], clave, libros.es);
    return params ? interpolar(texto, params) : texto;
  };

  return { t, idioma };
}

export function getT(idioma?: Idioma): (clave: string, params?: Params) => string {
  const idiomaActivo = idioma ?? (() => {
    if (typeof localStorage === 'undefined') {
      return 'es' as Idioma;
    }

    const guardado = localStorage.getItem('kamples_idioma');
    return guardado === 'es' || guardado === 'en' || guardado === 'ja' ? guardado : 'es';
  })();

  return (clave, params) => {
    const texto = resolver(libros[idiomaActivo], clave, libros.es);
    return params ? interpolar(texto, params) : texto;
  };
}