use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Utility functions for HIMS SDK
pub struct HimsUtils;

impl HimsUtils {
    /// Generate a new UUID
    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// Get current UTC timestamp
    pub fn current_timestamp() -> DateTime<Utc> {
        Utc::now()
    }

    /// Validate email format
    pub fn validate_email(email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }

    /// Sanitize string for logging
    pub fn sanitize_for_logs(input: &str) -> String {
        // Remove potentially sensitive information
        input.replace("password", "***")
             .replace("token", "***")
             .replace("secret", "***")
    }
}