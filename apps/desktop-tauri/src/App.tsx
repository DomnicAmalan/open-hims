import React from 'react';
import { Provider } from 'react-redux';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { MantineProvider, createTheme } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import { ModalsProvider } from '@mantine/modals';
import { store } from '@open-hims/store';
import { DashboardScreen, PatientsScreen, TestButton } from '@open-hims/screens-web';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';

// Simple theme for healthcare app
const theme = createTheme({
  primaryColor: 'blue',
  defaultRadius: 'md',
});

function App() {
  return (
    <MantineProvider theme={theme}>
      <Notifications position="top-right" />
      <ModalsProvider>
        <Provider store={store}>
          <Router>
            <Routes>
              <Route path="/" element={<DashboardScreen />} />
              <Route path="/patients" element={<PatientsScreen />} />
              <Route path="/test" element={
                <div style={{ padding: '2rem' }}>
                  <h1>Desktop Test Page</h1>
                  <TestButton label="Desktop Test Button" />
                </div>
              } />
            </Routes>
          </Router>
        </Provider>
      </ModalsProvider>
    </MantineProvider>
  );
}

export default App;