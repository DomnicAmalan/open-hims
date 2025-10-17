use std::sync::Arc;
use uniffi::*;

// Core modules
pub mod core;
pub mod standards;
pub mod security;
pub mod exporters;
pub mod countries;

// Re-exports for easier access
pub use crate::core::*;
pub use crate::standards::*;
pub use crate::security::*;
pub use crate::exporters::*;
pub use crate::countries::*;

/// Configuration for HIMS SDK
pub struct HimsConfig {
    pub api_endpoint: String,
    pub auth_token: Option<String>,
    pub enable_logging: bool,
    pub country_code: Option<String>,
    pub state_code: Option<String>,
}

/// Compliance check result
pub struct ComplianceCheck {
    pub level: String,
    pub authority: String,
    pub compliant: bool,
    pub requirements_checked: Vec<String>,
}

/// Common error type for HIMS operations
#[derive(Debug, thiserror::Error)]
pub enum HimsError {
    #[error("Authentication failed: {message}")]
    AuthenticationError { message: String },
    #[error("Network error: {message}")]
    NetworkError { message: String },
    #[error("Validation error: {field}: {message}")]
    ValidationError { field: String, message: String },
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    #[error("Internal error: {message}")]
    InternalError { message: String },
}

/// Main HIMS SDK interface for React Native
pub struct HimsCore {
    inner: Arc<HimsCoreImpl>,
}

struct HimsCoreImpl {
    config: HimsConfig,
}

impl HimsCore {
    pub fn new(config: HimsConfig) -> Self {
        let inner = Arc::new(HimsCoreImpl { config });
        Self { inner }
    }

    /// Initialize the HIMS SDK
    pub fn initialize(&self) -> String {
        "HIMS Core SDK initialized successfully".to_string()
    }

    /// Get supported countries
    pub fn get_supported_countries(&self) -> Result<Vec<String>, HimsError> {
        Ok(vec!["US".to_string(), "IN".to_string()])
    }

    /// Get supported states for a country
    pub fn get_supported_states(&self, country_code: String) -> Result<Vec<String>, HimsError> {
        match country_code.as_str() {
            "US" => Ok(vec!["CA".to_string(), "TX".to_string(), "FL".to_string(), "NY".to_string(), "IL".to_string()]),
            "IN" => Ok(vec![]), // States not yet implemented
            _ => Err(HimsError::ValidationError {
                field: "country_code".to_string(),
                message: format!("Unsupported country: {}", country_code),
            }),
        }
    }

    /// Validate compliance for a country/state and operation
    pub fn validate_compliance(
        &self,
        country_code: String,
        state_code: Option<String>,
        operation: String,
    ) -> Result<bool, HimsError> {
        // TODO: Implement actual validation logic using country modules
        Ok(true)
    }

    /// Get compliance requirements for a country/state
    pub fn get_compliance_requirements(
        &self,
        country_code: String,
        state_code: Option<String>,
    ) -> Result<Vec<ComplianceCheck>, HimsError> {
        let checks = vec![
            ComplianceCheck {
                level: "Federal".to_string(),
                authority: if country_code == "US" {
                    "HHS - Department of Health and Human Services".to_string()
                } else {
                    "Ministry of Health".to_string()
                },
                compliant: true,
                requirements_checked: vec!["Data Privacy".to_string(), "Security Standards".to_string()],
            },
        ];
        Ok(checks)
    }
}

/// Get SDK version
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

uniffi::setup_scaffolding!();