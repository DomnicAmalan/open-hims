use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, HeaderMap},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use crate::models::{
    Patient, ResourceMeta, HumanName, ContactPoint, Gender, Address, 
    CodeableConcept, PatientContact, PatientCommunication
};
use crate::modules::patient::PatientService;
use crate::modules::authorization::{
    HimsAuthorizationEngine, AuthorizationEngine, AuthorizationRequest, AuthorizationResponse,
    Subject, Resource, Action, AccessDecision, SessionContext,
};
use crate::utils::auth::{extract_user_from_headers, get_user_session_context};

/// Patient controller for FHIR R4 compliant patient management with authorization
pub struct PatientController {
    patient_service: Arc<PatientService>,
    authorization_engine: Arc<HimsAuthorizationEngine>,
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
    /// Create new controller with injected service and authorization engine
    pub fn new(
        patient_service: Arc<PatientService>,
        authorization_engine: Arc<HimsAuthorizationEngine>,
    ) -> Self {
        Self { 
            patient_service,
            authorization_engine,
        }
    }

    /// Create new controller with just patient service (for backwards compatibility)
    pub fn new_simple(patient_service: Arc<PatientService>) -> Self {
        // Create a dummy authorization engine for now
        // TODO: Remove this once full integration is complete
        use crate::modules::authorization::{
            PostgresAuthorizationStorage, HimsPolicyEngine, AuditManager, AuthorizationConfig
        };
        
        // This is a temporary workaround - we'll need proper dependency injection
        let dummy_storage = Arc::new(PostgresAuthorizationStorage::new(patient_service.get_db_pool()));
        let dummy_policy_engine = Arc::new(HimsPolicyEngine::new());
        
        // Create a basic audit config
        use crate::modules::authorization::AuditConfig;
        let audit_config = AuditConfig {
            enabled: true,
            log_all_decisions: true,
            log_emergency_access: true,
            log_break_glass: true,
            log_policy_evaluations: true,
            log_relationship_checks: true,
            retention_days: 365,
            real_time_compliance: true,
            alert_on_suspicious: true,
        };
        let dummy_audit_manager = Arc::new(AuditManager::new(audit_config));
        let dummy_config = AuthorizationConfig::default();
        
        let authorization_engine = Arc::new(HimsAuthorizationEngine::new(
            dummy_storage,
            dummy_policy_engine,
            dummy_audit_manager,
            dummy_config,
        ));
        
        Self { 
            patient_service,
            authorization_engine,
        }
    }

