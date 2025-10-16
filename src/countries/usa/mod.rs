pub mod states;
pub mod federal;

use crate::countries::{CountryConfig, RegulatoryFramework, AuditRequirements};

/// Get USA configuration
pub fn get_usa_config() -> CountryConfig {
    CountryConfig {
        country_code: "US".to_string(),
        country_name: "United States".to_string(),
        regulatory_framework: RegulatoryFramework {
            primary_authority: "HHS - Department of Health and Human Services".to_string(),
            compliance_standards: vec![
                "HIPAA".to_string(),
                "HITECH".to_string(),
                "21 CFR Part 11".to_string(),
                "SOX".to_string(),
            ],
            audit_requirements: AuditRequirements {
                retention_period_years: 6,
                real_time_monitoring: true,
                third_party_audit_required: true,
            },
        },
        data_localization_required: false,
        supported_standards: vec![
            "FHIR R4".to_string(),
            "HL7v2".to_string(),
            "DICOM".to_string(),
            "X12 EDI".to_string(),
            "C-CDA".to_string(),
            "USCDI".to_string(),
        ],
        privacy_regulations: vec![
            "HIPAA Privacy Rule".to_string(),
            "HIPAA Security Rule".to_string(),
            "HITECH Act".to_string(),
        ],
    }
}

pub use states::*;
pub use federal::*;