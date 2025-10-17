import type { StorageEngine } from '../types';

// React Native AsyncStorage engine
export const createAsyncStorage = (): StorageEngine => {
  // Dynamic import to avoid issues when not in React Native environment
  let AsyncStorage: any;
  
  try {
    AsyncStorage = require('@react-native-async-storage/async-storage').default;
  } catch (error) {
    console.warn('AsyncStorage not available:', error);
    return createMobileMemoryStorage();
  }

  return {
    async getItem(key: string): Promise<string | null> {
      try {
        return await AsyncStorage.getItem(key);
      } catch (error) {
        console.warn('AsyncStorage getItem failed:', error);
        return null;
      }
    },

    async setItem(key: string, value: string): Promise<void> {
      try {
        await AsyncStorage.setItem(key, value);
      } catch (error) {
        console.warn('AsyncStorage setItem failed:', error);
      }
    },

    async removeItem(key: string): Promise<void> {
      try {
        await AsyncStorage.removeItem(key);
      } catch (error) {
        console.warn('AsyncStorage removeItem failed:', error);
      }
    },

    async getAllKeys(): Promise<string[]> {
      try {
        return await AsyncStorage.getAllKeys();
      } catch (error) {
        console.warn('AsyncStorage getAllKeys failed:', error);
        return [];
      }
    },
  };
};

// Fallback memory storage for mobile
export const createMobileMemoryStorage = (): StorageEngine => {
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

// Expo SecureStore for sensitive data
export const createSecureStorage = (): StorageEngine => {
  let SecureStore: any;
  
  try {
    SecureStore = require('expo-secure-store');
  } catch (error) {
    console.warn('SecureStore not available, falling back to AsyncStorage');
    return createAsyncStorage();
  }

  return {
    async getItem(key: string): Promise<string | null> {
      try {
        return await SecureStore.getItemAsync(key);
      } catch (error) {
        console.warn('SecureStore getItem failed:', error);
        return null;
      }
    },

    async setItem(key: string, value: string): Promise<void> {
      try {
        await SecureStore.setItemAsync(key, value);
      } catch (error) {
        console.warn('SecureStore setItem failed:', error);
      }
    },

    async removeItem(key: string): Promise<void> {
      try {
        await SecureStore.deleteItemAsync(key);
      } catch (error) {
        console.warn('SecureStore removeItem failed:', error);
      }
    },

    async getAllKeys(): Promise<string[]> {
      // SecureStore doesn't have getAllKeys, return empty array
      console.warn('SecureStore does not support getAllKeys');
      return [];
    },
  };
};