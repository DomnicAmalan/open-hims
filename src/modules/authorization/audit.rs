// src/modules/authorization/audit.rs
//! Audit and logging system for authorization decisions
//! 
//! This module provides comprehensive audit logging for all authorization
//! decisions, supporting compliance with healthcare regulations like HIPAA.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::relations::{Action, Subject, Resource};
use super::healthcare_context::{RequestContext, EmergencyContext};

/// Represents an audit entry for an authorization decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique audit entry ID
    pub id: Uuid,
    /// User who made the request
    pub user_id: Option<Uuid>,
    /// Action that was requested
    pub action: Action,
    /// Resource namespace (patient, appointment, etc.)
    pub resource_namespace: String,
    /// Resource ID
    pub resource_id: String,
    /// Authorization decision
    pub decision: AccessDecision,
    /// Reasons for the decision
    pub reasons: Vec<String>,
    /// Request context information
    pub request_context: Option<RequestContext>,
    /// IP address of the requester
    pub ip_address: Option<String>,
    /// User agent string
    pub user_agent: Option<String>,
    /// Timestamp of the decision
    pub timestamp: DateTime<Utc>,
    /// Session ID if available
    pub session_id: Option<String>,
    /// Emergency context if applicable
    pub emergency_context: Option<EmergencyContext>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Authorization decision types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessDecision {
    /// Access was allowed
    Allow,
    /// Access was denied
    Deny,
    /// Access requires additional approval
    RequireApproval,
    /// Access requires multi-factor authentication
    RequireMFA,
    /// Access was allowed but with restrictions
    AllowWithRestrictions,
    /// Emergency access was granted
    EmergencyAccess,
    /// Break-glass access was granted
    BreakGlassAccess,
}

/// Audit trail for authorization operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationAudit {
    /// Unique audit trail ID
    pub id: Uuid,
    /// Subject who initiated the request
    pub subject: Subject,
    /// Action that was attempted
    pub action: Action,
    /// Resource that was accessed
    pub resource: Resource,
    /// Final decision
    pub decision: AccessDecision,
    /// Policies that were evaluated
    pub evaluated_policies: Vec<String>,
    /// Relationships that were checked
    pub checked_relationships: Vec<String>,
    /// Time taken for evaluation (milliseconds)
    pub evaluation_time_ms: u64,
    /// Request context
    pub context: RequestContext,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl AuditEntry {
    /// Create a new audit entry
    pub fn new(
        user_id: Option<Uuid>,
        action: Action,
        resource_namespace: String,
        resource_id: String,
        decision: AccessDecision,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            action,
            resource_namespace,
            resource_id,
            decision,
            reasons: Vec::new(),
            request_context: None,
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now(),
            session_id: None,
            emergency_context: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Add a reason for the decision
    pub fn add_reason(mut self, reason: String) -> Self {
        self.reasons.push(reason);
        self
    }
    
    /// Set the request context
    pub fn with_context(mut self, context: RequestContext) -> Self {
        // Extract relevant fields from context
        self.ip_address = context.ip_address.clone();
        self.user_agent = context.user_agent.clone();
        self.session_id = context.session_id.clone();
        self.emergency_context = context.emergency.clone();
        self.request_context = Some(context);
        self
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    /// Check if this was an emergency access
    pub fn is_emergency_access(&self) -> bool {
        matches!(self.decision, AccessDecision::EmergencyAccess | AccessDecision::BreakGlassAccess)
            || self.emergency_context.as_ref().map_or(false, |e| e.is_emergency)
    }
    
    /// Get a summary of the audit entry for logging
    pub fn summary(&self) -> String {
        format!(
            "User {:?} {} {} on {}:{} - {} ({})",
            self.user_id,
            self.action,
            match self.decision {
                AccessDecision::Allow => "ALLOWED",
                AccessDecision::Deny => "DENIED",
                AccessDecision::RequireApproval => "APPROVAL_REQUIRED",
                AccessDecision::RequireMFA => "MFA_REQUIRED",
                AccessDecision::AllowWithRestrictions => "ALLOWED_RESTRICTED",
                AccessDecision::EmergencyAccess => "EMERGENCY_ACCESS",
                AccessDecision::BreakGlassAccess => "BREAK_GLASS",
            },
            self.resource_namespace,
            self.resource_id,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.reasons.join(", ")
        )
    }
}

impl AuthorizationAudit {
    /// Create a new authorization audit
    pub fn new(
        subject: Subject,
        action: Action,
        resource: Resource,
        decision: AccessDecision,
        context: RequestContext,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            subject,
            action,
            resource,
            decision,
            evaluated_policies: Vec::new(),
            checked_relationships: Vec::new(),
            evaluation_time_ms: 0,
            context,
            timestamp: Utc::now(),
        }
    }
    
    /// Add an evaluated policy
    pub fn add_evaluated_policy(mut self, policy_id: String) -> Self {
        self.evaluated_policies.push(policy_id);
        self
    }
    
    /// Add a checked relationship
    pub fn add_checked_relationship(mut self, relationship: String) -> Self {
        self.checked_relationships.push(relationship);
        self
    }
    
    /// Set the evaluation time
    pub fn with_evaluation_time(mut self, time_ms: u64) -> Self {
        self.evaluation_time_ms = time_ms;
        self
    }
}

impl std::fmt::Display for AccessDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessDecision::Allow => write!(f, "allow"),
            AccessDecision::Deny => write!(f, "deny"),
            AccessDecision::RequireApproval => write!(f, "require_approval"),
            AccessDecision::RequireMFA => write!(f, "require_mfa"),
            AccessDecision::AllowWithRestrictions => write!(f, "allow_with_restrictions"),
            AccessDecision::EmergencyAccess => write!(f, "emergency_access"),
            AccessDecision::BreakGlassAccess => write!(f, "break_glass_access"),
        }
    }
}

