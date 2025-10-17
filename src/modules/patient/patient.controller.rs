use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::{
    Patient, ResourceMeta, HumanName, ContactPoint, Gender, Address, 
    CodeableConcept, PatientContact, PatientCommunication
};
use crate::modules::patient::PatientService;
use std::sync::Arc;

/// Patient controller for FHIR R4 compliant patient management
pub struct PatientController {
    patient_service: Arc<PatientService>,
}

#[derive(Debug, Deserialize)]
pub struct PatientQuery {
    pub active: Option<bool>,
    pub gender: Option<String>,
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub _count: Option<u32>,
    pub _offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientCreateRequest {
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub gender: Gender,
    pub birth_date: Option<chrono::NaiveDate>,
    pub address: Vec<Address>,
    pub marital_status: Option<CodeableConcept>,
    pub contact: Vec<PatientContact>,
    pub communication: Vec<PatientCommunication>,
}

#[derive(Debug, Serialize)]
pub struct PatientResponse {
    pub resourceType: String,
    pub id: Uuid,
    pub meta: ResourceMeta,
    pub active: bool,
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub gender: Gender,
    pub birthDate: Option<chrono::NaiveDate>,
    pub address: Vec<Address>,
    pub maritalStatus: Option<CodeableConcept>,
    pub contact: Vec<PatientContact>,
    pub communication: Vec<PatientCommunication>,
}

#[derive(Debug, Serialize)]
pub struct PatientBundle {
    pub resourceType: String,
    pub id: Uuid,
    pub meta: ResourceMeta,
    pub total: u32,
    pub entry: Vec<PatientBundleEntry>,
}

#[derive(Debug, Serialize)]
pub struct PatientBundleEntry {
    pub fullUrl: String,
    pub resource: PatientResponse,
}

impl PatientController {
    /// Create new controller with injected service
    pub fn new(patient_service: Arc<PatientService>) -> Self {
        Self { patient_service }
    }

    /// Create router with dependency injection
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/", post(Self::create_patient))
            .route("/", get(Self::search_patients))
            .route("/:id", get(Self::get_patient))
            .route("/:id", put(Self::update_patient))
            .route("/:id", delete(Self::delete_patient))
            .with_state(self.patient_service.clone())
    }

    /// Create new patient
    pub async fn create_patient(
        State(patient_service): State<Arc<PatientService>>,
        Json(payload): Json<PatientCreateRequest>,
    ) -> Result<(StatusCode, Json<PatientResponse>), (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Creating new patient");
        
        match patient_service.create_patient(payload).await {
            Ok(patient) => {
                tracing::info!("Patient created successfully: {}", patient.id);
                Ok((StatusCode::CREATED, Json(Self::patient_to_response(patient))))
            }
            Err(e) => {
                tracing::error!("Failed to create patient: {}", e);
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Failed to create patient".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Get patient by ID
    pub async fn get_patient(
        State(patient_service): State<Arc<PatientService>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<PatientResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Retrieving patient: {}", id);
        
        match patient_service.get_patient(id).await {
            Ok(Some(patient)) => {
                tracing::info!("Patient retrieved successfully: {}", id);
                Ok(Json(Self::patient_to_response(patient)))
            }
            Ok(None) => {
                tracing::warn!("Patient not found: {}", id);
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Patient not found".to_string(),
                        message: format!("Patient with id {} not found", id),
                    }),
                ))
            }
            Err(e) => {
                tracing::error!("Failed to retrieve patient {}: {}", id, e);
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

    /// Search patients with FHIR query parameters
    pub async fn search_patients(
        State(patient_service): State<Arc<PatientService>>,
        Query(params): Query<PatientQuery>,
    ) -> Result<Json<PatientBundle>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Searching patients with params: {:?}", params);
        
        match patient_service.search_patients(params).await {
            Ok(patients) => {
                tracing::info!("Found {} patients", patients.len());
                Ok(Json(Self::patients_to_bundle(patients)))
            }
            Err(e) => {
                tracing::error!("Failed to search patients: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to search patients".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Update patient
    pub async fn update_patient(
        State(patient_service): State<Arc<PatientService>>,
        Path(id): Path<Uuid>,
        Json(payload): Json<PatientCreateRequest>,
    ) -> Result<Json<PatientResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Updating patient: {}", id);
        
        match patient_service.update_patient(id, payload).await {
            Ok(patient) => {
                tracing::info!("Patient updated successfully: {}", id);
                Ok(Json(Self::patient_to_response(patient)))
            }
            Err(e) => {
                tracing::error!("Failed to update patient {}: {}", id, e);
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Failed to update patient".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Delete patient (soft delete)
    pub async fn delete_patient(
        State(patient_service): State<Arc<PatientService>>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Deleting patient: {}", id);
        
        match patient_service.delete_patient(id).await {
            Ok(true) => {
                tracing::info!("Patient deleted successfully: {}", id);
                Ok(StatusCode::NO_CONTENT)
            }
            Ok(false) => {
                tracing::warn!("Patient not found for deletion: {}", id);
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Patient not found".to_string(),
                        message: format!("Patient with id {} not found", id),
                    }),
                ))
            }
            Err(e) => {
                tracing::error!("Failed to delete patient {}: {}", id, e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to delete patient".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Convert Patient model to FHIR response format
    fn patient_to_response(patient: Patient) -> PatientResponse {
        PatientResponse {
            resourceType: "Patient".to_string(),
            id: patient.id,
            meta: patient.meta,
            active: patient.active,
            name: patient.name,
            telecom: patient.telecom,
            gender: patient.gender,
            birthDate: patient.birth_date,
            address: patient.address,
            maritalStatus: patient.marital_status,
            contact: patient.contact,
            communication: patient.communication,
        }
    }

    /// Convert multiple patients to FHIR Bundle format
    fn patients_to_bundle(patients: Vec<Patient>) -> PatientBundle {
        let total = patients.len() as u32;
        let entries: Vec<PatientBundleEntry> = patients
            .into_iter()
            .map(|patient| PatientBundleEntry {
                fullUrl: format!("Patient/{}", patient.id),
                resource: Self::patient_to_response(patient),
            })
            .collect();

        PatientBundle {
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


}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}