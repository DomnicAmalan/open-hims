import { defineConfig } from 'tsup';

export default defineConfig({
  entry: {
    index: 'src/index.ts',
    web: 'src/web.ts',
    mobile: 'src/mobile.ts',
    tauri: 'src/tauri.ts',
  },
  format: ['cjs', 'esm'],
  dts: true,
  splitting: false,
  sourcemap: true,
  clean: true,
  external: [
    'react',
    'react-redux',
    'react-native',
    '@react-native-async-storage/async-storage',
    'expo-secure-store',
    '@tauri-apps/plugin-store',
    '@tauri-apps/api',
  ],
  target: 'es2020',
  minify: false,
});