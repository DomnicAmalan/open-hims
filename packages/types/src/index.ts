// Re-export all types
export * from './common';
export * from './fhir';
export * from './hl7';
export * from './healthcare';
export * from './ui';

// Version information
export const TYPES_VERSION = '1.0.0';

// Type guards
export function isFhirResource(obj: any): obj is import('./fhir').FhirResource {
  return obj && typeof obj === 'object' && 'resourceType' in obj;
}

export function isFhirPatient(obj: any): obj is import('./fhir').FhirPatient {
  return isFhirResource(obj) && obj.resourceType === 'Patient';
}

export function isFhirObservation(obj: any): obj is import('./fhir').FhirObservation {
  return isFhirResource(obj) && obj.resourceType === 'Observation';
}

export function isFhirEncounter(obj: any): obj is import('./fhir').FhirEncounter {
  return isFhirResource(obj) && obj.resourceType === 'Encounter';
}

export function isHl7Message(obj: any): obj is import('./hl7').Hl7Message {
  return obj && typeof obj === 'object' && 'messageType' in obj && 'segments' in obj;
}

export function isHl7AdtMessage(obj: any): obj is import('./hl7').Hl7AdtMessage {
  return isHl7Message(obj) && obj.messageType === 'ADT';
}

export function isHl7OrmMessage(obj: any): obj is import('./hl7').Hl7OrmMessage {
  return isHl7Message(obj) && obj.messageType === 'ORM';
}

export function isHl7OruMessage(obj: any): obj is import('./hl7').Hl7OruMessage {
  return isHl7Message(obj) && obj.messageType === 'ORU';
}

export function isApiError(obj: any): obj is import('./common').ApiError {
  return obj && typeof obj === 'object' && 'code' in obj && 'message' in obj;
}

// Utility functions
export function createApiResponse<T>(
  data: T,
  success: boolean = true,
  message?: string
): import('./common').ApiResponse<T> {
  const response: import('./common').ApiResponse<T> = {
    success,
    data,
    timestamp: new Date().toISOString(),
  };
  
  if (message !== undefined) {
    response.message = message;
  }
  
  return response;
}

export function createApiError(
  code: string,
  message: string,
  details?: Record<string, any>
): import('./common').ApiError {
  const error: import('./common').ApiError = {
    code,
    message,
  };
  
  if (details !== undefined) {
    error.details = details;
  }
  
  return error;
}

export function createPaginatedResponse<T>(
  data: T[],
  page: number,
  pageSize: number,
  total: number
): import('./common').PaginatedResponse<T> {
  const totalPages = Math.ceil(total / pageSize);
  return {
    data,
    pagination: {
      page,
      pageSize,
      total,
      totalPages,
      hasNext: page < totalPages,
      hasPrevious: page > 1,
    },
  };
}