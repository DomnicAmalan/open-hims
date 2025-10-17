// React Native AsyncStorage engine
export const createAsyncStorage = () => {
    // Dynamic import to avoid issues when not in React Native environment
    let AsyncStorage;
    try {
        AsyncStorage = require('@react-native-async-storage/async-storage').default;
    }
    catch (error) {
        console.warn('AsyncStorage not available:', error);
        return createMobileMemoryStorage();
    }
    return {
        async getItem(key) {
            try {
                return await AsyncStorage.getItem(key);
            }
            catch (error) {
                console.warn('AsyncStorage getItem failed:', error);
                return null;
            }
        },
        async setItem(key, value) {
            try {
                await AsyncStorage.setItem(key, value);
            }
            catch (error) {
                console.warn('AsyncStorage setItem failed:', error);
            }
        },
        async removeItem(key) {
            try {
                await AsyncStorage.removeItem(key);
            }
            catch (error) {
                console.warn('AsyncStorage removeItem failed:', error);
            }
        },
        async getAllKeys() {
            try {
                return await AsyncStorage.getAllKeys();
            }
            catch (error) {
                console.warn('AsyncStorage getAllKeys failed:', error);
                return [];
            }
        },
    };
};
// Fallback memory storage for mobile
export const createMobileMemoryStorage = () => {
    const storage = new Map();
    return {
        async getItem(key) {
            return storage.get(key) || null;
        },
        async setItem(key, value) {
            storage.set(key, value);
        },
        async removeItem(key) {
            storage.delete(key);
        },
        async getAllKeys() {
            return Array.from(storage.keys());
        },
    };
};
// Expo SecureStore for sensitive data
export const createSecureStorage = () => {
    let SecureStore;
    try {
        SecureStore = require('expo-secure-store');
    }
    catch (error) {
        console.warn('SecureStore not available, falling back to AsyncStorage');
        return createAsyncStorage();
    }
    return {
        async getItem(key) {
            try {
                return await SecureStore.getItemAsync(key);
            }
            catch (error) {
                console.warn('SecureStore getItem failed:', error);
                return null;
            }
        },
        async setItem(key, value) {
            try {
                await SecureStore.setItemAsync(key, value);
            }
            catch (error) {
                console.warn('SecureStore setItem failed:', error);
            }
        },
        async removeItem(key) {
            try {
                await SecureStore.deleteItemAsync(key);
            }
            catch (error) {
                console.warn('SecureStore removeItem failed:', error);
            }
        },
        async getAllKeys() {
            // SecureStore doesn't have getAllKeys, return empty array
            console.warn('SecureStore does not support getAllKeys');
            return [];
        },
    };
};
