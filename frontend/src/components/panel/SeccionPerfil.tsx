/**
 * Componente: SeccionPerfil
 * Formulario de configuracion de perfil del usuario.
 * La logica de estado se delega al hook usePerfil (SRP).
 * TO-DO: Conectar con REST API backend para persistir cambios.
 */
import React from 'react';
import {useTranslation} from 'react-i18next';
import {Button} from '../ui/Button';
import {usePerfil} from '../../hooks/usePerfil';
import './SeccionPerfil.css';

export const SeccionPerfil: React.FC = () => {
    const {t} = useTranslation();
    const {estado, guardado, actualizarCampo, handleGuardar, usuario} = usePerfil();

    return (
        <div className="perfilSeccion">
            <p className="perfilDescripcionIntro">
                {t('panel.profile_intro')}
            </p>

            <form className="perfilFormulario" onSubmit={handleGuardar}>
                {/* Avatar */}
                <div className="perfilAvatarSeccion">
                    <div className="perfilAvatar">
                        <img
                            src={usuario?.avatar || 'https://i.pravatar.cc/100?u=default'}
                            alt={t('accessibility.profile_photo')}
                        />
                    </div>
                    <div className="perfilAvatarAcciones">
                        <Button variante="outline" tamano="pequeno">{t('panel.change_photo')}</Button>
                        <span className="perfilAvatarNota">{t('panel.photo_help')}</span>
                    </div>
                </div>

                <div className="perfilFormGrid">
                    <div className="perfilCampo">
                        <label htmlFor="perfilNombre" className="perfilCampoEtiqueta">{t('panel.display_name')}</label>
                        <input
                            type="text"
                            id="perfilNombre"
                            value={estado.nombre}
                            onChange={(e) => actualizarCampo('nombre', e.target.value)}
                            placeholder={t('panel.display_name_placeholder')}
                            className="perfilCampoInput"
                        />
                    </div>
                    <div className="perfilCampo">
                        <label htmlFor="perfilEmail" className="perfilCampoEtiqueta">{t('panel.email')}</label>
                        <input
                            type="email"
                            id="perfilEmail"
                            value={usuario?.email || ''}
                            className="perfilCampoInput"
                            disabled
                        />
                    </div>
                </div>

                <div className="perfilCampo">
                    <label htmlFor="perfilDescripcion" className="perfilCampoEtiqueta">{t('panel.bio_label')}</label>
                    <textarea
                        id="perfilDescripcion"
                        value={estado.descripcion}
                        onChange={(e) => actualizarCampo('descripcion', e.target.value)}
                        placeholder={t('panel.bio_placeholder')}
                        className="perfilCampoTextarea"
                        rows={4}
                    />
                </div>

                <h3 className="perfilSubseccionTitulo">{t('panel.social_title')}</h3>
                <div className="perfilFormGrid">
                    <div className="perfilCampo">
                        <label htmlFor="perfilLinkedin" className="perfilCampoEtiqueta">LinkedIn</label>
                        <input
                            type="url"
                            id="perfilLinkedin"
                            value={estado.linkedin}
                            onChange={(e) => actualizarCampo('linkedin', e.target.value)}
                            placeholder="https://linkedin.com/in/tu-perfil"
                            className="perfilCampoInput"
                        />
                    </div>
                    <div className="perfilCampo">
                        <label htmlFor="perfilTwitter" className="perfilCampoEtiqueta">Twitter / X</label>
                        <input
                            type="url"
                            id="perfilTwitter"
                            value={estado.twitter}
                            onChange={(e) => actualizarCampo('twitter', e.target.value)}
                            placeholder="https://x.com/tu-usuario"
                            className="perfilCampoInput"
                        />
                    </div>
                    <div className="perfilCampo">
                        <label htmlFor="perfilWebsite" className="perfilCampoEtiqueta">{t('panel.website')}</label>
                        <input
                            type="url"
                            id="perfilWebsite"
                            value={estado.website}
                            onChange={(e) => actualizarCampo('website', e.target.value)}
                            placeholder="https://tu-sitio.com"
                            className="perfilCampoInput"
                        />
                    </div>
                </div>

                <div className="perfilFormAcciones">
                    <Button variante="primario" tamano="mediano">{t('panel.save')}</Button>
                    {guardado && <span className="perfilGuardadoExito">{t('panel.saved_success')}</span>}
                </div>
            </form>
        </div>
    );
};
