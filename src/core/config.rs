use serde::{Deserialize, Serialize};

/// HIMS SDK Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HimsConfig {
    pub api_endpoint: String,
    pub auth_token: Option<String>,
    pub enable_logging: bool,
    pub environment: Environment,
    pub security_settings: SecuritySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Testing,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub enable_audit_logging: bool,
    pub enable_encryption: bool,
    pub audit_retention_days: u32,
}

impl Default for HimsConfig {
    fn default() -> Self {
        Self {
            api_endpoint: "https://api.example.com/fhir".to_string(),
            auth_token: None,
            enable_logging: true,
            environment: Environment::Development,
            security_settings: SecuritySettings {
                enable_audit_logging: true,
                enable_encryption: true,
                audit_retention_days: 2555, // 7 years for healthcare data
            },
        }
    }
}