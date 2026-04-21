/* [174A-109b-fase2] Store de compatibilidad para `@app/stores/crearModalStore`.
 * El Mezclador solo necesita abrir el modal con un archivo pre-cargado; la UI final
 * del modal vive fuera de esta fase y consumirá este estado cuando se conecte. */

import { create } from 'zustand';

export type LadoRelacion = 'fuente' | 'destino';

export interface LadoOpcion {
  cancionId: number;
  titulo: string;
  artista?: string;
}

export interface ContextoAdjuntar {
  cancionOrigenId?: number;
  relacionId?: number;
  ladoRelacion?: LadoRelacion;
  ladoFuente?: LadoOpcion;
  ladoDestino?: LadoOpcion;
  inicioSegundos?: number;
}

interface EstadoCrearModal {
  abierto: boolean;
  archivoPreCargado: File | null;
  esMezcla: boolean;
  contextoAdjuntar: ContextoAdjuntar | null;
  abrir: (archivo?: File, esMezcla?: boolean) => void;
  abrirConContexto: (contexto: ContextoAdjuntar) => void;
  seleccionarLado: (cancionOrigenId: number, lado: LadoRelacion) => void;
  cerrar: () => void;
  consumirArchivo: () => File | null;
}

export const useCrearModalStore = create<EstadoCrearModal>((set, get) => ({
  abierto: false,
  archivoPreCargado: null,
  esMezcla: false,
  contextoAdjuntar: null,
  abrir: (archivo, esMezcla) => set({
    abierto: true,
    archivoPreCargado: archivo ?? null,
    esMezcla: esMezcla ?? false,
    contextoAdjuntar: null,
  }),
  abrirConContexto: (contexto) => set({
    abierto: true,
    archivoPreCargado: null,
    esMezcla: false,
    contextoAdjuntar: contexto,
  }),
  seleccionarLado: (cancionOrigenId, lado) => set((state) => ({
    contextoAdjuntar: state.contextoAdjuntar
      ? { ...state.contextoAdjuntar, cancionOrigenId, ladoRelacion: lado }
      : null,
  })),
  cerrar: () => set({ abierto: false, archivoPreCargado: null, esMezcla: false, contextoAdjuntar: null }),
  consumirArchivo: () => {
    const archivo = get().archivoPreCargado;
    if (archivo) {
      set({ archivoPreCargado: null });
    }
    return archivo;
  },
}));
