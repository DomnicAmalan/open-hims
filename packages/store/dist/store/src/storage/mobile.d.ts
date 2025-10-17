import type { StorageEngine } from '../types';
export declare const createAsyncStorage: () => StorageEngine;
export declare const createMobileMemoryStorage: () => StorageEngine;
export declare const createSecureStorage: () => StorageEngine;
