/* [044A-1] Entry point - importa estilos globales antes de montar el app */
import React from 'react';
import ReactDOM from 'react-dom/client';
import './styles/variables.css';
import './styles/init.css';
import App from './App';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
