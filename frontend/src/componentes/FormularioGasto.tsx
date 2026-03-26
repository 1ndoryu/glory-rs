/* 253A-7: Formulario para registrar un nuevo gasto
   253A-10: hook useFormularioGasto + componentes UI atómicos
   253A-14: acepta onExito para uso en modales
   253A-20: Menú inicial de 3 opciones (manual / digitalizar / correo).
   "Por correo" descartado por el cliente — se muestra deshabilitado con tooltip.
   "Digitalizar" es placeholder hasta que el backend soporte OCR de imágenes. */

import { useState } from 'react';
import { MetodoPago, TipoDocumento } from '../api/generated';
import useFormularioGasto from '../hooks/useFormularioGasto';
import { Input, Select, Boton } from '@glory/componentes/ui';
import '../estilos/Formularios.css';

type ModoGasto = 'menu' | 'manual' | 'digitalizar';

interface Props {
  onExito?: () => void;
}

function FormularioGasto({ onExito }: Props) {
  const [modo, setModo] = useState<ModoGasto>('menu');
  const { campos, cambiarCampo, error, manejarEnvio, cargando, categorias } = useFormularioGasto(onExito);

  if (modo === 'menu') {
    return (
      <div className={onExito ? '' : 'formularioPagina'}>
        {!onExito && (
          <div className="cabeceraPagina">
            <h1 className="tituloPagina">Nuevo Gasto</h1>
            <p className="subtituloPagina">¿Cómo quieres registrar el gasto?</p>
          </div>
        )}
        <div className="menuGasto">
          <Boton variante="fantasma" type="button" className="tarjetaModoGasto" onClick={() => setModo('manual')}>
            <span className="iconoModoGasto">📋</span>
            <span className="tituloModoGasto">Gasto manual</span>
            <span className="descripcionModoGasto">Introduce los datos a mano</span>
          </Boton>
          <Boton variante="fantasma" type="button" className="tarjetaModoGasto" onClick={() => setModo('digitalizar')}>
            <span className="iconoModoGasto">📷</span>
            <span className="tituloModoGasto">Digitalizar archivos</span>
            <span className="descripcionModoGasto">Sube una foto del documento</span>
          </Boton>
          <Boton variante="fantasma" type="button" className="tarjetaModoGasto deshabilitado" disabled title="Funcionalidad no disponible en esta versión">
            <span className="iconoModoGasto">✉️</span>
            <span className="tituloModoGasto">Por correo</span>
            <span className="descripcionModoGasto">Próximamente</span>
          </Boton>
        </div>
      </div>
    );
  }

  if (modo === 'digitalizar') {
    return (
      <div className={onExito ? '' : 'formularioPagina'}>
        {!onExito && (
          <div className="cabeceraPagina">
            <h1 className="tituloPagina">Digitalizar documento</h1>
          </div>
        )}
        <div className="formulario">
          <div className="areaDigitalizar">
            <span className="iconoDigitalizar">📷</span>
            <p className="textoDigitalizar">Funcionalidad de digitalización próximamente disponible</p>
            <p className="subtextoDigitalizar">Pronto podrás subir una foto de tu factura o albarán y los datos se extraerán automáticamente.</p>
          </div>
          <Boton variante="secundario" ancho type="button" onClick={() => setModo('menu')}>
            Volver
          </Boton>
        </div>
      </div>
    );
  }

  return (
    <div className={onExito ? '' : 'formularioPagina'}>
      {!onExito && (
        <div className="cabeceraPagina">
          <h1 className="tituloPagina">Gasto manual</h1>
          <p className="subtituloPagina">Registrar un gasto del restaurante</p>
        </div>
      )}

      {error && <div className="errorFormulario">{error}</div>}

      <form className="formulario" onSubmit={manejarEnvio}>
        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="fecha">Fecha</label>
            <Input id="fecha" type="date" value={campos.fecha} onChange={(e) => cambiarCampo('fecha', e.target.value)} />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="proveedor">Proveedor</label>
            <Input id="proveedor" type="text" value={campos.proveedor} onChange={(e) => cambiarCampo('proveedor', e.target.value)} placeholder="Opcional" />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="categoria">Categoría</label>
            <Select id="categoria" value={campos.categoriaId} onChange={(e) => cambiarCampo('categoriaId', e.target.value)}>
              <option value="">Sin categoría</option>
              {categorias.map((cat) => (
                <option key={cat.id} value={cat.id}>{cat.nombre}</option>
              ))}
            </Select>
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="tipoDocumento">Tipo documento</label>
            <Select id="tipoDocumento" value={campos.tipoDocumento} onChange={(e) => cambiarCampo('tipoDocumento', e.target.value as TipoDocumento)}>
              <option value={TipoDocumento.factura}>Factura</option>
              <option value={TipoDocumento.albaran}>Albarán</option>
              <option value={TipoDocumento.ticket}>Ticket</option>
            </Select>
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="metodoPago">Método de pago <span className="etiquetaOpcional">(Opcional)</span></label>
            <Select id="metodoPago" value={campos.metodoPago} onChange={(e) => cambiarCampo('metodoPago', e.target.value as MetodoPago | '')}>
              <option value="">— sin especificar —</option>
              <option value={MetodoPago.efectivo}>Efectivo</option>
              <option value={MetodoPago.tarjeta}>Tarjeta</option>
              <option value={MetodoPago.transferencia}>Transferencia</option>
              <option value={MetodoPago.otros}>Otros</option>
            </Select>
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="numeroDocumento">
              Nº documento <span className="etiquetaOpcional">(Opcional)</span>
            </label>
            <Input id="numeroDocumento" type="text" value={campos.numeroDocumento} onChange={(e) => cambiarCampo('numeroDocumento', e.target.value)} placeholder="Para programación automática futura" />
          </div>
        </div>

        <div className="filaFormulario">
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="importeBase">Importe base (€)</label>
            <Input id="importeBase" type="number" step="0.01" min="0" value={campos.importeBase} onChange={(e) => cambiarCampo('importeBase', e.target.value)} placeholder="0.00" />
          </div>
          <div className="grupoFormulario">
            <label className="etiquetaFormulario" htmlFor="importeIva">Importe IVA (€)</label>
            <Input id="importeIva" type="number" step="0.01" min="0" value={campos.importeIva} onChange={(e) => cambiarCampo('importeIva', e.target.value)} placeholder="0.00" />
          </div>
        </div>

        <div className="checkboxFormulario">
          <Input id="recurrente" type="checkbox" checked={campos.recurrente} onChange={(e) => cambiarCampo('recurrente', e.target.checked)} />
          <label htmlFor="recurrente">Gasto recurrente</label>
        </div>

        <div className="filaFormulario">
          <Boton variante="secundario" type="button" onClick={() => setModo('menu')}>Volver</Boton>
          <Boton variante="primario" type="submit" cargando={cargando}>Registrar Gasto</Boton>
        </div>
      </form>
    </div>
  );
}

export default FormularioGasto;

