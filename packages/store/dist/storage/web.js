// Web storage engine using localStorage
export const createWebStorage = () => ({
    async getItem(key) {
        try {
            return localStorage.getItem(key);
        }
        catch (error) {
            console.warn('localStorage getItem failed:', error);
            return null;
        }
    },
    async setItem(key, value) {
        try {
            localStorage.setItem(key, value);
        }
        catch (error) {
            console.warn('localStorage setItem failed:', error);
        }
    },
    async removeItem(key) {
        try {
            localStorage.removeItem(key);
        }
        catch (error) {
            console.warn('localStorage removeItem failed:', error);
        }
    },
    async getAllKeys() {
        try {
            return Object.keys(localStorage);
        }
        catch (error) {
            console.warn('localStorage getAllKeys failed:', error);
            return [];
        }
    },
});
// Fallback storage for SSR or when localStorage is not available
export const createMemoryStorage = () => {
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
// Auto-detect best storage for web
export const createAutoWebStorage = () => {
    try {
        // Test if localStorage is available
        const testKey = '__hims_storage_test__';
        localStorage.setItem(testKey, 'test');
        localStorage.removeItem(testKey);
        return createWebStorage();
    }
    catch {
        // Fallback to memory storage
        console.warn('localStorage not available, using memory storage');
        return createMemoryStorage();
    }
};
