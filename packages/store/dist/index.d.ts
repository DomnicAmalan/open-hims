import type { StorageConfig, StorageEngine, Platform } from './types';
import patientsSlice from './slices/patientsSlice';
export interface StoreConfig {
    platform?: Platform;
    storage?: StorageEngine;
    persistConfig?: Partial<StorageConfig>;
    enableSaga?: boolean;
    enableDevTools?: boolean;
}
declare const createRootReducer: () => import("@reduxjs/toolkit").Reducer<import("@reduxjs/toolkit").CombinedState<{
    patients: import(".").PatientsState;
}>, import("@reduxjs/toolkit").AnyAction>;
export declare const createHimsStore: (config?: StoreConfig) => {
    store: import("@reduxjs/toolkit/dist/configureStore").ToolkitStore<import("@reduxjs/toolkit").EmptyObject & {
        patients: import(".").PatientsState;
    } & import("redux-persist/es/persistReducer").PersistPartial, import("@reduxjs/toolkit").AnyAction, import("@reduxjs/toolkit").MiddlewareArray<[import("@reduxjs/toolkit").ThunkMiddleware<import("@reduxjs/toolkit").EmptyObject & {
        patients: import(".").PatientsState;
    } & import("redux-persist/es/persistReducer").PersistPartial, import("@reduxjs/toolkit").AnyAction>]> | import("@reduxjs/toolkit").MiddlewareArray<[import("@reduxjs/toolkit").ThunkMiddleware<import("@reduxjs/toolkit").EmptyObject & {
        patients: import(".").PatientsState;
    } & import("redux-persist/es/persistReducer").PersistPartial, import("@reduxjs/toolkit").AnyAction>, import("redux-saga").SagaMiddleware<object>]>>;
    persistor: import("redux-persist").Persistor;
    sagaMiddleware: import("redux-saga").SagaMiddleware<object>;
};
export type RootState = ReturnType<ReturnType<typeof createRootReducer>>;
export type AppDispatch = ReturnType<typeof createHimsStore>['store']['dispatch'];
export { patientsSlice };
export * from './slices/patientsSlice';
export * from './storage';
export * from './types';
