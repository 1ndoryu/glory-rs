/**
 * Componente: HeaderPanel
 * Header minimalista exclusivo para el panel de usuario.
 * Solo muestra: logo, "Chat", "Salir" (estilo enlaceNavegacionWrapper), y foto de usuario.
 * [044A-38 Fase 1] Logout real conectado a authStore.
 */
import React, {useCallback} from 'react';
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

    const handleLogout = useCallback(() => {
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

                {/* Acciones: Chat, Salir, Avatar */}
                <div className="headerPanelAcciones">
                    <NotificationBell />
                    <GloryLink to="/contacto/" className="headerPanelEnlace">
                        {t('nav.chat')}
                    </GloryLink>
                    <Button onClick={handleLogout} className="headerPanelEnlace headerPanelBtnLogout" type="button" variante="texto">
                        {t('nav.logout')}
                    </Button>
                    <div className="headerPanelAvatar">
                        <img src={avatarUrl} alt={t('accessibility.profile_photo')} className="headerPanelAvatarImg" />
                    </div>
                </div>
            </div>
        </header>
    );
};
