import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { fileURLToPath, URL } from 'node:url';

export default defineConfig({
  plugins: [react()],
  /* [204A-1] Aliases que replican exactamente los del frontend WordPress legacy.
   * Ver glorytemplate/App/React/tsconfig.json. */
  resolve: {
    alias: {
      '@app': fileURLToPath(new URL('./src/legacy', import.meta.url)),
      '@': fileURLToPath(new URL('./src/glory-core', import.meta.url)),
      '@mezclador': fileURLToPath(new URL('./src/mezclador', import.meta.url)),
      /* [204A-1] Stub modulos nativos (Tauri/Capacitor) para builds web.
       * El legacy importa dinamicamente estos modulos solo en desktop/movil; en web
       * Rollup necesita resolverlos a un stub para no fallar el build. */
      '@tauri-apps/api/app': fileURLToPath(new URL('./src/bootstrap/tauriShim.ts', import.meta.url)),
      '@tauri-apps/plugin-fs': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
      '@tauri-apps/plugin-shell': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
      '@tauri-apps/plugin-notification': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
      '@capacitor/filesystem': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
      '@capacitor/app': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
      '@capacitor/browser': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
      '@capacitor/core': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
      '@capacitor/push-notifications': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
      '@capacitor/share': fileURLToPath(new URL('./src/bootstrap/tauriPluginsShim.ts', import.meta.url)),
    },
  },
  server: {
    port: 5173,
    /* Proxy API requests al backend Rust en desarrollo */
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/swagger-ui': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/api-docs': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
    },
  },
});
