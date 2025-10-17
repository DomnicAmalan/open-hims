// Web-specific store setup
import { createHimsStore } from './index';
import { createAutoWebStorage } from './storage/web';

// Create web store with localStorage
export const createWebStore = (options: {
  enableDevTools?: boolean;
  persistWhitelist?: string[];
  persistBlacklist?: string[];
} = {}) => {
  const {
    enableDevTools = process.env.NODE_ENV !== 'production',
    persistWhitelist = ['patients', 'config'],
    persistBlacklist = ['audit'],
  } = options;

  return createHimsStore({
    platform: 'web',
    storage: createAutoWebStorage(),
    persistConfig: {
      whitelist: persistWhitelist,
      blacklist: persistBlacklist,
    },
    enableDevTools,
  });
};

// Export for direct use in web applications
export default createWebStore;

// Re-export everything from main store
export * from './index';