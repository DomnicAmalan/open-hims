// UI-specific types for healthcare applications
// These are simplified types optimized for frontend display and forms

export interface UIPatient {
  id: string;
  mrn: string;
  firstName: string;
  lastName: string;
  dateOfBirth: string;
  gender: 'male' | 'female' | 'other' | 'unknown';
  phone?: string;
  email?: string;
  address?: UIAddress;
  emergencyContact?: UIContact;
  insurance?: UIInsurance;
  createdAt: string;
  updatedAt: string;
}

export interface UIAddress {
  line1: string;
  line2?: string;
  city: string;
  state: string;
  postalCode: string;
  country: string;
}

export interface UIContact {
  name: string;
  relationship: string;
  phone: string;
  email?: string;
}

export interface UIInsurance {
  provider: string;
  policyNumber: string;
  groupNumber?: string;
  subscriberName?: string;
  effectiveDate?: string;
  expirationDate?: string;
}

// Request types for API operations
export interface CreatePatientRequest {
  patient: Partial<UIPatient>;
}

export interface UpdatePatientRequest {
  id: string;
  patient: Partial<UIPatient>;
}

export interface DeletePatientRequest {
  id: string;
  reason?: string;
}

// Filter and search types
export interface PatientFilters {
  search: string;
  gender?: string;
  ageRange?: {
    min: number;
    max: number;
  };
  state?: string;
  insuranceProvider?: string;
  dateRange?: {
    startDate: string;
    endDate: string;
  };
}

export interface PatientSearchQuery {
  query: string;
  filters?: PatientFilters;
  sortBy?: 'firstName' | 'lastName' | 'dateOfBirth' | 'createdAt' | 'mrn';
  sortOrder?: 'asc' | 'desc';
  page?: number;
  pageSize?: number;
}

// UI State types for components
export interface PatientListState {
  patients: UIPatient[];
  loading: boolean;
  error: string | null;
  pagination: {
    page: number;
    pageSize: number;
    total: number;
    totalPages: number;
  };
  selectedPatient: UIPatient | null;
  filters: PatientFilters;
  searchResults: UIPatient[];
}

export interface PatientFormState {
  patient: Partial<UIPatient>;
  errors: Record<string, string>;
  touched: Record<string, boolean>;
  isSubmitting: boolean;
  isDirty: boolean;
}

// Common UI types
export interface UILoadingStates {
  [key: string]: boolean;
}

export interface UIErrorStates {
  [key: string]: string | null;
}

// Theme and styling types
export interface HealthcareTheme {
  colors: {
    primary: string;
    secondary: string;
    success: string;
    warning: string;
    error: string;
    info: string;
    background: string;
    surface: string;
    text: {
      primary: string;
      secondary: string;
      disabled: string;
    };
  };
  spacing: {
    xs: number;
    sm: number;
    md: number;
    lg: number;
    xl: number;
  };
  borderRadius: {
    sm: number;
    md: number;
    lg: number;
  };
  shadows: {
    sm: string;
    md: string;
    lg: string;
  };
}

// Component prop types
export interface PatientCardProps {
  patient: UIPatient;
  onClick?: (patient: UIPatient) => void;
  showActions?: boolean;
  compact?: boolean;
}

export interface PatientFormProps {
  patient?: Partial<UIPatient>;
  onSubmit: (patient: UIPatient) => void;
  onCancel?: () => void;
  loading?: boolean;
  errors?: Record<string, string>;
}

export interface PatientListProps {
  patients: UIPatient[];
  loading?: boolean;
  error?: string | null;
  onPatientSelect?: (patient: UIPatient) => void;
  onPatientEdit?: (patient: UIPatient) => void;
  onPatientDelete?: (patientId: string) => void;
  filters?: PatientFilters;
  onFiltersChange?: (filters: PatientFilters) => void;
}

// Data transformation types
export interface FhirToUIPatientTransform {
  (fhirPatient: import('./fhir').FhirPatient): UIPatient;
}

export interface UIToFhirPatientTransform {
  (uiPatient: UIPatient): import('./fhir').FhirPatient;
}

// Validation types
export interface ValidationRule {
  required?: boolean;
  minLength?: number;
  maxLength?: number;
  pattern?: RegExp;
  custom?: (value: any) => string | null;
}

export interface ValidationSchema {
  [fieldName: string]: ValidationRule[];
}

export interface PatientValidationSchema {
  firstName: ValidationRule[];
  lastName: ValidationRule[];
  dateOfBirth: ValidationRule[];
  mrn: ValidationRule[];
  email?: ValidationRule[];
  phone?: ValidationRule[];
}