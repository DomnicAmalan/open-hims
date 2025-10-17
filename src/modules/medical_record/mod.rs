//! Medical Record Module
//! 
//! This module provides medical record management functionality including:
//! - Electronic Health Records (EHR)
//! - FHIR R4 compliance
//! - Clinical document management
//! - Medical history tracking
//! - Audit logging

#[path = "medical_record.controller.rs"]
pub mod medical_record_controller;
#[path = "medical_record.service.rs"]
pub mod medical_record_service;
#[path = "medical_record.sql.rs"]
pub mod medical_record_sql;

pub use medical_record_controller::MedicalRecordController;
pub use medical_record_service::MedicalRecordService;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

/// Medical Record Module Configuration
pub struct MedicalRecordModule {
    pub service: Arc<MedicalRecordService>,
    pub controller: Arc<MedicalRecordController>,
}

impl MedicalRecordModule {
    /// Create a new Medical Record Module with dependency injection
    pub fn new(db_pool: PgPool) -> Self {
        let service = Arc::new(MedicalRecordService::new(db_pool));
        let controller = Arc::new(MedicalRecordController::new(service.clone()));
        
        Self {
            service,
            controller,
        }
    }

    /// Register routes for this module
    pub fn routes(&self) -> Router {
        self.controller.routes()
    }

    /// Get service instance for dependency injection
    pub fn get_service(&self) -> Arc<MedicalRecordService> {
        self.service.clone()
    }
}