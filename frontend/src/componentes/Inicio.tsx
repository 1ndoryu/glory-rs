/* 253A-7: Inicio -- dashboard con resumen economico y accesos rapidos
   253A-10: componentes UI atomicos
   253A-14: botones de accion abren modales en vez de navegar */

import { useState } from 'react';
import { useResumen, useConteoReservas } from '../api/generated';
import { Boton, Modal } from './ui';
import FormularioVenta from './FormularioVenta';
import FormularioGasto from './FormularioGasto';
import FormularioReserva from './FormularioReserva';
import '../estilos/Inicio.css';

function formatearMoneda(valor: string): string {
  const num = parseFloat(valor);
  return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(num);
}

function Inicio() {
  const [modalVenta, setModalVenta] = useState(false);
  const [modalGasto, setModalGasto] = useState(false);
  const [modalReserva, setModalReserva] = useState(false);
  const ahora = new Date();
  const year = ahora.getFullYear();
  const month = ahora.getMonth() + 1;

  const { data: resumen, isLoading: cargandoResumen, refetch: refetchResumen } = useResumen({ year, month });
  const { data: conteo, isLoading: cargandoConteo, refetch: refetchConteo } = useConteoReservas();

  const datosResumen = resumen?.status === 200 ? resumen.data : null;
  const datosConteo = conteo?.status === 200 ? conteo.data : null;

  return (
    <div>
      <div className="cabeceraPagina">
        <h1 className="tituloPagina">Inicio</h1>
        <p className="subtituloPagina">Resumen del mes actual</p>
      </div>

      <div className="accionesInicio">
        <Boton variante="exito" onClick={() => setModalVenta(true)}>
          + Nueva Venta
        </Boton>
        <Boton variante="primario" onClick={() => setModalGasto(true)}>
          + Nuevo Gasto
        </Boton>
        <Boton variante="secundario" onClick={() => setModalReserva(true)}>
          + Nueva Reserva
        </Boton>
      </div>

      <Modal abierto={modalVenta} onCerrar={() => setModalVenta(false)} titulo="Nueva Venta">
        <FormularioVenta onExito={() => { setModalVenta(false); refetchResumen(); }} />
      </Modal>
      <Modal abierto={modalGasto} onCerrar={() => setModalGasto(false)} titulo="Nuevo Gasto">
        <FormularioGasto onExito={() => { setModalGasto(false); refetchResumen(); }} />
      </Modal>
      <Modal abierto={modalReserva} onCerrar={() => setModalReserva(false)} titulo="Nueva Reserva">
        <FormularioReserva onExito={() => { setModalReserva(false); refetchConteo(); }} />
      </Modal>

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
