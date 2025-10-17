use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::types::*;

/// FHIR R4 compliant Appointment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    pub id: Uuid,
    pub status: AppointmentStatus,
    pub service_category: Vec<CodeableConcept>,
    pub service_type: Vec<CodeableConcept>,
    pub specialty: Vec<CodeableConcept>,
    pub appointment_type: Option<CodeableConcept>,
    pub reason_code: Vec<CodeableConcept>,
    pub priority: Option<u32>,
    pub description: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub minutes_duration: Option<u32>,
    pub participant: Vec<AppointmentParticipant>,
    pub meta: ResourceMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentParticipant {
    pub actor: Option<Reference>,
    pub required: Option<ParticipantRequired>,
    pub status: ParticipationStatus,
}

impl Appointment {
    pub fn new(
        service_category: Vec<CodeableConcept>,
        service_type: Vec<CodeableConcept>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: AppointmentStatus::Proposed,
            service_category,
            service_type,
            specialty: Vec::new(),
            appointment_type: None,
            reason_code: Vec::new(),
            priority: None,
            description: None,
            start: Some(start),
            end: Some(end),
            minutes_duration: Some(((end - start).num_minutes() as u32).max(1)),
            participant: Vec::new(),
            meta: ResourceMeta {
                version_id: Some("1".to_string()),
                last_updated: Utc::now(),
                profile: vec![crate::models::constants::APPOINTMENT_PROFILE.to_string()],
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

    pub fn book(&mut self) {
        self.status = AppointmentStatus::Booked;
        self.update_metadata();
    }

    pub fn cancel(&mut self) {
        self.status = AppointmentStatus::Cancelled;
        self.update_metadata();
    }

    pub fn arrive(&mut self) {
        self.status = AppointmentStatus::Arrived;
        self.update_metadata();
    }

    pub fn fulfill(&mut self) {
        self.status = AppointmentStatus::Fulfilled;
        self.update_metadata();
    }

    pub fn add_participant(&mut self, participant: AppointmentParticipant) {
        self.participant.push(participant);
        self.update_metadata();
    }

    pub fn set_duration(&mut self, minutes: u32) {
        self.minutes_duration = Some(minutes);
        if let Some(start) = self.start {
            self.end = Some(start + chrono::Duration::minutes(minutes as i64));
        }
        self.update_metadata();
    }
}