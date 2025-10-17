pub mod usa;
pub mod india;
// TODO: Implement remaining country modules
// pub mod eu;
// pub mod canada;
// pub mod australia;
// pub mod uk;
pub mod common;
pub mod inheritance_examples;

pub use usa::*;
pub use india::*;
// pub use eu::*;
// pub use canada::*;
// pub use australia::*;
// pub use uk::*;
pub use common::*;
pub use inheritance_examples::*;

use serde::{Deserialize, Serialize};
use crate::core::HimsError;

/// Country-specific healthcare configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryConfig {
    pub country_code: String,
    pub country_name: String,
    pub regulatory_framework: RegulatoryFramework,
    pub data_localization_required: bool,
    pub supported_standards: Vec<String>,
    pub privacy_regulations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFramework {
    pub primary_authority: String,
    pub compliance_standards: Vec<String>,
    pub audit_requirements: AuditRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequirements {
    pub retention_period_years: u32,
    pub real_time_monitoring: bool,
    pub third_party_audit_required: bool,
}

/// Country registry for managing different healthcare systems
pub struct CountryRegistry {
    countries: std::collections::HashMap<String, CountryConfig>,
}

impl CountryRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            countries: std::collections::HashMap::new(),
        };
        registry.initialize_default_countries();
        registry
    }

    fn initialize_default_countries(&mut self) {
        // Initialize with default country configurations
        self.register_country(usa::get_usa_config());
        self.register_country(india::get_india_config());
        // TODO: Register remaining countries as they are implemented
        // self.register_country(eu::get_eu_config());
        // self.register_country(canada::get_canada_config());
        // self.register_country(australia::get_australia_config());
        // self.register_country(uk::get_uk_config());
    }

    pub fn register_country(&mut self, config: CountryConfig) {
        self.countries.insert(config.country_code.clone(), config);
    }

    pub fn get_country_config(&self, country_code: &str) -> Result<&CountryConfig, HimsError> {
        self.countries.get(country_code).ok_or_else(|| HimsError::ConfigurationError {
            message: format!("Country configuration not found for: {}", country_code),
        })
    }

    pub fn list_supported_countries(&self) -> Vec<&String> {
        self.countries.keys().collect()
    }
}

impl Default for CountryRegistry {
    fn default() -> Self {
        Self::new()
    }
}