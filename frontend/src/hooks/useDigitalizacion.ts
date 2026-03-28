/* [283A-8] Hook para digitalización de documentos vía Groq IA.
 * Envía imagen base64 al backend, recibe datos extraídos para pre-rellenar formulario. */

import { useState, useCallback } from 'react';
import instance from '../api/axios-instance';

export interface DatosDocumentoExtraidos {
  fecha: string | null;
  proveedor: string | null;
  numero_documento: string | null;
  tipo_documento: string | null;
  importe_base: string | null;
  importe_iva: string | null;
  importe_total: string | null;
  confianza: number;
  notas: string | null;
}

interface EstadoDigitalizacion {
  cargando: boolean;
  error: string;
  datos: DatosDocumentoExtraidos | null;
}

export function useDigitalizacion() {
  const [estado, setEstado] = useState<EstadoDigitalizacion>({
    cargando: false,
    error: '',
    datos: null,
  });

  const digitalizar = useCallback(async (archivo: File) => {
    setEstado({ cargando: true, error: '', datos: null });

    /* Validar tipo de archivo */
    const tiposValidos = ['image/jpeg', 'image/png', 'image/webp', 'image/gif'];
    if (!tiposValidos.includes(archivo.type)) {
      setEstado({ cargando: false, error: 'Tipo de archivo no soportado. Usa JPEG, PNG, WebP o GIF.', datos: null });
      return null;
    }

    /* Validar tamaño (máx 4MB para base64 compatible con Groq) */
    if (archivo.size > 4 * 1024 * 1024) {
      setEstado({ cargando: false, error: 'La imagen es demasiado grande (máximo 4MB).', datos: null });
      return null;
    }

    try {
      /* Convertir a base64 */
      const base64 = await archivoABase64(archivo);

      const response = await instance.post<DatosDocumentoExtraidos>('/api/gastos/digitalizar', {
        imagen_base64: base64,
        mime_type: archivo.type,
      });

      setEstado({ cargando: false, error: '', datos: response.data });
      return response.data;
    } catch (err: unknown) {
      const mensaje = extraerMensajeError(err);
      setEstado({ cargando: false, error: mensaje, datos: null });
      return null;
    }
  }, []);

  const limpiar = useCallback(() => {
    setEstado({ cargando: false, error: '', datos: null });
  }, []);

  return { ...estado, digitalizar, limpiar };
}

function archivoABase64(archivo: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const result = reader.result as string;
      /* Extraer solo el contenido base64 (quitar el prefijo data:..;base64,) */
      const base64 = result.split(',')[1];
      if (base64) resolve(base64);
      else reject(new Error('No se pudo convertir la imagen a base64'));
    };
    reader.onerror = () => reject(reader.error);
    reader.readAsDataURL(archivo);
  });
}

function extraerMensajeError(err: unknown): string {
  if (err && typeof err === 'object' && 'response' in err) {
    const resp = (err as { response?: { data?: { message?: string } } }).response;
    if (resp?.data?.message) return resp.data.message;
  }
  return 'Error al digitalizar el documento. Verifica tu conexión e intenta de nuevo.';
}
