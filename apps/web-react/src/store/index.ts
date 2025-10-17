import { configureStore } from '@reduxjs/toolkit';
import createSagaMiddleware from 'redux-saga';
import { persistStore, persistReducer } from 'redux-persist';
import storage from 'redux-persist/lib/storage';

// Import slices
import patientsSlice from './slices/patientsSlice';
import complianceSlice from './slices/complianceSlice';
import hl7Slice from './slices/hl7Slice';
import auditSlice from './slices/auditSlice';
import dataTransferSlice from './slices/dataTransferSlice';
import configSlice from './slices/configSlice';

// Import root saga
import rootSaga from './sagas/rootSaga';

// Redux Persist Configuration
const persistConfig = {
  key: 'hims-root',
  storage,
  whitelist: ['config', 'patients'], // Only persist these slices
  blacklist: ['audit'], // Don't persist audit logs for security
};

// Combine reducers
const rootReducer = {
  patients: patientsSlice.reducer,
  compliance: complianceSlice.reducer,
  hl7: hl7Slice.reducer,
  audit: auditSlice.reducer,
  dataTransfer: dataTransferSlice.reducer,
  config: configSlice.reducer,
};

// Create saga middleware
const sagaMiddleware = createSagaMiddleware();

// Create persisted reducer
const persistedReducer = persistReducer(persistConfig, rootReducer);

// Configure store
export const store = configureStore({
  reducer: persistedReducer,
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: {
        ignoredActions: ['persist/PERSIST', 'persist/REHYDRATE'],
      },
    }).concat(sagaMiddleware),
  devTools: process.env.NODE_ENV !== 'production',
});

// Run root saga
sagaMiddleware.run(rootSaga);

// Create persistor
export const persistor = persistStore(store);

// Types
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;