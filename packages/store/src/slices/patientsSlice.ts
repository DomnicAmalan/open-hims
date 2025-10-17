import { createSlice, PayloadAction, createAsyncThunk } from '@reduxjs/toolkit';
import { 
  UIPatient, 
  CreatePatientRequest, 
  UpdatePatientRequest, 
  PatientFilters,
  PatientListState
} from '@open-hims/types';

// Use the PatientListState from types with some Redux-specific additions
export interface PatientsState extends Omit<PatientListState, 'loading'> {
  loading: {
    fetchPatients: boolean;
    createPatient: boolean;
    updatePatient: boolean;
    deletePatient: boolean;
  };
}

const initialState: PatientsState = {
  patients: [],
  pagination: {
    total: 0,
    page: 1,
    pageSize: 20,
    totalPages: 0,
  },
  selectedPatient: null,
  filters: {
    search: '',
  },
  loading: {
    fetchPatients: false,
    createPatient: false,
    updatePatient: false,
    deletePatient: false,
  },
  error: null,
  searchResults: [],
};

// Async thunks for API calls
export const fetchPatients = createAsyncThunk(
  'patients/fetchPatients',
  async (params: { page?: number; pageSize?: number; filters?: PatientFilters }) => {
    // Mock API call - replace with actual API integration
    return new Promise<{ patients: UIPatient[]; total: number }>((resolve) => {
      setTimeout(() => {
        const mockPatients: UIPatient[] = [
          {
            id: '1',
            mrn: 'MRN001',
            firstName: 'John',
            lastName: 'Doe',
            dateOfBirth: '1990-01-15',
            gender: 'male',
            phone: '+1-555-0123',
            email: 'john.doe@email.com',
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
          },
        ];
        resolve({ patients: mockPatients, total: mockPatients.length });
      }, 1000);
    });
  }
);

export const createPatient = createAsyncThunk(
  'patients/createPatient',
  async (request: CreatePatientRequest) => {
    // Mock API call - replace with actual API integration
    return new Promise<UIPatient>((resolve) => {
      setTimeout(() => {
        const newPatient: UIPatient = {
          id: Date.now().toString(),
          mrn: `MRN${Date.now()}`,
          firstName: '',
          lastName: '',
          dateOfBirth: '',
          gender: 'unknown',
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          ...request.patient,
        };
        resolve(newPatient);
      }, 1000);
    });
  }
);

export const updatePatient = createAsyncThunk(
  'patients/updatePatient',
  async (request: UpdatePatientRequest) => {
    // Mock API call - replace with actual API integration
    return new Promise<UIPatient>((resolve) => {
      setTimeout(() => {
        const updatedPatient: UIPatient = {
          id: request.id,
          mrn: '',
          firstName: '',
          lastName: '',
          dateOfBirth: '',
          gender: 'unknown',
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          ...request.patient,
        };
        resolve(updatedPatient);
      }, 1000);
    });
  }
);

export const deletePatient = createAsyncThunk(
  'patients/deletePatient',
  async (patientId: string) => {
    // Mock API call - replace with actual API integration
    return new Promise<string>((resolve) => {
      setTimeout(() => {
        resolve(patientId);
      }, 1000);
    });
  }
);