/// Audit event types for different kinds of authorization events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    /// Standard authorization check
    AuthorizationCheck,
    /// Emergency access granted
    EmergencyAccess,
    /// Break-glass access activated
    BreakGlassAccess,
    /// Policy evaluation
    PolicyEvaluation,
    /// Relationship check
    RelationshipCheck,
    /// Administrative action
    AdminAction,
    /// Configuration change
    ConfigurationChange,
    /// Bulk operation
    BulkOperation,
}

/// Compliance audit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAudit {
    /// Audit ID
    pub id: Uuid,
    /// Type of compliance check
    pub compliance_type: ComplianceType,
    /// Result of compliance check
    pub result: ComplianceResult,
    /// Details of the check
    pub details: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Associated authorization audit
    pub authorization_audit_id: Option<Uuid>,
}

/// Types of compliance checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceType {
    /// HIPAA compliance check
    HIPAA,
    /// GDPR compliance check
    GDPR,
    /// SOX compliance check
    SOX,
    /// Local healthcare regulation
    LocalRegulation(String),
    /// Custom compliance check
    Custom(String),
}

/// Result of compliance check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceResult {
    /// Compliant
    Compliant,
    /// Non-compliant
    NonCompliant,
    /// Requires review
    RequiresReview,
    /// Exempt from compliance
    Exempt,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Log all decisions (not just denials)
    pub log_all_decisions: bool,
    /// Log emergency access
    pub log_emergency_access: bool,
    /// Log break-glass access
    pub log_break_glass: bool,
    /// Log policy evaluations
    pub log_policy_evaluations: bool,
    /// Log relationship checks
    pub log_relationship_checks: bool,
    /// Audit log retention period (days)
    pub retention_days: u32,
    /// Enable real-time compliance monitoring
    pub real_time_compliance: bool,
    /// Alert on suspicious activity
    pub alert_on_suspicious: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_all_decisions: true,
            log_emergency_access: true,
            log_break_glass: true,
            log_policy_evaluations: false, // Can be verbose
            log_relationship_checks: false, // Can be verbose
            retention_days: 2555, // 7 years for healthcare compliance
            real_time_compliance: true,
            alert_on_suspicious: true,
        }
    }
}

/// Audit manager for handling all audit operations
pub struct AuditManager {
    config: AuditConfig,
}

