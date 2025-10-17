use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::audit::AuditService;

/// Audit controller for compliance reporting and audit trail management
pub struct AuditController {
    audit_service: Arc<AuditService>,
}

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    pub user_id: Option<String>,
    pub patient_id: Option<String>,
    pub resource_type: Option<String>,
    pub event_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub _count: Option<u32>,
    pub _offset: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct HipaaReportQuery {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UserActivityQuery {
    pub user_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl AuditController {
    /// Create new controller with injected service
    pub fn new(audit_service: Arc<AuditService>) -> Self {
        Self { audit_service }
    }

    /// Create router with dependency injection
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/logs", get(Self::get_audit_logs))
            .route("/logs/:id", get(Self::get_audit_log))
            .route("/reports/hipaa", get(Self::generate_hipaa_report))
            .route("/reports/user-activity", get(Self::generate_user_activity_report))
            .with_state(self.audit_service.clone())
    }

    /// Get audit logs with filtering
    pub async fn get_audit_logs(
        State(audit_service): State<Arc<AuditService>>,
        Query(params): Query<AuditQuery>,
    ) -> Result<Json<Vec<crate::models::AuditLog>>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Retrieving audit logs with params: {:?}", params);
        
        match audit_service
            .get_audit_logs(
                params.user_id.as_deref(),
                params.patient_id.as_deref(),
                params.resource_type.as_deref(),
                params.event_type.as_deref(),
                params.start_date,
                params.end_date,
                params._count.map(|c| c as i32),
                params._offset.map(|o| o as i32),
            )
            .await
        {
            Ok(logs) => {
                tracing::info!("Retrieved {} audit logs", logs.len());
                Ok(Json(logs))
            }
            Err(e) => {
                tracing::error!("Failed to retrieve audit logs: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to retrieve audit logs".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Get specific audit log by ID
    pub async fn get_audit_log(
        State(audit_service): State<Arc<AuditService>>,
        Path(id): Path<String>,
    ) -> Result<Json<crate::models::AuditLog>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Retrieving audit log: {}", id);
        
        match audit_service.get_audit_log(&id).await {
            Ok(Some(log)) => {
                tracing::info!("Audit log retrieved successfully: {}", id);
                Ok(Json(log))
            }
            Ok(None) => {
                tracing::warn!("Audit log not found: {}", id);
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Audit log not found".to_string(),
                        message: format!("Audit log with id {} not found", id),
                    }),
                ))
            }
            Err(e) => {
                tracing::error!("Failed to retrieve audit log {}: {}", id, e);
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

    /// Generate HIPAA compliance report
    pub async fn generate_hipaa_report(
        State(audit_service): State<Arc<AuditService>>,
        Query(params): Query<HipaaReportQuery>,
    ) -> Result<Json<crate::modules::audit::audit_service::HipaaComplianceReport>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Generating HIPAA compliance report for period: {} to {}", 
                      params.start_date, params.end_date);
        
        match audit_service
            .generate_hipaa_report(params.start_date, params.end_date)
            .await
        {
            Ok(report) => {
                tracing::info!("HIPAA compliance report generated successfully");
                Ok(Json(report))
            }
            Err(e) => {
                tracing::error!("Failed to generate HIPAA report: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to generate HIPAA compliance report".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }

    /// Generate user activity report
    pub async fn generate_user_activity_report(
        State(audit_service): State<Arc<AuditService>>,
        Query(params): Query<UserActivityQuery>,
    ) -> Result<Json<crate::modules::audit::audit_service::UserActivityReport>, (StatusCode, Json<ErrorResponse>)> {
        tracing::info!("Generating user activity report for user: {} (period: {} to {})", 
                      params.user_id, params.start_date, params.end_date);
        
        match audit_service
            .generate_user_activity_report(&params.user_id)
            .await
        {
            Ok(report) => {
                tracing::info!("User activity report generated successfully for user: {}", params.user_id);
                Ok(Json(report))
            }
            Err(e) => {
                tracing::error!("Failed to generate user activity report for {}: {}", params.user_id, e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to generate user activity report".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    }
}