/* [174A-109b-fase2] Tipos mínimos de compatibilidad `@app/types` para el Mezclador.
 * No replica todo el legacy; solo expone el contrato que el feature realmente consume. */

export type NotaMusical = string;
export type EscalaMusical = string;
export type TipoSample = string;

export interface UsuarioResumen {
  id: number;
  username: string;
  nombreVisible?: string | null;
  avatarUrl?: string | null;
  verificado?: boolean;
}

export interface MetadataSample {
  [key: string]: unknown;
}

export interface SampleResumen {
  id: number;
  titulo: string;
  slug: string;
  descripcion?: string;
  bpm: number | null;
  key: NotaMusical | null;
  escala: EscalaMusical | null;
  duracion: number;
  formato?: string;
  tags: string[];
  tipo: TipoSample;
  esPremium: boolean;
  precio: number | null;
  rutaPreview: string;
  rutaWaveform: string;
  imagenUrl: string | null;
  totalDescargas: number;
  totalLikes: number;
  totalReproducciones: number;
  metadata: MetadataSample | null;
  creador: UsuarioResumen;
  liked?: boolean;
  reaccion?: string | null;
  verificado?: boolean;
  mostrarEnComunidad?: boolean;
  yaColeccionado?: boolean;
  yaGuardadoEnColeccion?: boolean;
  yaComentado?: boolean;
  esMio?: boolean;
  yaComprado?: boolean;
}
