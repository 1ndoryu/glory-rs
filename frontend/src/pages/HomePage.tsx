/* [174A-104] HomePage demuestra el patrón Orval/React Query.
 *
 * useHealthCheck() está generado por Orval desde la spec OpenAPI:
 *   frontend/src/api/generated/health/health.ts
 *
 * Cualquier feature nueva debe seguir este patrón:
 *   1. Verificar que el endpoint esté anotado con #[utoipa::path] en Rust.
 *   2. `cargo run -- --emit-openapi` regenera `openapi.json`.
 *   3. `npm run codegen` regenera `frontend/src/api/generated/<tag>/`.
 *   4. Importar el hook `use<Operation>` y usarlo en el componente.
 *
 * NUNCA escribir tipos de API a mano — todo viene de Orval. */

import { useHealthCheck } from '../api/generated/health/health';

export default function HomePage() {
  const health = useHealthCheck({
    query: { staleTime: 30 * 1000, refetchOnWindowFocus: false },
  });

  return (
    <section className="pagina">
      <h1 className="titulo">Glory RS — Kamples</h1>
      <p className="descripcion">
        SPA placeholder. Las features se migrarán progresivamente desde
        <code> frontend/src/features/</code>.
      </p>
      <div className="estadoBackend">
        {health.isLoading && <p>Comprobando backend…</p>}
        {health.isError && <p className="error">Backend no disponible.</p>}
        {health.data && (
          <p>
            Backend OK — versión <code>{(health.data as { data?: { version?: string } }).data?.version ?? '?'}</code>
          </p>
        )}
      </div>
    </section>
  );
}
