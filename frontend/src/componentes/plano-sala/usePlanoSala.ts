/* [263A-14] Hook con lógica de negocio del plano de sala.
 * [263A-28] Reemplazados prompt/confirm/alert nativos por estado React + toast.
 * Los diálogos se renderizan en PlanoSala.tsx usando shadcn Dialog. */

import { useState, useCallback, useRef, type RefObject } from 'react';
import type { DragEndEvent, DragStartEvent } from '@dnd-kit/core';
import { toast } from 'sonner';
import {
  useObtenerPlano,
  ActualizarMesaRequest,
  ActualizarPosicionesRequest,
  CrearCombinacionRequest,
  CrearMesaRequest,
  CrearZonaRequest,
  ActualizarZonaRequest,
  PlanoExport,
  Mesa,
  crearZona,
  eliminarZona,
  actualizarZona as actualizarZonaApi,
  crearMesa,
  actualizarMesa as actualizarMesaApi,
  eliminarMesa as eliminarMesaApi,
  actualizarPosiciones,
  crearCombinacion,
  eliminarCombinacion,
  importarPlano,
} from '../../api/generated';

/* Tipos de diálogo para reemplazar prompt/confirm nativos */
export interface DialogoEntrada {
  titulo: string;
  label: string;
  valorInicial: string;
  tipo?: 'text' | 'number';
  onConfirmar: (valor: string) => void;
}
export interface DialogoConfirmar {
  titulo: string;
  mensaje: string;
  onConfirmar: () => void;
}
export interface DialogoCombinacion {
  onConfirmar: (nombre: string, maxPersonas: number, mesaIds: string[]) => void;
}

/* [283A-25] Recibe canvasRef para que los clamps de drag usen el ancho real
 * del DOM. Zoom vive aquí para centralizar lógica de escalado. */
