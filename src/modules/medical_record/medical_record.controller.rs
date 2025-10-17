use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::{MedicalRecord, MedicalRecordType, DocumentStatus, Reference, ResourceMeta};
use crate::modules::medical_record::MedicalRecordService;
use std::sync::Arc;

/// Medical Record controller for clinical documentation management
pub struct MedicalRecordController {
    medical_record_service: Arc<MedicalRecordService>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MedicalRecordQuery {
    pub patient_id: Option<Uuid>,
    pub record_type: Option<MedicalRecordType>,
    pub status: Option<DocumentStatus>,
    pub author: Option<Uuid>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub _count: Option<u32>,
    pub _offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedicalRecordCreateRequest {
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub record_type: MedicalRecordType,
    pub content: String,
    pub author: Reference,
}

#[derive(Debug, Serialize)]
pub struct MedicalRecordResponse {
    pub resourceType: String,
    pub id: Uuid,
    pub meta: ResourceMeta,
    pub patientId: Uuid,
    pub encounterId: Option<Uuid>,
    pub recordType: MedicalRecordType,
    pub status: DocumentStatus,
    pub subject: Reference,
    pub author: Vec<Reference>,
    pub content: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MedicalRecordBundle {
    pub resourceType: String,
    pub id: Uuid,
    pub meta: ResourceMeta,
    pub total: u32,
    pub entry: Vec<MedicalRecordBundleEntry>,
}

#[derive(Debug, Serialize)]
pub struct MedicalRecordBundleEntry {
    pub fullUrl: String,
    pub resource: MedicalRecordResponse,
}

impl MedicalRecordController {
    /// Create new controller with injected service
    pub fn new(medical_record_service: Arc<MedicalRecordService>) -> Self {
        Self { medical_record_service }
    }

    /// Create router with dependency injection
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/", post(Self::create_record))
            .route("/", get(Self::search_records))
            .route("/:id", get(Self::get_record))
            .route("/:id", put(Self::update_record))
            .route("/:id", delete(Self::delete_record))
            .with_state(self.medical_record_service.clone())
    }

    /// Create new medical record
    pub async fn create_record(
        State(record_service): State<Arc<MedicalRecordService>>,
        Json(payload): Json<MedicalRecordCreateRequest>,
    ) -> Result<(StatusCode, Json<MedicalRecordResponse>), (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Creating new medical record for patient: {}", payload.patient_id);
        
        match record_service.create_record_from_request(&payload, "system").await {
            Ok(record_id) => {
                tracing::info!("Medical record created successfully: {}", record_id);
                // Fetch the created record to return it
                match record_service.get_record(&record_id).await {
                    Ok(Some(record)) => {
                        Ok((StatusCode::CREATED, Json(Self::record_to_response(record))))
                    }
                    Ok(None) => {
                        tracing::error!("Created record not found: {}", record_id);
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                error: "Record created but not found".to_string(),
                                message: "Internal server error".to_string(),
                            }),
                        ))
                    }
                    Err(e) => {
                        tracing::error!("Failed to fetch created record: {}", e);
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                error: "Failed to fetch created record".to_string(),
                                message: e.to_string(),
                            }),
                        ))
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to create medical record: {}", e);
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Failed to create medical record".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Get medical record by ID
    pub async fn get_record(
        State(record_service): State<Arc<MedicalRecordService>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<MedicalRecordResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Retrieving medical record: {}", id);
        
        match record_service.get_record_by_uuid(id).await {
            Ok(Some(record)) => {
                tracing::info!("Medical record retrieved successfully: {}", id);
                Ok(Json(Self::record_to_response(record)))
            }
            Ok(None) => {
                tracing::warn!("Medical record not found: {}", id);
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Medical record not found".to_string(),
                        message: format!("Medical record with id {} not found", id),
                    }),
                ))
            }
            Err(e) => {
                tracing::error!("Failed to retrieve medical record {}: {}", id, e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Internal server error".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Delete a medical record by ID
    pub async fn delete_record(
        State(record_service): State<Arc<MedicalRecordService>>,
        Path(id): Path<String>,
    ) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Deleting medical record: {}", id);
        
        match record_service.delete_record(&id, "system").await {
            Ok(true) => {
                tracing::info!("Medical record deleted successfully: {}", id);
                Ok(StatusCode::NO_CONTENT)
            }
            Ok(false) => {
                tracing::warn!("Medical record not found for deletion: {}", id);
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Medical record not found".to_string(),
                        message: format!("Medical record with ID {} not found", id),
                    }),
                ))
            }
            Err(e) => {
                tracing::error!("Failed to delete medical record {}: {}", id, e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to delete medical record".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Search medical records
    pub async fn search_records(
        State(record_service): State<Arc<MedicalRecordService>>,
        Query(params): Query<MedicalRecordQuery>,
    ) -> Result<Json<MedicalRecordBundle>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Searching medical records with params: {:?}", params);
        
        match record_service.search_records_by_query(params).await {
            Ok(records) => {
                tracing::info!("Found {} medical records", records.len());
                Ok(Json(Self::records_to_bundle(records)))
            }
            Err(e) => {
                tracing::error!("Failed to search medical records: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to search medical records".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Update medical record content
    pub async fn update_record(
        State(record_service): State<Arc<MedicalRecordService>>,
        Path(id): Path<Uuid>,
        Json(content): Json<String>,
    ) -> Result<Json<MedicalRecordResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Updating medical record: {}", id);
        
        match record_service.update_record_content_by_uuid(id, content).await {
            Ok(record) => {
                tracing::info!("Medical record updated successfully: {}", id);
                Ok(Json(Self::record_to_response(record)))
            }
            Err(e) => {
                tracing::error!("Failed to update medical record {}: {}", id, e);
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Failed to update medical record".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Finalize medical record (mark as final)
    pub async fn finalize_record(
        State(record_service): State<Arc<MedicalRecordService>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<MedicalRecordResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Finalizing medical record: {}", id);
        
        match record_service.finalize_record_by_uuid(id).await {
            Ok(record) => {
                tracing::info!("Medical record finalized successfully: {}", id);
                Ok(Json(Self::record_to_response(record)))
            }
            Err(e) => {
                tracing::error!("Failed to finalize medical record {}: {}", id, e);
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Failed to finalize medical record".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Convert MedicalRecord model to response format
    fn record_to_response(record: MedicalRecord) -> MedicalRecordResponse {
        MedicalRecordResponse {
            resourceType: "DocumentReference".to_string(),
            id: record.id,
            meta: record.meta,
            patientId: record.patient_id,
            encounterId: record.encounter_id,
            recordType: record.record_type,
            status: record.status,
            subject: record.subject,
            author: record.author,
            content: record.content,
            createdAt: record.created_at,
            updatedAt: record.updated_at,
        }
    }

    /// Convert multiple records to bundle format
    fn records_to_bundle(records: Vec<MedicalRecord>) -> MedicalRecordBundle {
        let total = records.len() as u32;
        let entries: Vec<MedicalRecordBundleEntry> = records
            .into_iter()
            .map(|record| MedicalRecordBundleEntry {
                fullUrl: format!("DocumentReference/{}", record.id),
                resource: Self::record_to_response(record),
            })
            .collect();

        MedicalRecordBundle {
            resourceType: "Bundle".to_string(),
            id: Uuid::new_v4(),
            meta: ResourceMeta {
                version_id: Some("1".to_string()),
                last_updated: chrono::Utc::now(),
                profile: vec!["http://hl7.org/fhir/StructureDefinition/Bundle".to_string()],
                security: vec![],
                tag: vec![],
            },
            total,
            entry: entries,
        }
    }

    /// Create router for medical record endpoints
    pub fn router() -> Router<Arc<MedicalRecordService>> {
        Router::new()
            .route("/", post(Self::create_record))
            .route("/", get(Self::search_records))
            .route("/:id", get(Self::get_record))
            .route("/:id", put(Self::update_record))
            .route("/:id/finalize", put(Self::finalize_record))
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl From<&MedicalRecordCreateRequest> for MedicalRecord {
    fn from(request: &MedicalRecordCreateRequest) -> Self {
        MedicalRecord {
            id: Uuid::new_v4(),
            patient_id: request.patient_id,
            encounter_id: request.encounter_id,
            record_type: request.record_type.clone(),
            status: DocumentStatus::Preliminary,
            subject: Reference {
                reference: format!("Patient/{}", request.patient_id),
                display: None,
            },
            author: vec![request.author.clone()],
            content: request.content.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            meta: ResourceMeta {
                version_id: Some("1".to_string()),
                last_updated: chrono::Utc::now(),
                profile: vec!["http://hl7.org/fhir/StructureDefinition/DocumentReference".to_string()],
                security: vec![],
                tag: vec![],
            },
        }
    }
}