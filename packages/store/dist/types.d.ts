export interface StorageEngine {
    getItem(key: string): Promise<string | null>;
    setItem(key: string, value: string): Promise<void>;
    removeItem(key: string): Promise<void>;
    getAllKeys?(): Promise<string[]>;
}
export type Platform = 'web' | 'mobile' | 'tauri';
export interface StorageConfig {
    key: string;
    platform: Platform;
    storage: StorageEngine;
    whitelist?: string[];
    blacklist?: string[];
    transforms?: any[];
    debug?: boolean;
}
