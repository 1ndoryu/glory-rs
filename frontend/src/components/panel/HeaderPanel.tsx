/**
 * Componente: HeaderPanel
 * Header minimalista exclusivo para el panel de usuario.
 * Solo muestra: logo, "Chat", "Inicio" (ir a landing), y foto de usuario con submenú.
 * [044A-38 Fase 1] Logout real conectado a authStore.
 * [074A-45] "Salir" → "Inicio" navega sin desloguear. Avatar abre submenú con logout.
 */
import React, {useCallback, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router-dom';
import {GloryLink} from '../../core/router';
import {useAuthStore} from '../../stores/authStore';
import {useCurrentProfile} from '../../hooks/useCurrentProfile';
import {Button} from '../ui/Button';
import {MenuContextual} from '../ui/ContextMenu';
import {Logo} from '../ui/Logo';
import NotificationBell from './NotificationBell';
import './HeaderPanel.css';

export const HeaderPanel: React.FC = () => {
    const {t} = useTranslation();
    const {avatarUrl} = useCurrentProfile();
    const logout = useAuthStore(s => s.logout);
    const navigate = useNavigate();
    const [menuAbierto, setMenuAbierto] = useState(false);

    const handleLogout = useCallback(() => {
        setMenuAbierto(false);
        logout();
        navigate('/');
    }, [logout, navigate]);

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
                    {/* [084A-11] Avatar → MenuContextual en vez de dropdown artesanal */}
                    <MenuContextual
                        abierto={menuAbierto}
                        onToggle={() => setMenuAbierto(prev => !prev)}
                        onCerrar={() => setMenuAbierto(false)}
                        ariaLabel={t('accessibility.profile_photo')}
                        triggerContent={<img src={avatarUrl} alt={t('accessibility.profile_photo')} className="headerPanelAvatarImg" />}
                        triggerClassName="headerPanelAvatar"
                        triggerVariante="texto"
                        triggerTamano="pequeno"
                        panelClassName="headerPanelSubmenu"
                        items={[{
                            id: 'logout',
                            label: t('nav.logout'),
                            onSelect: handleLogout,
                        }]}
                    />
                </div>
            </div>
        </header>
    );
};
