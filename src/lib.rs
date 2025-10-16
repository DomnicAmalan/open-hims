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

/// Main HIMS SDK interface for React Native
#[derive(uniffi::Object)]
pub struct HimsCore {
    inner: Arc<HimsCoreImpl>,
}

struct HimsCoreImpl {
    // Internal implementation
}

#[uniffi::export]
impl HimsCore {
    #[uniffi::constructor]
    pub fn new(config: HimsConfig) -> Self {
        let inner = Arc::new(HimsCoreImpl {});
        Self { inner }
    }

    /// Initialize the HIMS SDK
    pub fn initialize(&self) -> Result<String, HimsError> {
        Ok("HIMS Core SDK initialized successfully".to_string())
    }
}

/// Configuration for HIMS SDK
#[derive(uniffi::Record)]
pub struct HimsConfig {
    pub api_endpoint: String,
    pub auth_token: Option<String>,
    pub enable_logging: bool,
}

/// Common error type for HIMS operations
#[derive(uniffi::Error, Debug, thiserror::Error)]
pub enum HimsError {
    #[error("Authentication failed: {message}")]
    AuthenticationError { message: String },
    #[error("Network error: {message}")]
    NetworkError { message: String },
    #[error("Validation error: {message}")]
    ValidationError { message: String },
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    #[error("Internal error: {message}")]
    InternalError { message: String },
}

uniffi::include_scaffolding!("hims_core_sdk");