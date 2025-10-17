export * from './web';
export * from './mobile';
export * from './tauri';

import type { StorageEngine, Platform } from '../types';
import { createAutoWebStorage } from './web';
import { createAsyncStorage } from './mobile';
import { createTauriStorage } from './tauri';

// Auto-detect platform and create appropriate storage
export const createPlatformStorage = (platform?: Platform): StorageEngine => {
  if (platform) {
    switch (platform) {
      case 'web':
        return createAutoWebStorage();
      case 'mobile':
        return createAsyncStorage();
      case 'tauri':
        return createTauriStorage();
      default:
        throw new Error(`Unsupported platform: ${platform}`);
    }
  }

  // Auto-detect platform
  if (typeof window !== 'undefined') {
    // Check if we're in Tauri
    if ('__TAURI__' in window) {
      return createTauriStorage();
    }
    // Check if we're in React Native
    if (typeof navigator !== 'undefined' && navigator.product === 'ReactNative') {
      return createAsyncStorage();
    }
    // Default to web
    return createAutoWebStorage();
  }

  // Node.js environment - use memory storage
  console.warn('Unknown environment, using memory storage');
  const storage = new Map<string, string>();
  return {
    async getItem(key: string): Promise<string | null> {
      return storage.get(key) || null;
    },
    async setItem(key: string, value: string): Promise<void> {
      storage.set(key, value);
    },
    async removeItem(key: string): Promise<void> {
      storage.delete(key);
    },
    async getAllKeys(): Promise<string[]> {
      return Array.from(storage.keys());
    },
  };
};