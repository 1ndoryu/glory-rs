/* 253A-7: Inicio — dashboard con resumen económico y accesos rápidos
   253A-10: componentes UI atómicos */

import { useNavigate } from 'react-router-dom';
import { useResumen, useConteoReservas } from '../api/generated';
import { Boton } from './ui';
import '../estilos/Inicio.css';

function formatearMoneda(valor: string): string {
  const num = parseFloat(valor);
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(num);
}

function Inicio() {
  const navigate = useNavigate();
  const ahora = new Date();
  const year = ahora.getFullYear();
  const month = ahora.getMonth() + 1;

  const { data: resumen, isLoading: cargandoResumen } = useResumen({ year, month });
  const { data: conteo, isLoading: cargandoConteo } = useConteoReservas();

  const datosResumen = resumen?.status === 200 ? resumen.data : null;
  const datosConteo = conteo?.status === 200 ? conteo.data : null;

  return (
    <div>
      <div className="cabeceraPagina">
        <h1 className="tituloPagina">Inicio</h1>
        <p className="subtituloPagina">Resumen del mes actual</p>
      </div>

      <div className="accionesInicio">
        <Boton variante="exito" onClick={() => navigate('/ventas/nueva')}>
          + Nueva Venta
        </Boton>
        <Boton variante="primario" onClick={() => navigate('/gastos/nuevo')}>
          + Nuevo Gasto
        </Boton>
        <Boton variante="secundario" onClick={() => navigate('/reservas/nueva')}>
          + Nueva Reserva
        </Boton>
      </div>

      {cargandoResumen ? (
        <p className="cargando">Cargando resumen...</p>
      ) : datosResumen ? (
        <div className="inicioGrid">
          <div className="tarjetaResumen">
            <div className="etiquetaResumen">Ventas</div>
            <div className="valorResumen positivo">{formatearMoneda(datosResumen.total_ventas)}</div>
          </div>
          <div className="tarjetaResumen">
            <div className="etiquetaResumen">Gastos</div>
            <div className="valorResumen negativo">{formatearMoneda(datosResumen.total_gastos)}</div>
          </div>
          <div className="tarjetaResumen">
            <div className="etiquetaResumen">Margen</div>
            <div className={`valorResumen ${parseFloat(datosResumen.margen) >= 0 ? 'positivo' : 'negativo'}`}>
              {formatearMoneda(datosResumen.margen)}
            </div>
          </div>
        </div>
      ) : (
        <div className="inicioGrid">
          <div className="tarjetaResumen">
            <div className="etiquetaResumen">Ventas</div>
            <div className="valorResumen neutro">—</div>
          </div>
          <div className="tarjetaResumen">
            <div className="etiquetaResumen">Gastos</div>
            <div className="valorResumen neutro">—</div>
          </div>
          <div className="tarjetaResumen">
            <div className="etiquetaResumen">Margen</div>
            <div className="valorResumen neutro">—</div>
          </div>
        </div>
      )}

      <div className="seccionReservas">
        <h2 className="tituloSeccion">Reservas</h2>
        {cargandoConteo ? (
          <p className="cargando">Cargando...</p>
        ) : datosConteo ? (
          <div className="conteoReservas">
            <div className="conteoItem">
              <div className="conteoNumero">{datosConteo.total_hoy}</div>
              <div className="conteoEtiqueta">Hoy</div>
            </div>
            <div className="conteoItem">
              <div className="conteoNumero">{datosConteo.total_mes}</div>
              <div className="conteoEtiqueta">Este mes</div>
            </div>
          </div>
        ) : (
          <div className="conteoReservas">
            <div className="conteoItem">
              <div className="conteoNumero">0</div>
              <div className="conteoEtiqueta">Hoy</div>
            </div>
            <div className="conteoItem">
              <div className="conteoNumero">0</div>
              <div className="conteoEtiqueta">Este mes</div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default Inicio;
