/* [044A-2] Selector de idioma compacto para el Header.
 * Muestra código del idioma actual y dropdown con opciones. */
import {useState, useRef, useEffect} from 'react';
import {useTranslation} from 'react-i18next';
import './LanguageSelector.css';

const LANGUAGES = [
    {code: 'es', label: 'ES'},
    {code: 'en', label: 'EN'},
    {code: 'ja', label: 'JA'}
];

export const LanguageSelector = () => {
    const {i18n} = useTranslation();
    const [open, setOpen] = useState(false);
    const ref = useRef<HTMLDivElement>(null);

    /* Cerrar al hacer click fuera */
    useEffect(() => {
        const handler = (e: MouseEvent) => {
            if (ref.current && !ref.current.contains(e.target as Node)) {
                setOpen(false);
            }
        };
        document.addEventListener('mousedown', handler);
        return () => document.removeEventListener('mousedown', handler);
    }, []);

    const currentLang = LANGUAGES.find(l => l.code === i18n.language) || LANGUAGES[0];

    const cambiarIdioma = (code: string) => {
        i18n.changeLanguage(code);
        setOpen(false);
    };

    return (
        <div className="selectorIdioma" ref={ref}>
            <button
                className="selectorIdiomaBoton"
                onClick={() => setOpen(!open)}
                aria-label="Change language"
            >
                {currentLang.label}
            </button>
            {open && (
                <ul className="selectorIdiomaLista">
                    {LANGUAGES.map(lang => (
                        <li key={lang.code}>
                            <button
                                className={`selectorIdiomaOpcion ${lang.code === i18n.language ? 'activo' : ''}`}
                                onClick={() => cambiarIdioma(lang.code)}
                            >
                                {lang.label}
                            </button>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
};
