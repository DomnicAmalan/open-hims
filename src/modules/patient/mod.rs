//! Patient Module
//! 
//! This module provides patient management functionality including:
//! - Patient CRUD operations
//! - FHIR R4 compliance
//! - Audit logging
//! - Healthcare data validation

#[path = "patient.controller.rs"]
pub mod patient_controller;
#[path = "patient.service.rs"] 
pub mod patient_service;
#[path = "patient.sql.rs"]
pub mod patient_sql;

pub use patient_controller::PatientController;
pub use patient_service::PatientService;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

/// Patient Module Configuration
pub struct PatientModule {
    pub service: Arc<PatientService>,
    pub controller: Arc<PatientController>,
}

impl PatientModule {
    /// Create a new Patient Module with simplified setup
    pub fn new(db_pool: PgPool) -> Self {
        let service = Arc::new(PatientService::new(db_pool));
        let controller = Arc::new(PatientController::new(service.clone()));
        
        Self {
            service,
            controller,
        }
    }

    /// Register routes for this module
    pub fn routes(&self) -> Router {
        self.controller.routes()
    }

    /// Get service instance 
    pub fn get_service(&self) -> Arc<PatientService> {
        self.service.clone()
    }
}