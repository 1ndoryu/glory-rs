/* [263A-24] Hook para formulario de creación de plantillas WhatsApp.
 * Maneja estado del form, validación y envío. */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearPlantilla } from '../api/generated';

const CATEGORIAS = ['MARKETING', 'UTILITY', 'AUTHENTICATION'] as const;
const IDIOMAS = [
  { value: 'es', label: 'Español' },
  { value: 'en', label: 'Inglés' },
  { value: 'ca', label: 'Catalán' },
] as const;

export function useFormularioPlantilla() {
  const navigate = useNavigate();
  const [nombre, setNombre] = useState('');
  const [categoria, setCategoria] = useState<string>('MARKETING');
  const [idioma, setIdioma] = useState('es');
  const [cuerpoMensaje, setCuerpoMensaje] = useState('');
  const [cabeceraTexto, setCabeceraTexto] = useState('');
  const [pieTexto, setPieTexto] = useState('');
  const [cabeceraMediaUrl, setCabeceraMediaUrl] = useState('');
  const [cabeceraMediaTipo, setCabeceraMediaTipo] = useState('');
  const [enviando, setEnviando] = useState(false);

  const crearMutation = useCrearPlantilla();

  const formValido = nombre.trim().length > 0 && cuerpoMensaje.trim().length > 0;

  const crearPlantilla = async () => {
    if (!formValido || enviando) return;
    setEnviando(true);
    try {
      await crearMutation.mutateAsync({
        data: {
          nombre: nombre.trim(),
          categoria: categoria || undefined,
          idioma: idioma || undefined,
          cuerpo_mensaje: cuerpoMensaje.trim() || undefined,
          cabecera_texto: cabeceraTexto.trim() || undefined,
          pie_texto: pieTexto.trim() || undefined,
          cabecera_media_url: cabeceraMediaUrl.trim() || undefined,
          cabecera_media_tipo: cabeceraMediaTipo.trim() || undefined,
        },
      });
      navigate('/marketing/plantillas');
    } finally {
      setEnviando(false);
    }
  };

  return {
    nombre, setNombre,
    categoria, setCategoria,
    idioma, setIdioma,
    cuerpoMensaje, setCuerpoMensaje,
    cabeceraTexto, setCabeceraTexto,
    pieTexto, setPieTexto,
    cabeceraMediaUrl, setCabeceraMediaUrl,
    cabeceraMediaTipo, setCabeceraMediaTipo,
    formValido, enviando,
    crearPlantilla,
    cancelar: () => navigate('/marketing/plantillas'),
    CATEGORIAS,
    IDIOMAS,
  };
}
