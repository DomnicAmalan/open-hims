import { configureStore, combineReducers } from '@reduxjs/toolkit';
import createSagaMiddleware from 'redux-saga';
import { persistReducer, persistStore } from 'redux-persist';
import { createPlatformStorage } from './storage';
import patientsReducer from './slices/patientsSlice';
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
export const createHimsStore = (config = {}) => {
    const { platform, storage, persistConfig = {}, enableSaga = true, enableDevTools = process.env.NODE_ENV !== 'production', } = config;
    // Create storage engine
    const storageEngine = storage || createPlatformStorage(platform);
    // Redux persist configuration
    const defaultPersistConfig = {
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
    // Run root saga (to be implemented)
    if (enableSaga) {
        // sagaMiddleware.run(rootSaga);
    }
    return {
        store,
        persistor,
        sagaMiddleware,
    };
};
// Export slices
export { default as patientsReducer } from './slices/patientsSlice';
export * from './slices/patientsSlice';
// Export storage utilities
export * from './storage';
export * from './types';
