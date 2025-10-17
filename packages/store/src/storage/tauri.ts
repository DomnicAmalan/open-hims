import type { StorageEngine } from '../types';

// Tauri storage engine using Tauri's Store API
export const createTauriStorage = (): StorageEngine => {
  let tauriStore: any;
  
  try {
    // Dynamic import for Tauri Store
    const { Store } = require('@tauri-apps/plugin-store');
    tauriStore = new Store('.hims-store.dat');
  } catch (error) {
    console.warn('Tauri Store not available:', error);
    return createTauriMemoryStorage();
  }

  return {
    async getItem(key: string): Promise<string | null> {
      try {
        const value = await tauriStore.get(key);
        return value ? JSON.stringify(value) : null;
      } catch (error) {
        console.warn('Tauri Store getItem failed:', error);
        return null;
      }
    },

    async setItem(key: string, value: string): Promise<void> {
      try {
        const parsedValue = JSON.parse(value);
        await tauriStore.set(key, parsedValue);
        await tauriStore.save();
      } catch (error) {
        console.warn('Tauri Store setItem failed:', error);
      }
    },

    async removeItem(key: string): Promise<void> {
      try {
        await tauriStore.delete(key);
        await tauriStore.save();
      } catch (error) {
        console.warn('Tauri Store removeItem failed:', error);
      }
    },

    async getAllKeys(): Promise<string[]> {
      try {
        return await tauriStore.keys();
      } catch (error) {
        console.warn('Tauri Store getAllKeys failed:', error);
        return [];
      }
    },
  };
};

// Alternative using Tauri's filesystem API for more control
export const createTauriFileStorage = (fileName: string = 'hims-data.json'): StorageEngine => {
  let fs: any;
  let path: any;
  
  try {
    fs = require('@tauri-apps/api/fs');
    path = require('@tauri-apps/api/path');
  } catch (error) {
    console.warn('Tauri filesystem API not available:', error);
    return createTauriMemoryStorage();
  }

  let dataCache: Record<string, any> = {};
  let isLoaded = false;

  const loadData = async (): Promise<void> => {
    if (isLoaded) return;
    
    try {
      const appDataDir = await path.appDataDir();
      const filePath = await path.join(appDataDir, fileName);
      
      if (await fs.exists(filePath)) {
        const content = await fs.readTextFile(filePath);
        dataCache = JSON.parse(content);
      }
      isLoaded = true;
    } catch (error) {
      console.warn('Failed to load Tauri file storage:', error);
      dataCache = {};
      isLoaded = true;
    }
  };

  const saveData = async (): Promise<void> => {
    try {
      const appDataDir = await path.appDataDir();
      const filePath = await path.join(appDataDir, fileName);
      
      // Ensure directory exists
      await fs.createDir(appDataDir, { recursive: true });
      
      // Write data
      await fs.writeTextFile(filePath, JSON.stringify(dataCache, null, 2));
    } catch (error) {
      console.warn('Failed to save Tauri file storage:', error);
    }
  };

  return {
    async getItem(key: string): Promise<string | null> {
      await loadData();
      return dataCache[key] ? JSON.stringify(dataCache[key]) : null;
    },

    async setItem(key: string, value: string): Promise<void> {
      await loadData();
      try {
        dataCache[key] = JSON.parse(value);
        await saveData();
      } catch (error) {
        console.warn('Tauri file storage setItem failed:', error);
      }
    },

    async removeItem(key: string): Promise<void> {
      await loadData();
      delete dataCache[key];
      await saveData();
    },

    async getAllKeys(): Promise<string[]> {
      await loadData();
      return Object.keys(dataCache);
    },
  };
};

// Fallback memory storage for Tauri
export const createTauriMemoryStorage = (): StorageEngine => {
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