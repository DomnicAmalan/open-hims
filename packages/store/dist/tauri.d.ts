export declare const createTauriStore: (options?: {
    useFileStorage?: boolean;
    fileName?: string;
    enableDevTools?: boolean;
    persistWhitelist?: string[];
    persistBlacklist?: string[];
}) => {
    store: import("@reduxjs/toolkit/dist/configureStore").ToolkitStore<import("@reduxjs/toolkit").EmptyObject & {
        patients: import("./index").PatientsState;
    } & import("redux-persist/es/persistReducer").PersistPartial, import("@reduxjs/toolkit").AnyAction, import("@reduxjs/toolkit").MiddlewareArray<[import("@reduxjs/toolkit").ThunkMiddleware<import("@reduxjs/toolkit").EmptyObject & {
        patients: import("./index").PatientsState;
    } & import("redux-persist/es/persistReducer").PersistPartial, import("@reduxjs/toolkit").AnyAction>]> | import("@reduxjs/toolkit").MiddlewareArray<[import("@reduxjs/toolkit").ThunkMiddleware<import("@reduxjs/toolkit").EmptyObject & {
        patients: import("./index").PatientsState;
    } & import("redux-persist/es/persistReducer").PersistPartial, import("@reduxjs/toolkit").AnyAction>, import("redux-saga").SagaMiddleware<object>]>>;
    persistor: import("redux-persist/es/types").Persistor;
    sagaMiddleware: import("redux-saga").SagaMiddleware<object>;
};
export default createTauriStore;
export * from './index';
