/// Healthcare-specific validation rules

/// Patient validation
pub const MIN_PATIENT_AGE: u8 = 0;
pub const MAX_PATIENT_AGE: u8 = 150;

/// Phone number validation
pub const PHONE_NUMBER_MIN_LENGTH: usize = 10;
pub const PHONE_NUMBER_MAX_LENGTH: usize = 15;

/// Name validation
pub const NAME_MIN_LENGTH: usize = 1;
pub const NAME_MAX_LENGTH: usize = 50;

/// Medical record validation
pub const MEDICAL_RECORD_CONTENT_MAX_LENGTH: usize = 10000;
pub const MEDICAL_RECORD_TITLE_MAX_LENGTH: usize = 200;

/// Appointment validation
pub const MAX_APPOINTMENT_DURATION_MINUTES: u32 = 480; // 8 hours
pub const MIN_APPOINTMENT_DURATION_MINUTES: u32 = 15;
pub const MAX_FUTURE_APPOINTMENT_DAYS: u32 = 365; // 1 year

/// Email validation
pub const EMAIL_MAX_LENGTH: usize = 254;

/// Address validation
pub const ADDRESS_LINE_MAX_LENGTH: usize = 100;
pub const CITY_MAX_LENGTH: usize = 50;
pub const STATE_MAX_LENGTH: usize = 50;
pub const POSTAL_CODE_MAX_LENGTH: usize = 10;
pub const COUNTRY_MAX_LENGTH: usize = 50;

/// Audit log retention
pub const AUDIT_LOG_RETENTION_DAYS: u32 = 2555; // 7 years for healthcare compliance