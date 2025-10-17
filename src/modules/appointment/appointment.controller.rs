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

use crate::models::{Appointment, AppointmentStatus, ResourceMeta, CodeableConcept, 
                   AppointmentParticipant};
use crate::modules::appointment::AppointmentService;
use std::sync::Arc;

/// Appointment controller for FHIR R4 compliant appointment management
pub struct AppointmentController {
    appointment_service: Arc<AppointmentService>,
}

#[derive(Debug, Deserialize)]
pub struct AppointmentQuery {
    pub status: Option<AppointmentStatus>,
    pub date: Option<String>,
    pub patient: Option<Uuid>,
    pub practitioner: Option<Uuid>,
    pub service_type: Option<String>,
    pub _count: Option<u32>,
    pub _offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentCreateRequest {
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
}

#[derive(Debug, Serialize)]
pub struct AppointmentResponse {
    pub resourceType: String,
    pub id: Uuid,
    pub meta: ResourceMeta,
    pub status: AppointmentStatus,
    pub serviceCategory: Vec<CodeableConcept>,
    pub serviceType: Vec<CodeableConcept>,
    pub specialty: Vec<CodeableConcept>,
    pub appointmentType: Option<CodeableConcept>,
    pub reasonCode: Vec<CodeableConcept>,
    pub priority: Option<u32>,
    pub description: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub minutesDuration: Option<u32>,
    pub participant: Vec<AppointmentParticipant>,
}

#[derive(Debug, Serialize)]
pub struct AppointmentBundle {
    pub resourceType: String,
    pub id: Uuid,
    pub meta: ResourceMeta,
    pub total: u32,
    pub entry: Vec<AppointmentBundleEntry>,
}

#[derive(Debug, Serialize)]
pub struct AppointmentBundleEntry {
    pub fullUrl: String,
    pub resource: AppointmentResponse,
}

impl AppointmentController {
    /// Create new controller with injected service
    pub fn new(appointment_service: Arc<AppointmentService>) -> Self {
        Self { appointment_service }
    }

    /// Create router with dependency injection
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/", post(Self::create_appointment))
            .route("/", get(Self::search_appointments))
            .route("/:id", get(Self::get_appointment))
            .route("/:id", put(Self::update_appointment))
            .route("/:id", delete(Self::delete_appointment))
            .with_state(self.appointment_service.clone())
    }

