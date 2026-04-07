/**
 * Componente: HeaderPanel
 * Header minimalista exclusivo para el panel de usuario.
 * Solo muestra: logo, "Chat", "Inicio" (ir a landing), y foto de usuario con submenú.
 * [044A-38 Fase 1] Logout real conectado a authStore.
 * [074A-45] "Salir" → "Inicio" navega sin desloguear. Avatar abre submenú con logout.
 */
import React, {useCallback, useEffect, useRef, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router-dom';
import {GloryLink} from '../../core/router';
import {useAuthStore} from '../../stores/authStore';
import {useCurrentProfile} from '../../hooks/useCurrentProfile';
import {Button} from '../ui/Button';
import {Logo} from '../ui/Logo';
import NotificationBell from './NotificationBell';
import './HeaderPanel.css';

export const HeaderPanel: React.FC = () => {
    const {t} = useTranslation();
    const {avatarUrl} = useCurrentProfile();
    const logout = useAuthStore(s => s.logout);
    const navigate = useNavigate();
    const [menuAbierto, setMenuAbierto] = useState(false);
    const menuRef = useRef<HTMLDivElement>(null);

    const handleLogout = useCallback(() => {
        setMenuAbierto(false);
        logout();
        navigate('/');
    }, [logout, navigate]);

    /* [074A-45] Cerrar submenú al hacer click fuera */
    useEffect(() => {
        if (!menuAbierto) return;
        const cerrar = (e: MouseEvent) => {
            if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
                setMenuAbierto(false);
            }
        };
        document.addEventListener('mousedown', cerrar);
        return () => document.removeEventListener('mousedown', cerrar);
    }, [menuAbierto]);

    return (
        <header className="headerPanel" role="banner">
            <div className="headerPanelContenedor">
                {/* Logo */}
                <GloryLink to="/" className="headerPanelLogo" aria-label={t('accessibility.logo_home')}>
                    <Logo className="headerPanelLogoSvg" />
                </GloryLink>

                {/* Acciones: Chat, Inicio, Avatar con submenú */}
                <div className="headerPanelAcciones">
                    <NotificationBell />
                    {/* [064A-5] Enlace de Chat abre la sección mensajes del panel */}
                    <Button variante="texto" className="headerPanelEnlace" type="button" onClick={() => window.dispatchEvent(new CustomEvent('panel-cambiar-tab', {detail: 'mensajes'}))}>
                        {t('nav.chat')}
                    </Button>
                    {/* [074A-45] Ir a inicio sin desloguear */}
                    <GloryLink to="/" className="headerPanelEnlace">
                        {t('nav.home', 'Inicio')}
                    </GloryLink>
                    {/* [074A-45] Avatar abre submenú con opción de cerrar sesión */}
                    <div className="headerPanelAvatarContenedor" ref={menuRef}>
                        <button
                            type="button"
                            className="headerPanelAvatar"
                            onClick={() => setMenuAbierto(prev => !prev)}
                            aria-expanded={menuAbierto}
                            aria-haspopup="true"
                        >
                            <img src={avatarUrl} alt={t('accessibility.profile_photo')} className="headerPanelAvatarImg" />
                        </button>
                        {menuAbierto && (
                            <div className="headerPanelSubmenu" role="menu">
                                <Button
                                    variante="texto"
                                    className="headerPanelSubmenuItem"
                                    type="button"
                                    onClick={handleLogout}
                                    role="menuitem"
                                >
                                    {t('nav.logout')}
                                </Button>
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </header>
    );
};
