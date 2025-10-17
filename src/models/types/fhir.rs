use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// FHIR Reference structure for linking resources
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Reference {
    pub reference: String,
    pub display: Option<String>,
}

/// FHIR CodeableConcept for coded values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeableConcept {
    pub coding: Vec<Coding>,
    pub text: Option<String>,
}

/// FHIR Coding for individual codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coding {
    pub system: Option<String>,
    pub version: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

/// FHIR Resource metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceMeta {
    pub version_id: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub profile: Vec<String>,
    pub security: Vec<Coding>,
    pub tag: Vec<Coding>,
}