/* 253A-10: Hook useFormularioVenta — estado del formulario de ventas.
   253A-14: acepta onExito para uso en modales.
   253A-19: Refactorizado para turnos multi-select y detalles por turno.
   283A-22: soporte edición — acepta ventaInicial para pre-rellenar y usa PUT.
   El backend acepta un turno por venta, así que se crean simultáneamente N ventas
   (una por turno) usando Promise.all con crearVenta directamente. */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useQueryClient } from '@tanstack/react-query';
import { crearVenta, actualizarVenta, getListarVentasQueryKey, Turno, CanalVenta, MetodoPago, CrearVentaRequest, Venta } from '../api/generated';

export interface DetalleTurno {
  importeBase: string;
  metodoPago: MetodoPago;
}

interface CamposVenta {
  fecha: string;
  comensales: string;
  descripcion: string;
  ivaPorcentaje: string;
  turnos: Turno[];
  canal: CanalVenta;
  detalles: Record<Turno, DetalleTurno>;
}

const detallePorDefecto = (): DetalleTurno => ({ importeBase: '', metodoPago: MetodoPago.efectivo });

function camposIniciales(ventaInicial?: Venta): CamposVenta {
  if (ventaInicial) {
    const turno = (ventaInicial.turno as Turno) || Turno.mediodia;
    return {
      fecha: ventaInicial.fecha,
      comensales: ventaInicial.comensales?.toString() || '',
      descripcion: ventaInicial.descripcion || '',
      ivaPorcentaje: ventaInicial.iva_porcentaje,
      turnos: [turno],
      canal: (ventaInicial.canal as CanalVenta) || CanalVenta.comedor,
      detalles: {
        [Turno.manana]: turno === Turno.manana
          ? { importeBase: ventaInicial.importe_base, metodoPago: (ventaInicial.metodo_pago as MetodoPago) || MetodoPago.efectivo }
          : detallePorDefecto(),
        [Turno.mediodia]: turno === Turno.mediodia
          ? { importeBase: ventaInicial.importe_base, metodoPago: (ventaInicial.metodo_pago as MetodoPago) || MetodoPago.efectivo }
          : detallePorDefecto(),
        [Turno.noche]: turno === Turno.noche
          ? { importeBase: ventaInicial.importe_base, metodoPago: (ventaInicial.metodo_pago as MetodoPago) || MetodoPago.efectivo }
          : detallePorDefecto(),
      },
    };
  }
  return {
    fecha: new Date().toISOString().split('T')[0],
    comensales: '',
    descripcion: '',
    ivaPorcentaje: '10',
    turnos: [Turno.mediodia],
    canal: CanalVenta.comedor,
    detalles: {
      [Turno.manana]: detallePorDefecto(),
      [Turno.mediodia]: detallePorDefecto(),
      [Turno.noche]: detallePorDefecto(),
    },
  };
}

export function calcularIva(importeBase: string, ivaPorcentaje: string): string {
  const base = parseFloat(importeBase);
  const pct = parseFloat(ivaPorcentaje);
  if (isNaN(base) || isNaN(pct)) return '0.00';
  return (base * pct / 100).toFixed(2);
}

function useFormularioVenta(onExito?: () => void, ventaInicial?: Venta) {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [error, setError] = useState('');
  const [cargando, setCargando] = useState(false);
  const [campos, setCampos] = useState<CamposVenta>(() => camposIniciales(ventaInicial));
  const esEdicion = !!ventaInicial;

  function cambiarCampo<K extends keyof CamposVenta>(campo: K, valor: CamposVenta[K]) {
    setCampos(prev => ({ ...prev, [campo]: valor }));
  }

  function toggleTurno(turno: Turno) {
    /* [283A-22] En modo edición no se permite cambiar de turno (es 1 venta existente) */
    if (esEdicion) return;
    setCampos(prev => {
      const ya = prev.turnos.includes(turno);
      const nuevos = ya ? prev.turnos.filter(t => t !== turno) : [...prev.turnos, turno];
      /* Mínimo 1 turno siempre seleccionado */
      return nuevos.length > 0 ? { ...prev, turnos: nuevos } : prev;
    });
  }

  function cambiarDetalle(turno: Turno, campo: keyof DetalleTurno, valor: string | MetodoPago) {
    setCampos(prev => ({
      ...prev,
      detalles: { ...prev.detalles, [turno]: { ...prev.detalles[turno], [campo]: valor } },
    }));
  }

  async function manejarEnvio(e: FormEvent) {
    e.preventDefault();
    setError('');
    if (!campos.fecha) { setError('La fecha es obligatoria'); return; }

    if (esEdicion) {
      /* [283A-22] Modo edición: actualizar la venta existente vía PUT */
      const turno = campos.turnos[0];
      const d = campos.detalles[turno];
      if (!d.importeBase) { setError('El importe es obligatorio'); return; }
      setCargando(true);
      try {
        await actualizarVenta(ventaInicial!.id, {
          fecha: campos.fecha,
          comensales: campos.comensales ? parseInt(campos.comensales, 10) : null,
          descripcion: campos.descripcion || null,
          iva_porcentaje: campos.ivaPorcentaje,
          turno: turno,
          canal: campos.canal,
          metodo_pago: d.metodoPago,
          importe_base: d.importeBase,
          importe_iva: calcularIva(d.importeBase, campos.ivaPorcentaje),
        });
        await queryClient.invalidateQueries({ queryKey: getListarVentasQueryKey() });
        if (onExito) onExito(); else navigate('/ventas');
      } catch {
        setError('Error al actualizar la venta');
      } finally {
        setCargando(false);
      }
      return;
    }

    /* Modo creación: crear N ventas (una por turno) */
    const sinImporte = campos.turnos.filter(t => !campos.detalles[t].importeBase);
    if (sinImporte.length > 0) { setError('Todos los turnos seleccionados deben tener importe'); return; }
    setCargando(true);
    try {
      await Promise.all(campos.turnos.map(turno => {
        const d = campos.detalles[turno];
        const req: CrearVentaRequest = {
          fecha: campos.fecha,
          comensales: campos.comensales ? parseInt(campos.comensales, 10) : null,
          descripcion: campos.descripcion || null,
          iva_porcentaje: campos.ivaPorcentaje,
          turno,
          canal: campos.canal,
          metodo_pago: d.metodoPago,
          importe_base: d.importeBase,
          importe_iva: calcularIva(d.importeBase, campos.ivaPorcentaje),
        };
        return crearVenta(req);
      }));
      await queryClient.invalidateQueries({ queryKey: getListarVentasQueryKey() });
      if (onExito) onExito(); else navigate('/ventas');
    } catch {
      setError('Error al registrar la venta');
    } finally {
      setCargando(false);
    }
  }

  return { campos, cambiarCampo, toggleTurno, cambiarDetalle, error, manejarEnvio, cargando, esEdicion };
}

export default useFormularioVenta;
