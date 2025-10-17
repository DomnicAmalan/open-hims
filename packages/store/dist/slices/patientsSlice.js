import { createSlice } from '@reduxjs/toolkit';
const initialState = {
    patients: [],
    pagination: {
        total: 0,
        page: 1,
        pageSize: 20,
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
const patientsSlice = createSlice({
    name: 'patients',
    initialState,
    reducers: {
        // Fetch patients
        fetchPatientsStart: (state) => {
            state.loading.fetchPatients = true;
            state.error = null;
        },
        fetchPatientsSuccess: (state, action) => {
            state.loading.fetchPatients = false;
            state.patients = action.payload.patients;
            state.pagination.total = action.payload.total;
        },
        fetchPatientsFailure: (state, action) => {
            state.loading.fetchPatients = false;
            state.error = action.payload;
        },
        // Create patient
        createPatientStart: (state) => {
            state.loading.createPatient = true;
            state.error = null;
        },
        createPatientSuccess: (state, action) => {
            state.loading.createPatient = false;
            state.patients.unshift(action.payload);
            state.pagination.total += 1;
        },
        createPatientFailure: (state, action) => {
            state.loading.createPatient = false;
            state.error = action.payload;
        },
        // Update patient
        updatePatientStart: (state) => {
            state.loading.updatePatient = true;
            state.error = null;
        },
        updatePatientSuccess: (state, action) => {
            state.loading.updatePatient = false;
            const index = state.patients.findIndex(p => p.id === action.payload.id);
            if (index !== -1) {
                state.patients[index] = action.payload;
            }
            if (state.selectedPatient?.id === action.payload.id) {
                state.selectedPatient = action.payload;
            }
        },
        updatePatientFailure: (state, action) => {
            state.loading.updatePatient = false;
            state.error = action.payload;
        },
        // Delete patient
        deletePatientStart: (state) => {
            state.loading.deletePatient = true;
            state.error = null;
        },
        deletePatientSuccess: (state, action) => {
            state.loading.deletePatient = false;
            state.patients = state.patients.filter(p => p.id !== action.payload);
            if (state.selectedPatient?.id === action.payload) {
                state.selectedPatient = null;
            }
            state.pagination.total -= 1;
        },
        deletePatientFailure: (state, action) => {
            state.loading.deletePatient = false;
            state.error = action.payload;
        },
        // Select patient
        selectPatient: (state, action) => {
            state.selectedPatient = action.payload;
        },
        // Update filters
        updateFilters: (state, action) => {
            state.filters = { ...state.filters, ...action.payload };
        },
        // Update pagination
        updatePagination: (state, action) => {
            state.pagination = { ...state.pagination, ...action.payload };
        },
        // Search patients
        searchPatients: (state, action) => {
            state.filters.search = action.payload;
            if (action.payload.trim() === '') {
                state.searchResults = [];
            }
            else {
                const searchTerm = action.payload.toLowerCase();
                state.searchResults = state.patients.filter(patient => patient.firstName.toLowerCase().includes(searchTerm) ||
                    patient.lastName.toLowerCase().includes(searchTerm) ||
                    patient.mrn.toLowerCase().includes(searchTerm) ||
                    patient.email?.toLowerCase().includes(searchTerm));
            }
        },
        // Clear error
        clearError: (state) => {
            state.error = null;
        },
        // Reset state
        resetPatientsState: () => initialState,
    },
});
export const { fetchPatientsStart, fetchPatientsSuccess, fetchPatientsFailure, createPatientStart, createPatientSuccess, createPatientFailure, updatePatientStart, updatePatientSuccess, updatePatientFailure, deletePatientStart, deletePatientSuccess, deletePatientFailure, selectPatient, updateFilters, updatePagination, searchPatients, clearError, resetPatientsState, } = patientsSlice.actions;
export default patientsSlice;
