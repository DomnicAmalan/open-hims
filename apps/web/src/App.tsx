import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { Provider } from 'react-redux';
import { PersistGate } from 'redux-persist/integration/react';
import { MantineProvider, createTheme } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import { ModalsProvider } from '@mantine/modals';
import { createWebStore } from '@open-hims/store/web';
import { DashboardScreen, PatientsScreen } from '@open-hims/screens-web';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';

// Simple theme for healthcare app
const theme = createTheme({
  primaryColor: 'blue',
  defaultRadius: 'md',
});

function App() {
  // Create store instance
  const { store, persistor } = createWebStore({
    enableDevTools: process.env.NODE_ENV === 'development',
    persistWhitelist: ['patients', 'config'],
    persistBlacklist: ['audit'],
  });

  return (
    <MantineProvider theme={theme}>
      <Notifications position="top-right" />
      <ModalsProvider>
        <Provider store={store}>
          <PersistGate loading={<div>Loading...</div>} persistor={persistor}>
            <Router>
              <Routes>
                <Route path="/" element={<Navigate to="/dashboard" replace />} />
                <Route path="/dashboard" element={<DashboardScreen />} />
                <Route path="/patients" element={<PatientsScreen />} />
              </Routes>
            </Router>
          </PersistGate>
        </Provider>
      </ModalsProvider>
    </MantineProvider>
  );
}

export default App;