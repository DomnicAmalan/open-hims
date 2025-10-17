import { all, fork } from 'redux-saga/effects';
import { SagaIterator } from 'redux-saga';
import { patientsSaga } from './patientsSaga';

// Root saga that forks all feature sagas
export function* rootSaga(): SagaIterator {
  yield all([
    fork(patientsSaga),
    // Add more feature sagas here as they are created
    // fork(complianceSaga),
    // fork(auditSaga),
    // fork(hl7Saga),
  ]);
}

// Re-export all sagas for direct access if needed
export {
  patientsSaga,
  handleFetchPatients,
  handleCreatePatient,
  handleUpdatePatient,
  handleDeletePatient,
} from './patientsSaga';