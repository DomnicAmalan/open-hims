// Tauri-specific store setup
import { createHimsStore } from './index';
import { createTauriStorage, createTauriFileStorage } from './storage/tauri';

// Create Tauri store with Tauri's storage API
export const createTauriStore = (options: {
  useFileStorage?: boolean;
  fileName?: string;
  enableDevTools?: boolean;
  persistWhitelist?: string[];
  persistBlacklist?: string[];
} = {}) => {
  const {
    useFileStorage = false,
    fileName = 'hims-data.json',
    enableDevTools = process.env.NODE_ENV !== 'production',
    persistWhitelist = ['patients', 'config'],
    persistBlacklist = ['audit'],
  } = options;

  const storage = useFileStorage 
    ? createTauriFileStorage(fileName)
    : createTauriStorage();

  return createHimsStore({
    platform: 'tauri',
    storage,
    persistConfig: {
      whitelist: persistWhitelist,
      blacklist: persistBlacklist,
    },
    enableDevTools,
  });
};

// Export for direct use in Tauri applications
export default createTauriStore;

// Re-export everything from main store
export * from './index';