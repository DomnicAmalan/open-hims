use sqlx::{PgPool, Row};
use uuid::Uuid;
use anyhow::{Context, Result};
use chrono::Utc;

use crate::models::{MedicalRecord, AuditLog, AuditEventType, AuditAction, AuditOutcome};
use crate::models::{MedicalRecordType, DocumentStatus, Reference, ResourceMeta};
use crate::core::HimsError;

// Import SQL queries from separate file
use crate::modules::medical_record::medical_record_sql::*;

/// Medical record service for healthcare business logic
#[derive(Debug, Clone)]
pub struct MedicalRecordService {
    pool: PgPool,
}

impl MedicalRecordService {
    /// Create new medical record service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new medical record with FHIR compliance
    pub async fn create_medical_record(
        &self,
        medical_record: &MedicalRecord,
        user_id: &str,
    ) -> Result<String, HimsError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        let record_id = medical_record.id.to_string();

        // Insert medical record into database
        let _result = sqlx::query(INSERT_MEDICAL_RECORD)
            .bind(&record_id)
            .bind(medical_record.patient_id)
            .bind(medical_record.encounter_id)
            .bind(serde_json::to_string(&medical_record.record_type).unwrap())
            .bind(serde_json::to_string(&medical_record.status).unwrap())
            .bind(serde_json::to_value(&medical_record.subject).unwrap())
            .bind(serde_json::to_value(&medical_record.author).unwrap())
            .bind(&medical_record.content)
            .bind(medical_record.created_at)
            .bind(medical_record.updated_at)
            .bind(serde_json::to_value(&medical_record.meta).unwrap())
            .execute(&mut *tx)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        // Create audit log
        self.log_medical_record_audit(
            &mut tx,
            &record_id,
            user_id,
            AuditEventType::Create,
            Some("Medical record created".to_string()),
        ).await?;

