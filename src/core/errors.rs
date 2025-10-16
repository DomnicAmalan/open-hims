/// Common error types for HIMS SDK
#[derive(Debug, thiserror::Error)]
pub enum HimsError {
    #[error("Authentication failed: {message}")]
    AuthenticationError { message: String },
    
    #[error("Network error: {message}")]
    NetworkError { message: String },
    
    #[error("Validation error: {message}")]
    ValidationError { message: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Internal error: {message}")]
    InternalError { message: String },
    
    #[error("FHIR error: {message}")]
    FhirError { message: String },
    
    #[error("HL7 parsing error: {message}")]
    Hl7Error { message: String },
    
    #[error("DICOM error: {message}")]
    DicomError { message: String },
    
    #[error("Security error: {message}")]
    SecurityError { message: String },
}