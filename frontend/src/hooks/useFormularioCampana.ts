/* [263A-23] Hook para formulario de creación de campañas.
 * Maneja estado del form, contador SMS, preview de segmento y envío. */

import { useState, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearCampana, usePreviewSegmento } from '../api/generated';

const SEGMENTOS_VALIDOS = [
  'habitual', 'sin_1m', 'sin_3m', 'sin_6m', 'sin_9m', 'sin_1a', 'sin_mas_1a', 'todos',
] as const;

const CANALES_VALIDOS = ['sms', 'email', 'whatsapp'] as const;

/* Caracteres GSM especiales que ocupan 2 posiciones en SMS */
const CHARS_DOBLES = new Set('|^€{}[]~\\');

function contarCaracteresSms(texto: string): number {
  let count = 0;
  for (const ch of texto) {
    count += CHARS_DOBLES.has(ch) ? 2 : 1;
  }
  return count;
}

export function useFormularioCampana() {
  const navigate = useNavigate();
  const [nombre, setNombre] = useState('');
  const [descripcionInterna, setDescripcionInterna] = useState('');
  const [canales, setCanales] = useState<Set<string>>(new Set());
  const [segmento, setSegmento] = useState('');
  const [cuerpoMensaje, setCuerpoMensaje] = useState('');
  const [incluirBaja, setIncluirBaja] = useState(false);
  const [telefonoBaja, setTelefonoBaja] = useState('');
  const [enviando, setEnviando] = useState(false);

  const crearMutation = useCrearCampana();

  /* Preview de segmento — solo se consulta si hay segmento válido */
  const previewHabilitado = SEGMENTOS_VALIDOS.includes(segmento as typeof SEGMENTOS_VALIDOS[number]);
  const { data: previewData, isLoading: previewLoading } = usePreviewSegmento(
    { segmento },
    { query: { enabled: previewHabilitado } },
  );
  const preview = previewData?.data ?? null;

  /* Contador SMS: max 160, si incluir_baja reservar 17 chars para "\nNo: XXXXXXX" */
  const tieneSms = canales.has('sms');
  const maxSms = incluirBaja ? 143 : 160;
  const caracteresSms = useMemo(() => contarCaracteresSms(cuerpoMensaje), [cuerpoMensaje]);

  const toggleCanal = (canal: string) => {
    setCanales(prev => {
      const next = new Set(prev);
      if (next.has(canal)) next.delete(canal);
      else next.add(canal);
      return next;
    });
  };

  const formValido =
    nombre.trim().length > 0 &&
    canales.size > 0 &&
    (!tieneSms || caracteresSms <= maxSms);

  const crearCampana = async () => {
    if (!formValido || enviando) return;
    setEnviando(true);
    try {
      await crearMutation.mutateAsync({
        data: {
          nombre: nombre.trim(),
          descripcion_interna: descripcionInterna.trim() || null,
          canales: [...canales],
          segmento: segmento || null,
          cuerpo_mensaje: cuerpoMensaje.trim() || null,
          incluir_baja: incluirBaja || null,
          telefono_baja: incluirBaja ? telefonoBaja.trim() || null : null,
        },
      });
      navigate('/marketing/campanas');
    } finally {
      setEnviando(false);
    }
  };

  return {
    nombre, setNombre,
    descripcionInterna, setDescripcionInterna,
    canales, toggleCanal,
    segmento, setSegmento,
    cuerpoMensaje, setCuerpoMensaje,
    incluirBaja, setIncluirBaja,
    telefonoBaja, setTelefonoBaja,
    tieneSms, maxSms, caracteresSms,
    preview, previewLoading, previewHabilitado,
    formValido, enviando,
    crearCampana,
    cancelar: () => navigate('/marketing/campanas'),
    SEGMENTOS_VALIDOS,
    CANALES_VALIDOS,
  };
}