        tx.commit().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        Ok(record_id)
    }

    /// Get a specific medical record by ID
    pub async fn get_medical_record(&self, id: &str) -> Result<Option<MedicalRecord>, HimsError> {
        let row = sqlx::query(GET_MEDICAL_RECORD_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            let record = MedicalRecord {
                id: row.get("id"),
                patient_id: row.get("patient_id"),
                encounter_id: row.get("encounter_id"),
                record_type: serde_json::from_str(&row.get::<String, _>("record_type")).unwrap_or_default(),
                status: serde_json::from_str(&row.get::<String, _>("status")).unwrap_or_default(),
                subject: serde_json::from_value(row.get("subject")).unwrap_or_default(),
                author: serde_json::from_value(row.get("author")).unwrap_or_default(),
                content: row.get("content"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                meta: serde_json::from_value(row.get("meta")).unwrap_or_default(),
            };
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }

    /// Search medical records by patient ID
    pub async fn get_medical_records_by_patient(
        &self,
        patient_id: Uuid,
    ) -> Result<Vec<MedicalRecord>, HimsError> {
        let rows = sqlx::query(GET_MEDICAL_RECORDS_BY_PATIENT)
            .bind(patient_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        let records: Result<Vec<MedicalRecord>, _> = rows.into_iter().map(|row| {
            Ok(MedicalRecord {
                id: row.get("id"),
                patient_id: row.get("patient_id"),
                encounter_id: row.get("encounter_id"),
                record_type: serde_json::from_str(&row.get::<String, _>("record_type")).unwrap_or_default(),
                status: serde_json::from_str(&row.get::<String, _>("status")).unwrap_or_default(),
                subject: serde_json::from_value(row.get("subject")).unwrap_or_default(),
                author: serde_json::from_value(row.get("author")).unwrap_or_default(),
                content: row.get("content"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                meta: serde_json::from_value(row.get("meta")).unwrap_or_default(),
            })
        }).collect();

        records.map_err(|e: serde_json::Error| HimsError::DatabaseError(e.to_string()))
    }

    /// Soft delete medical record with audit logging
    pub async fn delete_medical_record(&self, id: &str, user_id: &str) -> Result<bool, HimsError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        let deleted_at = Utc::now();
        let result = sqlx::query(SOFT_DELETE_MEDICAL_RECORD)
            .bind(deleted_at)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        if result.rows_affected() > 0 {
            self.log_medical_record_audit(
                &mut tx,
                id,
                user_id,
                AuditEventType::Delete,
                Some("Medical record soft deleted".to_string()),
            ).await?;

            tx.commit().await
                .map_err(|e| HimsError::DatabaseError(e.to_string()))?;
            Ok(true)
        } else {
            tx.rollback().await
                .map_err(|e| HimsError::DatabaseError(e.to_string()))?;
            Ok(false)
        }
    }

    /// Private helper for audit logging
    async fn log_medical_record_audit(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        record_id: &str,
        user_id: &str,
        event_type: AuditEventType,
        details: Option<String>,
    ) -> Result<(), HimsError> {
        let audit_log = AuditLog::new(
            event_type,
            AuditAction::Create,
            "MedicalRecord".to_string(),
        )
        .with_user(Uuid::parse_str(user_id).unwrap_or_default())
        .with_resource(Uuid::parse_str(record_id).unwrap_or_default())
        .with_details(details.unwrap_or_default());

        sqlx::query(
            r#"
            INSERT INTO audit_logs (
                id, event_type, user_id, patient_id, appointment_id, resource_type,
                resource_id, action, outcome, timestamp, details
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            )
            "#
        )
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
        .bind(audit_log.details.as_ref())
        .execute(&mut **tx)
        .await
        .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // Alias methods for controller compatibility
    pub async fn create_record(&self, record: &MedicalRecord, user_id: &str) -> Result<String, HimsError> {
        self.create_medical_record(record, user_id).await
    }

    pub async fn get_record(&self, id: &str) -> Result<Option<MedicalRecord>, HimsError> {
        self.get_medical_record(id).await
    }

    pub async fn delete_record(&self, id: &str, user_id: &str) -> Result<bool, HimsError> {
        self.delete_medical_record(id, user_id).await
    }

    pub async fn search_records(&self, params: serde_json::Value) -> Result<Vec<MedicalRecord>, HimsError> {
        // Simple implementation that returns a basic list for now
        // TODO: Implement proper filtering based on params
        let rows = sqlx::query("SELECT id, patient_id, encounter_id, record_type, status, content, created_at, updated_at FROM medical_records ORDER BY created_at DESC LIMIT 100")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;

        let mut records = Vec::new();
        for row in rows {
            let record = MedicalRecord {
                id: row.get::<String, _>("id").parse().unwrap_or_else(|_| Uuid::new_v4()),
                patient_id: row.get::<String, _>("patient_id").parse().unwrap_or_else(|_| Uuid::new_v4()),
                encounter_id: row.get::<Option<String>, _>("encounter_id").and_then(|s| s.parse().ok()),
                record_type: MedicalRecordType::from_string(&row.get::<String, _>("record_type")),
                status: DocumentStatus::from_string(&row.get::<String, _>("status")),
                subject: Reference {
                    reference: format!("Patient/{}", row.get::<String, _>("patient_id")),
                    display: None,
                },
                author: vec![],
                content: row.get("content"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                meta: ResourceMeta {
                    version_id: Some("1".to_string()),
                    last_updated: row.get("updated_at"),
                    profile: vec!["http://hl7.org/fhir/StructureDefinition/DocumentReference".to_string()],
                    security: vec![],
                    tag: vec![],
                },
            };
            records.push(record);
        }
        Ok(records)
    }

    pub async fn update_record_content(&self, id: &str, content: serde_json::Value) -> Result<MedicalRecord, HimsError> {
        let content_str = content.to_string();
        sqlx::query("UPDATE medical_records SET content = $1, updated_at = NOW() WHERE id = $2")
            .bind(&content_str)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;
        
        self.get_medical_record(id).await?.ok_or(HimsError::DatabaseError("Record not found after update".to_string()))
    }

    pub async fn finalize_record(&self, id: &str) -> Result<MedicalRecord, HimsError> {
        sqlx::query("UPDATE medical_records SET status = 'FINAL', updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| HimsError::DatabaseError(e.to_string()))?;
        
        self.get_medical_record(id).await?.ok_or(HimsError::DatabaseError("Record not found after finalization".to_string()))
    }

    // Controller adapter methods
    pub async fn create_record_from_request(
        &self,
        request: &crate::modules::medical_record::medical_record_controller::MedicalRecordCreateRequest,
        user_id: &str,
    ) -> Result<String, HimsError> {
        let record: MedicalRecord = request.into();
        self.create_medical_record(&record, user_id).await
    }

    pub async fn get_record_by_uuid(&self, id: uuid::Uuid) -> Result<Option<MedicalRecord>, HimsError> {
        self.get_medical_record(&id.to_string()).await
    }

    pub async fn search_records_by_query(
        &self,
        query: crate::modules::medical_record::medical_record_controller::MedicalRecordQuery,
    ) -> Result<Vec<MedicalRecord>, HimsError> {
        // Convert query to JSON for now
        let params = serde_json::to_value(query).unwrap_or(serde_json::Value::Null);
        self.search_records(params).await
    }

    pub async fn update_record_content_by_uuid(
        &self,
        id: uuid::Uuid,
        content: String,
    ) -> Result<MedicalRecord, HimsError> {
        let content_value = serde_json::Value::String(content);
        self.update_record_content(&id.to_string(), content_value).await
    }

    pub async fn finalize_record_by_uuid(&self, id: uuid::Uuid) -> Result<MedicalRecord, HimsError> {
        self.finalize_record(&id.to_string()).await
    }
}