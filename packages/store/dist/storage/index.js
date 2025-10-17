export * from './web';
export * from './mobile';
export * from './tauri';
import { createAutoWebStorage } from './web';
import { createAsyncStorage } from './mobile';
import { createTauriStorage } from './tauri';
// Auto-detect platform and create appropriate storage
export const createPlatformStorage = (platform) => {
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
