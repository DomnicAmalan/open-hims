// Re-export all types
export * from './common';
export * from './fhir';
export * from './hl7';
export * from './healthcare';
export * from './ui';
// Version information
export const TYPES_VERSION = '1.0.0';
// Type guards
export function isFhirResource(obj) {
    return obj && typeof obj === 'object' && 'resourceType' in obj;
}
export function isFhirPatient(obj) {
    return isFhirResource(obj) && obj.resourceType === 'Patient';
}
export function isFhirObservation(obj) {
    return isFhirResource(obj) && obj.resourceType === 'Observation';
}
export function isFhirEncounter(obj) {
    return isFhirResource(obj) && obj.resourceType === 'Encounter';
}
export function isHl7Message(obj) {
    return obj && typeof obj === 'object' && 'messageType' in obj && 'segments' in obj;
}
export function isHl7AdtMessage(obj) {
    return isHl7Message(obj) && obj.messageType === 'ADT';
}
export function isHl7OrmMessage(obj) {
    return isHl7Message(obj) && obj.messageType === 'ORM';
}
export function isHl7OruMessage(obj) {
    return isHl7Message(obj) && obj.messageType === 'ORU';
}
export function isApiError(obj) {
    return obj && typeof obj === 'object' && 'code' in obj && 'message' in obj;
}
// Utility functions
export function createApiResponse(data, success = true, message) {
    const response = {
        success,
        data,
        timestamp: new Date().toISOString(),
    };
    if (message !== undefined) {
        response.message = message;
    }
    return response;
}
export function createApiError(code, message, details) {
    const error = {
        code,
        message,
    };
    if (details !== undefined) {
        error.details = details;
    }
    return error;
}
export function createPaginatedResponse(data, page, pageSize, total) {
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
