import React from 'react';
import { PaperProvider, MD3LightTheme, MD3DarkTheme } from 'react-native-paper';
import { useColorScheme } from 'react-native';

// Healthcare-focused theme colors
const himsLightTheme = {
  ...MD3LightTheme,
  colors: {
    ...MD3LightTheme.colors,
    primary: '#0066cc',
    primaryContainer: '#e6f2ff',
    onPrimary: '#ffffff',
    onPrimaryContainer: '#001429',
    secondary: '#4caf50',
    secondaryContainer: '#e8f5e8',
    onSecondary: '#ffffff',
    onSecondaryContainer: '#1b5e20',
    error: '#f44336',
    errorContainer: '#ffebee',
    onError: '#ffffff',
    onErrorContainer: '#b71c1c',
    surface: '#ffffff',
    onSurface: '#1c1b1f',
    surfaceVariant: '#f5f5f5',
    onSurfaceVariant: '#49454f',
    outline: '#d1d5db',
    background: '#fafafa',
    onBackground: '#1c1b1f',
  },
};

const himsDarkTheme = {
  ...MD3DarkTheme,
  colors: {
    ...MD3DarkTheme.colors,
    primary: '#4da6ff',
    primaryContainer: '#003d7a',
    onPrimary: '#001429',
    onPrimaryContainer: '#e6f2ff',
    secondary: '#81c784',
    secondaryContainer: '#2e7d32',
    onSecondary: '#1b5e20',
    onSecondaryContainer: '#e8f5e8',
    error: '#ef5350',
    errorContainer: '#d32f2f',
    onError: '#b71c1c',
    onErrorContainer: '#ffebee',
    surface: '#1c1b1f',
    onSurface: '#e6e1e5',
    surfaceVariant: '#49454f',
    onSurfaceVariant: '#cac4d0',
    outline: '#938f99',
    background: '#121212',
    onBackground: '#e6e1e5',
  },
};

interface HimsPaperProviderProps {
  children: React.ReactNode;
  theme?: 'light' | 'dark' | 'auto';
}

export function HimsPaperProvider({ children, theme = 'auto' }: HimsPaperProviderProps) {
  const systemColorScheme = useColorScheme();
  
  const selectedTheme = theme === 'auto' 
    ? (systemColorScheme === 'dark' ? himsDarkTheme : himsLightTheme)
    : theme === 'dark' 
    ? himsDarkTheme 
    : himsLightTheme;

  return (
    <PaperProvider theme={selectedTheme}>
      {children}
    </PaperProvider>
  );
}