import { defineConfig } from 'orval';

export default defineConfig({
  glory: {
    input: {
      /* En desarrollo: schema local generado con dump_openapi.
       * En producción: target: 'http://localhost:3000/api-docs/openapi.json' */
      target: './openapi-debug.json',
    },
    output: {
      target: './src/api/generated',
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
