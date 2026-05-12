import { defineConfig } from 'orval';

export default defineConfig({
  server: {
    input: '../backend/docs/openapi.json',
    output: {
      target: './src/api/server.ts',
      schemas: './src/api/model',
      client: 'react-query',

      formatter: 'prettier',

      httpClient: 'axios',
      override: {
        mutator: {
          path: './src/api/mutator/custom-instance.ts',
          name: 'customInstance',
        },
      },
    },
  },
});
