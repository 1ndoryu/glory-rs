/* [263A-25] ThemeProvider — migrado a next-themes para dark mode correcto en Vite SPA.
 * La implementación custom anterior no funcionaba correctamente en producción.
 * next-themes gestiona la clase .dark en documentElement con timing correcto,
 * sin parpadeos y con soporte nativo de sistema. storageKey preserva preferencia anterior.
 * useTheme re-exportado para compatibilidad con theme-toggle.tsx. */

import { ThemeProvider as NextThemesProvider } from 'next-themes';
import type { ReactNode } from 'react';

export function ThemeProvider({ children }: { children: ReactNode }) {
  return (
    <NextThemesProvider
      attribute="class"
      defaultTheme="system"
      enableSystem
      storageKey="glory-theme"
    >
      {children}
    </NextThemesProvider>
  );
}

export { useTheme } from 'next-themes';
