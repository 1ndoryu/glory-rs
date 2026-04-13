/* [263A-14] Hook con lógica de negocio del plano de sala.
 * [263A-28] Reemplazados prompt/confirm/alert nativos por estado React + toast.
 * [014A-13] Tras cada mutación se invalidate también /api/plano-sala/ocupacion
 * para que PlanoOcupacion (reservas) refleje cambios en tiempo real.
 * Los diálogos se renderizan en PlanoSala.tsx usando shadcn Dialog. */

import { useState, useCallback, useRef, type RefObject } from 'react';
import type { DragEndEvent, DragStartEvent } from '@dnd-kit/core';
import { useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import axios from '@/api/axios-instance';
import { useZoomStore } from '../../stores/zoomStore';
import type { CanvasTool } from './CanvasToolbar';
import {
  useObtenerPlano,
  getObtenerOcupacionQueryKey,
  ActualizarMesaRequest,
  ActualizarPosicionesRequest,
  CrearCombinacionRequest,
  CrearMesaRequest,
  CrearZonaRequest,
  ActualizarZonaRequest,
  PlanoExport,
  Mesa,
  ParedSala,
  CrearParedRequest,
  ActualizarParedRequest,
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
  crearPared,
  eliminarPared as eliminarParedApi,
  actualizarPared as actualizarParedApi,
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
  const queryClient = useQueryClient();
  const { data, refetch } = useObtenerPlano();
  const plano = data?.status === 200 ? data.data : null;

  /* [014A-13] Invalida tanto /api/plano-sala como /api/plano-sala/ocupacion
   * para que PlanoOcupacion (reservas) refleje cambios de mesas al instante. */
  const refetchPlano = useCallback(async () => {
    await refetch();
    queryClient.invalidateQueries({ queryKey: getObtenerOcupacionQueryKey() });
  }, [refetch, queryClient]);

  const [zonaActiva, setZonaActiva] = useState<string | null>(null);
  const [mesaSeleccionada, setMesaSeleccionada] = useState<Mesa | null>(null);
  const [paredSeleccionada, setParedSeleccionada] = useState<ParedSala | null>(null);
  const [arrastrando, setArrastrando] = useState<string | null>(null);
  const [posicionesLocales, setPosicionesLocales] = useState<
    Record<string, { x: number; y: number }>
  >({});
  const [dimensionesLocales, setDimensionesLocales] = useState<
    Record<string, { ancho: number; alto: number }>
  >({});
  const zoom = useZoomStore(s => s.zoom);
  const setZoom = useZoomStore(s => s.setZoom);
  const canvasHeight = useZoomStore(s => s.canvasHeight);

  /* [134A-15] Sistema de herramientas tipo Illustrator */
  const [activeTool, setActiveToolRaw] = useState<CanvasTool>('select');
  /* Estado para dibujar paredes arrastrando de punto A a punto B */
  const [wallDrawStart, setWallDrawStart] = useState<{ x: number; y: number } | null>(null);
  const [wallDrawPreview, setWallDrawPreview] = useState<{
    x: number; y: number; w: number; rotation: number;
  } | null>(null);

  const setActiveTool = useCallback((tool: CanvasTool) => {
    setActiveToolRaw(tool);
    setMesaSeleccionada(null);
    setParedSeleccionada(null);
    setWallDrawStart(null);
    setWallDrawPreview(null);
  }, []);

  /* Estado de diálogos — reemplazan prompt/confirm/alert nativos */
  const [dialogoEntrada, setDialogoEntrada] = useState<DialogoEntrada | null>(null);
  const [dialogoConfirmar, setDialogoConfirmar] = useState<DialogoConfirmar | null>(null);
  const [dialogoCombinacion, setDialogoCombinacion] = useState<DialogoCombinacion | null>(null);
  /* Ref para archivo importado pendiente de confirmación */
  const archivoImportRef = useRef<PlanoExport | null>(null);

  const zonaData = plano?.zonas.find((z) => z.id === zonaActiva);
  const mesasZona = zonaData?.mesas ?? [];
  const paredesZona: ParedSala[] = zonaData?.paredes ?? [];

  if (plano && plano.zonas.length > 0 && !zonaActiva) {
    setZonaActiva(plano.zonas[0].id);
  }

  const cambiarZona = (id: string) => {
    setZonaActiva(id);
    setMesaSeleccionada(null);
    setPosicionesLocales({});
    setDimensionesLocales({});
  };

  const handleCrearZona = () => {
    setDialogoEntrada({
      titulo: 'Nueva zona', label: 'Nombre de la zona', valorInicial: '', tipo: 'text',
      onConfirmar: async (nombre) => {
        await crearZona({ nombre } as CrearZonaRequest);
        refetchPlano();
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
        refetchPlano();
      },
    });
  };

  const handleEditarZona = () => {
    if (!zonaActiva || !zonaData) return;
    setDialogoEntrada({
      titulo: 'Renombrar zona', label: 'Nuevo nombre', valorInicial: zonaData.nombre, tipo: 'text',
      onConfirmar: async (nombre) => {
        await actualizarZonaApi(zonaActiva, { nombre } as ActualizarZonaRequest);
        refetchPlano();
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
          refetchPlano();
        } catch (err: unknown) {
          const axiosErr = err as { response?: { data?: { message?: string } } };
          toast.error(axiosErr?.response?.data?.message || 'Error al crear mesa');
        }
      },
    });
  };

  /* [134A-15+134A-18] Creación rápida de mesa via herramienta del toolbar.
   * Auto-genera el número (max actual + 1). Sin diálogo. */
  const handleCrearMesaRapida = async (pos: { x: number; y: number }, forma: string) => {
    if (!zonaActiva) return;
    const nextNum = mesasZona.length > 0
      ? Math.max(...mesasZona.map(m => m.numero)) + 1
      : 1;
    try {
      await crearMesa({
        zona_id: zonaActiva, numero: nextNum, forma,
        ancho: forma === 'rectangular' ? 144 : 80,
        alto: 80,
        pos_x: pos.x, pos_y: pos.y,
      } as CrearMesaRequest);
      refetchPlano();
      toast.success(`Mesa ${nextNum} creada`);
    } catch (err: unknown) {
      const axiosErr = err as { response?: { data?: { message?: string } } };
      toast.error(axiosErr?.response?.data?.message || 'Error al crear mesa');
    }
  };

  /* [134A-15] Eliminar directamente (herramienta borrar) sin diálogo de confirmación */
  const handleEliminarMesaDirecta = async (id: string) => {
    try {
      await eliminarMesaApi(id);
      setMesaSeleccionada(null);
      refetchPlano();
    } catch { toast.error('Error al eliminar mesa'); }
  };

  const handleEliminarParedDirecta = async (id: string) => {
    try {
      await eliminarParedApi(id);
      setParedSeleccionada(null);
      refetchPlano();
    } catch { toast.error('Error al eliminar pared'); }
  };

  /* [134A-16] Dibujar pared arrastrando de punto A a punto B.
   * mousedown → marca start. mousemove → calcula preview. mouseup → crea pared. */
  const handleWallDrawStart = useCallback((canvasX: number, canvasY: number) => {
    setWallDrawStart({ x: canvasX, y: canvasY });
    setWallDrawPreview(null);
  }, []);

  const handleWallDrawMove = useCallback((canvasX: number, canvasY: number) => {
    if (!wallDrawStart) return;
    const dx = canvasX - wallDrawStart.x;
    const dy = canvasY - wallDrawStart.y;
    const length = Math.sqrt(dx * dx + dy * dy);
    if (length < 5) return;
    const angleDeg = Math.atan2(dy, dx) * (180 / Math.PI);
    const midX = (wallDrawStart.x + canvasX) / 2;
    const midY = (wallDrawStart.y + canvasY) / 2;
    setWallDrawPreview({
      x: midX - length / 2,
      y: midY - 5,
      w: length,
      rotation: Math.round(angleDeg),
    });
  }, [wallDrawStart]);

  const handleWallDrawEnd = useCallback(async () => {
    if (!wallDrawStart || !wallDrawPreview || !zonaActiva) {
      setWallDrawStart(null);
      setWallDrawPreview(null);
      return;
    }
    const rot = ((wallDrawPreview.rotation % 360) + 360) % 360;
    try {
      await crearPared({
        zona_id: zonaActiva,
        ancho: Math.round(wallDrawPreview.w),
        alto: 10,
        rotacion: rot,
        pos_x: Math.round(wallDrawPreview.x),
        pos_y: Math.round(wallDrawPreview.y),
      } as CrearParedRequest);
      refetchPlano();
    } catch { toast.error('Error al crear pared'); }
    setWallDrawStart(null);
    setWallDrawPreview(null);
  }, [wallDrawStart, wallDrawPreview, zonaActiva, refetchPlano]);

  const handleGuardarMesa = async (id: string, req: ActualizarMesaRequest) => {
    if (req.pos_x !== undefined || req.pos_y !== undefined) {
      setPosicionesLocales((prev) => ({ ...prev, [id]: { x: req.pos_x ?? prev[id]?.x ?? 0, y: req.pos_y ?? prev[id]?.y ?? 0 } }));
    }
    if (req.ancho !== undefined || req.alto !== undefined) {
      const mesaActual = mesasZona.find((m) => m.id === id);
      setDimensionesLocales((prev) => ({ ...prev, [id]: { ancho: req.ancho ?? prev[id]?.ancho ?? mesaActual?.ancho ?? 80, alto: req.alto ?? prev[id]?.alto ?? mesaActual?.alto ?? 80 } }));
    }
    await actualizarMesaApi(id, req);
    setMesaSeleccionada(null);
    refetchPlano();
  };

  const handleResizeMesa = async (id: string, req: ActualizarMesaRequest) => {
    const mesaActual = mesasZona.find((m) => m.id === id);
    if (!mesaActual) return;

    const prevPosicion = posicionesLocales[id];
    const prevDimension = dimensionesLocales[id];
    const prevSeleccionada = mesaSeleccionada;

    setPosicionesLocales((prev) => ({ ...prev, [id]: { x: req.pos_x ?? prev[id]?.x ?? mesaActual.pos_x, y: req.pos_y ?? prev[id]?.y ?? mesaActual.pos_y } }));
    setDimensionesLocales((prev) => ({ ...prev, [id]: { ancho: req.ancho ?? prev[id]?.ancho ?? mesaActual.ancho, alto: req.alto ?? prev[id]?.alto ?? mesaActual.alto } }));

    if (mesaSeleccionada?.id === id) {
      setMesaSeleccionada({ ...mesaSeleccionada, pos_x: req.pos_x ?? mesaSeleccionada.pos_x, pos_y: req.pos_y ?? mesaSeleccionada.pos_y, ancho: req.ancho ?? mesaSeleccionada.ancho, alto: req.alto ?? mesaSeleccionada.alto });
    }

    try {
      await actualizarMesaApi(id, req);
      refetchPlano();
    } catch {
      setPosicionesLocales((prev) => {
        const next = { ...prev };
        if (prevPosicion) next[id] = prevPosicion; else delete next[id];
        return next;
      });
      setDimensionesLocales((prev) => {
        const next = { ...prev };
        if (prevDimension) next[id] = prevDimension; else delete next[id];
        return next;
      });
      if (prevSeleccionada?.id === id) setMesaSeleccionada(prevSeleccionada);
      toast.error('No se pudo redimensionar la mesa');
    }
  };

  const handleEliminarMesa = (id: string) => {
    setDialogoConfirmar({
      titulo: 'Eliminar mesa',
      mensaje: '¿Eliminar esta mesa?',
      onConfirmar: async () => {
        await eliminarMesaApi(id);
        setMesaSeleccionada(null);
        refetchPlano();
      },
    });
  };

  const handleDragStart = (event: DragStartEvent) => setArrastrando(String(event.active.id));

  /* [283A-25] Clamp usa el ancho real del canvas (DOM) dividido por zoom en vez
   * de zonaData.ancho para que las mesas ocupen todo el ancho visible.
   * Deltas divididos por zoom para mantener coordenadas canónicas.
   * [283A-43] try/catch + refetch: si falla el PATCH, revierte posición local
   * y muestra toast error. Sin esto el drag parecía no guardar. */
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
    try {
      await actualizarPosiciones(req);
      refetchPlano();
    } catch {
      setPosicionesLocales((p) => ({ ...p, [mesaId]: { x: prev?.x ?? mesa.pos_x, y: prev?.y ?? mesa.pos_y } }));
      toast.error('No se pudo guardar la posición');
    }
  }, [mesasZona, posicionesLocales, zonaData, canvasRef, zoom, refetchPlano]);

  /* [303A-2] Migrado de raw fetch a axios para usar interceptors JWT/401 */
  const handleExportar = async () => {
    try {
      const resp = await axios.get('/api/plano-sala/export');
      const json = resp.data;
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
            refetchPlano();
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
          refetchPlano();
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
        refetchPlano();
      },
    });
  };

  /* [134A-3] Handlers de paredes — CRUD para muros decorativos en el plano.
   * Las paredes son rectángulos con color y rotación que representan muros,
   * columnas u otros elementos físicos de la sala. */
  /* [134A-8] Crear pared: grosor fijo 10, largo por defecto 120 */
  const handleCrearPared = async () => {
    if (!zonaActiva) return;
    try {
      await crearPared({ zona_id: zonaActiva, ancho: 120, alto: 10 } as CrearParedRequest);
      refetchPlano();
    } catch {
      toast.error('Error al crear pared');
    }
  };

  const handleEliminarPared = (id: string) => {
    setDialogoConfirmar({
      titulo: 'Eliminar pared',
      mensaje: '¿Eliminar esta pared?',
      onConfirmar: async () => {
        try {
          await eliminarParedApi(id);
          setParedSeleccionada(null);
          refetchPlano();
        } catch {
          toast.error('Error al eliminar pared');
        }
      },
    });
  };

  const handleGuardarPared = async (id: string, req: ActualizarParedRequest) => {
    try {
      await actualizarParedApi(id, req);
      setParedSeleccionada(null);
      refetchPlano();
    } catch {
      toast.error('Error al actualizar pared');
    }
  };

  /* [134A-3] Mover/rotar pared via drag directo — no limpia la selección para UX fluida. */
  const handleMoverPared = async (id: string, pos_x: number, pos_y: number) => {
    const pared = paredesZona.find(p => p.id === id);
    if (!pared) return;
    /* [134A-14] Snap-back correcto con rotación.
     * pos_x/pos_y es la esquina top-left del rect sin rotar; CSS rota alrededor del centro.
     * El clamp debe operar sobre el CENTRO del bounding box rotado, luego volver a top-left.
     *
     * centro visual = (pos_x + w/2, pos_y + h/2)
     * bbW/bbH = dimensiones del bounding box rotado
     * clamp centro a [bbW/2, zonaW - bbW/2] × [bbH/2, zonaH - bbH/2]
     * clampedX = centro_clampado - w/2
     */
    const w = pared.ancho;
    const h = pared.alto;
    const rad = (pared.rotacion * Math.PI) / 180;
    const bbW = w * Math.abs(Math.cos(rad)) + h * Math.abs(Math.sin(rad));
    const bbH = w * Math.abs(Math.sin(rad)) + h * Math.abs(Math.cos(rad));
    const zonaW = zonaData?.ancho ?? 600;
    const zonaH = zonaData?.alto ?? 600;
    const cx = Math.min(zonaW - bbW / 2, Math.max(bbW / 2, pos_x + w / 2));
    const cy = Math.min(zonaH - bbH / 2, Math.max(bbH / 2, pos_y + h / 2));
    const clampedX = Math.round(cx - w / 2);
    const clampedY = Math.round(cy - h / 2);
    try {
      await actualizarParedApi(id, {
        ancho: pared.ancho, alto: pared.alto,
        rotacion: pared.rotacion,
        pos_x: clampedX, pos_y: clampedY,
      });
      refetchPlano();
    } catch {
      toast.error('Error al mover pared');
    }
  };

  const handleRotarPared = async (id: string, rotacion: number) => {
    const pared = paredesZona.find(p => p.id === id);
    if (!pared) return;
    try {
      await actualizarParedApi(id, {
        ancho: pared.ancho, alto: pared.alto,
        rotacion,
        pos_x: pared.pos_x, pos_y: pared.pos_y,
      });
      refetchPlano();
    } catch {
      toast.error('Error al rotar pared');
    }
  };

  const handleRedimensionarPared = async (id: string, req: ActualizarParedRequest) => {
    try {
      await actualizarParedApi(id, req);
      refetchPlano();
    } catch {
      toast.error('Error al redimensionar pared');
    }
  };

  /* [134A-7] Duplicar pared — crea copia con offset de 20px */
  const handleDuplicarPared = async (pared: ParedSala) => {
    if (!zonaActiva) return;
    try {
      await crearPared({
        zona_id: zonaActiva,
        ancho: pared.ancho, alto: pared.alto,
        rotacion: pared.rotacion,
        pos_x: pared.pos_x + 20, pos_y: pared.pos_y + 20,
      } as CrearParedRequest);
      refetchPlano();
      toast.success('Pared duplicada');
    } catch {
      toast.error('Error al duplicar pared');
    }
  };

  /* [134A-7] Duplicar mesa — crea copia con siguiente número disponible + offset 20px */
  const handleDuplicarMesa = async (mesa: Mesa) => {
    if (!zonaActiva) return;
    const maxNum = mesasZona.reduce((max, m) => Math.max(max, m.numero), 0);
    try {
      await crearMesa({
        zona_id: zonaActiva,
        numero: maxNum + 1,
        forma: mesa.forma, ancho: mesa.ancho, alto: mesa.alto,
        min_personas: mesa.min_personas, max_personas: mesa.max_personas,
        pos_x: mesa.pos_x + 20, pos_y: mesa.pos_y + 20,
      } as CrearMesaRequest);
      refetchPlano();
      toast.success(`Mesa ${maxNum + 1} duplicada`);
    } catch (err: unknown) {
      const axiosErr = err as { response?: { data?: { message?: string } } };
      toast.error(axiosErr?.response?.data?.message || 'Error al duplicar mesa');
    }
  };

  return {
    plano, zonaActiva, zonaData, mesasZona, paredesZona, mesaSeleccionada, arrastrando,
    paredSeleccionada, setParedSeleccionada,
    posicionesLocales, dimensionesLocales, setMesaSeleccionada, cambiarZona, zoom, setZoom,
    canvasHeight,
    /* [134A-15] Herramientas del toolbar */
    activeTool, setActiveTool,
    wallDrawStart, wallDrawPreview,
    handleCrearMesaRapida, handleEliminarMesaDirecta, handleEliminarParedDirecta,
    handleWallDrawStart, handleWallDrawMove, handleWallDrawEnd,
    handleCrearZona, handleEliminarZona, handleEditarZona,
    handleCrearMesa, handleGuardarMesa, handleResizeMesa, handleEliminarMesa,
    handleCrearPared, handleEliminarPared, handleGuardarPared,
    handleMoverPared, handleRotarPared, handleRedimensionarPared,
    handleDuplicarPared, handleDuplicarMesa,
    handleDragStart, handleDragEnd,
    handleExportar, handleImportar,
    handleCrearCombinacion, handleEliminarCombinacion,
    dialogoEntrada, setDialogoEntrada,
    dialogoConfirmar, setDialogoConfirmar,
    dialogoCombinacion, setDialogoCombinacion,
  };
}
