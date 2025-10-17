// Core Common Types
export interface BaseEntity {
  id: string;
  createdAt: string;
  updatedAt: string;
  createdBy?: string;
  updatedBy?: string;
  version?: number;
}

export interface PaginationParams {
  page: number;
  pageSize: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    pageSize: number;
    total: number;
    totalPages: number;
    hasNext: boolean;
    hasPrevious: boolean;
  };
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: ApiError;
  message?: string;
  timestamp: string;
}

export interface ApiError {
  code: string;
  message: string;
  details?: Record<string, any>;
  stack?: string;
}

// User and Authentication Types
export interface User extends BaseEntity {
  username: string;
  email: string;
  firstName: string;
  lastName: string;
  role: UserRole;
  status: UserStatus;
  lastLoginAt?: string;
  emailVerified: boolean;
  phoneNumber?: string;
  phoneVerified: boolean;
  preferences: UserPreferences;
  permissions: Permission[];
  organizationId?: string;
}

export type UserRole = 
  | 'super_admin'
  | 'org_admin'
  | 'doctor'
  | 'nurse'
  | 'technician'
  | 'pharmacist'
  | 'receptionist'
  | 'patient'
  | 'viewer';

export type UserStatus = 'active' | 'inactive' | 'suspended' | 'pending_verification';

export interface UserPreferences {
  language: string;
  timezone: string;
  dateFormat: string;
  timeFormat: '12h' | '24h';
  notifications: NotificationPreferences;
  theme: 'light' | 'dark' | 'auto';
}

export interface NotificationPreferences {
  email: boolean;
  sms: boolean;
  push: boolean;
  inApp: boolean;
  appointmentReminders: boolean;
  labResults: boolean;
  medicationReminders: boolean;
  securityAlerts: boolean;
}

export interface Permission {
  resource: string;
  action: string;
  scope?: 'own' | 'organization' | 'all';
  conditions?: Record<string, any>;
}

export interface LoginCredentials {
  username: string;
  password: string;
  rememberMe?: boolean;
  mfaCode?: string;
}

export interface AuthTokens {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
  tokenType: 'Bearer';
}

export interface AuthUser {
  user: User;
  tokens: AuthTokens;
  permissions: Permission[];
}

// Organization and Location Types
export interface Organization extends BaseEntity {
  name: string;
  type: OrganizationType;
  identifier: string; // License number, registration ID
  description?: string;
  website?: string;
  email?: string;
  phone?: string;
  address: Address;
  status: 'active' | 'inactive' | 'suspended';
  accreditations: Accreditation[];
  settings: OrganizationSettings;
}

export type OrganizationType = 
  | 'hospital'
  | 'clinic'
  | 'pharmacy'
  | 'laboratory'
  | 'imaging_center'
  | 'nursing_home'
  | 'home_health'
  | 'insurance'
  | 'government'
  | 'other';

export interface Address {
  line1: string;
  line2?: string;
  city: string;
  state: string;
  postalCode: string;
  country: string;
  coordinates?: {
    latitude: number;
    longitude: number;
  };
}

export interface Accreditation {
  type: 'jci' | 'nabh' | 'nabl' | 'cap' | 'iso' | 'hipaa' | 'other';
  authority: string;
  certificateNumber: string;
  issuedDate: string;
  expiryDate: string;
  status: 'active' | 'expired' | 'suspended' | 'pending';
  documentUrl?: string;
}

export interface OrganizationSettings {
  timezone: string;
  workingHours: WorkingHours;
  holidays: Holiday[];
  departments: Department[];
  integrations: Integration[];
  complianceSettings: ComplianceSettings;
}

export interface WorkingHours {
  monday: DaySchedule;
  tuesday: DaySchedule;
  wednesday: DaySchedule;
  thursday: DaySchedule;
  friday: DaySchedule;
  saturday: DaySchedule;
  sunday: DaySchedule;
}

export interface DaySchedule {
  isWorkingDay: boolean;
  shifts: TimeSlot[];
}

export interface TimeSlot {
  startTime: string; // HH:mm format
  endTime: string; // HH:mm format
  description?: string;
}

export interface Holiday {
  date: string;
  name: string;
  type: 'public' | 'organization' | 'religious' | 'cultural';
  recurring: boolean;
}

export interface Department extends BaseEntity {
  name: string;
  code: string;
  description?: string;
  organizationId: string;
  parentDepartmentId?: string;
  location?: string;
  phone?: string;
  email?: string;
  head?: string; // User ID
  status: 'active' | 'inactive';
}

