use serde::{Deserialize, Serialize};
use crate::core::HimsError;
use std::collections::HashMap;

/// Base trait for healthcare regulations that can be inherited
pub trait HealthcareRegulation {
    fn get_base_regulations(&self) -> Vec<String>;
    fn get_audit_requirements(&self) -> AuditRequirements;
    fn get_privacy_regulations(&self) -> Vec<String>;
    fn validate_compliance(&self, operation: &str) -> Result<bool, HimsError>;
}

/// Base configuration that can be inherited by countries and states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseHealthcareConfig {
    pub base_regulations: Vec<String>,
    pub audit_requirements: AuditRequirements,
    pub privacy_regulations: Vec<String>,
    pub data_retention_years: u32,
    pub encryption_required: bool,
}

/// Federal/National level configuration (inherited by states)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederalConfig {
    pub base: BaseHealthcareConfig,
    pub federal_authority: String,
    pub interstate_data_sharing_rules: Vec<String>,
    pub national_standards: Vec<String>,
}

/// State configuration that inherits from federal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritableStateConfig {
    pub state_code: String,
    pub state_name: String,
    pub inherits_from: ConfigInheritance,
    pub additional_regulations: Vec<String>,
    pub overrides: Option<ConfigOverrides>,
    pub state_specific_requirements: StateSpecificRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigInheritance {
    Federal(FederalConfig),
    State(String), // Inherit from another state
    Custom(BaseHealthcareConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOverrides {
    pub audit_requirements: Option<AuditRequirements>,
    pub data_retention_years: Option<u32>,
    pub additional_privacy_regulations: Vec<String>,
    pub stricter_encryption: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSpecificRequirements {
    pub licensing_authority: String,
    pub professional_licenses_required: Vec<String>,
    pub telemedicine_rules: TelemedicineRegulations,
    pub prescription_monitoring: PrescriptionMonitoring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequirements {
    pub retention_period_years: u32,
    pub real_time_monitoring: bool,
    pub third_party_audit_required: bool,
    pub log_encryption_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemedicineRegulations {
    pub allowed: bool,
    pub cross_state_practice: bool,
    pub prescription_restrictions: Vec<String>,
    pub required_standards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrescriptionMonitoring {
    pub pdmp_required: bool,
    pub reporting_timeframe_hours: u32,
    pub controlled_substances_only: bool,
}

/// Inheritable configuration manager
pub struct InheritableConfigManager {
    federal_configs: HashMap<String, FederalConfig>,
    state_configs: HashMap<String, InheritableStateConfig>,
    inheritance_cache: HashMap<String, BaseHealthcareConfig>,
}

impl InheritableConfigManager {
    pub fn new() -> Self {
        Self {
            federal_configs: HashMap::new(),
            state_configs: HashMap::new(),
            inheritance_cache: HashMap::new(),
        }
    }

    /// Register a federal configuration
    pub fn register_federal_config(&mut self, country_code: String, config: FederalConfig) {
        self.federal_configs.insert(country_code, config);
    }

    /// Register a state configuration
    pub fn register_state_config(&mut self, state_key: String, config: InheritableStateConfig) {
        self.state_configs.insert(state_key, config);
    }

    /// Get effective configuration for a state (with inheritance resolved)
    pub fn get_effective_config(&mut self, state_key: &str) -> Result<BaseHealthcareConfig, HimsError> {
        // Check cache first
        if let Some(cached) = self.inheritance_cache.get(state_key) {
            return Ok(cached.clone());
        }

        let state_config = self.state_configs.get(state_key)
            .ok_or_else(|| HimsError::ConfigurationError {
                message: format!("State configuration not found: {}", state_key),
            })?;

        let mut effective_config = self.resolve_inheritance(&state_config.inherits_from)?;

        // Apply additional regulations
        effective_config.base_regulations.extend(state_config.additional_regulations.clone());

        // Apply overrides
        if let Some(overrides) = &state_config.overrides {
            if let Some(audit_req) = &overrides.audit_requirements {
                effective_config.audit_requirements = audit_req.clone();
            }
            if let Some(retention) = overrides.data_retention_years {
                effective_config.data_retention_years = retention;
            }
            effective_config.privacy_regulations.extend(overrides.additional_privacy_regulations.clone());
            if let Some(encryption) = overrides.stricter_encryption {
                effective_config.encryption_required = encryption;
            }
        }

        // Cache the result
        self.inheritance_cache.insert(state_key.to_string(), effective_config.clone());

        Ok(effective_config)
    }

    fn resolve_inheritance(&self, inheritance: &ConfigInheritance) -> Result<BaseHealthcareConfig, HimsError> {
        match inheritance {
            ConfigInheritance::Federal(federal_config) => Ok(federal_config.base.clone()),
            ConfigInheritance::State(parent_state) => {
                // Recursive inheritance from another state
                let parent_config = self.state_configs.get(parent_state)
                    .ok_or_else(|| HimsError::ConfigurationError {
                        message: format!("Parent state configuration not found: {}", parent_state),
                    })?;
                self.resolve_inheritance(&parent_config.inherits_from)
            },
            ConfigInheritance::Custom(base_config) => Ok(base_config.clone()),
        }
    }

    /// Find states with similar regulations (for compliance grouping)
    pub fn find_similar_states(&self, regulations: &[String]) -> Vec<String> {
        let mut similar_states = Vec::new();

        for (state_key, state_config) in &self.state_configs {
            let state_regulations = &state_config.additional_regulations;
            let common_count = regulations.iter()
                .filter(|reg| state_regulations.contains(reg))
                .count();

            // If more than 50% of regulations match, consider it similar
            if common_count > regulations.len() / 2 {
                similar_states.push(state_key.clone());
            }
        }

        similar_states
    }

    /// Validate compliance across inheritance chain
    pub fn validate_compliance_chain(&mut self, state_key: &str, operation: &str) -> Result<Vec<ComplianceCheck>, HimsError> {
        let effective_config = self.get_effective_config(state_key)?;
        let state_config = self.state_configs.get(state_key).unwrap();

        let mut compliance_checks = Vec::new();

        // Check federal compliance
        if let ConfigInheritance::Federal(federal_config) = &state_config.inherits_from {
            compliance_checks.push(ComplianceCheck {
                level: "Federal".to_string(),
                authority: federal_config.federal_authority.clone(),
                compliant: true, // Placeholder
                requirements_checked: federal_config.base.base_regulations.clone(),
            });
        }

        // Check state compliance
        compliance_checks.push(ComplianceCheck {
            level: "State".to_string(),
            authority: state_config.state_specific_requirements.licensing_authority.clone(),
            compliant: true, // Placeholder
            requirements_checked: state_config.additional_regulations.clone(),
        });

        Ok(compliance_checks)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub level: String,
    pub authority: String,
    pub compliant: bool,
    pub requirements_checked: Vec<String>,
}

impl Default for InheritableConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper trait for creating inheritance hierarchies
pub trait ConfigInheritanceBuilder {
    fn inherit_from_federal(federal_config: FederalConfig) -> ConfigInheritance;
    fn inherit_from_state(state_code: String) -> ConfigInheritance;
    fn create_custom(base_config: BaseHealthcareConfig) -> ConfigInheritance;
}

impl ConfigInheritanceBuilder for ConfigInheritance {
    fn inherit_from_federal(federal_config: FederalConfig) -> ConfigInheritance {
        ConfigInheritance::Federal(federal_config)
    }

    fn inherit_from_state(state_code: String) -> ConfigInheritance {
        ConfigInheritance::State(state_code)
    }

    fn create_custom(base_config: BaseHealthcareConfig) -> ConfigInheritance {
        ConfigInheritance::Custom(base_config)
    }
}