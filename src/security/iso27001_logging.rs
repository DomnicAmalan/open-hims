use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// ISO 27001 Security Event Log Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: SecurityEventType,
    pub severity: SecuritySeverity,
    pub source: String,
    pub description: String,
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    LoginSuccess,
    LoginFailure,
    PasswordChange,
    AccountLocked,
    DataAccess,
    DataModification,
    SystemError,
    SecurityViolation,
    ConfigurationChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct Iso27001Logger;

impl Iso27001Logger {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn log_security_event(
        &self,
        event_type: SecurityEventType,
        severity: SecuritySeverity,
        source: String,
        description: String,
        user_id: Option<String>,
        ip_address: Option<String>,
    ) -> Result<(), crate::core::HimsError> {
        let entry = SecurityLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            severity,
            source,
            description,
            user_id,
            ip_address,
            additional_data: None,
        };
        
        log::info!("Security Event: {:?}", entry);
        Ok(())
    }
}

impl Default for Iso27001Logger {
    fn default() -> Self {
        Self::new()
    }
}