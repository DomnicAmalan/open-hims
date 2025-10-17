/// API Version
pub const API_VERSION: &str = "v1";

/// HTTP Headers
pub const CONTENT_TYPE_FHIR_JSON: &str = "application/fhir+json";
pub const CONTENT_TYPE_JSON: &str = "application/json";
pub const ACCEPT_FHIR_JSON: &str = "application/fhir+json";

/// Pagination
pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;
pub const MIN_PAGE_SIZE: u32 = 1;

/// Rate Limiting
pub const DEFAULT_RATE_LIMIT: u32 = 100; // requests per minute
pub const BURST_RATE_LIMIT: u32 = 10; // burst requests

/// Timeouts (in seconds)
pub const DEFAULT_REQUEST_TIMEOUT: u64 = 30;
pub const LONG_REQUEST_TIMEOUT: u64 = 300; // for bulk operations
pub const DATABASE_TIMEOUT: u64 = 10;

/// Cache TTL (in seconds)
pub const PATIENT_CACHE_TTL: u64 = 300; // 5 minutes
pub const APPOINTMENT_CACHE_TTL: u64 = 60; // 1 minute
pub const METADATA_CACHE_TTL: u64 = 3600; // 1 hour

/// Security
pub const JWT_SECRET_MIN_LENGTH: usize = 32;
pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const SESSION_TIMEOUT: u64 = 3600; // 1 hour in seconds
pub const REFRESH_TOKEN_TTL: u64 = 86400 * 7; // 1 week in seconds

/// File Upload
pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const ALLOWED_FILE_TYPES: &[&str] = &["pdf", "jpg", "jpeg", "png", "doc", "docx"];

/// Database
pub const MAX_CONNECTION_POOL_SIZE: u32 = 10;
pub const MIN_CONNECTION_POOL_SIZE: u32 = 1;
pub const CONNECTION_TIMEOUT: u64 = 30;