/* 253A-10: Hook para FormularioVenta — reduce 10 useState a 2 (regla usestate-excesivo)
   253A-14: acepta onExito para uso en modales */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearVenta, Turno, CanalVenta, MetodoPago } from '../api/generated';

interface CamposVenta {
  fecha: string;
  comensales: string;
  descripcion: string;
  ivaPorcentaje: string;
  turno: Turno;
  canal: CanalVenta;
  metodoPago: MetodoPago;
  importeBase: string;
  importeIva: string;
}

function useFormularioVenta(onExito?: () => void) {
  const navigate = useNavigate();
  const [error, setError] = useState('');
  const [campos, setCampos] = useState<CamposVenta>({
    fecha: new Date().toISOString().split('T')[0],
    comensales: '',
    descripcion: '',
    ivaPorcentaje: '10',
    turno: Turno.mediodia,
    canal: CanalVenta.comedor,
    metodoPago: MetodoPago.efectivo,
    importeBase: '',
    importeIva: '',
  });

  const cambiarCampo = <K extends keyof CamposVenta>(campo: K, valor: CamposVenta[K]) => {
    setCampos(prev => {
      const nuevo = { ...prev, [campo]: valor };
      /* Auto-calcular IVA al cambiar importe base o porcentaje */
      if (campo === 'importeBase' || campo === 'ivaPorcentaje') {
        const base = campo === 'importeBase' ? parseFloat(valor as string) : parseFloat(prev.importeBase);
        const pct = campo === 'ivaPorcentaje' ? parseFloat(valor as string) : parseFloat(prev.ivaPorcentaje);
        if (!isNaN(base) && !isNaN(pct)) {
          nuevo.importeIva = (base * pct / 100).toFixed(2);
        }
      }
      return nuevo;
    });
  };

  const mutation = useCrearVenta({
    mutation: {
      onSuccess: (res) => {
        if (res.status === 201) {
          if (onExito) onExito();
          else navigate('/ventas');
        }
      },
      onError: () => setError('Error al crear la venta'),
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');
    if (!campos.fecha || !campos.importeBase || !campos.importeIva) {
      setError('Completa los campos obligatorios');
      return;
    }
    mutation.mutate({
      data: {
        fecha: campos.fecha,
        comensales: campos.comensales ? parseInt(campos.comensales, 10) : null,
        descripcion: campos.descripcion || null,
        iva_porcentaje: campos.ivaPorcentaje,
        turno: campos.turno,
        canal: campos.canal,
        metodo_pago: campos.metodoPago,
        importe_base: campos.importeBase,
        importe_iva: campos.importeIva,
      },
    });
  };

  return { campos, cambiarCampo, error, manejarEnvio, cargando: mutation.isPending };
}

export default useFormularioVenta;
