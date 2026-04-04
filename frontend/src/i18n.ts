/* [044A-2] Configuración de i18next con detección de idioma del navegador.
 * Idiomas soportados: es (español), en (inglés), ja (japonés).
 * Fallback: español. */
import i18n from 'i18next';
import {initReactI18next} from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import es from './locales/es.json';
import en from './locales/en.json';
import ja from './locales/ja.json';

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

export default i18n;
