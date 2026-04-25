/* [174A-109b-fase2] Adaptador `@app/services/apiExplorador` sobre el catálogo Orval de Rust.
 * El backend nuevo aún no expone la estructura exacta de carpetas legacy, así que el
 * browser del DAW se alimenta agrupando `GET /api/samples` por `tipo` para recuperar
 * un explorador funcional sin seguir pegado a `wp-json/...`. */

import { listSamples } from '../../api/generated/sample-catalog/sample-catalog';
import type { SampleSummary } from '../../api/generated/model';
import type { SampleResumen } from '../types';

export interface RespuestaApi<T> {
  ok: boolean;
  data: T | null;
  error?: string;
  status: number;
}

export interface SubcarpetaInfo {
  nombre: string;
  total: number;
}

export interface CarpetaInfo {
  primaria: string;
  total: number;
  subcarpetas: SubcarpetaInfo[];
}

export interface PaginacionColeccionados {
  page: number;
  per_page: number;
  total: number;
  pages: number;
}

export interface RespuestaColeccionados {
  data: SampleResumen[];
  pagination: PaginacionColeccionados;
}

function normalizarSample(sample: SampleSummary): SampleResumen {
  return {
    id: sample.id,
    titulo: sample.titulo,
    slug: sample.slug,
    descripcion: sample.descripcion,
    bpm: sample.bpm ?? null,
    key: sample.key ?? null,
    escala: sample.escala ?? null,
    duracion: sample.duracion,
    formato: sample.formato,
    tags: sample.tags,
    tipo: sample.tipo,
    esPremium: sample.es_premium,
    precio: sample.precio ?? null,
    rutaPreview: sample.ruta_preview ?? '',
    rutaWaveform: sample.ruta_waveform ?? '',
    imagenUrl: sample.imagen_url ?? null,
    totalDescargas: sample.total_descargas,
    totalLikes: sample.total_likes,
    totalReproducciones: sample.total_reproducciones,
    metadata: (sample.metadata as Record<string, unknown>) ?? null,
    creador: {
      id: sample.creador.id,
      username: sample.creador.username,
      nombreVisible: sample.creador.nombre_visible ?? null,
      avatarUrl: sample.creador.avatar_url ?? null,
      verificado: sample.creador.verificado,
    },
    verificado: sample.verificado,
  };
}

function resolverCarpeta(sample: SampleSummary): string {
  return sample.tipo?.trim() || 'otros';
}

export const obtenerCarpetas = async (): Promise<RespuestaApi<CarpetaInfo[]>> => {
  try {
    const response = await listSamples({ page: 1, per_page: 100 });
    if (response.status !== 200) {
      return {
        ok: false,
        data: null,
        error: response.data.message,
        status: response.status,
      };
    }

    const folders = new Map<string, { total: number; creators: Map<string, number> }>();

    for (const sample of response.data.data) {
      const primary = resolverCarpeta(sample);
      const creatorName = sample.creador.nombre_visible ?? sample.creador.username;
      const entry = folders.get(primary) ?? { total: 0, creators: new Map<string, number>() };
      entry.total += 1;
      entry.creators.set(creatorName, (entry.creators.get(creatorName) ?? 0) + 1);
      folders.set(primary, entry);
    }

    const data = Array.from(folders.entries())
      .map(([primary, entry]) => ({
        primaria: primary,
        total: entry.total,
        subcarpetas: Array.from(entry.creators.entries()).map(([name, total]) => ({
          nombre: name,
          total,
        })),
      }))
      .sort((left, right) => left.primaria.localeCompare(right.primaria));

    return { ok: true, data, status: 200 };
  } catch (error) {
    return {
      ok: false,
      data: null,
      error: error instanceof Error ? error.message : 'Error consultando catálogo público',
      status: 500,
    };
  }
};

export const obtenerColeccionados = async (
  page = 1,
  perPage = 100,
  carpeta = '',
  _orden = 'recientes',
  busqueda = '',
  _soloEncanta = false,
  _soloLike = false,
): Promise<RespuestaApi<RespuestaColeccionados>> => {
  try {
    const response = await listSamples({
      page,
      per_page: perPage,
      search: busqueda || undefined,
      type: carpeta || undefined,
    });

    if (response.status !== 200) {
      return {
        ok: false,
        data: null,
        error: response.data.message,
        status: response.status,
      };
    }

    const normalizedSamples = response.data.data
      .filter((sample) => (carpeta ? resolverCarpeta(sample) === carpeta : true))
      .map(normalizarSample);

    return {
      ok: true,
      data: {
        data: normalizedSamples,
        pagination: response.data.pagination,
      },
      status: 200,
    };
  } catch (error) {
    return {
      ok: false,
      data: null,
      error: error instanceof Error ? error.message : 'Error consultando samples del browser',
      status: 500,
    };
  }
};
