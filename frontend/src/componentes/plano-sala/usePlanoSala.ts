/* [263A-14] Hook con lógica de negocio del plano de sala.
 * Maneja zona activa, mesa seleccionada, CRUD, drag-and-drop y export/import. */

import { useState, useCallback } from 'react';
import type { DragEndEvent, DragStartEvent } from '@dnd-kit/core';
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

export function usePlanoSala() {
  const { data, refetch } = useObtenerPlano();
  const plano = data?.status === 200 ? data.data : null;

  const [zonaActiva, setZonaActiva] = useState<string | null>(null);
  const [mesaSeleccionada, setMesaSeleccionada] = useState<Mesa | null>(null);
  const [arrastrando, setArrastrando] = useState<string | null>(null);
  const [posicionesLocales, setPosicionesLocales] = useState<
    Record<string, { x: number; y: number }>
  >({});

  const zonaData = plano?.zonas.find((z) => z.id === zonaActiva);
  const mesasZona = zonaData?.mesas ?? [];

  /* Auto-seleccionar primera zona */
  if (plano && plano.zonas.length > 0 && !zonaActiva) {
    setZonaActiva(plano.zonas[0].id);
  }

  const cambiarZona = (id: string) => {
    setZonaActiva(id);
    setMesaSeleccionada(null);
    setPosicionesLocales({});
  };

  const handleCrearZona = async () => {
    const nombre = prompt('Nombre de la zona:');
    if (!nombre) return;
    await crearZona({ nombre } as CrearZonaRequest);
    refetch();
  };

  const handleEliminarZona = async () => {
    if (!zonaActiva || !confirm('¿Eliminar esta zona y todas sus mesas?')) return;
    await eliminarZona(zonaActiva);
    setZonaActiva(null);
    setMesaSeleccionada(null);
    refetch();
  };

  const handleEditarZona = async () => {
    if (!zonaActiva || !zonaData) return;
    const nombre = prompt('Nuevo nombre:', zonaData.nombre);
    if (!nombre) return;
    await actualizarZonaApi(zonaActiva, { nombre } as ActualizarZonaRequest);
    refetch();
  };

  const handleCrearMesa = async () => {
    if (!zonaActiva) return;
    const numStr = prompt('Número de mesa:');
    if (!numStr) return;
    const numero = Number(numStr);
    if (Number.isNaN(numero) || numero < 1) return;
    await crearMesa({ zona_id: zonaActiva, numero, pos_x: 50, pos_y: 50 } as CrearMesaRequest);
    refetch();
  };

  const handleGuardarMesa = async (id: string, req: ActualizarMesaRequest) => {
    await actualizarMesaApi(id, req);
    setMesaSeleccionada(null);
    refetch();
  };

  const handleEliminarMesa = async (id: string) => {
    if (!confirm('¿Eliminar esta mesa?')) return;
    await eliminarMesaApi(id);
    setMesaSeleccionada(null);
    refetch();
  };

  const handleDragStart = (event: DragStartEvent) => setArrastrando(String(event.active.id));

  const handleDragEnd = useCallback(async (event: DragEndEvent) => {
    setArrastrando(null);
    const mesaId = String(event.active.id);
    const mesa = mesasZona.find((m) => m.id === mesaId);
    if (!mesa) return;
    const prev = posicionesLocales[mesaId];
    const nuevoX = Math.max(0, (prev?.x ?? mesa.pos_x) + event.delta.x);
    const nuevoY = Math.max(0, (prev?.y ?? mesa.pos_y) + event.delta.y);
    setPosicionesLocales((p) => ({ ...p, [mesaId]: { x: nuevoX, y: nuevoY } }));
    const req: ActualizarPosicionesRequest = {
      posiciones: [{ id: mesaId, pos_x: Math.round(nuevoX), pos_y: Math.round(nuevoY) }],
    };
    await actualizarPosiciones(req);
  }, [mesasZona, posicionesLocales]);

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
    } catch { alert('Error al exportar'); }
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
        if (!confirm('Esto reemplazará todo el plano actual. ¿Continuar?')) return;
        await importarPlano(d);
        setZonaActiva(null);
        setMesaSeleccionada(null);
        refetch();
      } catch { alert('Error al importar: archivo inválido'); }
    };
    input.click();
  };

  const handleCrearCombinacion = async () => {
    const nombre = prompt('Nombre de la combinación:');
    if (!nombre) return;
    const maxStr = prompt('Máx personas en combinación:');
    if (!maxStr) return;
    await crearCombinacion({ nombre, max_personas: Number(maxStr), mesa_ids: [] } as CrearCombinacionRequest);
    refetch();
  };

  const handleEliminarCombinacion = async (id: string) => {
    if (!confirm('¿Eliminar esta combinación?')) return;
    await eliminarCombinacion(id);
    refetch();
  };

  return {
    plano, zonaActiva, zonaData, mesasZona, mesaSeleccionada, arrastrando,
    posicionesLocales, setMesaSeleccionada, cambiarZona,
    handleCrearZona, handleEliminarZona, handleEditarZona,
    handleCrearMesa, handleGuardarMesa, handleEliminarMesa,
    handleDragStart, handleDragEnd,
    handleExportar, handleImportar,
    handleCrearCombinacion, handleEliminarCombinacion,
  };
}