export interface Integration {
  id: string;
  name: string;
  type: 'ehr' | 'his' | 'lis' | 'ris' | 'pacs' | 'billing' | 'pharmacy' | 'lab' | 'api' | 'file';
  status: 'active' | 'inactive' | 'error' | 'pending';
  configuration: Record<string, any>;
  lastSyncAt?: string;
  errorMessage?: string;
}

export interface ComplianceSettings {
  enabledRegulations: string[];
  auditLogRetentionDays: number;
  dataRetentionPolicies: DataRetentionPolicy[];
  consentManagement: ConsentManagementSettings;
  securitySettings: SecuritySettings;
}

export interface DataRetentionPolicy {
  dataType: string;
  retentionPeriodDays: number;
  archiveAfterDays?: number;
  purgeAfterDays?: number;
  legalHoldExemptions: string[];
}

export interface ConsentManagementSettings {
  requireExplicitConsent: boolean;
  allowConsentWithdrawal: boolean;
  consentExpiryDays?: number;
  consentCategories: ConsentCategory[];
}

export interface ConsentCategory {
  id: string;
  name: string;
  description: string;
  required: boolean;
  purposes: string[];
}

export interface SecuritySettings {
  passwordPolicy: PasswordPolicy;
  sessionTimeoutMinutes: number;
  maxFailedLoginAttempts: number;
  lockoutDurationMinutes: number;
  requireMfa: boolean;
  allowedIpRanges?: string[];
  encryptionStandard: 'aes256' | 'rsa2048' | 'ecc';
}

export interface PasswordPolicy {
  minLength: number;
  requireUppercase: boolean;
  requireLowercase: boolean;
  requireNumbers: boolean;
  requireSpecialChars: boolean;
  preventReuse: number;
  expiryDays?: number;
}

// File and Document Types
export interface FileUpload {
  id: string;
  filename: string;
  originalName: string;
  mimeType: string;
  size: number;
  uploadedBy: string;
  uploadedAt: string;
  path: string;
  url?: string;
  checksum: string;
  encrypted: boolean;
  metadata?: Record<string, any>;
}

export interface Document extends BaseEntity {
  title: string;
  type: DocumentType;
  description?: string;
  content?: string;
  fileId?: string;
  patientId?: string;
  encounterId?: string;
  organizationId: string;
  status: DocumentStatus;
  confidentialityLevel: 'public' | 'internal' | 'confidential' | 'restricted';
  tags: string[];
  signatures: DigitalSignature[];
}

export type DocumentType = 
  | 'medical_record'
  | 'lab_report'
  | 'imaging_report'
  | 'prescription'
  | 'consent_form'
  | 'insurance_claim'
  | 'billing_statement'
  | 'discharge_summary'
  | 'referral_letter'
  | 'policy_document'
  | 'other';

export type DocumentStatus = 'draft' | 'final' | 'amended' | 'superseded' | 'archived';

export interface DigitalSignature {
  signerId: string;
  signerName: string;
  signerRole: string;
  signedAt: string;
  signatureType: 'electronic' | 'digital_certificate' | 'biometric';
  certificate?: string;
  algorithm?: string;
  valid: boolean;
}

// System and Configuration Types
export interface SystemHealth {
  status: 'healthy' | 'degraded' | 'unhealthy';
  services: ServiceHealth[];
  timestamp: string;
  uptime: number;
  version: string;
}

export interface ServiceHealth {
  name: string;
  status: 'healthy' | 'degraded' | 'unhealthy';
  responseTime?: number;
  lastCheck: string;
  details?: Record<string, any>;
}

export interface SystemConfiguration {
  environment: 'development' | 'staging' | 'production';
  version: string;
  features: FeatureFlag[];
  integrations: Integration[];
  maintenance: MaintenanceWindow[];
}

export interface FeatureFlag {
  name: string;
  enabled: boolean;
  description: string;
  rolloutPercentage?: number;
  targetUsers?: string[];
  targetOrganizations?: string[];
  validFrom?: string;
  validUntil?: string;
}

export interface MaintenanceWindow {
  id: string;
  title: string;
  description: string;
  startTime: string;
  endTime: string;
  recurring: boolean;
  affectedServices: string[];
  notificationSent: boolean;
}

// Utility Types
export type LoadingState = 'idle' | 'loading' | 'succeeded' | 'failed';

export interface AsyncState<T> {
  data: T | null;
  loading: LoadingState;
  error: string | null;
}

export type RecursivePartial<T> = {
  [P in keyof T]?: T[P] extends (infer U)[]
    ? RecursivePartial<U>[]
    : T[P] extends object
    ? RecursivePartial<T[P]>
    : T[P];
};