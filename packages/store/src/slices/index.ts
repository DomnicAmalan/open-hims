// Re-export all slices and their actions/selectors
// This file keeps exports thin and organized

export {
  default as patientsReducer,
  fetchPatients,
  createPatient,
  updatePatient,
  deletePatient,
  selectPatient,
  setFilters,
  setPagination,
  searchPatients,
  clearSearch,
  clearError,
  selectPatientsState,
  selectAllPatients,
  selectSelectedPatient,
  selectPatientsLoading,
  selectPatientsError,
  selectPatientsPagination,
  selectPatientsFilters,
  selectPatientsSearchResults,
  type PatientsState,
} from './patientsSlice';

// Export additional slices here as they are created
// export { default as complianceReducer } from './complianceSlice.js';
// export { default as auditReducer } from './auditSlice.js';
// export { default as hl7Reducer } from './hl7Slice.js';