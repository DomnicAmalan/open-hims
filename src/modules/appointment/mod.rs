//! Appointment Module
//! 
//! This module provides appointment management functionality including:
//! - Appointment CRUD operations
//! - FHIR R4 compliance
//! - Calendar integration support
//! - Provider scheduling
//! - Audit logging

#[path = "appointment.controller.rs"]
pub mod appointment_controller;
#[path = "appointment.service.rs"]
pub mod appointment_service;
#[path = "appointment.sql.rs"]
pub mod appointment_sql;

pub use appointment_controller::AppointmentController;
pub use appointment_service::AppointmentService;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

/// Appointment Module Configuration
pub struct AppointmentModule {
    pub service: Arc<AppointmentService>,
    pub controller: Arc<AppointmentController>,
}

impl AppointmentModule {
    /// Create a new Appointment Module with dependency injection
    pub fn new(db_pool: PgPool) -> Self {
        let service = Arc::new(AppointmentService::new(db_pool));
        let controller = Arc::new(AppointmentController::new(service.clone()));
        
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
    pub fn get_service(&self) -> Arc<AppointmentService> {
        self.service.clone()
    }
}