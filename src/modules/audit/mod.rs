//! Audit Module
//! 
//! This module provides audit logging and compliance functionality including:
//! - HIPAA audit trails
//! - GDPR compliance logging
//! - Security event tracking
//! - Access control monitoring
//! - Compliance reporting

#[path = "audit.controller.rs"]
pub mod audit_controller;
#[path = "audit.service.rs"]
pub mod audit_service;
#[path = "audit.sql.rs"]
pub mod audit_sql;

pub use audit_controller::AuditController;
pub use audit_service::AuditService;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

/// Audit Module Configuration
pub struct AuditModule {
    pub service: Arc<AuditService>,
    pub controller: Arc<AuditController>,
}

impl AuditModule {
    /// Create a new Audit Module with dependency injection
    pub fn new(db_pool: PgPool) -> Self {
        let service = Arc::new(AuditService::new(db_pool));
        let controller = Arc::new(AuditController::new(service.clone()));
        
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
    pub fn get_service(&self) -> Arc<AuditService> {
        self.service.clone()
    }
}