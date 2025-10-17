import type { StorageEngine } from '../types';
export declare const createTauriStorage: () => StorageEngine;
export declare const createTauriFileStorage: (fileName?: string) => StorageEngine;
export declare const createTauriMemoryStorage: () => StorageEngine;
