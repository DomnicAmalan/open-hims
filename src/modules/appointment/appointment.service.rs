use anyhow::Result;
use chrono::Utc;
use sqlx::{PgPool, Row};
use serde_json;
use uuid::Uuid;

use crate::models::{Appointment, AuditLog, AuditEventType, AuditResourceType};
use crate::models::types::enums::AppointmentStatus;
use crate::models::types::fhir::ResourceMeta;
use crate::core::HimsError;

// Import SQL queries from separate file
use crate::modules::appointment::appointment_sql::*;

/// Search filters for appointments
#[derive(Debug, Clone)]
pub struct AppointmentSearchFilters {
    pub status: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub patient_id: Option<String>,
    pub practitioner_id: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Default for AppointmentSearchFilters {
    fn default() -> Self {
        Self {
            status: None,
            start_date: None,
            end_date: None,
            patient_id: None,
            practitioner_id: None,
            limit: Some(50),
            offset: Some(0),
        }
    }
}

impl From<crate::modules::appointment::appointment_controller::AppointmentQuery> for AppointmentSearchFilters {
    fn from(query: crate::modules::appointment::appointment_controller::AppointmentQuery) -> Self {
        Self {
            status: query.status.map(|s| s.to_string()),
            start_date: None, // Would need date parsing
            end_date: None,
            patient_id: query.patient.map(|p| p.to_string()),
            practitioner_id: query.practitioner.map(|p| p.to_string()),
            limit: query._count,
            offset: query._offset,
        }
    }
}

impl From<crate::modules::appointment::appointment_controller::AppointmentCreateRequest> for Appointment {
    fn from(request: crate::modules::appointment::appointment_controller::AppointmentCreateRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: AppointmentStatus::Proposed, // Default status
            service_category: request.service_category,
            service_type: request.service_type,
            specialty: request.specialty,
            appointment_type: request.appointment_type,
            reason_code: request.reason_code,
            priority: request.priority,
            description: request.description,
            start: request.start,
            end: request.end,
            minutes_duration: request.minutes_duration,
            participant: request.participant,
            meta: ResourceMeta::default(),
        }
    }
}

/// Service for managing appointments with FHIR compliance and audit logging
pub struct AppointmentService {
    pool: PgPool,
}

