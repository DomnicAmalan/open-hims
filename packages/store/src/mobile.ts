// Mobile-specific store setup with Redux Saga
import { configureStore, combineReducers } from '@reduxjs/toolkit';
import { persistReducer, persistStore } from 'redux-persist';
import createSagaMiddleware from 'redux-saga';
import { patientsReducer } from './slices';
import { rootSaga } from './sagas';
import { createAsyncStorage, createSecureStorage } from './storage/mobile';
import type { StorageConfig } from './types';

// Handle Redux Saga import for different module systems
// const createSagaMiddleware = sagaModule.default || sagaModule;

// Create root reducer for mobile
const createMobileRootReducer = () => combineReducers({
  patients: patientsReducer,
  // Add other mobile-specific reducers here
});

// Mobile store configuration
export const createMobileStore = (options: {
  useSecureStorage?: boolean;
  enableDevTools?: boolean;
  persistWhitelist?: string[];
  persistBlacklist?: string[];
  enableSaga?: boolean;
} = {}) => {
  const {
    useSecureStorage = false,
    enableDevTools = (global as any).__DEV__ || false,
    persistWhitelist = ['patients'],
    persistBlacklist = ['audit'],
    enableSaga = !!createSagaMiddleware,
  } = options;

  // Create storage engine
  const storage = useSecureStorage ? createSecureStorage() : createAsyncStorage();

  // Redux persist configuration
  const persistConfig: StorageConfig = {
    key: 'hims-mobile-root',
    platform: 'mobile',
    storage,
    whitelist: persistWhitelist,
    blacklist: persistBlacklist,
    debug: enableDevTools,
  };

  // Create root reducer
  const rootReducer = createMobileRootReducer();

  // Create persisted reducer
  const persistedReducer = persistReducer(persistConfig, rootReducer);

  // Create saga middleware if enabled
  const sagaMiddleware = enableSaga ? createSagaMiddleware() : null;

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

      return sagaMiddleware ? middleware.concat(sagaMiddleware) : middleware;
    },
    devTools: enableDevTools,
  });

  // Create persistor
  const persistor = persistStore(store);

  // Run root saga if enabled
  if (sagaMiddleware && enableSaga) {
    sagaMiddleware.run(rootSaga);
  }

  return {
    store,
    persistor,
    sagaMiddleware,
  };
};

// Create default mobile store instance
const defaultMobileStore = createMobileStore();

// Export default instances
export const store = defaultMobileStore.store;
export const persistor = defaultMobileStore.persistor;

// Export for direct use
export default createMobileStore;

// Export types
export type MobileRootState = ReturnType<ReturnType<typeof createMobileRootReducer>>;
export type MobileAppDispatch = typeof store.dispatch;

// Re-export essential store items
export * from './slices';
export * from './storage/mobile';
export * from './types';