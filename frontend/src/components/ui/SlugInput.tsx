/* [074A-6] SlugInput — Input que auto-genera slug desde un título.
 * Convierte a minúsculas, elimina acentos, reemplaza espacios por guiones.
 * Permite edición manual del slug si el usuario lo desea. */
import React, { useCallback, useEffect, useState } from 'react';
import './SlugInput.css';

interface SlugInputProps {
    titulo: string;
    valor: string;
    onChange: (slug: string) => void;
    className?: string;
}

/* Genera slug: minúsculas, sin acentos, sin caracteres especiales, guiones */
function generarSlug(texto: string): string {
    return texto
        .normalize('NFD')
        .replace(/[\u0300-\u036f]/g, '')
        .toLowerCase()
        .replace(/[^a-z0-9\s-]/g, '')
        .replace(/\s+/g, '-')
        .replace(/-+/g, '-')
        .replace(/^-|-$/g, '');
}

export const SlugInput: React.FC<SlugInputProps> = ({
    titulo,
    valor,
    onChange,
    className = '',
}) => {
    const [manual, setManual] = useState(false);

    /* Auto-genera slug cuando cambia el título (si no es manual) */
    useEffect(() => {
        if (!manual && titulo) {
            onChange(generarSlug(titulo));
        }
    }, [titulo, manual, onChange]);

    const handleChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
        setManual(true);
        onChange(generarSlug(e.target.value));
    }, [onChange]);

    const handleReset = useCallback(() => {
        setManual(false);
        onChange(generarSlug(titulo));
    }, [titulo, onChange]);

    return (
        <div className={`slugInput ${className}`}>
            <span className="slugInputEtiqueta">Slug</span>
            <div className="slugInputContenedor">
                <span className="slugInputPrefijo">/</span>
                <input
                    type="text"
                    className="slugInputCampo"
                    value={valor}
                    onChange={handleChange}
                    placeholder="slug-automatico"
                />
                {manual && (
                    <button
                        type="button"
                        className="slugInputReset"
                        onClick={handleReset}
                        title="Regenerar desde título"
                    >
                        ↻
                    </button>
                )}
            </div>
        </div>
    );
};
