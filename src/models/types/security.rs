use serde::{Deserialize, Serialize};

/// User role enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "doctor")]
    Doctor,
    #[serde(rename = "nurse")]
    Nurse,
    #[serde(rename = "receptionist")]
    Receptionist,
    #[serde(rename = "patient")]
    Patient,
    #[serde(rename = "technician")]
    Technician,
}

/// Audit event type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "export")]
    Export,
    #[serde(rename = "authentication")]
    Authentication,
    #[serde(rename = "system-access")]
    SystemAccess,
    #[serde(rename = "patient-access")]
    PatientAccess,
    #[serde(rename = "data-modification")]
    DataModification,
}

impl AuditEventType {
    /// Convert from string representation
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "create" => Self::Create,
            "read" => Self::Read,
            "update" => Self::Update,
            "delete" => Self::Delete,
            "access" => Self::Access,
            "export" => Self::Export,
            "authentication" => Self::Authentication,
            "system-access" => Self::SystemAccess,
            "patient-access" => Self::PatientAccess,
            "data-modification" => Self::DataModification,
            _ => Self::Access, // Default fallback
        }
    }
    
    /// Convert to string representation  
    pub fn to_string(&self) -> String {
        match self {
            Self::Create => "create".to_string(),
            Self::Read => "read".to_string(),
            Self::Update => "update".to_string(),
            Self::Delete => "delete".to_string(),
            Self::Access => "access".to_string(),
            Self::Export => "export".to_string(),
            Self::Authentication => "authentication".to_string(),
            Self::SystemAccess => "system-access".to_string(),
            Self::PatientAccess => "patient-access".to_string(),
            Self::DataModification => "data-modification".to_string(),
        }
    }
}

/// Audit action enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "execute")]
    Execute,
}

impl AuditAction {
    /// Convert to string representation  
    pub fn to_string(&self) -> String {
        match self {
            Self::Create => "create".to_string(),
            Self::Read => "read".to_string(),
            Self::Update => "update".to_string(),
            Self::Delete => "delete".to_string(),
            Self::Execute => "execute".to_string(),
        }
    }
}

/// Audit outcome enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditOutcome {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "minor-failure")]
    MinorFailure,
    #[serde(rename = "serious-failure")]
    SeriousFailure,
    #[serde(rename = "major-failure")]
    MajorFailure,
}

impl AuditOutcome {
    /// Convert to string representation  
    pub fn to_string(&self) -> String {
        match self {
            Self::Success => "success".to_string(),
            Self::MinorFailure => "minor-failure".to_string(),
            Self::SeriousFailure => "serious-failure".to_string(),
            Self::MajorFailure => "major-failure".to_string(),
        }
    }
}

/// Audit resource type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResourceType {
    #[serde(rename = "patient")]
    Patient,
    #[serde(rename = "appointment")]
    Appointment,
    #[serde(rename = "medical-record")]
    MedicalRecord,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
}

impl AuditResourceType {
    /// Convert from string representation
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "patient" => Self::Patient,
            "appointment" => Self::Appointment,
            "medical-record" => Self::MedicalRecord,
            "user" => Self::User,
            "system" => Self::System,
            _ => Self::System, // Default fallback
        }
    }
    
    /// Convert to string representation  
    pub fn to_string(&self) -> String {
        match self {
            Self::Patient => "patient".to_string(),
            Self::Appointment => "appointment".to_string(),
            Self::MedicalRecord => "medical-record".to_string(),
            Self::User => "user".to_string(),
            Self::System => "system".to_string(),
        }
    }
}