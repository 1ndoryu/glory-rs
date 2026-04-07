/* [044A-2] Configuración de i18next con detección de idioma del navegador.
 * Idiomas soportados: es (español), en (inglés), ja (japonés).
 * Fallback: español.
 * [064A-64] Traducciones de contenido de negocio registradas via addResourceBundle. */
import i18n from 'i18next';
import {initReactI18next} from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import es from './locales/es.json';
import en from './locales/en.json';
import ja from './locales/ja.json';
import {contentEs, contentEn, contentJa} from './i18n/contentTranslations';

i18n
    .use(LanguageDetector)
    .use(initReactI18next)
    .init({
        resources: {
            es: {translation: es},
            en: {translation: en},
            ja: {translation: ja}
        },
        fallbackLng: 'es',
        supportedLngs: ['es', 'en', 'ja'],
        interpolation: {
            escapeValue: true
        },
        detection: {
            order: ['localStorage', 'navigator'],
            caches: ['localStorage']
        }
    });

/* [064A-64] Registrar traducciones de contenido (servicios, equipo, planes, soluciones).
 * Se usa addResourceBundle con deep=true, overwrite=true para mergear con las traducciones UI existentes. */
i18n.addResourceBundle('es', 'translation', {content: contentEs}, true, true);
i18n.addResourceBundle('en', 'translation', {content: contentEn}, true, true);
i18n.addResourceBundle('ja', 'translation', {content: contentJa}, true, true);

export default i18n;
