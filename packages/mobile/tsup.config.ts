import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  clean: true,
  external: [
    'react',
    'react-native',
    'react-native-paper',
    'react-native-vector-icons',
    'react-native-safe-area-context',
    '@react-navigation/native',
    '@react-navigation/native-stack',
    '@react-navigation/bottom-tabs',
    'react-native-gesture-handler',
    'react-native-reanimated',
    'react-native-screens',
    'expo',
    'expo-constants',
    'expo-status-bar',
    'expo-font',
    'expo-asset',
    'expo-splash-screen',
    'expo-router'
  ],
  treeshake: true,
  splitting: false,
  sourcemap: true,
  minify: false,
});