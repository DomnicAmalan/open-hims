export * from './web';
export * from './mobile';
export * from './tauri';
import type { StorageEngine, Platform } from '../types';
export declare const createPlatformStorage: (platform?: Platform) => StorageEngine;
