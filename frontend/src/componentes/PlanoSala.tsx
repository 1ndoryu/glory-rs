/* [263A-14] PlanoSala — Constructor visual de plano de sala con drag-and-drop.
 * Lógica en usePlanoSala, mesa arrastrable en MesaDraggable, config en PanelConfigMesa. */

import { useRef } from 'react';
import { DndContext, PointerSensor, useSensor, useSensors } from '@dnd-kit/core';
import { Boton } from '@glory/componentes/ui';
import MesaDraggable from './plano-sala/MesaDraggable';
import PanelConfigMesa from './plano-sala/PanelConfigMesa';
import { usePlanoSala } from './plano-sala/usePlanoSala';
import '../estilos/PlanoSala.css';

function PlanoSala() {
  const {
    plano, zonaActiva, zonaData, mesasZona, mesaSeleccionada, arrastrando,
    posicionesLocales, setMesaSeleccionada, cambiarZona,
    handleCrearZona, handleEliminarZona, handleEditarZona,
    handleCrearMesa, handleGuardarMesa, handleEliminarMesa,
    handleDragStart, handleDragEnd,
    handleExportar, handleImportar,
    handleCrearCombinacion, handleEliminarCombinacion,
  } = usePlanoSala();

  const canvasRef = useRef<HTMLDivElement>(null);
  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 5 } })
  );

  return (
    <div className="planoSala">
      <div className="cabeceraPagina">
        <h1 className="tituloPagina">Plano de Sala</h1>
        <p className="subtituloPagina">Arrastra mesas para configurar tu restaurante</p>
      </div>

      {/* Barra de herramientas */}
      <div className="planoBarraHerramientas">
        <Boton tamano="sm" variante="primario" onClick={handleCrearMesa} disabled={!zonaActiva}>
          + Mesa
        </Boton>
        {zonaActiva && (
          <>
            <Boton tamano="sm" variante="fantasma" onClick={handleEditarZona}>
              Renombrar zona
            </Boton>
            <Boton tamano="sm" variante="peligro" onClick={handleEliminarZona}>
              Eliminar zona
            </Boton>
          </>
        )}
        <div style={{ marginLeft: 'auto', display: 'flex', gap: '0.5rem' }}>
          <Boton tamano="sm" variante="fantasma" onClick={handleExportar}>
            Exportar
          </Boton>
          <Boton tamano="sm" variante="fantasma" onClick={handleImportar}>
            Importar
          </Boton>
        </div>
      </div>

      {/* Zonas (tabs) */}
      <div className="planoZonas">
        {plano?.zonas.map((z) => (
          <Boton
            key={z.id}
            variante="fantasma"
            tamano="sm"
            className={`planoZonaTab ${z.id === zonaActiva ? 'activa' : ''}`}
            onClick={() => cambiarZona(z.id)}
          >
            {z.nombre} ({z.mesas.length})
          </Boton>
        ))}
        <Boton
          variante="fantasma"
          tamano="sm"
          className="planoZonaTab planoZonaTabNueva"
          onClick={handleCrearZona}
        >
          + Zona
        </Boton>
      </div>

      {/* Info de zona */}
      {zonaData && (
        <div className="planoZonaInfo">
          <span>
            {zonaData.nombre} &mdash; {mesasZona.length} mesas &mdash; {zonaData.ancho}&times;{zonaData.alto}px
          </span>
        </div>
      )}

      {/* Canvas con mesas */}
      {zonaData ? (
        <DndContext sensors={sensors} onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
          <div
            ref={canvasRef}
            className="planoCanvas"
            style={{ width: zonaData.ancho, height: zonaData.alto }}
            onClick={() => setMesaSeleccionada(null)}
          >
            {mesasZona.map((mesa) => {
              const pos = posicionesLocales[mesa.id];
              const mesaConPos = pos ? { ...mesa, pos_x: pos.x, pos_y: pos.y } : mesa;
              return (
                <MesaDraggable
                  key={mesa.id}
                  mesa={mesaConPos}
                  seleccionada={mesaSeleccionada?.id === mesa.id}
                  arrastrando={arrastrando === mesa.id}
                  onClick={() => setMesaSeleccionada(mesa)}
                />
              );
            })}
          </div>
        </DndContext>
      ) : (
        <div className="planoCanvas planoCanvasVacio">
          {plano && plano.zonas.length === 0
            ? 'Crea tu primera zona para empezar a diseñar el plano'
            : 'Selecciona una zona'}
        </div>
      )}

      {/* Panel de configuración de mesa seleccionada */}
      {mesaSeleccionada && (
        <PanelConfigMesa
          mesa={mesaSeleccionada}
          onGuardar={handleGuardarMesa}
          onEliminar={handleEliminarMesa}
          onCerrar={() => setMesaSeleccionada(null)}
        />
      )}

      {/* Combinaciones */}
      {plano && plano.combinaciones.length > 0 && (
        <div className="planoCombinaciones">
          <h3>Combinaciones de mesas</h3>
          {plano.combinaciones.map((c) => (
            <div key={c.id} className="planoCombinacionItem">
              <div>
                <strong>{c.nombre}</strong>
                <div className="planoCombinacionMesas">
                  {c.min_personas}-{c.max_personas} personas &middot;{' '}
                  {c.mesas.map((m) => `Mesa ${m.numero}`).join(', ')}
                </div>
              </div>
              <Boton tamano="sm" variante="peligro" onClick={() => handleEliminarCombinacion(c.id)}>
                &times;
              </Boton>
            </div>
          ))}
        </div>
      )}
      <div style={{ marginTop: '1rem' }}>
        <Boton tamano="sm" variante="fantasma" onClick={handleCrearCombinacion}>
          + Combinación de mesas
        </Boton>
      </div>
    </div>
  );
}

export default PlanoSala;
