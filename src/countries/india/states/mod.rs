// TODO: Implement state modules
// pub mod maharashtra;
// pub mod karnataka;
// pub mod tamil_nadu;
// pub mod kerala;
// pub mod gujarat;
// pub mod rajasthan;
// pub mod uttar_pradesh;
// pub mod west_bengal;
// pub mod telangana;
// pub mod andhra_pradesh;

use serde::{Deserialize, Serialize};
use crate::core::HimsError;

/// Indian state-specific healthcare configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndianStateConfig {
    pub state_code: String,
    pub state_name: String,
    pub local_regulations: Vec<String>,
    pub state_health_authority: String,
    pub abdm_integration_status: AbdmIntegrationStatus,
    pub ayush_regulations: AyushRegulations,
    pub tribal_healthcare_provisions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbdmIntegrationStatus {
    pub hip_enabled: bool,
    pub hiu_enabled: bool,
    pub health_id_adoption_rate: f32,
    pub digital_health_mission_participant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AyushRegulations {
    pub ayurveda_licensed: bool,
    pub yoga_naturopathy_licensed: bool,
    pub unani_licensed: bool,
    pub siddha_licensed: bool,
    pub homeopathy_licensed: bool,
}

pub struct IndianStateRegistry {
    states: std::collections::HashMap<String, IndianStateConfig>,
}

impl IndianStateRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            states: std::collections::HashMap::new(),
        };
        registry.initialize_states();
        registry
    }

    fn initialize_states(&mut self) {
        // TODO: Register states as they are implemented
        // self.register_state(maharashtra::get_maharashtra_config());
        // self.register_state(karnataka::get_karnataka_config());
        // self.register_state(tamil_nadu::get_tamil_nadu_config());
        // self.register_state(kerala::get_kerala_config());
    }

    pub fn register_state(&mut self, config: IndianStateConfig) {
        self.states.insert(config.state_code.clone(), config);
    }

    pub fn get_state_config(&self, state_code: &str) -> Result<&IndianStateConfig, HimsError> {
        self.states.get(state_code).ok_or_else(|| HimsError::ConfigurationError {
            message: format!("Indian state configuration not found for: {}", state_code),
        })
    }
}

impl Default for IndianStateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// pub use maharashtra::*;
// pub use karnataka::*;
// pub use tamil_nadu::*;
// pub use kerala::*;