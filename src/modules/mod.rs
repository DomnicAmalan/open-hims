//! Modules
//! 
//! This module organizes the application into feature-based modules following NestJS patterns.
//! Each module contains its own controllers, services, and related functionality.

pub mod patient;
pub mod appointment;
pub mod medical_record;
pub mod audit;
pub mod auth;

pub use patient::PatientModule;
pub use appointment::AppointmentModule;
pub use medical_record::MedicalRecordModule;
pub use audit::AuditModule;
pub use auth::AuthModule;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

/// Application Module Registry
/// 
/// Central registry for all application modules with dependency injection
pub struct AppModules {
    pub patient: Arc<PatientModule>,
    pub appointment: Arc<AppointmentModule>,
    pub medical_record: Arc<MedicalRecordModule>,
    pub audit: Arc<AuditModule>,
    pub auth: Arc<AuthModule>,
}

impl AppModules {
    /// Initialize all application modules with shared dependencies
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            patient: Arc::new(PatientModule::new(db_pool.clone())),
            appointment: Arc::new(AppointmentModule::new(db_pool.clone())),
            medical_record: Arc::new(MedicalRecordModule::new(db_pool.clone())),
            audit: Arc::new(AuditModule::new(db_pool.clone())),
            auth: Arc::new(AuthModule::new(db_pool)),
        }
    }

    /// Register all module routes
    pub fn routes(&self) -> Router {
        Router::new()
            .nest("/api/v1/patients", self.patient.routes())
            .nest("/api/v1/appointments", self.appointment.routes())
            .nest("/api/v1/medical-records", self.medical_record.routes())
            .nest("/api/v1/audit", self.audit.routes())
            .nest("/api/v1/auth", self.auth.routes())
    }
}