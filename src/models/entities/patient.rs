use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::types::*;

/// FHIR R4 compliant Patient model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    /// Unique identifier for the patient
    pub id: Uuid,
    
    /// Active status of the patient record
    pub active: bool,
    
    /// Patient name components
    pub name: Vec<HumanName>,
    
    /// Contact details (phone, email, etc.)
    pub telecom: Vec<ContactPoint>,
    
    /// Administrative gender
    pub gender: Gender,
    
    /// Date of birth
    pub birth_date: Option<chrono::NaiveDate>,
    
    /// Deceased status
    pub deceased: Option<bool>,
    
    /// Addresses
    pub address: Vec<Address>,
    
    /// Marital status
    pub marital_status: Option<CodeableConcept>,
    
    /// Emergency contacts
    pub contact: Vec<PatientContact>,
    
    /// Communication preferences
    pub communication: Vec<PatientCommunication>,
    
    /// Managing organization
    pub managing_organization: Option<Reference>,
    
    /// Record metadata
    pub meta: ResourceMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientContact {
    pub relationship: Vec<CodeableConcept>,
    pub name: Option<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub address: Option<Address>,
    pub gender: Option<Gender>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCommunication {
    pub language: CodeableConcept,
    pub preferred: Option<bool>,
}

impl Patient {
    pub fn new(
        name: Vec<HumanName>,
        telecom: Vec<ContactPoint>,
        gender: Gender,
        birth_date: Option<chrono::NaiveDate>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            active: true,
            name,
            telecom,
            gender,
            birth_date,
            deceased: None,
            address: Vec::new(),
            marital_status: None,
            contact: Vec::new(),
            communication: Vec::new(),
            managing_organization: None,
            meta: ResourceMeta {
                version_id: Some("1".to_string()),
                last_updated: Utc::now(),
                profile: vec![crate::models::constants::PATIENT_PROFILE.to_string()],
                security: Vec::new(),
                tag: Vec::new(),
            },
        }
    }

    pub fn update_metadata(&mut self) {
        self.meta.last_updated = Utc::now();
        if let Some(version) = &self.meta.version_id {
            if let Ok(v) = version.parse::<u32>() {
                self.meta.version_id = Some((v + 1).to_string());
            }
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.update_metadata();
    }

    pub fn add_contact(&mut self, contact: PatientContact) {
        self.contact.push(contact);
        self.update_metadata();
    }

    pub fn add_address(&mut self, address: Address) {
        self.address.push(address);
        self.update_metadata();
    }
}