/**
 * Componente: Header
 * Cabecera global del sitio.
 * Enlaces centralizados en data/navegacion.ts (DRY).
 * Incluye submenú dropdown para "Soluciones".
 * [044A-13] Sesión conectada con authStore (Zustand + JWT).
 * Accesibilidad: aria-labels, aria-expanded, navegación por teclado.
 * [054A-19] Lógica extraída a useHeader (SRP). Links internos usan GloryLink.
 */
import React from 'react';
import {useTranslation} from 'react-i18next';
import {ChevronDown, ChevronRight, Menu, X} from 'lucide-react';
import {Button} from '../ui/Button';
import {ENLACES_HEADER} from '../../data/navegacion';
import {ModalAutenticacion} from './ModalAutenticacion';
import {navegar} from '../../navegacionSPA';
import {GloryLink} from '../../core/router';
import {Logo} from '../ui/Logo';
import {useHeader} from '../../hooks/useHeader';
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

export const Header: React.FC = () => {
    const {t} = useTranslation();
    const {
        dropdownAbierto, setDropdownAbierto,
        modalAbierto, setModalAbierto,
        menuMovilAbierto, setMenuMovilAbierto,
        logueado, logout, enPanel,
        dropdownRef,
        handleKeyDownDropdown,
        handleKeyDownSubmenu,
    } = useHeader();

    /* Texto y destino del botón de sesión / panel */
    const textoAccion = logueado ? (enPanel ? t('nav.back') : t('nav.panel')) : null;
    const hrefAccion = logueado ? (enPanel ? '/' : '/panel/') : null;

    /* Botón secundario: Chat (logueado) o Contacto */
    const textoCta = logueado ? t('nav.chat') : t('nav.contact');
    const hrefCta = '/contacto/';

    return (
        <>
            <header className="cabeceraPrincipal" role="banner">
                <div className="logoContenedor">
                    <GloryLink to="/" className="logoEnlace" aria-label={t('accessibility.logo_home')}>
                        <Logo className="logoSvg" />
                    </GloryLink>
                </div>

                {/* Botón hamburguesa para móvil */}
                <Button variante="texto" className="botonMenuMovil" onClick={() => setMenuMovilAbierto(!menuMovilAbierto)} aria-expanded={menuMovilAbierto} aria-controls="navegacion-principal" aria-label={menuMovilAbierto ? t('accessibility.close_menu') : t('accessibility.open_menu')}>
                    {menuMovilAbierto ? <X size={24} /> : <Menu size={24} />}
                </Button>

                <nav className={`navegacionPrincipal ${menuMovilAbierto ? 'navegacionAbierta' : ''}`} id="navegacion-principal" aria-label={t('accessibility.main_nav')}>
                    {ENLACES_HEADER.map(link => (
                        <div key={link.label} className="enlaceNavegacionWrapper" ref={link.hasDropdown ? dropdownRef : undefined} onMouseEnter={() => (link.hasDropdown ? setDropdownAbierto(link.label) : null)} onMouseLeave={() => setDropdownAbierto(null)}>
                            <GloryLink to={link.href} className="enlaceNavegacion" aria-expanded={link.hasDropdown ? dropdownAbierto === link.label : undefined} aria-haspopup={link.hasDropdown ? 'true' : undefined} onKeyDown={link.hasDropdown ? e => handleKeyDownDropdown(e, link.label) : undefined}>
                                {t(NAV_KEYS[link.label] || link.label)}
                                {link.hasDropdown && <ChevronDown size={14} className="iconoDesplegable" aria-hidden="true" />}
                            </GloryLink>
                            {link.hasDropdown && link.subEnlaces && dropdownAbierto === link.label && (
                                <div className="subMenuDesplegable" role="menu" aria-label={`${t('accessibility.main_nav')}: ${t(NAV_KEYS[link.label] || link.label)}`}>
                                    {link.subEnlaces.map(sub => (
                                        <GloryLink key={sub.label} to={sub.href} className="subMenuEnlace" role="menuitem" tabIndex={0} onKeyDown={handleKeyDownSubmenu}>
                                            {t(NAV_KEYS[sub.label] || sub.label)}
                                        </GloryLink>
                                    ))}
                                </div>
                            )}
                        </div>
                    ))}
                </nav>

                <div className="accionCabecera" role="group" aria-label={t('accessibility.user_actions')}>
                    {/* [044A-37] LanguageSelector removido del header por petición del usuario. Se mantiene en footer. */}
                    {logueado ? (
                        <>
                            <GloryLink to={hrefAccion!} className="enlaceAcceder">
                                {textoAccion}
                            </GloryLink>
                            <Button variante="texto" className="enlaceAcceder" onClick={logout}>
                                {t('nav.logout')}
                            </Button>
                        </>
                    ) : (
                        <Button variante="texto" className="enlaceAcceder" onClick={() => setModalAbierto(true)}>
                            {t('nav.login')}
                        </Button>
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
