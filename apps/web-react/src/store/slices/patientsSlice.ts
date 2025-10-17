import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import type { FhirPatient, FhirIdentifier, FhirHumanName, FhirAddress, FhirContactPoint } from '@open-hims/types';

// Patient interface extending FHIR Patient
export interface Patient extends FhirPatient {
  mrn?: string; // Medical Record Number
  insuranceInfo?: {
    provider: string;
    policyNumber: string;
    groupNumber: string;
  };
}

// Patient search filters
export interface PatientFilters {
  searchTerm: string;
  gender: string;
  ageRange: { min: number; max: number };
  country: string;
  state: string;
  active: boolean | null;
}

// Patients state interface
export interface PatientsState {
  patients: Patient[];
  selectedPatient: Patient | null;
  filters: PatientFilters;
  pagination: {
    page: number;
    pageSize: number;
    total: number;
  };
  loading: {
    fetchPatients: boolean;
    createPatient: boolean;
    updatePatient: boolean;
    deletePatient: boolean;
  };
  error: string | null;
  searchResults: Patient[];
}

// Initial state
const initialState: PatientsState = {
  patients: [],
  selectedPatient: null,
  filters: {
    searchTerm: '',
    gender: '',
    ageRange: { min: 0, max: 120 },
    country: '',
    state: '',
    active: null,
  },
  pagination: {
    page: 1,
    pageSize: 10,
    total: 0,
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

// Mock patients data
const mockPatients: Patient[] = [
  {
    id: '1',
    name: [{ family: 'Doe', given: ['John', 'William'] }],
    gender: 'male',
    birthDate: '1985-06-15',
    identifier: [
      { system: 'SSN', value: '123-45-6789' },
      { system: 'MRN', value: 'MRN001' }
    ],
    address: [{
      line: ['123 Main St', 'Apt 4B'],
      city: 'San Francisco',
      state: 'CA',
      postalCode: '94102',
      country: 'US'
    }],
    telecom: [
      { system: 'phone', value: '+1-555-123-4567', use: 'home' },
      { system: 'email', value: 'john.doe@email.com', use: 'work' }
    ],
    active: true,
    createdAt: '2024-01-15T10:30:00Z',
    updatedAt: '2024-10-16T14:20:00Z',
    mrn: 'MRN001',
    insuranceInfo: {
      provider: 'Blue Cross Blue Shield',
      policyNumber: 'BCBS123456',
      groupNumber: 'GRP789'
    }
  },
  {
    id: '2',
    name: [{ family: 'Smith', given: ['Jane', 'Elizabeth'] }],
    gender: 'female',
    birthDate: '1990-03-22',
    identifier: [
      { system: 'SSN', value: '987-65-4321' },
      { system: 'MRN', value: 'MRN002' }
    ],
    address: [{
      line: ['456 Oak Avenue'],
      city: 'Los Angeles',
      state: 'CA',
      postalCode: '90210',
      country: 'US'
    }],
    telecom: [
      { system: 'phone', value: '+1-555-987-6543', use: 'mobile' },
      { system: 'email', value: 'jane.smith@email.com', use: 'work' }
    ],
    active: true,
    createdAt: '2024-02-20T09:15:00Z',
    updatedAt: '2024-10-15T16:45:00Z',
    mrn: 'MRN002',
    insuranceInfo: {
      provider: 'Aetna',
      policyNumber: 'AETNA789012',
      groupNumber: 'GRP456'
    }
  },
  {
    id: '3',
    name: [{ family: 'Patel', given: ['Raj', 'Kumar'] }],
    gender: 'male',
    birthDate: '1978-11-30',
    identifier: [
      { system: 'ABHA', value: '12-3456-7890-1234' },
      { system: 'MRN', value: 'MRN003' }
    ],
    address: [{
      line: ['789 Tech Park', 'Building A, Floor 5'],
      city: 'Mumbai',
      state: 'MH',
      postalCode: '400001',
      country: 'IN'
    }],
    telecom: [
      { system: 'phone', value: '+91-98765-43210', use: 'mobile' },
      { system: 'email', value: 'raj.patel@techcorp.in', use: 'work' }
    ],
    active: true,
    createdAt: '2024-03-10T11:00:00Z',
    updatedAt: '2024-10-16T13:30:00Z',
    mrn: 'MRN003',
    insuranceInfo: {
      provider: 'Star Health Insurance',
      policyNumber: 'STAR345678',
      groupNumber: 'GRP123'
    }
  }
];

// Patients slice
const patientsSlice = createSlice({
  name: 'patients',
  initialState: {
    ...initialState,
    patients: mockPatients,
    pagination: {
      ...initialState.pagination,
      total: mockPatients.length,
    },
  },
  reducers: {
    // Fetch patients actions
    fetchPatientsRequest: (state) => {
      state.loading.fetchPatients = true;
      state.error = null;
    },
    fetchPatientsSuccess: (state, action: PayloadAction<{ patients: Patient[]; total: number }>) => {
      state.loading.fetchPatients = false;
      state.patients = action.payload.patients;
      state.pagination.total = action.payload.total;
      state.error = null;
    },
    fetchPatientsFailure: (state, action: PayloadAction<string>) => {
      state.loading.fetchPatients = false;
      state.error = action.payload;
    },

    // Create patient actions
    createPatientRequest: (state, action: PayloadAction<Omit<Patient, 'id' | 'createdAt' | 'updatedAt'>>) => {
      state.loading.createPatient = true;
      state.error = null;
    },
    createPatientSuccess: (state, action: PayloadAction<Patient>) => {
      state.loading.createPatient = false;
      state.patients.unshift(action.payload);
      state.pagination.total += 1;
      state.error = null;
    },
    createPatientFailure: (state, action: PayloadAction<string>) => {
      state.loading.createPatient = false;
      state.error = action.payload;
    },

    // Update patient actions
    updatePatientRequest: (state, action: PayloadAction<{ id: string; updates: Partial<Patient> }>) => {
      state.loading.updatePatient = true;
      state.error = null;
    },
    updatePatientSuccess: (state, action: PayloadAction<Patient>) => {
      state.loading.updatePatient = false;
      const index = state.patients.findIndex(p => p.id === action.payload.id);
      if (index !== -1) {
        state.patients[index] = action.payload;
      }
      if (state.selectedPatient?.id === action.payload.id) {
        state.selectedPatient = action.payload;
      }
      state.error = null;
    },
    updatePatientFailure: (state, action: PayloadAction<string>) => {
      state.loading.updatePatient = false;
      state.error = action.payload;
    },

    // Delete patient actions
    deletePatientRequest: (state, action: PayloadAction<string>) => {
      state.loading.deletePatient = true;
      state.error = null;
    },
    deletePatientSuccess: (state, action: PayloadAction<string>) => {
      state.loading.deletePatient = false;
      state.patients = state.patients.filter(p => p.id !== action.payload);
      state.pagination.total -= 1;
      if (state.selectedPatient?.id === action.payload) {
        state.selectedPatient = null;
      }
      state.error = null;
    },
    deletePatientFailure: (state, action: PayloadAction<string>) => {
      state.loading.deletePatient = false;
      state.error = action.payload;
    },

    // Search patients
    searchPatientsRequest: (state, action: PayloadAction<string>) => {
      state.loading.fetchPatients = true;
      state.filters.searchTerm = action.payload;
    },
    searchPatientsSuccess: (state, action: PayloadAction<Patient[]>) => {
      state.loading.fetchPatients = false;
      state.searchResults = action.payload;
    },

    // Select patient
    selectPatient: (state, action: PayloadAction<Patient | null>) => {
      state.selectedPatient = action.payload;
    },

    // Update filters
    updateFilters: (state, action: PayloadAction<Partial<PatientFilters>>) => {
      state.filters = { ...state.filters, ...action.payload };
    },

    // Update pagination
    updatePagination: (state, action: PayloadAction<Partial<typeof initialState.pagination>>) => {
      state.pagination = { ...state.pagination, ...action.payload };
    },

    // Clear error
    clearError: (state) => {
      state.error = null;
    },

    // Reset state
    resetPatientsState: () => initialState,
  },
});

// Export actions
export const {
  fetchPatientsRequest,
  fetchPatientsSuccess,
  fetchPatientsFailure,
  createPatientRequest,
  createPatientSuccess,
  createPatientFailure,
  updatePatientRequest,
  updatePatientSuccess,
  updatePatientFailure,
  deletePatientRequest,
  deletePatientSuccess,
  deletePatientFailure,
  searchPatientsRequest,
  searchPatientsSuccess,
  selectPatient,
  updateFilters,
  updatePagination,
  clearError,
  resetPatientsState,
} = patientsSlice.actions;

// Export default reducer
export default patientsSlice;