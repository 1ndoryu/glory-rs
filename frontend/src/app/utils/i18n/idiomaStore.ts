import { create } from 'zustand';

export type Idioma = 'es' | 'en' | 'ja';

const CLAVE_STORAGE = 'kamples_idioma';

function aplicarIdiomaDocumento(idioma: Idioma) {
  if (typeof document !== 'undefined') {
    document.documentElement.lang = idioma;
  }
}

function detectarIdiomaInicial(): Idioma {
  if (typeof window === 'undefined') {
    return 'es';
  }

  const guardado = localStorage.getItem(CLAVE_STORAGE);
  if (guardado === 'es' || guardado === 'en' || guardado === 'ja') {
    aplicarIdiomaDocumento(guardado);
    return guardado;
  }

  const idiomaNavegador = navigator.language.toLowerCase();
  const idioma = idiomaNavegador.startsWith('ja')
    ? 'ja'
    : idiomaNavegador.startsWith('en')
      ? 'en'
      : 'es';

  aplicarIdiomaDocumento(idioma);
  return idioma;
}

interface IdiomaState {
  idioma: Idioma;
  setIdioma: (idioma: Idioma) => void;
}

export const useIdiomaStore = create<IdiomaState>((set) => ({
  idioma: detectarIdiomaInicial(),
  setIdioma: (idioma) => {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(CLAVE_STORAGE, idioma);
    }

    aplicarIdiomaDocumento(idioma);
    set({ idioma });
  },
}));