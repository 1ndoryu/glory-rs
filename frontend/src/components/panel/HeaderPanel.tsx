/**
 * Componente: HeaderPanel
 * Header minimalista exclusivo para el panel de usuario.
 * Solo muestra: logo, "Chat", "Salir" (estilo enlaceNavegacionWrapper), y foto de usuario.
 * No incluye la navegacion principal del sitio.
 */
import React from 'react';
import {useTranslation} from 'react-i18next';
import {obtenerUsuarioActual} from '../../data/panel';
import {Logo} from '../ui/Logo';
import './HeaderPanel.css';

export const HeaderPanel: React.FC = () => {
    const {t} = useTranslation();
    const usuario = obtenerUsuarioActual();
    const avatarUrl = usuario?.avatar || 'https://i.pravatar.cc/40?u=default';

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
                    <a href="/" className="headerPanelEnlace">
                        {t('nav.logout')}
                    </a>
                    <div className="headerPanelAvatar">
                        <img src={avatarUrl} alt={t('accessibility.profile_photo')} className="headerPanelAvatarImg" />
                    </div>
                </div>
            </div>
        </header>
    );
};
