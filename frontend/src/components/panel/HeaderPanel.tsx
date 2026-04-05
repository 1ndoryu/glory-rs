/**
 * Componente: HeaderPanel
 * Header minimalista exclusivo para el panel de usuario.
 * Solo muestra: logo, "Chat", "Salir" (estilo enlaceNavegacionWrapper), y foto de usuario.
 * [044A-38 Fase 1] Logout real conectado a authStore.
 */
import React, {useCallback} from 'react';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router-dom';
import {obtenerUsuarioActual} from '../../data/panel';
import {useAuthStore} from '../../stores/authStore';
import {Logo} from '../ui/Logo';
import './HeaderPanel.css';

export const HeaderPanel: React.FC = () => {
    const {t} = useTranslation();
    const usuario = obtenerUsuarioActual();
    const avatarUrl = usuario?.avatar || 'https://i.pravatar.cc/40?u=default';
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
                <a href="/" className="headerPanelLogo" aria-label={t('accessibility.logo_home')}>
                    <Logo className="headerPanelLogoSvg" />
                </a>

                {/* Acciones: Chat, Salir, Avatar */}
                <div className="headerPanelAcciones">
                    <a href="/contacto/" className="headerPanelEnlace">
                        {t('nav.chat')}
                    </a>
                    <button onClick={handleLogout} className="headerPanelEnlace headerPanelBtnLogout" type="button">
                        {t('nav.logout')}
                    </button>
                    <div className="headerPanelAvatar">
                        <img src={avatarUrl} alt={t('accessibility.profile_photo')} className="headerPanelAvatarImg" />
                    </div>
                </div>
            </div>
        </header>
    );
};
