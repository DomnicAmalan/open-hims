use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::types::*;

/// Audit log entry for healthcare compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub event_type: AuditEventType,
    pub user_id: String,
    pub patient_id: Option<String>,
    pub appointment_id: Option<String>,
    pub resource_type: AuditResourceType,
    pub resource_id: String,
    pub action: String,
    pub outcome: String,
    pub timestamp: DateTime<Utc>,
    pub source_ip: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<String>,
}

impl AuditLog {
    pub fn new(
        event_type: AuditEventType,
        action: AuditAction,
        resource_type: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type,
            user_id: String::new(),
            patient_id: None,
            appointment_id: None,
            resource_type: AuditResourceType::from_string(&resource_type),
            resource_id: String::new(),
            action: action.to_string(),
            outcome: AuditOutcome::Success.to_string(),
            timestamp: Utc::now(),
            source_ip: None,
            user_agent: None,
            details: None,
        }
    }

    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = user_id.to_string();
        self
    }

    pub fn with_patient(mut self, patient_id: Uuid) -> Self {
        self.patient_id = Some(patient_id.to_string());
        self
    }

    pub fn with_resource(mut self, resource_id: Uuid) -> Self {
        self.resource_id = resource_id.to_string();
        self
    }

    pub fn with_outcome(mut self, outcome: AuditOutcome) -> Self {
        self.outcome = outcome.to_string();
        self
    }

    pub fn with_source_info(mut self, ip: Option<String>, user_agent: Option<String>) -> Self {
        self.source_ip = ip;
        self.user_agent = user_agent;
        self
    }

    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    pub fn patient_access(patient_id: Uuid, user_id: Uuid, action: AuditAction) -> Self {
        Self::new(
            AuditEventType::PatientAccess,
            action,
            "Patient".to_string(),
        )
        .with_patient(patient_id)
        .with_user(user_id)
        .with_resource(patient_id)
    }

    pub fn authentication_event(user_id: Option<Uuid>, outcome: AuditOutcome) -> Self {
        let mut log = Self::new(
            AuditEventType::Authentication,
            AuditAction::Execute,
            "User".to_string(),
        )
        .with_outcome(outcome);
        
        if let Some(uid) = user_id {
            log = log.with_user(uid).with_resource(uid);
        }
        
        log
    }

    pub fn data_modification(
        resource_type: String,
        resource_id: Uuid,
        user_id: Uuid,
        action: AuditAction,
    ) -> Self {
        Self::new(
            AuditEventType::DataModification,
            action,
            resource_type,
        )
        .with_user(user_id)
        .with_resource(resource_id)
    }
}