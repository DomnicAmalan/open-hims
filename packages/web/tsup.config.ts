import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  clean: true,
  external: [
    'react',
    'react-dom',
    '@mantine/core',
    '@mantine/hooks',
    '@mantine/form',
    '@mantine/dates',
    '@mantine/notifications',
    '@mantine/modals',
    '@mantine/spotlight',
    '@mantine/charts',
    '@mantine/code-highlight',
    '@mantine/dropzone',
    '@mantine/nprogress',
    '@tabler/icons-react',
    'dayjs',
    'clsx'
  ],
  treeshake: true,
  splitting: false,
  sourcemap: true,
  minify: false,
});