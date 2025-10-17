import type { StorageEngine } from '../types';

// Web storage engine using localStorage
export const createWebStorage = (): StorageEngine => ({
  async getItem(key: string): Promise<string | null> {
    try {
      return localStorage.getItem(key);
    } catch (error) {
      console.warn('localStorage getItem failed:', error);
      return null;
    }
  },

  async setItem(key: string, value: string): Promise<void> {
    try {
      localStorage.setItem(key, value);
    } catch (error) {
      console.warn('localStorage setItem failed:', error);
    }
  },

  async removeItem(key: string): Promise<void> {
    try {
      localStorage.removeItem(key);
    } catch (error) {
      console.warn('localStorage removeItem failed:', error);
    }
  },

  async getAllKeys(): Promise<string[]> {
    try {
      return Object.keys(localStorage);
    } catch (error) {
      console.warn('localStorage getAllKeys failed:', error);
      return [];
    }
  },
});

// Fallback storage for SSR or when localStorage is not available
export const createMemoryStorage = (): StorageEngine => {
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

// Auto-detect best storage for web
export const createAutoWebStorage = (): StorageEngine => {
  try {
    // Test if localStorage is available
    const testKey = '__hims_storage_test__';
    localStorage.setItem(testKey, 'test');
    localStorage.removeItem(testKey);
    return createWebStorage();
  } catch {
    // Fallback to memory storage
    console.warn('localStorage not available, using memory storage');
    return createMemoryStorage();
  }
};