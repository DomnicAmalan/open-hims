import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  clean: true,
  external: [
    'react',
    'react-dom',
    'react-router-dom',
    'react-redux',
    '@mantine/core',
    '@mantine/hooks',
    '@mantine/form',
    '@mantine/dates',
    '@mantine/notifications',
    '@mantine/modals',
  ],
  treeshake: true,
  splitting: false,
  sourcemap: true,
  minify: false,
});