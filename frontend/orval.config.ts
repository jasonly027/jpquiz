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
          path: './src/api/mutator/custom-axios.ts',
          name: 'customInstance',
        },

        operations: {
          get_multi_choice: {
            query: {
              useQuery: false,
              useMutation: true,
            },
          },
        },
      },
    },
  },
});
