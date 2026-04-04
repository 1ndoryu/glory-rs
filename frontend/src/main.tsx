/* [044A-1] Entry point - importa estilos globales antes de montar el app */
/* [044A-2] Inicializa i18n antes del render para que esté disponible globalmente */
/* [044A-13] Restaura sesión JWT desde localStorage al iniciar */
/* [044A-28] HelmetProvider para gestión dinámica de meta tags SEO */
import React from 'react';
import ReactDOM from 'react-dom/client';
import {HelmetProvider} from 'react-helmet-async';
import './i18n';
import './styles/variables.css';
import './styles/init.css';
import App from './App';
import {useAuthStore} from './stores/authStore';

useAuthStore.getState().inicializar();

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <HelmetProvider>
      <App />
    </HelmetProvider>
  </React.StrictMode>,
);
