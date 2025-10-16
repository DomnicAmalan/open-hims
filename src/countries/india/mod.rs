pub mod states;
pub mod central;

use crate::countries::{CountryConfig, RegulatoryFramework, AuditRequirements};

/// Get India configuration
pub fn get_india_config() -> CountryConfig {
    CountryConfig {
        country_code: "IN".to_string(),
        country_name: "India".to_string(),
        regulatory_framework: RegulatoryFramework {
            primary_authority: "Ministry of Health and Family Welfare".to_string(),
            compliance_standards: vec![
                "ABDM Guidelines".to_string(),
                "Digital Personal Data Protection Act 2023".to_string(),
                "Information Technology Act 2000".to_string(),
                "Clinical Establishments Act".to_string(),
            ],
            audit_requirements: AuditRequirements {
                retention_period_years: 5,
                real_time_monitoring: true,
                third_party_audit_required: false,
            },
        },
        data_localization_required: true,
        supported_standards: vec![
            "FHIR R4".to_string(),
            "HL7v2".to_string(),
            "SNOMED CT".to_string(),
            "ICD-10".to_string(),
            "ABDM Standards".to_string(),
        ],
        privacy_regulations: vec![
            "Digital Personal Data Protection Act 2023".to_string(),
            "Information Technology (Reasonable Security Practices) Rules 2011".to_string(),
        ],
    }
}

pub use states::*;
pub use central::*;