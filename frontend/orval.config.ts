import { defineConfig } from 'orval';

export default defineConfig({
  glory: {
    input: {
      /* [174A-29] Consumir el schema versionado evita depender de un backend vivo
       * al regenerar el cliente. El contrato se actualiza con `cargo run -- --emit-openapi`. */
      target: '../openapi.json',
    },
    output: {
      target: './src/api/generated.ts',
      client: 'react-query',
      mode: 'single',
      override: {
        mutator: {
          path: './src/api/axios-instance.ts',
          name: 'customInstance',
        },
      },
    },
  },
});
