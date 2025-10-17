use hims_core_sdk::countries::usa::states::{CaliforniaHealthcareConfig, NevadaHealthcareConfig};
use hims_core_sdk::countries::usa::USAHealthcareConfig;
use hims_core_sdk::countries::common::{BaseHealthcareConfig, InheritableStateConfig};
use hims_core_sdk::countries::india::{IndiaHealthcareConfig, states::MaharashtraHealthcareConfig};

#[cfg(test)]
mod healthcare_regulation_tests {
    use super::*;

    #[test]
    fn test_usa_federal_config() {
        let usa_config = USAHealthcareConfig::new();
        
        assert!(usa_config.is_hipaa_required());
        assert!(usa_config.requires_audit_logs());
        assert_eq!(usa_config.get_audit_retention_years(), 6);
        assert_eq!(usa_config.get_data_retention_years(), 7);
    }

    #[test]
    fn test_california_inheritance() {
        let ca_config = CaliforniaHealthcareConfig::new();
        
        // Should inherit federal requirements
        assert!(ca_config.is_hipaa_required());
        assert!(ca_config.requires_audit_logs());
        
        // California-specific requirements
        assert!(ca_config.is_ccpa_required());
        assert!(ca_config.requires_breach_notification());
        assert_eq!(ca_config.get_breach_notification_hours(), 72);
        
        // Should have stricter data retention
        assert_eq!(ca_config.get_data_retention_years(), 10);
    }

    #[test]
    fn test_nevada_inheritance_from_california() {
        let nv_config = NevadaHealthcareConfig::new();
        
        // Should inherit from California (which inherits from federal)
        assert!(nv_config.is_hipaa_required());
        assert!(nv_config.is_ccpa_required());
        assert!(nv_config.requires_breach_notification());
        
        // Nevada-specific modifications
        assert_eq!(nv_config.get_breach_notification_hours(), 48);
        assert_eq!(nv_config.get_data_retention_years(), 8);
    }

    #[test]
    fn test_india_config() {
        let india_config = IndiaHealthcareConfig::new();
        
        assert!(india_config.is_abdm_required());
        assert!(india_config.requires_consent_management());
        assert!(india_config.supports_ayushman_bharat());
        assert_eq!(india_config.get_data_retention_years(), 5);
    }

    #[test]
    fn test_maharashtra_inheritance() {
        let mh_config = MaharashtraHealthcareConfig::new();
        
        // Should inherit central Indian requirements
        assert!(mh_config.is_abdm_required());
        assert!(mh_config.requires_consent_management());
        
        // Maharashtra-specific requirements
        assert!(mh_config.requires_marathi_support());
        assert!(mh_config.has_state_health_card());
        assert_eq!(mh_config.get_data_retention_years(), 7);
    }

    #[test]
    fn test_config_serialization() {
        let ca_config = CaliforniaHealthcareConfig::new();
        let json = serde_json::to_string(&ca_config).expect("Should serialize");
        
        let deserialized: CaliforniaHealthcareConfig = 
            serde_json::from_str(&json).expect("Should deserialize");
        
        assert_eq!(ca_config.get_data_retention_years(), deserialized.get_data_retention_years());
    }

    #[test]
    fn test_inheritance_chain() {
        let nv_config = NevadaHealthcareConfig::new();
        
        // Test that inheritance chain works properly
        // Nevada -> California -> USA Federal
        assert!(nv_config.is_hipaa_required()); // From federal
        assert!(nv_config.is_ccpa_required()); // From California
        assert_eq!(nv_config.get_breach_notification_hours(), 48); // Nevada override
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_multiple_countries_comparison() {
        let usa_config = USAHealthcareConfig::new();
        let india_config = IndiaHealthcareConfig::new();
        
        // Different countries should have different requirements
        assert!(usa_config.is_hipaa_required());
        assert!(!india_config.is_hipaa_required());
        
        assert!(india_config.is_abdm_required());
        assert!(!usa_config.is_abdm_required());
    }

    #[test]
    fn test_state_variations_within_country() {
        let ca_config = CaliforniaHealthcareConfig::new();
        let nv_config = NevadaHealthcareConfig::new();
        
        // Both should have federal requirements
        assert!(ca_config.is_hipaa_required());
        assert!(nv_config.is_hipaa_required());
        
        // But different state-specific timelines
        assert_eq!(ca_config.get_breach_notification_hours(), 72);
        assert_eq!(nv_config.get_breach_notification_hours(), 48);
    }
}