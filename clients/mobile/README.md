# Kamples Mobile

Shell Capacitor para Android que **empaqueta la SPA Rust** del frontend (`frontend/dist`) en una APK nativa.

## Arquitectura

- `webDir = ../../frontend/dist` → la APK incluye el bundle Vite del SPA
- FCM, Google PKCE deep links y push notifications los maneja el SPA en `frontend/src/legacy/services/`
- Backend Rust se consume vía cliente Orval (apunta a `https://api.kamples.com` o env)
- En producción **no** hay `server.url` → todo corre offline-first desde el bundle local

## Scripts

- `npm run prepare:web`: construye `frontend/dist` (delega a `npm run build` del SPA)
- `npm run prepare:web -- --force`: fuerza rebuild aunque ya exista
- `npm run android:sync`: prepare:web + `npx cap sync android`
- `npm run android:open`: abre Android Studio sin resincronizar
- `npm run android:open:sync`: sincroniza y luego abre Android Studio
- `npm run android:run`: sincroniza y ejecuta en dispositivo/emulador

## Live reload contra vite dev server

1. Arranca el dev server SPA: `npm run dev --prefix frontend` (puerto 5173)
2. PowerShell: `$env:KAMPLES_CAP_SERVER_URL = 'http://10.0.2.2:5173'`
3. `npm run android:sync` → `npm run android:open`
4. La APK cargará el dev server (con HMR) en lugar del bundle local

Para device físico, reemplaza `10.0.2.2` por la IP LAN de la máquina.

## Flujo normal

1. `npm install --prefix clients/mobile`
2. `npm install --prefix frontend && npm run build --prefix frontend`
3. `npm run android:sync --prefix clients/mobile`
4. `npm run android:open --prefix clients/mobile`

## Permisos Android

- `INTERNET`: requerido para fetch al backend
- `POST_NOTIFICATIONS`: Android 13+ requiere prompt explícito (lo gestiona `fcmToken.ts`)

## Deep links

- `kamples://auth/...` → flujo Google PKCE (`googleAuthMobileCapacitor.ts`)
- `kamples://notification/...` → click en push FCM (`fcmToken.ts` lo persiste en sessionStorage)