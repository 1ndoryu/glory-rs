import { defineConfig } from 'orval';

export default defineConfig({
  glory: {
    input: {
      /* [174A-29] Consumir el schema versionado evita depender de un backend vivo
       * al regenerar el cliente. El contrato se actualiza con `cargo run -- --emit-openapi`. */
      target: '../openapi.json',
    },
    output: {
      /* [174A-100] `tags-split` genera un archivo por cada tag OpenAPI
       * (auth, samples, comments, ...) en lugar de un único bundle gigante.
       * Mejora tree-shaking, navegación en VS Code y reduce diffs en regen. */
      target: './src/api/generated',
      schemas: './src/api/generated/model',
      client: 'react-query',
      mode: 'tags-split',
      override: {
        mutator: {
          path: './src/api/axios-instance.ts',
          name: 'customInstance',
        },
      },
    },
  },
});
