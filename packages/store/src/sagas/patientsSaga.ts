import { takeEvery, call, put } from 'redux-saga/effects';
import { PayloadAction } from '@reduxjs/toolkit';
import { SagaIterator } from 'redux-saga';
import { mockPatientsApi } from '@open-hims/api';
import { 
  UIPatient,
  CreatePatientRequest,
  UpdatePatientRequest,
  PatientFilters,
  PaginatedResponse
} from '@open-hims/types';
import { 
  fetchPatients, 
  createPatient, 
  updatePatient, 
  deletePatient 
} from '../slices/patientsSlice';

// Properly typed saga workers
function* handleFetchPatients(
  action: PayloadAction<{ page?: number; pageSize?: number; filters?: PatientFilters }>
): SagaIterator {
  try {
    const response: PaginatedResponse<UIPatient> = yield call(
      mockPatientsApi.fetchPatients, 
      action.payload
    );
    
    yield put(fetchPatients.fulfilled({
      patients: response.data,
      total: response.pagination.total
    }, '', action.payload));
  } catch (error) {
    yield put(fetchPatients.rejected(error as any, '', action.payload));
  }
}

function* handleCreatePatient(action: PayloadAction<CreatePatientRequest>): SagaIterator {
  try {
    const response: UIPatient = yield call(
      mockPatientsApi.createPatient, 
      action.payload
    );
    
    yield put(createPatient.fulfilled(response, '', action.payload));
  } catch (error) {
    yield put(createPatient.rejected(error as any, '', action.payload));
  }
}

function* handleUpdatePatient(action: PayloadAction<UpdatePatientRequest>): SagaIterator {
  try {
    const response: UIPatient = yield call(
      mockPatientsApi.updatePatient, 
      action.payload
    );
    
    yield put(updatePatient.fulfilled(response, '', action.payload));
  } catch (error) {
    yield put(updatePatient.rejected(error as any, '', action.payload));
  }
}

function* handleDeletePatient(action: PayloadAction<string>): SagaIterator {
  try {
    const response: string = yield call(
      mockPatientsApi.deletePatient, 
      action.payload
    );
    
    yield put(deletePatient.fulfilled(response, '', action.payload));
  } catch (error) {
    yield put(deletePatient.rejected(error as any, '', action.payload));
  }
}

// Export the root patients saga
export function* patientsSaga(): SagaIterator {
  yield takeEvery(fetchPatients.pending.type, handleFetchPatients);
  yield takeEvery(createPatient.pending.type, handleCreatePatient);
  yield takeEvery(updatePatient.pending.type, handleUpdatePatient);
  yield takeEvery(deletePatient.pending.type, handleDeletePatient);
}

// Export individual saga workers for testing
export {
  handleFetchPatients,
  handleCreatePatient,
  handleUpdatePatient,
  handleDeletePatient,
};