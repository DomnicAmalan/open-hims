// Platform-specific storage abstraction
// Web version uses localStorage, desktop version will use Tauri storage

export interface StorageAdapter {
  getItem: (key: string) => Promise<string | null>;
  setItem: (key: string, value: string) => Promise<void>;
  removeItem: (key: string) => Promise<void>;
  clear: () => Promise<void>;
  keys: () => Promise<string[]>;
}

// Web implementation using localStorage
const webStorageAdapter: StorageAdapter = {
  getItem: async (key: string) => {
    try {
      return localStorage.getItem(key);
    } catch (error) {
      console.error('Storage getItem error:', error);
      return null;
    }
  },
  setItem: async (key: string, value: string) => {
    try {
      localStorage.setItem(key, value);
    } catch (error) {
      console.error('Storage setItem error:', error);
    }
  },
  removeItem: async (key: string) => {
    try {
      localStorage.removeItem(key);
    } catch (error) {
      console.error('Storage removeItem error:', error);
    }
  },
  clear: async () => {
    try {
      localStorage.clear();
    } catch (error) {
      console.error('Storage clear error:', error);
    }
  },
  keys: async () => {
    try {
      return Object.keys(localStorage);
    } catch (error) {
      console.error('Storage keys error:', error);
      return [];
    }
  },
};

// Hook to use the appropriate storage adapter
export function useStorage(): StorageAdapter {
  // In the future, this could detect the platform and return the appropriate adapter
  // For now, it returns the web adapter
  // Desktop version would check for Tauri API and return Tauri storage adapter
  
  return webStorageAdapter;
}