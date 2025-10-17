use sqlx::{PgPool, Row};
use uuid::Uuid;
use anyhow::{Context, Result};
use chrono::Utc;

use crate::models::{Patient, AuditLog, AuditEventType, AuditAction, AuditOutcome};
use crate::modules::patient::patient_controller::PatientCreateRequest;

// Import SQL queries from separate file
use crate::modules::patient::patient_sql::*;

/// Patient service for healthcare business logic
#[derive(Debug, Clone)]
pub struct PatientService {
    pool: PgPool,
}

impl PatientService {
    /// Create new patient service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new patient with FHIR compliance
    pub async fn create_patient(&self, request: PatientCreateRequest) -> Result<Patient> {
        // Create patient with FHIR metadata
        let patient = Patient::new(
            request.name,
            request.telecom,
            request.gender,
            request.birth_date,
        );

        // Begin transaction for data consistency
        let mut tx = self.pool.begin().await
            .context("Failed to begin transaction")?;

        // Insert patient into database using simple query
        let _result = sqlx::query(INSERT_PATIENT)
            .bind(&patient.id)
            .bind(patient.active)
            .bind(serde_json::to_value(&patient.name)?)
            .bind(serde_json::to_value(&patient.telecom)?)
            .bind(patient.gender.to_string())
            .bind(patient.birth_date)
            .bind(serde_json::to_value(&patient.address)?)
            .bind(serde_json::to_value(&patient.marital_status)?)
            .bind(serde_json::to_value(&patient.contact)?)
            .bind(serde_json::to_value(&patient.communication)?)
            .bind(serde_json::to_value(&patient.meta)?)
            .execute(&mut *tx)
            .await
            .context("Failed to insert patient")?;

        // Create audit log with correct event type
        let audit_log = AuditLog::new(
            AuditEventType::Create,
            AuditAction::Create,
            "Patient".to_string(),
        )
        .with_user(Uuid::new_v4()) // TODO: Get from authentication context
        .with_resource(patient.id);

        self.create_audit_log(&mut tx, &audit_log).await?;

        // Commit transaction
        tx.commit().await
            .context("Failed to commit patient creation")?;

        tracing::info!("Patient created successfully: {}", patient.id);
        Ok(patient)
    }

