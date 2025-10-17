import { configureStore, combineReducers } from '@reduxjs/toolkit';
import createSagaMiddleware from 'redux-saga';
import { persistReducer, persistStore } from 'redux-persist';
import type { StorageConfig, StorageEngine, Platform } from './types';
import { createPlatformStorage } from './storage';
import { patientsReducer } from './slices';
import { rootSaga } from './sagas';

// Import other slices (to be created)
// import complianceSlice from './slices/complianceSlice';
// import hl7Slice from './slices/hl7Slice';
// import auditSlice from './slices/auditSlice';
// import dataTransferSlice from './slices/dataTransferSlice';
// import configSlice from './slices/configSlice';

// Store configuration interface
export interface StoreConfig {
  platform?: Platform;
  storage?: StorageEngine;
  persistConfig?: Partial<StorageConfig>;
  enableSaga?: boolean;
  enableDevTools?: boolean;
}

// Create root reducer
const createRootReducer = () => combineReducers({
  patients: patientsReducer,
  // compliance: complianceSlice,
  // hl7: hl7Slice,
  // audit: auditSlice,
  // dataTransfer: dataTransferSlice,
  // config: configSlice,
});

// Create store factory function
export const createHimsStore = (config: StoreConfig = {}) => {
  const {
    platform,
    storage,
    persistConfig = {},
    enableSaga = true,
    enableDevTools = process.env.NODE_ENV !== 'production',
  } = config;

  // Create storage engine
  const storageEngine = storage || createPlatformStorage(platform);

  // Redux persist configuration
  const defaultPersistConfig: StorageConfig = {
    key: 'hims-root',
    platform: platform || 'web',
    storage: storageEngine,
    whitelist: ['patients'], // Only persist these slices
    blacklist: ['audit'], // Don't persist audit logs for security
    debug: process.env.NODE_ENV === 'development',
    ...persistConfig,
  };

  // Create root reducer
  const rootReducer = createRootReducer();

  // Create persisted reducer
  const persistedReducer = persistReducer(defaultPersistConfig, rootReducer);

  // Create saga middleware
  const sagaMiddleware = createSagaMiddleware();

  // Configure store
  const store = configureStore({
    reducer: persistedReducer,
    middleware: (getDefaultMiddleware) => {
      const middleware = getDefaultMiddleware({
        serializableCheck: {
          ignoredActions: [
            'persist/PERSIST',
            'persist/REHYDRATE',
            'persist/PAUSE',
            'persist/PURGE',
            'persist/REGISTER',
          ],
        },
      });

      return enableSaga ? middleware.concat(sagaMiddleware) : middleware;
    },
    devTools: enableDevTools,
  });

  // Create persistor
  const persistor = persistStore(store);

  // Run root saga
  if (enableSaga) {
    sagaMiddleware.run(rootSaga);
  }

  return {
    store,
    persistor,
    sagaMiddleware,
  };
};

// Types
export type RootState = ReturnType<ReturnType<typeof createRootReducer>>;
export type AppDispatch = ReturnType<typeof createHimsStore>['store']['dispatch'];

// Create default store instance for easy consumption
const { store: defaultStore, persistor: defaultPersistor } = createHimsStore({
  platform: 'web', // Default to web, can be overridden
  enableDevTools: process.env.NODE_ENV !== 'production',
});

// Export default instances
export const store = defaultStore;
export const persistor = defaultPersistor;

// Export slices and sagas (thin re-exports)
export * from './slices';
export * from './sagas';

// Export storage utilities
export * from './storage';
export * from './types';

// Version information
export const STORE_VERSION = '0.1.0';