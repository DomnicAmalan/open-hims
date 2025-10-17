# @open-hims/store

Shared Redux store for Open HIMS with platform-agnostic persistence support.

## Features

- **Platform Agnostic**: Works with Web (localStorage), React Native (AsyncStorage), and Tauri (Store API)
- **Type Safe**: Full TypeScript support with proper type inference
- **Healthcare Focused**: Pre-configured slices for patients, compliance, HL7, audit logs, etc.
- **Secure Persistence**: Configurable persistence with security considerations
- **Redux Toolkit**: Modern Redux with RTK and Redux Saga for side effects

## Installation

```bash
pnpm add @open-hims/store
```

## Usage

### Web Applications

```typescript
import { createWebStore } from '@open-hims/store/web';

const { store, persistor } = createWebStore({
  enableDevTools: process.env.NODE_ENV === 'development',
  persistWhitelist: ['patients', 'config'],
  persistBlacklist: ['audit'], // Don't persist sensitive audit logs
});

// Use with React
import { Provider } from 'react-redux';
import { PersistGate } from 'redux-persist/integration/react';

function App() {
  return (
    <Provider store={store}>
      <PersistGate loading={<div>Loading...</div>} persistor={persistor}>
        {/* Your app components */}
      </PersistGate>
    </Provider>
  );
}
```

### React Native Applications

```typescript
import { createMobileStore } from '@open-hims/store/mobile';

const { store, persistor } = createMobileStore({
  useSecureStorage: true, // Use Expo SecureStore for sensitive data
  enableDevTools: __DEV__,
  persistWhitelist: ['patients', 'config'],
  persistBlacklist: ['audit'],
});
```

### Tauri Desktop Applications

```typescript
import { createTauriStore } from '@open-hims/store/tauri';

const { store, persistor } = createTauriStore({
  useFileStorage: true, // Use filesystem instead of Tauri Store
  fileName: 'hims-data.json',
  enableDevTools: process.env.NODE_ENV !== 'production',
  persistWhitelist: ['patients', 'config'],
  persistBlacklist: ['audit'],
});
```

### Custom Storage Engine

```typescript
import { createHimsStore, type StorageEngine } from '@open-hims/store';

// Create custom storage engine
const customStorage: StorageEngine = {
  async getItem(key: string): Promise<string | null> {
    // Your custom implementation
    return null;
  },
  async setItem(key: string, value: string): Promise<void> {
    // Your custom implementation
  },
  async removeItem(key: string): Promise<void> {
    // Your custom implementation
  },
};

const { store, persistor } = createHimsStore({
  platform: 'custom',
  storage: customStorage,
  persistConfig: {
    whitelist: ['patients'],
    blacklist: ['audit'],
  },
});
```

## Available Slices

### Patients Slice

```typescript
import { useSelector, useDispatch } from 'react-redux';
import { 
  fetchPatientsStart, 
  createPatientStart, 
  selectPatient,
  type RootState 
} from '@open-hims/store';

function PatientList() {
  const dispatch = useDispatch();
  const { patients, loading, selectedPatient } = useSelector((state: RootState) => state.patients);

  const handleLoadPatients = () => {
    dispatch(fetchPatientsStart());
  };

  const handleSelectPatient = (patient) => {
    dispatch(selectPatient(patient));
  };

  return (
    <div>
      <button onClick={handleLoadPatients} disabled={loading.fetchPatients}>
        Load Patients
      </button>
      {patients.map(patient => (
        <div key={patient.id} onClick={() => handleSelectPatient(patient)}>
          {patient.firstName} {patient.lastName}
        </div>
      ))}
    </div>
  );
}
```

## Security Considerations

- **Audit Logs**: Never persisted to prevent tampering
- **Sensitive Data**: Use secure storage options on mobile
- **Patient Data**: Encrypted at rest on all platforms
- **Compliance**: Configurable for HIPAA, GDPR requirements

## Platform Support

| Platform | Storage | Secure Storage | Offline Support |
|----------|---------|----------------|-----------------|
| Web | localStorage | ❌ | ✅ |
| React Native | AsyncStorage | SecureStore | ✅ |
| Tauri | Store API / FileSystem | ✅ | ✅ |

## API Reference

### Store Factory Functions

- `createWebStore(options)` - Web-optimized store
- `createMobileStore(options)` - React Native-optimized store  
- `createTauriStore(options)` - Tauri-optimized store
- `createHimsStore(config)` - Generic store factory

### Storage Engines

- `createAutoWebStorage()` - Auto-detect web storage
- `createAsyncStorage()` - React Native AsyncStorage
- `createSecureStorage()` - Expo SecureStore
- `createTauriStorage()` - Tauri Store API
- `createTauriFileStorage()` - Tauri filesystem storage