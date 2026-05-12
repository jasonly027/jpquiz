import babelPlugin from '@rolldown/plugin-babel';
import tailwindcss from '@tailwindcss/vite';
import { tanstackStart } from '@tanstack/react-start/plugin/vite';
import viteReact, { reactCompilerPreset } from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    port: 5173,
  },
  resolve: {
    tsconfigPaths: true,
  },
  plugins: [
    tanstackStart({
      importProtection: {
        behavior: 'error',
      },
      prerender: {
        enabled: true,
        crawlLinks: true,
        retryCount: 3,
      },
    }),
    // React's Vite plugin must come after Start's Vite plugin
    viteReact(),
    babelPlugin({
      presets: [reactCompilerPreset()],
    }),
    tailwindcss(),
  ],
});
