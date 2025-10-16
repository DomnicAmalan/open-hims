use crate::countries::common::*;
use crate::countries::usa::{federal::get_us_federal_config, states::california::get_california_config};
use crate::countries::india::{central::get_india_central_config, states::maharashtra::get_maharashtra_config};
use crate::core::HimsError;

/// Example usage of the inheritance system
pub struct ConfigurationExample;

impl ConfigurationExample {
    /// Demonstrate inheritance chain resolution
    pub fn demonstrate_inheritance() -> Result<(), HimsError> {
        let mut manager = InheritableConfigManager::new();

        // Register configurations
        manager.register_federal_config("US".to_string(), get_us_federal_config());
        manager.register_state_config("CA".to_string(), get_california_config());
        
        manager.register_federal_config("IN".to_string(), get_india_central_config());
        manager.register_state_config("MH".to_string(), get_maharashtra_config());

        // Get effective configuration for California (inherits from US Federal)
        let ca_config = manager.get_effective_config("CA")?;
        println!("California effective regulations: {:?}", ca_config.base_regulations);

        // Get effective configuration for Maharashtra (inherits from India Central)
        let mh_config = manager.get_effective_config("MH")?;
        println!("Maharashtra effective regulations: {:?}", mh_config.base_regulations);

        // Validate compliance chain
        let ca_compliance = manager.validate_compliance_chain("CA", "patient_access")?;
        println!("California compliance checks: {:?}", ca_compliance);

        Ok(())
    }

    /// Create a state that inherits from another state
    pub fn create_similar_state_config() -> InheritableStateConfig {
        // Nevada inherits from California due to similar privacy laws
        InheritableStateConfig {
            state_code: "NV".to_string(),
            state_name: "Nevada".to_string(),
            inherits_from: ConfigInheritance::State("CA".to_string()),
            additional_regulations: vec![
                "Nevada SB 220".to_string(),
            ],
            overrides: Some(ConfigOverrides {
                audit_requirements: None,
                data_retention_years: None,
                additional_privacy_regulations: vec![
                    "Nevada-specific privacy rules".to_string(),
                ],
                stricter_encryption: None,
            }),
            state_specific_requirements: StateSpecificRequirements {
                licensing_authority: "Nevada Medical Board".to_string(),
                professional_licenses_required: vec![
                    "Nevada Medical License".to_string(),
                ],
                telemedicine_rules: TelemedicineRegulations {
                    allowed: true,
                    cross_state_practice: true, // Different from California
                    prescription_restrictions: vec![],
                    required_standards: vec!["HIPAA compliance".to_string()],
                },
                prescription_monitoring: PrescriptionMonitoring {
                    pdmp_required: true,
                    reporting_timeframe_hours: 24,
                    controlled_substances_only: true,
                },
            },
        }
    }

    /// Demonstrate finding states with similar regulations
    pub fn find_similar_states_example(manager: &InheritableConfigManager) -> Vec<String> {
        let ccpa_like_regulations = vec![
            "Consumer Privacy Act".to_string(),
            "Data Breach Notification".to_string(),
            "Right to Delete".to_string(),
        ];

        manager.find_similar_states(&ccpa_like_regulations)
    }
}

/// Trait for creating regulation groups (states with similar laws)
pub trait RegulationGroup {
    fn get_group_name(&self) -> String;
    fn get_member_states(&self) -> Vec<String>;
    fn get_common_regulations(&self) -> Vec<String>;
}

/// Privacy-focused states group (California, Nevada, Virginia, etc.)
pub struct PrivacyFocusedStates;

impl RegulationGroup for PrivacyFocusedStates {
    fn get_group_name(&self) -> String {
        "Privacy-Focused States".to_string()
    }

    fn get_member_states(&self) -> Vec<String> {
        vec![
            "CA".to_string(),
            "NV".to_string(),
            "VA".to_string(),
            "CO".to_string(),
        ]
    }

    fn get_common_regulations(&self) -> Vec<String> {
        vec![
            "Consumer Privacy Rights".to_string(),
            "Data Breach Notification".to_string(),
            "Right to Delete Personal Information".to_string(),
            "Opt-out of Data Sales".to_string(),
        ]
    }
}

/// ABDM integrated states group (Indian states with strong ABDM adoption)
pub struct AbdmIntegratedStates;

impl RegulationGroup for AbdmIntegratedStates {
    fn get_group_name(&self) -> String {
        "ABDM Integrated States".to_string()
    }

    fn get_member_states(&self) -> Vec<String> {
        vec![
            "MH".to_string(), // Maharashtra
            "KA".to_string(), // Karnataka
            "TN".to_string(), // Tamil Nadu
            "KL".to_string(), // Kerala
        ]
    }

    fn get_common_regulations(&self) -> Vec<String> {
        vec![
            "ABDM Consent Framework".to_string(),
            "Health ID Integration".to_string(),
            "Digital Health Records".to_string(),
            "Interoperability Standards".to_string(),
        ]
    }
}