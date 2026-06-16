/* [174A-x] Plugin Vite: genera SVGs placeholder de color para imágenes faltantes
 * en /legacy-assets/colors/. Cuando el dir colors/ está vacío (dev local sin WP),
 * intercepta requests a *.jpg y devuelve un SVG con gradiente determinista.
 * Así la UI de tarjetas de sample se ve correctamente incluso sin las imágenes reales. */

import type { Plugin } from 'vite';
import { existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const COLORS_DIR = '/legacy-assets/colors/';

/* Genera un color HSL determinista a partir de un hash MD5 */
function colorDesdeHash(hash: string, saturación = 55, luminosidad = 35): string {
    const hue = (parseInt(hash.slice(0, 6), 16) % 360);
    return `hsl(${hue}, ${saturación}%, ${luminosidad}%)`;
}

/* Genera un segundo color complementario para el gradiente */
function colorComplementario(hash: string): string {
    const hue = (parseInt(hash.slice(0, 6), 16) + 180) % 360;
    return `hsl(${hue}, 50%, 30%)`;
}

function generarSvgPlaceholder(fileName: string): string {
    const hash = createHash('md5').update(fileName).digest('hex');
    const c1 = colorDesdeHash(hash);
    const c2 = colorComplementario(hash);
    const c3 = colorDesdeHash(hash.slice(4), 45, 40);

    return `<svg xmlns="http://www.w3.org/2000/svg" width="400" height="400" viewBox="0 0 400 400">
  <defs>
    <linearGradient id="g" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="${c1}"/>
      <stop offset="50%" stop-color="${c2}"/>
      <stop offset="100%" stop-color="${c3}"/>
    </linearGradient>
  </defs>
  <rect width="400" height="400" fill="url(#g)"/>
</svg>`;
}

export function colorsPlaceholderPlugin(): Plugin {
    const publicColorsDir = resolve(__dirname, '../public/legacy-assets/colors/');

    return {
        name: 'colors-placeholder',

        configureServer(server) {
            /* Middleware personalizado: intercepta GET a /legacy-assets/colors/* */
            server.middlewares.use((req, res, next) => {
                const url = req.url ?? '';
                if (!req.method || req.method !== 'GET') return next();
                if (!url.startsWith(COLORS_DIR) || !url.endsWith('.jpg')) return next();

                const fileName = url.slice(COLORS_DIR.length);
                if (!fileName || fileName.length < 5) return next();

                /* Verificar si el archivo real existe en public */
                const filePath = resolve(publicColorsDir, fileName);
                if (existsSync(filePath)) return next();

                /* Generar SVG placeholder inline */
                const svg = generarSvgPlaceholder(fileName);
                res.writeHead(200, {
                    'Content-Type': 'image/svg+xml',
                    'Cache-Control': 'public, max-age=86400',
                    'X-Placeholder': 'colors-placeholder-plugin',
                });
                res.end(svg);
            });
        },
    };
}