impl AuditManager {
    /// Create a new audit manager
    pub fn new(config: AuditConfig) -> Self {
        Self { config }
    }
    
    /// Check if audit logging is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
    
    /// Check if all decisions should be logged
    pub fn should_log_all_decisions(&self) -> bool {
        self.config.log_all_decisions
    }
    
    /// Check if emergency access should be logged
    pub fn should_log_emergency(&self) -> bool {
        self.config.log_emergency_access
    }
    
    /// Check if policy evaluations should be logged
    pub fn should_log_policy_evaluations(&self) -> bool {
        self.config.log_policy_evaluations
    }
    
    /// Create an audit entry for an authorization decision
    pub fn create_audit_entry(
        &self,
        user_id: Option<Uuid>,
        action: Action,
        resource: &Resource,
        decision: AccessDecision,
        context: Option<RequestContext>,
    ) -> AuditEntry {
        let (resource_namespace, resource_id) = match resource {
            Resource::Patient(id) => ("patient".to_string(), id.to_string()),
            Resource::MedicalRecord(id) => ("medical_record".to_string(), id.to_string()),
            Resource::Appointment(id) => ("appointment".to_string(), id.to_string()),
            Resource::Department(id) => ("department".to_string(), id.to_string()),
            Resource::Organization(id) => ("organization".to_string(), id.to_string()),
            Resource::Prescription(id) => ("prescription".to_string(), id.to_string()),
            Resource::LabResult(id) => ("lab_result".to_string(), id.to_string()),
            Resource::ImagingStudy(id) => ("imaging_study".to_string(), id.to_string()),
            Resource::Report(id) => ("report".to_string(), id.to_string()),
            Resource::Billing(id) => ("billing".to_string(), id.to_string()),
            Resource::CarePlan(id) => ("care_plan".to_string(), id.to_string()),
            Resource::Encounter(id) => ("encounter".to_string(), id.to_string()),
            Resource::ClinicalDecisionSupport(id) => ("clinical_decision_support".to_string(), id.to_string()),
            Resource::ResearchData(id) => ("research_data".to_string(), id.to_string()),
            Resource::SystemConfig(name) => ("system_config".to_string(), name.clone()),
        };
        
        let mut entry = AuditEntry::new(user_id, action, resource_namespace, resource_id, decision);
        
        if let Some(ctx) = context {
            entry = entry.with_context(ctx);
        }
        
        entry
    }
    
    /// Analyze audit patterns for suspicious activity
    pub fn analyze_suspicious_patterns(&self, _entries: &[AuditEntry]) -> Vec<String> {
        // This would implement pattern analysis for suspicious activities
        // Examples:
        // - Multiple failed access attempts
        // - Unusual access patterns
        // - Access outside normal hours without emergency context
        // - Bulk data access
        // For now, return empty
        Vec::new()
    }
    
    /// Generate compliance report
    pub fn generate_compliance_report(
        &self,
        _entries: &[AuditEntry],
        _compliance_type: ComplianceType,
    ) -> ComplianceReport {
        // This would generate compliance reports based on audit entries
        ComplianceReport {
            id: Uuid::new_v4(),
            compliance_type: _compliance_type,
            period_start: Utc::now() - chrono::Duration::days(30),
            period_end: Utc::now(),
            total_access_attempts: 0,
            compliant_accesses: 0,
            non_compliant_accesses: 0,
            findings: Vec::new(),
            generated_at: Utc::now(),
        }
    }
}

/// Compliance report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report ID
    pub id: Uuid,
    /// Type of compliance
    pub compliance_type: ComplianceType,
    /// Report period start
    pub period_start: DateTime<Utc>,
    /// Report period end
    pub period_end: DateTime<Utc>,
    /// Total access attempts
    pub total_access_attempts: u64,
    /// Compliant accesses
    pub compliant_accesses: u64,
    /// Non-compliant accesses
    pub non_compliant_accesses: u64,
    /// Compliance findings
    pub findings: Vec<String>,
    /// When the report was generated
    pub generated_at: DateTime<Utc>,
}