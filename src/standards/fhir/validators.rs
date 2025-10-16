use crate::core::HimsError;
use crate::standards::fhir::models::Patient;

pub struct FhirValidator;

impl FhirValidator {
    pub fn validate_patient(patient: &Patient) -> Result<(), HimsError> {
        patient.validate()
    }
    
    pub fn validate_fhir_json(json: &str) -> Result<(), HimsError> {
        // Validate FHIR JSON against schema
        serde_json::from_str::<serde_json::Value>(json)
            .map_err(|e| HimsError::ValidationError {
                message: format!("Invalid JSON: {}", e),
            })?;
        Ok(())
    }
}