// src/modules/authorization/error.rs
//! Authorization error types

use thiserror::Error;

/// Common authorization errors
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Storage error: {0}")]
    Storage(#[from] anyhow::Error),
    
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),
    
    #[error("Access denied")]
    AccessDenied,
    
    #[error("Resource not found")]
    ResourceNotFound,
    
    #[error("Invalid permissions")]
    InvalidPermissions,
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Authentication required")]
    AuthenticationRequired,
    
    #[error("Engine error: {0}")]
    Engine(String),
    
    #[error("Policy evaluation failed: {0}")]
    PolicyEvaluation(String),
    
    #[error("Relationship resolution failed: {0}")]
    RelationshipResolution(String),
    
    #[error("Context validation failed: {0}")]
    ContextValidation(String),
    
    #[error("Maximum depth exceeded")]
    MaxDepthExceeded,
    
    #[error("Circular dependency detected")]
    CircularDependency,
}

pub type AuthResult<T> = Result<T, AuthError>;