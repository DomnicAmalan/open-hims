//! Auth Module
//! 
//! This module provides authentication and authorization functionality including:
//! - JWT token management
//! - User authentication
//! - Role-based access control (RBAC)
//! - Healthcare provider verification
//! - Session management

#[path = "auth.controller.rs"]
pub mod auth_controller;
#[path = "auth.service.rs"]
pub mod auth_service;
#[path = "auth.middleware.rs"]
pub mod auth_middleware;

pub use auth_controller::AuthController;
pub use auth_service::{AuthService, AuthenticatedUser};
pub use auth_middleware::AuthMiddleware;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

/// Auth Module Configuration
pub struct AuthModule {
    pub service: Arc<AuthService>,
    pub controller: Arc<AuthController>,
    pub middleware: Arc<AuthMiddleware>,
}

impl AuthModule {
    /// Create a new Auth Module with dependency injection
    pub fn new(db_pool: PgPool) -> Self {
        let service = Arc::new(AuthService::new(db_pool));
        let controller = Arc::new(AuthController::new(service.clone()));
        let middleware = Arc::new(AuthMiddleware::new(service.clone()));
        
        Self {
            service,
            controller,
            middleware,
        }
    }

    /// Register routes for this module
    pub fn routes(&self) -> Router {
        self.controller.routes()
    }

    /// Get service instance for dependency injection
    pub fn get_service(&self) -> Arc<AuthService> {
        self.service.clone()
    }

    /// Get middleware instance
    pub fn get_middleware(&self) -> Arc<AuthMiddleware> {
        self.middleware.clone()
    }
}