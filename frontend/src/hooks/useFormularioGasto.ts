/* 253A-10: Hook para FormularioGasto — reduce 10 useState a 2 (regla usestate-excesivo)
   253A-14: acepta onExito para uso en modales
   253A-21: metodo_pago es opcional — puede enviarse null si el usuario no selecciona
   283A-22: soporte edición — acepta gastoInicial para pre-rellenar y usa PUT */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearGasto, useActualizarGasto, useListarCategorias, MetodoPago, TipoDocumento, Gasto } from '../api/generated';

interface CamposGasto {
  fecha: string;
  proveedor: string;
  categoriaId: string;
  tipoDocumento: TipoDocumento;
  metodoPago: MetodoPago | '';
  numeroDocumento: string;
  recurrente: boolean;
  importeBase: string;
  importeIva: string;
}

function useFormularioGasto(onExito?: () => void, gastoInicial?: Gasto) {
  const navigate = useNavigate();
  const [error, setError] = useState('');
  const esEdicion = !!gastoInicial;

  const [campos, setCampos] = useState<CamposGasto>(() => {
    if (gastoInicial) {
      return {
        fecha: gastoInicial.fecha,
        proveedor: gastoInicial.proveedor || '',
        categoriaId: gastoInicial.categoria_id || '',
        tipoDocumento: (gastoInicial.tipo_documento as TipoDocumento) || TipoDocumento.factura,
        metodoPago: (gastoInicial.metodo_pago as MetodoPago) || '',
        numeroDocumento: gastoInicial.numero_documento || '',
        recurrente: gastoInicial.recurrente,
        importeBase: gastoInicial.importe_base,
        importeIva: gastoInicial.importe_iva,
      };
    }
    return {
      fecha: new Date().toISOString().split('T')[0],
      proveedor: '',
      categoriaId: '',
      tipoDocumento: TipoDocumento.factura,
      metodoPago: MetodoPago.efectivo,
      numeroDocumento: '',
      recurrente: false,
      importeBase: '',
      importeIva: '',
    };
  });

  const { data: categoriasResp } = useListarCategorias();
  const categorias = categoriasResp?.status === 200 ? categoriasResp.data : [];

  const cambiarCampo = <K extends keyof CamposGasto>(campo: K, valor: CamposGasto[K]) => {
    setCampos(prev => ({ ...prev, [campo]: valor }));
  };

  const crearMutation = useCrearGasto({
    mutation: {
      onSuccess: (res) => {
        if (res.status === 201) {
          if (onExito) onExito();
          else navigate('/gastos');
        }
      },
      onError: () => setError('Error al crear el gasto'),
    },
  });

  const actualizarMutation = useActualizarGasto({
    mutation: {
      onSuccess: (res) => {
        if (res.status === 200) {
          if (onExito) onExito();
          else navigate('/gastos');
        }
      },
      onError: () => setError('Error al actualizar el gasto'),
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');
    if (!campos.fecha || !campos.importeBase || !campos.importeIva) {
      setError('Completa los campos obligatorios');
      return;
    }

    if (esEdicion) {
      actualizarMutation.mutate({
        id: gastoInicial!.id,
        data: {
          fecha: campos.fecha,
          proveedor: campos.proveedor || null,
          categoria_id: campos.categoriaId || null,
          tipo_documento: campos.tipoDocumento,
          metodo_pago: campos.metodoPago || null,
          numero_documento: campos.numeroDocumento || null,
          recurrente: campos.recurrente || null,
          importe_base: campos.importeBase,
          importe_iva: campos.importeIva,
        },
      });
    } else {
      crearMutation.mutate({
        data: {
          fecha: campos.fecha,
          proveedor: campos.proveedor || null,
          categoria_id: campos.categoriaId || null,
          tipo_documento: campos.tipoDocumento,
          metodo_pago: campos.metodoPago || null,
          numero_documento: campos.numeroDocumento || null,
          recurrente: campos.recurrente || null,
          importe_base: campos.importeBase,
          importe_iva: campos.importeIva,
        },
      });
    }
  };

  return {
    campos,
    cambiarCampo,
    error,
    manejarEnvio,
    cargando: crearMutation.isPending || actualizarMutation.isPending,
    categorias,
    esEdicion,
  };
}

export default useFormularioGasto;
