use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::types::*;

/// Medical Record model for clinical documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalRecord {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub record_type: MedicalRecordType,
    pub status: DocumentStatus,
    pub subject: Reference,
    pub author: Vec<Reference>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub meta: ResourceMeta,
}

impl MedicalRecord {
    pub fn new(
        patient_id: Uuid,
        record_type: MedicalRecordType,
        content: String,
        author: Reference,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            patient_id,
            encounter_id: None,
            record_type,
            status: DocumentStatus::Preliminary,
            subject: Reference {
                reference: format!("Patient/{}", patient_id),
                display: None,
            },
            author: vec![author],
            content,
            created_at: now,
            updated_at: now,
            meta: ResourceMeta {
                version_id: Some("1".to_string()),
                last_updated: now,
                profile: Vec::new(),
                security: Vec::new(),
                tag: Vec::new(),
            },
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now();
        self.update_metadata();
    }

    pub fn finalize(&mut self) {
        self.status = DocumentStatus::Final;
        self.update_metadata();
    }

    pub fn amend(&mut self, new_content: String) {
        self.content = new_content;
        self.status = DocumentStatus::Amended;
        self.updated_at = Utc::now();
        self.update_metadata();
    }

    pub fn mark_error(&mut self) {
        self.status = DocumentStatus::EnteredInError;
        self.update_metadata();
    }

    fn update_metadata(&mut self) {
        self.meta.last_updated = Utc::now();
        if let Some(version) = &self.meta.version_id {
            if let Ok(v) = version.parse::<u32>() {
                self.meta.version_id = Some((v + 1).to_string());
            }
        }
    }
}