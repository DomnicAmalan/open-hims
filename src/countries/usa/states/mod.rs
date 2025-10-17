pub mod california;
pub mod texas;
pub mod florida;
pub mod new_york;
pub mod illinois;
// TODO: Implement remaining state modules
// pub mod pennsylvania;
// pub mod ohio;
// pub mod georgia;
// pub mod north_carolina;
// pub mod michigan;

use serde::{Deserialize, Serialize};
use crate::core::HimsError;

/// State-specific healthcare configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    pub state_code: String,
    pub state_name: String,
    pub additional_regulations: Vec<String>,
    pub state_licensing_requirements: Vec<String>,
    pub data_breach_notification_laws: Vec<String>,
    pub telemedicine_regulations: TelemedicineRegulations,
    pub prescription_monitoring: PrescriptionMonitoring,
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

/// State registry for US healthcare regulations
pub struct UsStateRegistry {
    states: std::collections::HashMap<String, StateConfig>,
}

impl UsStateRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            states: std::collections::HashMap::new(),
        };
        registry.initialize_states();
        registry
    }

    fn initialize_states(&mut self) {
        // Convert InheritableStateConfig to StateConfig for California
        let ca_inheritable = california::get_california_config();
        let ca_config = StateConfig {
            state_code: ca_inheritable.state_code,
            state_name: ca_inheritable.state_name,
            additional_regulations: ca_inheritable.additional_regulations,
            state_licensing_requirements: vec!["California Medical Board License".to_string()],
            data_breach_notification_laws: vec!["California Civil Code Section 1798.82".to_string()],
            telemedicine_regulations: TelemedicineRegulations {
                allowed: true,
                cross_state_practice: false,
                prescription_restrictions: vec!["Controlled substances require in-person visit".to_string()],
                required_standards: vec!["CMBC Telemedicine Guidelines".to_string()],
            },
            prescription_monitoring: PrescriptionMonitoring {
                pdmp_required: true,
                reporting_timeframe_hours: 24,
                controlled_substances_only: true,
            },
        };
        self.register_state(ca_config);
        
        self.register_state(texas::get_texas_config());
        self.register_state(florida::get_florida_config());
        self.register_state(new_york::get_new_york_config());
        self.register_state(illinois::get_illinois_config());
        // Add more states as needed
    }

    pub fn register_state(&mut self, config: StateConfig) {
        self.states.insert(config.state_code.clone(), config);
    }

    pub fn get_state_config(&self, state_code: &str) -> Result<&StateConfig, HimsError> {
        self.states.get(state_code).ok_or_else(|| HimsError::ConfigurationError {
            message: format!("State configuration not found for: {}", state_code),
        })
    }

    pub fn validate_compliance(&self, state_code: &str, operation: &str) -> Result<bool, HimsError> {
        let config = self.get_state_config(state_code)?;
        // Implement state-specific compliance validation
        Ok(true) // Placeholder
    }
}

impl Default for UsStateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub use california::*;
pub use texas::*;
pub use florida::*;
pub use new_york::*;
pub use illinois::*;