const patientsSlice = createSlice({
  name: 'patients',
  initialState,
  reducers: {
    // Select patient
    selectPatient: (state, action: PayloadAction<UIPatient | null>) => {
      state.selectedPatient = action.payload;
    },

    // Set filters
    setFilters: (state, action: PayloadAction<Partial<PatientFilters>>) => {
      state.filters = { ...state.filters, ...action.payload };
    },

    // Set pagination
    setPagination: (state, action: PayloadAction<Partial<typeof initialState.pagination>>) => {
      state.pagination = { ...state.pagination, ...action.payload };
    },

    // Search patients
    searchPatients: (state, action: PayloadAction<string>) => {
      const searchTerm = action.payload.toLowerCase();
      if (!searchTerm) {
        state.searchResults = [];
        return;
      }

      state.searchResults = state.patients.filter((patient: UIPatient) =>
        patient.firstName?.toLowerCase().includes(searchTerm) ||
        patient.lastName?.toLowerCase().includes(searchTerm) ||
        patient.mrn?.toLowerCase().includes(searchTerm) ||
        patient.email?.toLowerCase().includes(searchTerm)
      );
    },

    // Clear search
    clearSearch: (state) => {
      state.searchResults = [];
      state.filters.search = '';
    },

    // Clear error
    clearError: (state) => {
      state.error = null;
    },
  },
  extraReducers: (builder) => {
    builder
      // Fetch patients
      .addCase(fetchPatients.pending, (state) => {
        state.loading.fetchPatients = true;
        state.error = null;
      })
      .addCase(fetchPatients.fulfilled, (state, action) => {
        state.loading.fetchPatients = false;
        state.patients = action.payload.patients;
        state.pagination.total = action.payload.total;
        state.pagination.totalPages = Math.ceil(action.payload.total / state.pagination.pageSize);
      })
      .addCase(fetchPatients.rejected, (state, action) => {
        state.loading.fetchPatients = false;
        state.error = action.error.message || 'Failed to fetch patients';
      })

      // Create patient
      .addCase(createPatient.pending, (state) => {
        state.loading.createPatient = true;
        state.error = null;
      })
      .addCase(createPatient.fulfilled, (state, action) => {
        state.loading.createPatient = false;
        state.patients.unshift(action.payload);
        state.pagination.total += 1;
        state.pagination.totalPages = Math.ceil(state.pagination.total / state.pagination.pageSize);
      })
      .addCase(createPatient.rejected, (state, action) => {
        state.loading.createPatient = false;
        state.error = action.error.message || 'Failed to create patient';
      })

      // Update patient
      .addCase(updatePatient.pending, (state) => {
        state.loading.updatePatient = true;
        state.error = null;
      })
      .addCase(updatePatient.fulfilled, (state, action) => {
        state.loading.updatePatient = false;
        const index = state.patients.findIndex((p: UIPatient) => p.id === action.payload.id);
        if (index !== -1) {
          state.patients[index] = action.payload;
        }
        if (state.selectedPatient?.id === action.payload.id) {
          state.selectedPatient = action.payload;
        }
      })
      .addCase(updatePatient.rejected, (state, action) => {
        state.loading.updatePatient = false;
        state.error = action.error.message || 'Failed to update patient';
      })

      // Delete patient
      .addCase(deletePatient.pending, (state) => {
        state.loading.deletePatient = true;
        state.error = null;
      })
      .addCase(deletePatient.fulfilled, (state, action) => {
        state.loading.deletePatient = false;
        state.patients = state.patients.filter((p: UIPatient) => p.id !== action.payload);
        state.pagination.total -= 1;
        state.pagination.totalPages = Math.ceil(state.pagination.total / state.pagination.pageSize);
        if (state.selectedPatient?.id === action.payload) {
          state.selectedPatient = null;
        }
      })
      .addCase(deletePatient.rejected, (state, action) => {
        state.loading.deletePatient = false;
        state.error = action.error.message || 'Failed to delete patient';
      });
  },
});

export const {
  selectPatient,
  setFilters,
  setPagination,
  searchPatients,
  clearSearch,
  clearError,
} = patientsSlice.actions;

export default patientsSlice.reducer;

// Selectors
export const selectPatientsState = (state: { patients: PatientsState }) => state.patients;
export const selectAllPatients = (state: { patients: PatientsState }) => state.patients.patients;
export const selectSelectedPatient = (state: { patients: PatientsState }) => state.patients.selectedPatient;
export const selectPatientsLoading = (state: { patients: PatientsState }) => state.patients.loading;
export const selectPatientsError = (state: { patients: PatientsState }) => state.patients.error;
export const selectPatientsPagination = (state: { patients: PatientsState }) => state.patients.pagination;
export const selectPatientsFilters = (state: { patients: PatientsState }) => state.patients.filters;
export const selectPatientsSearchResults = (state: { patients: PatientsState }) => state.patients.searchResults;