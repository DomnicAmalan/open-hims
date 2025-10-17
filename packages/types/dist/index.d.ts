export * from './common';
export * from './fhir';
export * from './hl7';
export * from './healthcare';
export * from './ui';
export declare const TYPES_VERSION = "1.0.0";
export declare function isFhirResource(obj: any): obj is import('./fhir').FhirResource;
export declare function isFhirPatient(obj: any): obj is import('./fhir').FhirPatient;
export declare function isFhirObservation(obj: any): obj is import('./fhir').FhirObservation;
export declare function isFhirEncounter(obj: any): obj is import('./fhir').FhirEncounter;
export declare function isHl7Message(obj: any): obj is import('./hl7').Hl7Message;
export declare function isHl7AdtMessage(obj: any): obj is import('./hl7').Hl7AdtMessage;
export declare function isHl7OrmMessage(obj: any): obj is import('./hl7').Hl7OrmMessage;
export declare function isHl7OruMessage(obj: any): obj is import('./hl7').Hl7OruMessage;
export declare function isApiError(obj: any): obj is import('./common').ApiError;
export declare function createApiResponse<T>(data: T, success?: boolean, message?: string): import('./common').ApiResponse<T>;
export declare function createApiError(code: string, message: string, details?: Record<string, any>): import('./common').ApiError;
export declare function createPaginatedResponse<T>(data: T[], page: number, pageSize: number, total: number): import('./common').PaginatedResponse<T>;
//# sourceMappingURL=index.d.ts.map