import { UIPatient, CreatePatientRequest, UpdatePatientRequest, PatientFilters, PatientListState } from '@open-hims/types';
export interface PatientsState extends Omit<PatientListState, 'loading'> {
    loading: {
        fetchPatients: boolean;
        createPatient: boolean;
        updatePatient: boolean;
        deletePatient: boolean;
    };
}
export declare const fetchPatients: import("@reduxjs/toolkit").AsyncThunk<{
    patients: UIPatient[];
    total: number;
}, {
    page?: number;
    pageSize?: number;
    filters?: PatientFilters;
}, {
    state?: unknown;
    dispatch?: import("@reduxjs/toolkit").Dispatch;
    extra?: unknown;
    rejectValue?: unknown;
    serializedErrorType?: unknown;
    pendingMeta?: unknown;
    fulfilledMeta?: unknown;
    rejectedMeta?: unknown;
}>;
export declare const createPatient: import("@reduxjs/toolkit").AsyncThunk<UIPatient, CreatePatientRequest, {
    state?: unknown;
    dispatch?: import("@reduxjs/toolkit").Dispatch;
    extra?: unknown;
    rejectValue?: unknown;
    serializedErrorType?: unknown;
    pendingMeta?: unknown;
    fulfilledMeta?: unknown;
    rejectedMeta?: unknown;
}>;
export declare const updatePatient: import("@reduxjs/toolkit").AsyncThunk<UIPatient, UpdatePatientRequest, {
    state?: unknown;
    dispatch?: import("@reduxjs/toolkit").Dispatch;
    extra?: unknown;
    rejectValue?: unknown;
    serializedErrorType?: unknown;
    pendingMeta?: unknown;
    fulfilledMeta?: unknown;
    rejectedMeta?: unknown;
}>;
export declare const deletePatient: import("@reduxjs/toolkit").AsyncThunk<string, string, {
    state?: unknown;
    dispatch?: import("@reduxjs/toolkit").Dispatch;
    extra?: unknown;
    rejectValue?: unknown;
    serializedErrorType?: unknown;
    pendingMeta?: unknown;
    fulfilledMeta?: unknown;
    rejectedMeta?: unknown;
}>;
export declare const selectPatient: import("@reduxjs/toolkit").ActionCreatorWithPayload<UIPatient | null, "patients/selectPatient">, setFilters: import("@reduxjs/toolkit").ActionCreatorWithPayload<Partial<PatientFilters>, "patients/setFilters">, setPagination: import("@reduxjs/toolkit").ActionCreatorWithPayload<Partial<{
    page: number;
    pageSize: number;
    total: number;
    totalPages: number;
}>, "patients/setPagination">, searchPatients: import("@reduxjs/toolkit").ActionCreatorWithPayload<string, "patients/searchPatients">, clearSearch: import("@reduxjs/toolkit").ActionCreatorWithoutPayload<"patients/clearSearch">, clearError: import("@reduxjs/toolkit").ActionCreatorWithoutPayload<"patients/clearError">;
declare const _default: import("@reduxjs/toolkit").Reducer<PatientsState>;
export default _default;
export declare const selectPatientsState: (state: {
    patients: PatientsState;
}) => PatientsState;
export declare const selectAllPatients: (state: {
    patients: PatientsState;
}) => UIPatient[];
export declare const selectSelectedPatient: (state: {
    patients: PatientsState;
}) => UIPatient | null;
export declare const selectPatientsLoading: (state: {
    patients: PatientsState;
}) => {
    fetchPatients: boolean;
    createPatient: boolean;
    updatePatient: boolean;
    deletePatient: boolean;
};
export declare const selectPatientsError: (state: {
    patients: PatientsState;
}) => string | null;
export declare const selectPatientsPagination: (state: {
    patients: PatientsState;
}) => {
    page: number;
    pageSize: number;
    total: number;
    totalPages: number;
};
export declare const selectPatientsFilters: (state: {
    patients: PatientsState;
}) => PatientFilters;
export declare const selectPatientsSearchResults: (state: {
    patients: PatientsState;
}) => UIPatient[];