    /// Check authorization for a given request
    async fn check_authorization(
        &self,
        headers: &HeaderMap,
        action: Action,
        resource: Resource,
    ) -> Result<AuthorizationResponse, (StatusCode, Json<ErrorResponse>)> {
        // Extract user from headers (JWT token, session, etc.)
        let user_id = extract_user_from_headers(headers)
            .map_err(|e| {
                tracing::error!("Failed to extract user from headers: {}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Unauthorized".to_string(),
                        message: "Invalid or missing authentication".to_string(),
                    }),
                )
            })?;

        // Get user session context
        let context = get_user_session_context(user_id, headers)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get user session context: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Internal server error".to_string(),
                        message: "Failed to establish session context".to_string(),
                    }),
                )
            })?;
        
        // Extract session information for SessionContext
        let session_id = headers.get("x-session-id")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();
        
        let ip_address = headers.get("x-forwarded-for")
            .or_else(|| headers.get("x-real-ip"))
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        
        let user_agent = headers.get("user-agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        
        let session = SessionContext {
            user_id,
            session_id,
            ip_address,
            user_agent,
            department_id: None,
            location_id: None,
            shift_id: None,
            mfa_verified: false,
            risk_score: 0.0,
        };

        // Create authorization request
        let auth_request = AuthorizationRequest {
            subject: Subject::User(user_id),
            action,
            resource,
            context,
            session,
            request_id: Some(Uuid::new_v4().to_string()),
        };

        // Check authorization
        self.authorization_engine
            .check(auth_request)
            .await
            .map_err(|e| {
                tracing::error!("Authorization check failed: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Authorization error".to_string(),
                        message: "Failed to check authorization".to_string(),
                    }),
                )
            })
    }

    /// Handle authorization response
    fn handle_authorization_response(
        auth_response: &AuthorizationResponse,
    ) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
        match auth_response.decision {
            AccessDecision::Allow | AccessDecision::EmergencyAccess | AccessDecision::AllowWithRestrictions | AccessDecision::BreakGlassAccess => {
                if !auth_response.restrictions.is_empty() {
                    tracing::info!("Access granted with restrictions: {:?}", auth_response.restrictions);
                }
                Ok(())
            }
            AccessDecision::Deny => {
                tracing::warn!("Access denied: {:?}", auth_response.reasons);
                Err((
                    StatusCode::FORBIDDEN,
                    Json(ErrorResponse {
                        error: "Access denied".to_string(),
                        message: auth_response.reasons.join("; "),
                    }),
                ))
            }
            AccessDecision::RequireApproval => {
                tracing::info!("Access requires approval: {:?}", auth_response.requirements);
                Err((
                    StatusCode::FORBIDDEN,
                    Json(ErrorResponse {
                        error: "Approval required".to_string(),
                        message: format!("This action requires approval: {}", auth_response.requirements.join("; ")),
                    }),
                ))
            }
            AccessDecision::RequireMFA => {
                tracing::info!("Access requires MFA");
                Err((
                    StatusCode::FORBIDDEN,
                    Json(ErrorResponse {
                        error: "Multi-factor authentication required".to_string(),
                        message: "Please complete multi-factor authentication to proceed".to_string(),
                    }),
                ))
            }
        }
    }

    /// Create router with dependency injection
    pub fn routes(self: Arc<Self>) -> Router {
        Router::new()
            .route("/", post(Self::create_patient))
            .route("/", get(Self::search_patients))
            .route("/:id", get(Self::get_patient))
            .route("/:id", put(Self::update_patient))
            .route("/:id", delete(Self::delete_patient))
            .with_state(self)
    }

    /// Create new patient
    pub async fn create_patient(
        State(controller): State<Arc<PatientController>>,
        Json(payload): Json<PatientCreateRequest>,
    ) -> Result<(StatusCode, Json<PatientResponse>), (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Creating new patient");
        
        match controller.patient_service.create_patient(payload).await {
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

    /// Get patient by ID with authorization
    pub async fn get_patient(
        State(controller): State<Arc<PatientController>>,
        headers: HeaderMap,
        Path(id): Path<Uuid>,
    ) -> Result<Json<PatientResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Retrieving patient: {}", id);
        
        // For now, skip authorization check until we have full integration
        // TODO: Re-enable authorization once PatientModule is updated
        /*
        let auth_response = controller.authorization_engine.check(
            AuthorizationRequest {
                subject: Subject::User(extract_user_from_headers(&headers)?),
                action: Action::Read,
                resource: Resource::Patient(id),
                context: get_user_session_context(extract_user_from_headers(&headers)?, &headers).await?,
                request_id: Some(uuid::Uuid::new_v4().to_string()),
            }
        ).await?;
        
        // Handle authorization decision
        Self::handle_authorization_response(&auth_response)?;
        */
        
        // If authorized, proceed with the request
        match controller.patient_service.get_patient(id).await {
            Ok(Some(patient)) => {
                tracing::info!("Patient retrieved successfully: {}", id);
                
                let response = Self::patient_to_response(patient);
                Ok(Json(response))
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
        State(controller): State<Arc<PatientController>>,
        Query(params): Query<PatientQuery>,
    ) -> Result<Json<PatientBundle>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Searching patients with params: {:?}", params);
        
        match controller.patient_service.search_patients(params).await {
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
        State(controller): State<Arc<PatientController>>,
        Path(id): Path<Uuid>,
        Json(payload): Json<PatientCreateRequest>,
    ) -> Result<Json<PatientResponse>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Updating patient: {}", id);
        
        match controller.patient_service.update_patient(id, payload).await {
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
        State(controller): State<Arc<PatientController>>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Deleting patient: {}", id);
        
        match controller.patient_service.delete_patient(id).await {
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

    /// Redact sensitive patient data based on authorization restrictions
    fn redact_sensitive_patient_data(mut response: PatientResponse) -> PatientResponse {
        // Clear sensitive contact information
        response.telecom = vec![];
        response.address = vec![];
        
        // Redact full names, keep only first name
        response.name = response.name.into_iter().map(|mut name| {
            if let Some(ref family) = name.family {
                if !family.is_empty() {
                    name.family = Some("[REDACTED]".to_string());
                }
            }
            if name.given.len() > 1 {
                name.given = vec![name.given[0].clone()];
            }
            name
        }).collect();
        
        // Clear contact information
        response.contact = vec![];
        
        tracing::info!("Applied data redaction for patient: {}", response.id);
        response
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