// src/modules/authorization/mod.rs
//! Healthcare Authorization Module
//! 
//! This module implements a Google Zanzibar-style authorization system
//! specifically designed for healthcare environments. It provides:
//! 
//! - Relationship-based access control
//! - Policy-based authorization
//! - Healthcare-specific context handling
//! - Comprehensive audit logging
//! - Emergency access management
//! - HIPAA/GDPR compliance features

use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod error;
pub mod relations;
pub mod healthcare_context;
pub mod policies;
pub mod storage;
pub mod audit;
pub mod engine;
pub mod authorization_sql;

pub use error::*;
pub use relations::*;
pub use healthcare_context::*;
pub use policies::*;
pub use storage::*;
pub use audit::*;
pub use engine::*;

/// Authorization configuration
#[derive(Debug, Clone)]
pub struct AuthorizationConfig {
    pub max_policy_evaluations: usize,
    pub max_relationship_depth: usize,
    pub enable_caching: bool,
    pub cache_ttl_seconds: u64,
    pub enable_audit: bool,
    pub emergency_access_enabled: bool,
    pub max_cache_size: usize,
    pub enable_emergency_access: bool,
    pub max_relation_depth: u8,
}

impl Default for AuthorizationConfig {
    fn default() -> Self {
        Self {
            max_policy_evaluations: 1000,
            max_relationship_depth: 10,
            enable_caching: true,
            cache_ttl_seconds: 300, // 5 minutes
            enable_audit: true,
            emergency_access_enabled: true,
            max_cache_size: 1000,
            enable_emergency_access: true,
            max_relation_depth: 10,
        }
    }
}

/// Session context for authorization requests
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub user_id: Uuid,
    pub session_id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub department_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub shift_id: Option<Uuid>,
    pub mfa_verified: bool,
    pub risk_score: f32,
}

/// Authorization request with full context
#[derive(Debug)]
pub struct AuthorizationRequest {
    pub subject: Subject,
    pub action: Action,
    pub resource: Resource,
    pub context: RequestContext,
    pub session: SessionContext,
    pub request_id: Option<String>,
}