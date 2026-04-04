/**
 * Componente: BarraFiltros
 * Barra de búsqueda y filtrado para la página de servicios.
 * Tipo FiltroCategoria centralizado en types/navegacion.ts (DRY).
 */
import React from 'react';
import {useTranslation} from 'react-i18next';
import {Search} from 'lucide-react';
import {FiltroCategoria} from '../../types/navegacion';
import './BarraFiltros.css';

interface BarraFiltrosProps {
    categorias: FiltroCategoria[];
    categoriaActiva: string;
    busqueda: string;
    onCategoriaChange: (id: string) => void;
    onBusquedaChange: (valor: string) => void;
}

export const BarraFiltros: React.FC<BarraFiltrosProps> = ({categorias, categoriaActiva, busqueda, onCategoriaChange, onBusquedaChange}) => {
    const {t} = useTranslation();

    /* [044A-2] Mapeo de labels de categoría a claves i18n */
    const CAT_KEYS: Record<string, string> = {
        'Todos': 'categories.all',
        'Diseño Web': 'categories.web',
        'Software': 'categories.software',
        'Inteligencia Artificial': 'categories.ai',
        'Branding': 'categories.branding',
        'Web': 'categories.web',
        'Aplicaciones': 'categories.software',
        'IA': 'categories.ai',
    };

    return (
        <div className="barraFiltros">
            <div className="filtrosCategorias">
                {categorias.map(cat => (
                    <button key={cat.id} className={`filtroBoton ${categoriaActiva === cat.id ? 'activo' : ''}`} onClick={() => onCategoriaChange(cat.id)}>
                        {t(CAT_KEYS[cat.label] || cat.label)}
                    </button>
                ))}
            </div>
            <div className="filtroBusqueda">
                <Search className="busquedaIcono" />
                <input type="text" className="busquedaInput" placeholder={t('services_page.search_placeholder')} value={busqueda} onChange={e => onBusquedaChange(e.target.value)} />
            </div>
        </div>
    );
};
