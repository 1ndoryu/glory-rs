/* [263A-14] Panel lateral para editar propiedades de una mesa seleccionada */

import { useState } from 'react';
import type { ActualizarMesaRequest, Mesa } from '../../api/generated';
import { Boton, Input, Select } from '@glory/componentes/ui';

interface PanelConfigMesaProps {
  mesa: Mesa;
  onGuardar: (id: string, data: ActualizarMesaRequest) => void;
  onEliminar: (id: string) => void;
  onCerrar: () => void;
}

function PanelConfigMesa({ mesa, onGuardar, onEliminar, onCerrar }: PanelConfigMesaProps) {
  const [form, setForm] = useState({
    numero: mesa.numero,
    minP: mesa.min_personas,
    maxP: mesa.max_personas,
    forma: mesa.forma,
    ancho: mesa.ancho,
    alto: mesa.alto,
    activa: mesa.activa,
  });

  const set = (campo: string, valor: number | string | boolean) =>
    setForm((prev) => ({ ...prev, [campo]: valor }));

  const guardar = () => {
    onGuardar(mesa.id, {
      numero: form.numero,
      min_personas: form.minP,
      max_personas: form.maxP,
      forma: form.forma,
      ancho: form.ancho,
      alto: form.alto,
      activa: form.activa,
    });
  };

  return (
    <div className="planoConfigPanel">
      <h3>Mesa {mesa.numero}</h3>
      <div className="planoConfigCampo">
        <label>Número</label>
        <Input
          type="number"
          tamano="sm"
          value={form.numero}
          onChange={(e) => set('numero', Number(e.target.value))}
        />
      </div>
      <div className="planoConfigCampo">
        <label>Mín personas</label>
        <Input
          type="number"
          tamano="sm"
          value={form.minP}
          onChange={(e) => set('minP', Number(e.target.value))}
        />
      </div>
      <div className="planoConfigCampo">
        <label>Máx personas</label>
        <Input
          type="number"
          tamano="sm"
          value={form.maxP}
          onChange={(e) => set('maxP', Number(e.target.value))}
        />
      </div>
      <div className="planoConfigCampo">
        <label>Forma</label>
        <Select
          tamano="sm"
          value={form.forma}
          onChange={(e) => set('forma', e.target.value)}
        >
          <option value="cuadrada">Cuadrada</option>
          <option value="redonda">Redonda</option>
          <option value="rectangular">Rectangular</option>
        </Select>
      </div>
      <div className="planoConfigCampo">
        <label>Ancho (px)</label>
        <Input
          type="number"
          tamano="sm"
          value={form.ancho}
          onChange={(e) => set('ancho', Number(e.target.value))}
        />
      </div>
      <div className="planoConfigCampo">
        <label>Alto (px)</label>
        <Input
          type="number"
          tamano="sm"
          value={form.alto}
          onChange={(e) => set('alto', Number(e.target.value))}
        />
      </div>
      <div className="planoConfigCampo">
        <label>
          <Input
            type="checkbox"
            checked={form.activa}
            onChange={(e) => set('activa', e.target.checked)}
          />{' '}
          Activa
        </label>
      </div>
      <div className="planoConfigAcciones">
        <Boton tamano="sm" variante="primario" onClick={guardar}>
          Guardar
        </Boton>
        <Boton tamano="sm" variante="peligro" onClick={() => onEliminar(mesa.id)}>
          Eliminar
        </Boton>
        <Boton tamano="sm" variante="fantasma" onClick={onCerrar}>
          Cerrar
        </Boton>
      </div>
    </div>
  );
}

export default PanelConfigMesa;
