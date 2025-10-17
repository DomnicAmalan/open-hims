// Mobile-specific store setup
import { createHimsStore } from './index';
import { createAsyncStorage, createSecureStorage } from './storage/mobile';

// Create mobile store with AsyncStorage
export const createMobileStore = (options: {
  useSecureStorage?: boolean;
  enableDevTools?: boolean;
  persistWhitelist?: string[];
  persistBlacklist?: string[];
} = {}) => {
  const {
    useSecureStorage = false,
    enableDevTools = process.env.NODE_ENV === 'development',
    persistWhitelist = ['patients', 'config'],
    persistBlacklist = ['audit'],
  } = options;

  const storage = useSecureStorage ? createSecureStorage() : createAsyncStorage();

  return createHimsStore({
    platform: 'mobile',
    storage,
    persistConfig: {
      whitelist: persistWhitelist,
      blacklist: persistBlacklist,
    },
    enableDevTools,
  });
};

// Export for direct use in React Native applications
export default createMobileStore;

// Re-export everything from main store
export * from './index';