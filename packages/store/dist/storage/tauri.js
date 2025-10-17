// Tauri storage engine using Tauri's Store API
export const createTauriStorage = () => {
    let tauriStore;
    try {
        // Dynamic import for Tauri Store
        const { Store } = require('@tauri-apps/plugin-store');
        tauriStore = new Store('.hims-store.dat');
    }
    catch (error) {
        console.warn('Tauri Store not available:', error);
        return createTauriMemoryStorage();
    }
    return {
        async getItem(key) {
            try {
                const value = await tauriStore.get(key);
                return value ? JSON.stringify(value) : null;
            }
            catch (error) {
                console.warn('Tauri Store getItem failed:', error);
                return null;
            }
        },
        async setItem(key, value) {
            try {
                const parsedValue = JSON.parse(value);
                await tauriStore.set(key, parsedValue);
                await tauriStore.save();
            }
            catch (error) {
                console.warn('Tauri Store setItem failed:', error);
            }
        },
        async removeItem(key) {
            try {
                await tauriStore.delete(key);
                await tauriStore.save();
            }
            catch (error) {
                console.warn('Tauri Store removeItem failed:', error);
            }
        },
        async getAllKeys() {
            try {
                return await tauriStore.keys();
            }
            catch (error) {
                console.warn('Tauri Store getAllKeys failed:', error);
                return [];
            }
        },
    };
};
// Alternative using Tauri's filesystem API for more control
export const createTauriFileStorage = (fileName = 'hims-data.json') => {
    let fs;
    let path;
    try {
        fs = require('@tauri-apps/api/fs');
        path = require('@tauri-apps/api/path');
    }
    catch (error) {
        console.warn('Tauri filesystem API not available:', error);
        return createTauriMemoryStorage();
    }
    let dataCache = {};
    let isLoaded = false;
    const loadData = async () => {
        if (isLoaded)
            return;
        try {
            const appDataDir = await path.appDataDir();
            const filePath = await path.join(appDataDir, fileName);
            if (await fs.exists(filePath)) {
                const content = await fs.readTextFile(filePath);
                dataCache = JSON.parse(content);
            }
            isLoaded = true;
        }
        catch (error) {
            console.warn('Failed to load Tauri file storage:', error);
            dataCache = {};
            isLoaded = true;
        }
    };
    const saveData = async () => {
        try {
            const appDataDir = await path.appDataDir();
            const filePath = await path.join(appDataDir, fileName);
            // Ensure directory exists
            await fs.createDir(appDataDir, { recursive: true });
            // Write data
            await fs.writeTextFile(filePath, JSON.stringify(dataCache, null, 2));
        }
        catch (error) {
            console.warn('Failed to save Tauri file storage:', error);
        }
    };
    return {
        async getItem(key) {
            await loadData();
            return dataCache[key] ? JSON.stringify(dataCache[key]) : null;
        },
        async setItem(key, value) {
            await loadData();
            try {
                dataCache[key] = JSON.parse(value);
                await saveData();
            }
            catch (error) {
                console.warn('Tauri file storage setItem failed:', error);
            }
        },
        async removeItem(key) {
            await loadData();
            delete dataCache[key];
            await saveData();
        },
        async getAllKeys() {
            await loadData();
            return Object.keys(dataCache);
        },
    };
};
// Fallback memory storage for Tauri
export const createTauriMemoryStorage = () => {
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
