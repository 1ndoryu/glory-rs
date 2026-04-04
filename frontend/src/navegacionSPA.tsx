/* [044A-1] Wrapper de navegacion sobre React Router.
 * Reemplaza el motor SPA basado en fetch/islands de WordPress.
 * Exporta navegar() con la misma API para compatibilidad con componentes existentes.
 * Los componentes que importan navegar() siguen funcionando sin cambios. */

let navigateRef: ((to: string) => void) | null = null;

/**
 * Registra la funcion navigate de React Router.
 * Se llama una vez desde App.tsx al montar el router.
 */
export function registrarNavigate(fn: (to: string) => void): void {
    navigateRef = fn;
}

/**
 * Navegacion programatica compatible con la API anterior.
 * Usa React Router internamente en lugar de fetch + parse HTML.
 */
export function navegar(url: string): void {
    if (navigateRef) {
        navigateRef(url);
    } else {
        window.location.href = url;
    }
}