    /// Get patient by ID with audit logging
    pub async fn get_patient(&self, id: Uuid) -> Result<Option<Patient>> {
        // Use simple query instead of macro
        let result = sqlx::query(GET_PATIENT_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch patient")?;

        match result {
            Some(row) => {
                let patient = Patient {
                    id: row.try_get("id")?,
                    active: row.try_get("active")?,
                    name: serde_json::from_value(row.try_get("name")?)
                        .context("Failed to deserialize patient name")?,
                    telecom: serde_json::from_value(row.try_get("telecom")?)
                        .context("Failed to deserialize patient telecom")?,
                    gender: serde_json::from_str(&format!("\"{}\"", row.try_get::<String, _>("gender")?))
                        .context("Failed to deserialize patient gender")?,
                    birth_date: row.try_get("birth_date")?,
                    deceased: None, // TODO: Add to query
                    address: serde_json::from_value(row.try_get("address")?)
                        .context("Failed to deserialize patient address")?,
                    marital_status: serde_json::from_value(row.try_get("marital_status")?)
                        .context("Failed to deserialize patient marital_status")?,
                    contact: serde_json::from_value(row.try_get("contact")?)
                        .context("Failed to deserialize patient contact")?,
                    communication: serde_json::from_value(row.try_get("communication")?)
                        .context("Failed to deserialize patient communication")?,
                    managing_organization: row.try_get::<Option<Uuid>, _>("managing_organization")?.map(|org_id| {
                        crate::models::Reference {
                            reference: format!("Organization/{}", org_id),
                            display: None,
                        }
                    }),
                    meta: serde_json::from_value(row.try_get("meta")?)
                        .context("Failed to deserialize patient meta")?,
                };

                // Log patient access with correct event type
                let audit_log = AuditLog::new(
                    AuditEventType::Access,
                    AuditAction::Read,
                    "Patient".to_string(),
                )
                .with_user(Uuid::new_v4()) // TODO: Get from authentication context
                .with_patient(id)
                .with_resource(id);

                self.create_audit_log_async(&audit_log).await?;

                tracing::info!("Patient retrieved: {}", id);
                Ok(Some(patient))
            }
            None => {
                tracing::warn!("Patient not found: {}", id);
                Ok(None)
            }
        }
    }

    /// Search patients with filters
    pub async fn search_patients(&self, query: crate::modules::patient::patient_controller::PatientQuery) -> Result<Vec<Patient>> {
        let limit = query._count.unwrap_or(20).min(100) as i64;
        let offset = query._offset.unwrap_or(0) as i64;

        // Use simple query with basic pagination
        let rows = sqlx::query(SEARCH_PATIENTS)
            .bind(query.active)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .context("Failed to search patients")?;

        let mut patients = Vec::new();
        for row in rows {
            let patient = Patient {
                id: row.try_get("id")?,
                active: row.try_get("active")?,
                name: serde_json::from_value(row.try_get("name")?)
                    .context("Failed to deserialize patient name")?,
                telecom: serde_json::from_value(row.try_get("telecom")?)
                    .context("Failed to deserialize patient telecom")?,
                gender: serde_json::from_str(&format!("\"{}\"", row.try_get::<String, _>("gender")?))
                    .context("Failed to deserialize patient gender")?,
                birth_date: row.try_get("birth_date")?,
                deceased: None,
                address: serde_json::from_value(row.try_get("address")?)
                    .context("Failed to deserialize patient address")?,
                marital_status: serde_json::from_value(row.try_get("marital_status")?)
                    .context("Failed to deserialize patient marital_status")?,
                contact: serde_json::from_value(row.try_get("contact")?)
                    .context("Failed to deserialize patient contact")?,
                communication: serde_json::from_value(row.try_get("communication")?)
                    .context("Failed to deserialize patient communication")?,
                managing_organization: row.try_get::<Option<Uuid>, _>("managing_organization")?.map(|org_id| {
                    crate::models::Reference {
                        reference: format!("Organization/{}", org_id),
                        display: None,
                    }
                }),
                meta: serde_json::from_value(row.try_get("meta")?)
                    .context("Failed to deserialize patient meta")?,
            };
            patients.push(patient);
        }

        tracing::info!("Found {} patients", patients.len());
        Ok(patients)
    }

    /// Update patient
    pub async fn update_patient(&self, id: Uuid, request: PatientCreateRequest) -> Result<Patient> {
        let pool = &self.pool;
        
        // Begin transaction
        let mut tx = pool.begin().await
            .context("Failed to begin transaction")?;

        // Update patient
        let updated_at = Utc::now();
        sqlx::query(UPDATE_PATIENT)
            .bind(id)
            .bind(serde_json::to_value(&request.name)?)
            .bind(serde_json::to_value(&request.telecom)?)
            .bind(request.gender.to_string())
            .bind(request.birth_date)
            .bind(serde_json::to_value(&request.address)?)
            .bind(serde_json::to_value(&request.marital_status)?)
            .bind(serde_json::to_value(&request.contact)?)
            .bind(serde_json::to_value(&request.communication)?)
            .bind(serde_json::to_value(updated_at.to_rfc3339())?)
            .execute(&mut *tx)
            .await
            .context("Failed to update patient")?;

        // Create audit log
        let audit_log = AuditLog::new(
            AuditEventType::Update,
            AuditAction::Update,
            "Patient".to_string(),
        )
        .with_user(Uuid::new_v4()) // TODO: Get from authentication context
        .with_resource(id);

        self.create_audit_log(&mut tx, &audit_log).await?;

        // Commit transaction
        tx.commit().await
            .context("Failed to commit patient update")?;

        // Fetch updated patient
        self.get_patient(id).await?
            .ok_or_else(|| anyhow::anyhow!("Patient not found after update"))
    }

    /// Soft delete patient
    pub async fn delete_patient(&self, id: Uuid) -> Result<bool> {
        let pool = &self.pool;
        
        let rows_affected = sqlx::query(DELETE_PATIENT)
            .bind(id)
            .execute(pool)
            .await
            .context("Failed to delete patient")?
            .rows_affected();

        if rows_affected > 0 {
            // Create audit log
            let audit_log = AuditLog::new(
                AuditEventType::Delete,
                AuditAction::Delete,
                "Patient".to_string(),
            )
            .with_user(Uuid::new_v4()) // TODO: Get from authentication context
            .with_resource(id);

            self.create_audit_log_async(&audit_log).await?;
            tracing::info!("Patient soft deleted: {}", id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Create audit log entry within transaction
    async fn create_audit_log(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, audit_log: &AuditLog) -> Result<()> {
        sqlx::query(INSERT_AUDIT_LOG)
            .bind(&audit_log.id)
            .bind(audit_log.event_type.to_string())
            .bind(&audit_log.user_id)
            .bind(audit_log.patient_id.as_ref())
            .bind(audit_log.resource_type.to_string())
            .bind(&audit_log.resource_id)
            .bind(audit_log.action.to_string())
            .bind(audit_log.outcome.to_string())
            .bind(audit_log.timestamp)
            .bind(audit_log.details.as_ref())
            .execute(&mut **tx)
            .await
            .context("Failed to create audit log")?;

        Ok(())
    }

    /// Create audit log entry asynchronously
    async fn create_audit_log_async(&self, audit_log: &AuditLog) -> Result<()> {
        let pool = &self.pool;
        
        sqlx::query(INSERT_AUDIT_LOG)
            .bind(&audit_log.id)
            .bind(audit_log.event_type.to_string())
            .bind(&audit_log.user_id)
            .bind(audit_log.patient_id.as_ref())
            .bind(audit_log.resource_type.to_string())
            .bind(&audit_log.resource_id)
            .bind(audit_log.action.to_string())
            .bind(audit_log.outcome.to_string())
            .bind(audit_log.timestamp)
            .bind(audit_log.details.as_ref())
            .execute(pool)
            .await
            .context("Failed to create audit log")?;

        Ok(())
    }
}

// Add trait implementations for enum serialization
impl ToString for crate::models::Gender {
    fn to_string(&self) -> String {
        match self {
            crate::models::Gender::Male => "male".to_string(),
            crate::models::Gender::Female => "female".to_string(),
            crate::models::Gender::Other => "other".to_string(),
            crate::models::Gender::Unknown => "unknown".to_string(),
        }
    }
}

impl ToString for AuditEventType {
    fn to_string(&self) -> String {
        match self {
            AuditEventType::Create => "create".to_string(),
            AuditEventType::Read => "read".to_string(),
            AuditEventType::Update => "update".to_string(),
            AuditEventType::Delete => "delete".to_string(),
            AuditEventType::Access => "access".to_string(),
            AuditEventType::Export => "export".to_string(),
            AuditEventType::Authentication => "authentication".to_string(),
            AuditEventType::SystemAccess => "system-access".to_string(),
            AuditEventType::PatientAccess => "patient-access".to_string(),
            AuditEventType::DataModification => "data-modification".to_string(),
        }
    }
}

impl ToString for AuditAction {
    fn to_string(&self) -> String {
        match self {
            AuditAction::Create => "create".to_string(),
            AuditAction::Read => "read".to_string(),
            AuditAction::Update => "update".to_string(),
            AuditAction::Delete => "delete".to_string(),
            AuditAction::Execute => "execute".to_string(),
        }
    }
}

impl ToString for AuditOutcome {
    fn to_string(&self) -> String {
        match self {
            AuditOutcome::Success => "success".to_string(),
            AuditOutcome::MinorFailure => "minor-failure".to_string(),
            AuditOutcome::SeriousFailure => "serious-failure".to_string(),
            AuditOutcome::MajorFailure => "major-failure".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_patient_service_creation() {
        // This would require a test database setup
        // For now, just test that the service can be created
        // TODO: Add proper integration tests with test database
    }
}