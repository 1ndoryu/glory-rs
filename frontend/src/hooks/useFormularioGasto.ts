/* 253A-10: Hook para FormularioGasto — reduce 10 useState a 2 (regla usestate-excesivo)
   253A-14: acepta onExito para uso en modales
   253A-21: metodo_pago es opcional — puede enviarse null si el usuario no selecciona */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearGasto, useListarCategorias, MetodoPago, TipoDocumento } from '../api/generated';

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

function useFormularioGasto(onExito?: () => void) {
  const navigate = useNavigate();
  const [error, setError] = useState('');
  const [campos, setCampos] = useState<CamposGasto>({
    fecha: new Date().toISOString().split('T')[0],
    proveedor: '',
    categoriaId: '',
    tipoDocumento: TipoDocumento.factura,
    metodoPago: MetodoPago.efectivo,
    numeroDocumento: '',
    recurrente: false,
    importeBase: '',
    importeIva: '',
  });

  const { data: categoriasResp } = useListarCategorias();
  const categorias = categoriasResp?.status === 200 ? categoriasResp.data : [];

  const cambiarCampo = <K extends keyof CamposGasto>(campo: K, valor: CamposGasto[K]) => {
    setCampos(prev => ({ ...prev, [campo]: valor }));
  };

  const mutation = useCrearGasto({
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
  };

  return { campos, cambiarCampo, error, manejarEnvio, cargando: mutation.isPending, categorias };
}

export default useFormularioGasto;
