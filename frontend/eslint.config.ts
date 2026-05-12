import { includeIgnoreFile } from '@eslint/config-helpers';
import js from '@eslint/js';
import pluginQuery from '@tanstack/eslint-plugin-query';
import pluginReact from 'eslint-plugin-react';
import reactCompiler from 'eslint-plugin-react-compiler';
import reactHooks from 'eslint-plugin-react-hooks';
import { defineConfig, globalIgnores } from 'eslint/config';
import globals from 'globals';
import { fileURLToPath } from 'node:url';
import tseslint from 'typescript-eslint';

const gitignorePath = fileURLToPath(new URL('.gitignore', import.meta.url));

export default defineConfig([
  includeIgnoreFile(gitignorePath, 'Imported .gitignore patterns'),
  globalIgnores(['./src/api/*']),
  {
    files: ['**/*.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    plugins: { js },
    extends: ['js/recommended'],
    languageOptions: { globals: globals.browser },
  },
  tseslint.configs.strict,
  {
    ...pluginReact.configs.flat['recommended'],
    settings: {
      react: {
        version: 'detect',
      },
    },
  },
  pluginReact.configs.flat['jsx-runtime']!,
  reactHooks.configs.flat.recommended,
  reactCompiler.configs.recommended,
  pluginQuery.configs['flat/recommended-strict'],
  {
    rules: {
      '@typescript-eslint/no-non-null-assertion': 'off',
    },
  },
]);