export function usePlanoSala(
  canvasRef: RefObject<HTMLDivElement | null>,
) {
  const { data, refetch } = useObtenerPlano();
  const plano = data?.status === 200 ? data.data : null;

  const [zonaActiva, setZonaActiva] = useState<string | null>(null);
  const [mesaSeleccionada, setMesaSeleccionada] = useState<Mesa | null>(null);
  const [arrastrando, setArrastrando] = useState<string | null>(null);
  const [posicionesLocales, setPosicionesLocales] = useState<
    Record<string, { x: number; y: number }>
  >({});
  const [zoom, setZoom] = useState(1);

  /* Estado de diálogos — reemplazan prompt/confirm/alert nativos */
  const [dialogoEntrada, setDialogoEntrada] = useState<DialogoEntrada | null>(null);
  const [dialogoConfirmar, setDialogoConfirmar] = useState<DialogoConfirmar | null>(null);
  const [dialogoCombinacion, setDialogoCombinacion] = useState<DialogoCombinacion | null>(null);
  /* Ref para archivo importado pendiente de confirmación */
  const archivoImportRef = useRef<PlanoExport | null>(null);

  const zonaData = plano?.zonas.find((z) => z.id === zonaActiva);
  const mesasZona = zonaData?.mesas ?? [];

  if (plano && plano.zonas.length > 0 && !zonaActiva) {
    setZonaActiva(plano.zonas[0].id);
  }

  const cambiarZona = (id: string) => {
    setZonaActiva(id);
    setMesaSeleccionada(null);
    setPosicionesLocales({});
  };

  const handleCrearZona = () => {
    setDialogoEntrada({
      titulo: 'Nueva zona', label: 'Nombre de la zona', valorInicial: '', tipo: 'text',
      onConfirmar: async (nombre) => {
        await crearZona({ nombre } as CrearZonaRequest);
        refetch();
      },
    });
  };

  const handleEliminarZona = () => {
    if (!zonaActiva) return;
    setDialogoConfirmar({
      titulo: 'Eliminar zona',
      mensaje: '¿Eliminar esta zona y todas sus mesas?',
      onConfirmar: async () => {
        await eliminarZona(zonaActiva);
        setZonaActiva(null);
        setMesaSeleccionada(null);
        refetch();
      },
    });
  };

  const handleEditarZona = () => {
    if (!zonaActiva || !zonaData) return;
    setDialogoEntrada({
      titulo: 'Renombrar zona', label: 'Nuevo nombre', valorInicial: zonaData.nombre, tipo: 'text',
      onConfirmar: async (nombre) => {
        await actualizarZonaApi(zonaActiva, { nombre } as ActualizarZonaRequest);
        refetch();
      },
    });
  };

  /* [283A-17] Acepta posición opcional para drag-to-create. Try-catch para
   * mostrar toast si el backend retorna Conflict (número duplicado). */
  const handleCrearMesa = (pos?: { x: number; y: number }) => {
    if (!zonaActiva) return;
    setDialogoEntrada({
      titulo: 'Nueva mesa', label: 'Número de mesa', valorInicial: '', tipo: 'number',
      onConfirmar: async (val) => {
        const numero = Number(val);
        if (Number.isNaN(numero) || numero < 1) return;
        try {
          await crearMesa({ zona_id: zonaActiva, numero, pos_x: pos?.x ?? 50, pos_y: pos?.y ?? 50 } as CrearMesaRequest);
          refetch();
        } catch (err: unknown) {
          const axiosErr = err as { response?: { data?: { message?: string } } };
          toast.error(axiosErr?.response?.data?.message || 'Error al crear mesa');
        }
      },
    });
  };

  const handleGuardarMesa = async (id: string, req: ActualizarMesaRequest) => {
    await actualizarMesaApi(id, req);
    setMesaSeleccionada(null);
    refetch();
  };

  const handleEliminarMesa = (id: string) => {
    setDialogoConfirmar({
      titulo: 'Eliminar mesa',
      mensaje: '¿Eliminar esta mesa?',
      onConfirmar: async () => {
        await eliminarMesaApi(id);
        setMesaSeleccionada(null);
        refetch();
      },
    });
  };

  const handleDragStart = (event: DragStartEvent) => setArrastrando(String(event.active.id));

  /* [283A-25] Clamp usa el ancho real del canvas (DOM) dividido por zoom en vez
   * de zonaData.ancho para que las mesas ocupen todo el ancho visible.
   * Deltas divididos por zoom para mantener coordenadas canónicas. */
  const handleDragEnd = useCallback(async (event: DragEndEvent) => {
    setArrastrando(null);
    const mesaId = String(event.active.id);
    const mesa = mesasZona.find((m) => m.id === mesaId);
    if (!mesa) return;
    const prev = posicionesLocales[mesaId];
    const canvasWidth = canvasRef.current?.clientWidth ?? 800;
    const maxX = canvasWidth / zoom - mesa.ancho;
    const maxY = (zonaData?.alto ?? 600) - mesa.alto;
    const dx = event.delta.x / zoom;
    const dy = event.delta.y / zoom;
    const nuevoX = Math.min(maxX, Math.max(0, (prev?.x ?? mesa.pos_x) + dx));
    const nuevoY = Math.min(maxY, Math.max(0, (prev?.y ?? mesa.pos_y) + dy));
    setPosicionesLocales((p) => ({ ...p, [mesaId]: { x: nuevoX, y: nuevoY } }));
    const req: ActualizarPosicionesRequest = {
      posiciones: [{ id: mesaId, pos_x: Math.round(nuevoX), pos_y: Math.round(nuevoY) }],
    };
    await actualizarPosiciones(req);
  }, [mesasZona, posicionesLocales, zonaData, canvasRef, zoom]);

  const handleExportar = async () => {
    try {
      const resp = await fetch('/api/plano-sala/export', {
        headers: { Authorization: `Bearer ${localStorage.getItem('token') ?? ''}` },
      });
      const json = await resp.json();
      const blob = new Blob([JSON.stringify(json, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'plano-sala.json';
      a.click();
      URL.revokeObjectURL(url);
    } catch { toast.error('Error al exportar'); }
  };

  const handleImportar = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      try {
        const text = await file.text();
        const d: PlanoExport = JSON.parse(text);
        archivoImportRef.current = d;
        setDialogoConfirmar({
          titulo: 'Importar plano',
          mensaje: 'Esto reemplazará todo el plano actual. ¿Continuar?',
          onConfirmar: async () => {
            if (!archivoImportRef.current) return;
            await importarPlano(archivoImportRef.current);
            archivoImportRef.current = null;
            setZonaActiva(null);
            setMesaSeleccionada(null);
            refetch();
          },
        });
      } catch { toast.error('Error al importar: archivo inválido'); }
    };
    input.click();
  };

  /* [283A-17] Ahora recibe mesa_ids desde el diálogo (selección visual de mesas). */
  const handleCrearCombinacion = () => {
    setDialogoCombinacion({
      onConfirmar: async (nombre, maxPersonas, mesaIds) => {
        try {
          await crearCombinacion({ nombre, max_personas: maxPersonas, mesa_ids: mesaIds } as CrearCombinacionRequest);
          refetch();
        } catch {
          toast.error('Error al crear combinación');
        }
      },
    });
  };

  const handleEliminarCombinacion = (id: string) => {
    setDialogoConfirmar({
      titulo: 'Eliminar combinación',
      mensaje: '¿Eliminar esta combinación?',
      onConfirmar: async () => {
        await eliminarCombinacion(id);
        refetch();
      },
    });
  };

  return {
    plano, zonaActiva, zonaData, mesasZona, mesaSeleccionada, arrastrando,
    posicionesLocales, setMesaSeleccionada, cambiarZona, zoom, setZoom,
    handleCrearZona, handleEliminarZona, handleEditarZona,
    handleCrearMesa, handleGuardarMesa, handleEliminarMesa,
    handleDragStart, handleDragEnd,
    handleExportar, handleImportar,
    handleCrearCombinacion, handleEliminarCombinacion,
    dialogoEntrada, setDialogoEntrada,
    dialogoConfirmar, setDialogoConfirmar,
    dialogoCombinacion, setDialogoCombinacion,
  };
}
