import React from 'react';
import { MantineProvider, createTheme, MantineColorsTuple } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import { ModalsProvider } from '@mantine/modals';

// Healthcare-focused color scheme
const himsBlue: MantineColorsTuple = [
  '#e6f2ff',
  '#b3d9ff',
  '#80bfff',
  '#4da6ff',
  '#1a8cff',
  '#0066cc', // Primary
  '#0052a3',
  '#003d7a',
  '#002952',
  '#001429'
];

const himsGreen: MantineColorsTuple = [
  '#e8f5e8',
  '#c8e6c9',
  '#a5d6a7',
  '#81c784',
  '#66bb6a',
  '#4caf50', // Success
  '#43a047',
  '#388e3c',
  '#2e7d32',
  '#1b5e20'
];

const himsRed: MantineColorsTuple = [
  '#ffebee',
  '#ffcdd2',
  '#ef9a9a',
  '#e57373',
  '#ef5350',
  '#f44336', // Error
  '#e53935',
  '#d32f2f',
  '#c62828',
  '#b71c1c'
];

const theme = createTheme({
  primaryColor: 'himsBlue',
  colors: {
    himsBlue,
    himsGreen,
    himsRed,
  },
  fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
  headings: {
    fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
    fontWeight: '600',
  },
  defaultRadius: 'md',
  components: {
    Button: {
      defaultProps: {
        radius: 'md',
      },
    },
    Card: {
      defaultProps: {
        shadow: 'sm',
        radius: 'md',
        withBorder: true,
      },
    },
    TextInput: {
      defaultProps: {
        radius: 'md',
      },
    },
    Select: {
      defaultProps: {
        radius: 'md',
      },
    },
  },
  other: {
    // Healthcare-specific theme values
    spacing: {
      xs: '0.5rem',
      sm: '0.75rem',
      md: '1rem',
      lg: '1.5rem',
      xl: '2rem',
    },
    // Accessibility improvements
    focusRing: 'always',
    respectReducedMotion: true,
  },
});

interface HimsThemeProviderProps {
  children: React.ReactNode;
}

export function HimsThemeProvider({ children }: HimsThemeProviderProps) {
  return (
    <MantineProvider theme={theme} defaultColorScheme="light">
      <Notifications position="top-right" zIndex={2077} />
      <ModalsProvider>
        {children}
      </ModalsProvider>
    </MantineProvider>
  );
}