    /// Create new appointment
    pub async fn create_appointment(
        State(appointment_service): State<Arc<AppointmentService>>,
        Json(payload): Json<AppointmentCreateRequest>,
    ) -> Result<(StatusCode, Json<AppointmentResponse>), (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Creating new appointment");
        
        match appointment_service.create_appointment_from_request(payload, "system").await {
            Ok(appointment) => {
                tracing::info!("Appointment created successfully: {}", appointment.id);
                Ok((StatusCode::CREATED, Json(Self::appointment_to_response(appointment))))
            }
            Err(e) => {
                tracing::error!("Failed to create appointment: {}", e);
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Failed to create appointment".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Get appointment by ID
    pub async fn get_appointment(
        State(appointment_service): State<Arc<AppointmentService>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<AppointmentResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Retrieving appointment: {}", id);
        
        match appointment_service.get_appointment_by_uuid(id).await {
            Ok(Some(appointment)) => {
                tracing::info!("Appointment retrieved successfully: {}", id);
                Ok(Json(Self::appointment_to_response(appointment)))
            }
            Ok(None) => {
                tracing::warn!("Appointment not found: {}", id);
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Appointment not found".to_string(),
                        message: format!("Appointment with id {} not found", id),
                    }),
                ))
            }
            Err(e) => {
                tracing::error!("Failed to retrieve appointment {}: {}", id, e);
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

    /// Search appointments with query parameters
    pub async fn search_appointments(
        State(appointment_service): State<Arc<AppointmentService>>,
        Query(params): Query<AppointmentQuery>,
    ) -> Result<Json<AppointmentBundle>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Searching appointments with params: {:?}", params);
        
        match appointment_service.search_appointments_from_query(params).await {
            Ok(appointments) => {
                tracing::info!("Found {} appointments", appointments.len());
                Ok(Json(Self::appointments_to_bundle(appointments)))
            }
            Err(e) => {
                tracing::error!("Failed to search appointments: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to search appointments".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Update appointment
    pub async fn update_appointment(
        State(appointment_service): State<Arc<AppointmentService>>,
        Path(id): Path<Uuid>,
        Json(payload): Json<AppointmentCreateRequest>,
    ) -> Result<Json<AppointmentResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Updating appointment: {}", id);
        
        match appointment_service.update_appointment_from_request(id, payload).await {
            Ok(appointment) => {
                tracing::info!("Appointment updated successfully: {}", id);
                Ok(Json(Self::appointment_to_response(appointment)))
            }
            Err(e) => {
                tracing::error!("Failed to update appointment {}: {}", id, e);
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Failed to update appointment".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Cancel appointment
    pub async fn cancel_appointment(
        State(appointment_service): State<Arc<AppointmentService>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<AppointmentResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Cancelling appointment: {}", id);
        
        match appointment_service.cancel_appointment_by_uuid(id).await {
            Ok(appointment) => {
                tracing::info!("Appointment cancelled successfully: {}", id);
                Ok(Json(Self::appointment_to_response(appointment)))
            }
            Err(e) => {
                tracing::error!("Failed to cancel appointment {}: {}", id, e);
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Failed to cancel appointment".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Delete appointment
    pub async fn delete_appointment(
        State(appointment_service): State<Arc<AppointmentService>>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Deleting appointment: {}", id);
        
        match appointment_service.delete_appointment(&id.to_string(), "system").await {
            Ok(true) => {
                tracing::info!("Appointment deleted successfully: {}", id);
                Ok(StatusCode::NO_CONTENT)
            }
            Ok(false) => {
                tracing::warn!("Appointment not found for deletion: {}", id);
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Appointment not found".to_string(),
                        message: format!("Appointment with ID {} not found", id),
                    }),
                ))
            }
            Err(e) => {
                tracing::error!("Failed to delete appointment {}: {}", id, e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to delete appointment".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Convert Appointment model to FHIR response format
    fn appointment_to_response(appointment: Appointment) -> AppointmentResponse {
        AppointmentResponse {
            resourceType: "Appointment".to_string(),
            id: appointment.id,
            meta: appointment.meta,
            status: appointment.status,
            serviceCategory: appointment.service_category,
            serviceType: appointment.service_type,
            specialty: appointment.specialty,
            appointmentType: appointment.appointment_type,
            reasonCode: appointment.reason_code,
            priority: appointment.priority,
            description: appointment.description,
            start: appointment.start,
            end: appointment.end,
            minutesDuration: appointment.minutes_duration,
            participant: appointment.participant,
        }
    }

    /// Convert multiple appointments to FHIR Bundle format
    fn appointments_to_bundle(appointments: Vec<Appointment>) -> AppointmentBundle {
        let total = appointments.len() as u32;
        let entries: Vec<AppointmentBundleEntry> = appointments
            .into_iter()
            .map(|appointment| AppointmentBundleEntry {
                fullUrl: format!("Appointment/{}", appointment.id),
                resource: Self::appointment_to_response(appointment),
            })
            .collect();

        AppointmentBundle {
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

    /// Create router for appointment endpoints
    pub fn router() -> Router<Arc<AppointmentService>> {
        Router::new()
            .route("/", post(Self::create_appointment))
            .route("/", get(Self::search_appointments))
            .route("/:id", get(Self::get_appointment))
            .route("/:id", put(Self::update_appointment))
            .route("/:id/cancel", put(Self::cancel_appointment))
            .route("/:id", delete(Self::delete_appointment))
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}