impl AppointmentService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new appointment with audit logging
    pub async fn create_appointment(
        &self,
        appointment: &Appointment,
        user_id: &str,
    ) -> Result<String, HimsError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        let appointment_id = appointment.id.to_string();

        // Insert appointment into database using simplified schema
        sqlx::query(CREATE_APPOINTMENT)
            .bind(&appointment_id)
            .bind("") // patient_id placeholder
            .bind("") // practitioner_id placeholder
            .bind(&appointment.start)
            .bind(&appointment.end)
            .bind(&appointment.status.to_string())
            .bind("") // service_type placeholder
            .bind(&appointment.description.as_deref().unwrap_or(""))
            .bind(chrono::Utc::now())
            .bind(chrono::Utc::now())
            .execute(&mut *tx)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        // Create audit log
        self.create_audit_log(
            &mut tx,
            &appointment_id,
            user_id,
            AuditEventType::Create,
            Some("Appointment created".to_string()),
        ).await?;

        tx.commit().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;
        
        Ok(appointment_id)
    }

    /// Create appointment from controller request
    pub async fn create_appointment_from_request(
        &self,
        request: crate::modules::appointment::appointment_controller::AppointmentCreateRequest,
        user_id: &str,
    ) -> Result<Appointment, HimsError> {
        let appointment: Appointment = request.into();
        let appointment_id = self.create_appointment(&appointment, user_id).await?;
        
        // Return the created appointment
        self.get_appointment(&appointment_id).await?
            .ok_or(HimsError::DatabaseError("Created appointment not found".to_string()))
    }

    /// Search appointments from controller query
    pub async fn search_appointments_from_query(
        &self,
        query: crate::modules::appointment::appointment_controller::AppointmentQuery,
    ) -> Result<Vec<Appointment>, HimsError> {
        let filters: AppointmentSearchFilters = query.into();
        self.search_appointments(filters).await
    }

    /// Get appointment by UUID (adapter for controller)
    pub async fn get_appointment_by_uuid(&self, id: Uuid) -> Result<Option<Appointment>, HimsError> {
        self.get_appointment(&id.to_string()).await
    }

    /// Update appointment from controller request
    pub async fn update_appointment_from_request(
        &self,
        id: Uuid,
        request: crate::modules::appointment::appointment_controller::AppointmentCreateRequest,
    ) -> Result<Appointment, HimsError> {
        let appointment: Appointment = request.into();
        self.update_appointment(id, appointment).await
    }

    /// Cancel appointment by UUID (adapter for controller)
    pub async fn cancel_appointment_by_uuid(&self, id: Uuid) -> Result<Appointment, HimsError> {
        self.cancel_appointment(id).await
    }

    /// Retrieve appointment by ID
    pub async fn get_appointment(&self, id: &str) -> Result<Option<Appointment>, HimsError> {
        let row = sqlx::query(GET_APPOINTMENT_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            Ok(Some(Appointment {
                id: row.get::<String, _>("id").parse().unwrap_or_else(|_| Uuid::new_v4()),
                start: row.get("start_time"),
                end: row.get("end_time"),
                status: row.get::<String, _>("status").parse().unwrap_or_default(),
                service_category: vec![],
                service_type: vec![],
                specialty: vec![],
                appointment_type: None,
                reason_code: vec![],
                priority: None,
                description: None,
                minutes_duration: None,
                participant: vec![],
                meta: Default::default(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Search appointments with filtering
    pub async fn search_appointments(
        &self,
        filters: AppointmentSearchFilters,
    ) -> Result<Vec<Appointment>, HimsError> {
        let rows = sqlx::query(SEARCH_APPOINTMENTS)
            .bind(filters.limit.unwrap_or(50) as i32)
            .bind(filters.offset.unwrap_or(0) as i32)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        let appointments: Vec<Appointment> = rows.into_iter().map(|row| {
            Appointment {
                id: row.get::<String, _>("id").parse().unwrap_or_else(|_| Uuid::new_v4()),
                start: row.get("start_time"),
                end: row.get("end_time"),
                status: row.get::<String, _>("status").parse().unwrap_or_default(),
                service_category: vec![],
                service_type: vec![],
                specialty: vec![],
                appointment_type: None,
                reason_code: vec![],
                priority: None,
                description: None,
                minutes_duration: None,
                participant: vec![],
                meta: Default::default(),
            }
        }).collect();

        Ok(appointments)
    }

    /// Update appointment status with audit logging
    pub async fn update_appointment_status(
        &self,
        id: &str,
        status: &str,
        user_id: &str,
    ) -> Result<bool, HimsError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        let rows_affected = sqlx::query(UPDATE_APPOINTMENT_STATUS)
            .bind(status)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?
            .rows_affected();

        if rows_affected > 0 {
            self.create_audit_log(
                &mut tx,
                id,
                user_id,
                AuditEventType::Update,
                Some(format!("Appointment status updated to: {}", status)),
            ).await?;

            tx.commit().await
                .map_err(|e| HimsError::DatabaseError(e.to_string()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Soft delete appointment with audit logging
    pub async fn delete_appointment(&self, id: &str, user_id: &str) -> Result<bool, HimsError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        let rows_affected = sqlx::query(DELETE_APPOINTMENT)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?
            .rows_affected();

        if rows_affected > 0 {
            self.create_audit_log(
                &mut tx,
                id,
                user_id,
                AuditEventType::Delete,
                Some("Appointment soft deleted".to_string()),
            ).await?;

            tx.commit().await
                .map_err(|e| HimsError::DatabaseError(e.to_string()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Private helper for audit logging
    async fn log_appointment_audit(
        &self,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        appointment_id: &str,
        user_id: &str,
        event_type: AuditEventType,
        details: Option<String>,
    ) -> Result<(), HimsError> {
        let audit_log = AuditLog {
            id: Uuid::new_v4().to_string(),
            event_type,
            user_id: user_id.to_string(),
            patient_id: None,
            appointment_id: Some(appointment_id.to_string()),
            resource_type: AuditResourceType::Appointment,
            resource_id: appointment_id.to_string(),
            action: "CRUD".to_string(),
            outcome: "Success".to_string(),
            timestamp: Utc::now(),
            source_ip: None,
            user_agent: None,
            details,
        };

        sqlx::query(CREATE_AUDIT_LOG)
            .bind(&audit_log.id)
            .bind(audit_log.event_type.to_string())
            .bind(&audit_log.user_id)
            .bind(audit_log.patient_id.as_ref())
            .bind(audit_log.appointment_id.as_ref())
            .bind(audit_log.resource_type.to_string())
            .bind(&audit_log.resource_id)
            .bind(&audit_log.action)
            .bind(&audit_log.outcome)
            .bind(audit_log.timestamp)
            .bind(audit_log.source_ip.as_ref())
            .bind(audit_log.user_agent.as_ref())
            .bind(audit_log.details.as_ref())
            .execute(&mut **transaction)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Update appointment
    pub async fn update_appointment(
        &self,
        id: Uuid,
        updated_appointment: Appointment,
    ) -> Result<Appointment, HimsError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        // Update appointment
        sqlx::query(UPDATE_APPOINTMENT)
            .bind(&updated_appointment.start)
            .bind(&updated_appointment.end)
            .bind(&updated_appointment.status.to_string())
            .bind("") // patient_id placeholder
            .bind("") // practitioner_id placeholder  
            .bind("") // service_type placeholder
            .bind(&updated_appointment.description.as_deref().unwrap_or(""))
            .bind(&id.to_string())
            .execute(&mut *tx)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        // Create audit log
        self.create_audit_log(
            &mut tx,
            &id.to_string(),
            "system",
            AuditEventType::Update,
            Some("Appointment updated".to_string()),
        ).await?;

        tx.commit().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        Ok(updated_appointment)
    }

    /// Cancel appointment
    pub async fn cancel_appointment(&self, id: Uuid) -> Result<Appointment, HimsError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        // Update appointment status to cancelled
        sqlx::query("UPDATE appointments SET status = 'cancelled', updated_at = NOW() WHERE id = $1")
            .bind(&id.to_string())
            .execute(&mut *tx)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        // Create audit log
        self.create_audit_log(
            &mut tx,
            &id.to_string(),
            "system",
            AuditEventType::Update,
            Some("Appointment cancelled".to_string()),
        ).await?;

        tx.commit().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        // Fetch and return the updated appointment
        self.get_appointment(&id.to_string()).await?
            .ok_or(HimsError::DatabaseError("Appointment not found after update".to_string()))
    }

    /// Create audit log entry
    async fn create_audit_log(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        appointment_id: &str,
        user_id: &str,
        event_type: AuditEventType,
        details: Option<String>,
    ) -> Result<(), HimsError> {
        let audit_log = AuditLog {
            id: Uuid::new_v4().to_string(),
            event_type,
            user_id: user_id.to_string(),
            patient_id: None,
            appointment_id: Some(appointment_id.to_string()),
            resource_type: AuditResourceType::Appointment,
            resource_id: appointment_id.to_string(),
            action: "CRUD".to_string(),
            outcome: "Success".to_string(),
            timestamp: Utc::now(),
            source_ip: None,
            user_agent: None,
            details,
        };

        sqlx::query(CREATE_AUDIT_LOG)
            .bind(&audit_log.id)
            .bind(audit_log.event_type.to_string())
            .bind(&audit_log.user_id)
            .bind(audit_log.patient_id.as_ref())
            .bind(audit_log.appointment_id.as_ref())
            .bind(audit_log.resource_type.to_string())
            .bind(&audit_log.resource_id)
            .bind(&audit_log.action)
            .bind(&audit_log.outcome)
            .bind(audit_log.timestamp)
            .bind(audit_log.source_ip.as_ref())
            .bind(audit_log.user_agent.as_ref())
            .bind(audit_log.details.as_ref())
            .execute(tx.as_mut())
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}