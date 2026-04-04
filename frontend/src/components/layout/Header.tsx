/**
 * Componente: Header
 * Cabecera global del sitio.
 * Enlaces centralizados en data/navegacion.ts (DRY).
 * Incluye submenú dropdown para "Soluciones".
 * [044A-13] Sesión conectada con authStore (Zustand + JWT).
 * Accesibilidad: aria-labels, aria-expanded, navegación por teclado.
 */
import React, {useState, useRef, useCallback, useEffect} from 'react';
import {useTranslation} from 'react-i18next';
import {ChevronDown, ChevronRight, Menu, X} from 'lucide-react';
import {Button} from '../ui/Button';
import {ENLACES_HEADER} from '../../data/navegacion';
import {ModalAutenticacion} from './ModalAutenticacion';
import {navegar} from '../../navegacionSPA';
import {Logo} from '../ui/Logo';
import {LanguageSelector} from '../ui/LanguageSelector';
import {useAuthStore} from '../../stores/authStore';
import '../../styles/header.css';

/* [044A-2] Mapeo de labels estáticos (español) a claves i18n.
 * navegacion.ts mantiene los labels originales como keys de referencia. */
const NAV_KEYS: Record<string, string> = {
    'Inicio': 'nav.home',
    'Servicios': 'nav.services',
    'Proyectos': 'nav.projects',
    'Nosotros': 'nav.about',
    'Blog': 'nav.blog',
    'Soluciones': 'nav.solutions',
    'Hosting': 'nav.hosting',
};

/* [044A-13] Sesión conectada con authStore (Zustand + JWT) */

/* Comprueba si la ruta actual coincide con un path dado */
function esRutaActual(path: string): boolean {
    if (typeof window === 'undefined') return false;
    return window.location.pathname.replace(/\/+$/, '') === path.replace(/\/+$/, '');
}

export const Header: React.FC = () => {
    const {t} = useTranslation();
    const [dropdownAbierto, setDropdownAbierto] = useState<string | null>(null);
    const [modalAbierto, setModalAbierto] = useState(false);
    const [menuMovilAbierto, setMenuMovilAbierto] = useState(false);
    const logueado = useAuthStore(s => s.logueado);
    const logout = useAuthStore(s => s.logout);
    const enPanel = esRutaActual('/panel');
    const dropdownRef = useRef<HTMLDivElement>(null);

    /* Texto y destino del botón de sesión / panel */
    const textoAccion = logueado ? (enPanel ? t('nav.back') : t('nav.panel')) : null;
    const hrefAccion = logueado ? (enPanel ? '/' : '/panel/') : null;

    /* Botón secundario: Chat (logueado) o Contacto */
    const textoCta = logueado ? t('nav.chat') : t('nav.contact');
    const hrefCta = '/contacto/';

    /* Cierra dropdown al hacer Escape */
    const handleKeyDownDropdown = useCallback(
        (e: React.KeyboardEvent, label: string) => {
            if (e.key === 'Escape') {
                setDropdownAbierto(null);
            } else if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                setDropdownAbierto(prev => (prev === label ? null : label));
            } else if (e.key === 'ArrowDown' && dropdownAbierto === label) {
                e.preventDefault();
                const submenu = dropdownRef.current?.querySelector('.subMenuEnlace') as HTMLElement | null;
                submenu?.focus();
            }
        },
        [dropdownAbierto]
    );

    /* Navegación por teclado dentro del submenú */
    const handleKeyDownSubmenu = useCallback((e: React.KeyboardEvent) => {
        const target = e.currentTarget as HTMLElement;
        if (e.key === 'Escape') {
            setDropdownAbierto(null);
            (target.closest('.enlaceNavegacionWrapper')?.querySelector('.enlaceNavegacion') as HTMLElement)?.focus();
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            (target.nextElementSibling as HTMLElement)?.focus();
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            (target.previousElementSibling as HTMLElement)?.focus();
        }
    }, []);

    /* Cerrar menú móvil con Escape */
    useEffect(() => {
        if (!menuMovilAbierto) return;
        const handleEsc = (e: KeyboardEvent) => {
            if (e.key === 'Escape') setMenuMovilAbierto(false);
        };
        document.addEventListener('keydown', handleEsc);
        return () => document.removeEventListener('keydown', handleEsc);
    }, [menuMovilAbierto]);

    return (
        <>
            <header className="cabeceraPrincipal" role="banner">
                <div className="logoContenedor">
                    <a href="/" className="logoEnlace" aria-label={t('accessibility.logo_home')}>
                        <Logo className="logoSvg" />
                    </a>
                </div>

                {/* Botón hamburguesa para móvil */}
                <button className="botonMenuMovil" onClick={() => setMenuMovilAbierto(!menuMovilAbierto)} aria-expanded={menuMovilAbierto} aria-controls="navegacion-principal" aria-label={menuMovilAbierto ? t('accessibility.close_menu') : t('accessibility.open_menu')}>
                    {menuMovilAbierto ? <X size={24} /> : <Menu size={24} />}
                </button>

                <nav className={`navegacionPrincipal ${menuMovilAbierto ? 'navegacionAbierta' : ''}`} id="navegacion-principal" aria-label={t('accessibility.main_nav')}>
                    {ENLACES_HEADER.map(link => (
                        <div key={link.label} className="enlaceNavegacionWrapper" ref={link.hasDropdown ? dropdownRef : undefined} onMouseEnter={() => (link.hasDropdown ? setDropdownAbierto(link.label) : null)} onMouseLeave={() => setDropdownAbierto(null)}>
                            <a href={link.href} className="enlaceNavegacion" aria-expanded={link.hasDropdown ? dropdownAbierto === link.label : undefined} aria-haspopup={link.hasDropdown ? 'true' : undefined} onKeyDown={link.hasDropdown ? e => handleKeyDownDropdown(e, link.label) : undefined}>
                                {t(NAV_KEYS[link.label] || link.label)}
                                {link.hasDropdown && <ChevronDown size={14} className="iconoDesplegable" aria-hidden="true" />}
                            </a>
                            {link.hasDropdown && link.subEnlaces && dropdownAbierto === link.label && (
                                <div className="subMenuDesplegable" role="menu" aria-label={`${t('accessibility.main_nav')}: ${t(NAV_KEYS[link.label] || link.label)}`}>
                                    {link.subEnlaces.map(sub => (
                                        <a key={sub.label} href={sub.href} className="subMenuEnlace" role="menuitem" tabIndex={0} onKeyDown={handleKeyDownSubmenu}>
                                            {t(NAV_KEYS[sub.label] || sub.label)}
                                        </a>
                                    ))}
                                </div>
                            )}
                        </div>
                    ))}
                </nav>

                <div className="accionCabecera" role="group" aria-label={t('accessibility.user_actions')}>
                    <LanguageSelector />
                    {logueado ? (
                        <>
                            <a className="enlaceAcceder" href={hrefAccion!}>
                                {textoAccion}
                            </a>
                            <button className="enlaceAcceder" onClick={logout}>
                                {t('nav.logout')}
                            </button>
                        </>
                    ) : (
                        <button className="enlaceAcceder" onClick={() => setModalAbierto(true)}>
                            {t('nav.login')}
                        </button>
                    )}
                    <Button variante="primario" tamano="pequeno" className="botonHeader" onClick={() => navegar(hrefCta)}>
                        {textoCta}
                        <ChevronRight size={14} strokeWidth={3} aria-hidden="true" />
                    </Button>
                </div>
            </header>

            {!logueado && <ModalAutenticacion abierto={modalAbierto} onCerrar={() => setModalAbierto(false)} />}
        </>
    );
};
