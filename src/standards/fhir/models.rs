use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::core::HimsError;

/// FHIR Patient resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: Option<String>,
    pub active: Option<bool>,
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub address: Vec<Address>,
    pub identifier: Vec<Identifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanName {
    pub use_type: Option<String>,
    pub family: Option<String>,
    pub given: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPoint {
    pub system: Option<String>,
    pub value: Option<String>,
    pub use_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub use_type: Option<String>,
    pub line: Vec<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub use_type: Option<String>,
    pub system: Option<String>,
    pub value: String,
}

/// FHIR Observation resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Option<String>,
    pub status: String,
    pub code: CodeableConcept,
    pub subject: Reference,
    pub effective_date_time: Option<DateTime<Utc>>,
    pub value: Option<ObservationValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeableConcept {
    pub coding: Vec<Coding>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub reference: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ObservationValue {
    Quantity { value: f64, unit: String },
    String { value: String },
    Boolean { value: bool },
}

/// FHIR Bundle resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub id: Option<String>,
    pub bundle_type: String,
    pub total: Option<u32>,
    pub entry: Vec<BundleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleEntry {
    pub resource: serde_json::Value,
    pub full_url: Option<String>,
}

impl Patient {
    pub fn new() -> Self {
        Self {
            id: Some(Uuid::new_v4().to_string()),
            active: Some(true),
            name: Vec::new(),
            telecom: Vec::new(),
            gender: None,
            birth_date: None,
            address: Vec::new(),
            identifier: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), HimsError> {
        if self.name.is_empty() {
            return Err(HimsError::ValidationError {
                message: "Patient must have at least one name".to_string(),
            });
        }
        Ok(())
    }
}