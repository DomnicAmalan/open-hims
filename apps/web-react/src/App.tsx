import React, { useState } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { Provider } from 'react-redux';
import { PersistGate } from 'redux-persist/integration/react';
import { createWebStore } from '@open-hims/store/web';
import { HimsProvider } from './context/HimsContext';
import { DashboardScreen, TestButton } from '@open-hims/screens-web';
// import './App.css';

function App() {
  const [currentCountry, setCurrentCountry] = useState('US');
  const [currentState, setCurrentState] = useState('CA');

  // Create store instance
  const { store, persistor } = createWebStore({
    enableDevTools: process.env.NODE_ENV === 'development',
    persistWhitelist: ['patients', 'config'],
    persistBlacklist: ['audit'],
  });

  return (
    <Provider store={store}>
      <PersistGate loading={<div>Loading...</div>} persistor={persistor}>
        <HimsProvider initialCountry={currentCountry} initialState={currentState}>
          <Router>
            <div className="min-h-screen bg-gray-50">
              <main className="container mx-auto px-4 py-8">
                <Routes>
                  <Route path="/" element={<Navigate to="/dashboard" replace />} />
                  <Route path="/dashboard" element={<DashboardScreen />} />
                </Routes>
              </main>
            </div>
          </Router>
        </HimsProvider>
      </PersistGate>
    </Provider>
  );
}

export default App;