pub mod california;
pub mod texas;
pub mod florida;
pub mod new_york;
pub mod illinois;
pub mod pennsylvania;
pub mod ohio;
pub mod georgia;
pub mod north_carolina;
pub mod michigan;

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
        self.register_state(california::get_california_config());
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