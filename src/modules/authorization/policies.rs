// src/modules/authorization/policies.rs
//! Policy engine for healthcare authorization
//! 
//! This module implements a policy-based authorization system that complements
//! the relationship-based authorization. Policies define rules based on context,
//! time, location, and other factors.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Timelike, Datelike, Weekday};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use std::collections::HashMap;

use super::relations::{Action, Resource, Subject};
use super::healthcare_context::{RequestContext, UrgencyLevel, EmergencyType, SecurityLevel};

/// A healthcare policy that defines authorization rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcarePolicy {
    /// Unique policy identifier
    pub id: String,
    /// Human-readable policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Type of policy
    pub policy_type: PolicyType,
    /// Conditions that must be met
    pub conditions: Vec<PolicyCondition>,
    /// Effect when conditions are met
    pub effect: PolicyEffect,
    /// Policy priority (higher numbers take precedence)
    pub priority: i32,
    /// Whether this policy is active
    pub is_active: bool,
    /// When this policy was created
    pub created_at: DateTime<Utc>,
    /// When this policy was last updated
    pub updated_at: DateTime<Utc>,
    /// Optional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of policies for different use cases
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PolicyType {
    /// Default policy type
    #[default]
    Default,
    /// Regulatory compliance policies (HIPAA, GDPR, etc.)
    RegulatoryCompliance,
    /// Clinical protocol policies
    ClinicalProtocol,
    /// Emergency access policies
    EmergencyAccess,
    /// Business hours policies
    BusinessHours,
    /// Location-based policies
    LocationBased,
    /// Role-based policies
    RoleBased,
    /// Patient consent policies
    PatientConsent,
    /// Data protection policies
    DataProtection,
    /// Audit requirement policies
    AuditRequired,
    /// Break-glass access policies
    BreakGlass,
    /// Resource-specific policies
    ResourceSpecific,
    /// Time-based policies
    TimeBased,
    /// Custom policy type
    Custom(String),
}

/// Conditions that can be evaluated in policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyCondition {
    // Time-based conditions
    TimeOfDay { start: String, end: String },
    DayOfWeek(Vec<String>),
    DateRange { start: String, end: String },
    AfterHours,
    Weekend,
    
    // Location-based conditions
    RequireLocation(Vec<String>),
    AllowedIpRanges(Vec<String>),
    RemoteAccess,
    SecureConnection,
    MinimumSecurityLevel(SecurityLevel),
    
    // Context-based conditions
    EmergencyDeclared,
    UrgencyLevel(UrgencyLevel),
    PatientConsent,
    ClinicalContext(String),
    BreakGlassActivated,
    
    // Role and relationship-based
    RequireRole(String),
    RequireRelation(String),
    DepartmentMembership(String),
    MinimumClearanceLevel(u8),
    MultiFactorAuthentication,
    
    // Resource-based
    ResourceType(String),
    ResourceOwnership,
    DataClassification(String),
    PatientRelated,
    
    // Audit and compliance
    AuditTrailRequired,
    SecondaryApproval,
    ReasonRequired,
    ComplianceFlag(String),
    
    // Workflow conditions
    WorkflowStep(String),
    WorkflowPriority(String),
    
    // Custom condition
    Custom { key: String, value: String },
}

/// Effects that can be applied when policy conditions are met
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PolicyEffect {
    /// Default effect - deny
    #[default]
    Deny,
    /// Allow the action
    Allow,
    /// Require additional approval
    RequireApproval,
    /// Require second factor authentication
    RequireSecondFactor,
    /// Allow but audit the action
    AuditOnly,
    /// Apply additional conditions
    Conditional(Vec<PolicyCondition>),
    /// Limit access duration
    TimeLimit(u64), // seconds
    /// Apply restrictions
    Restrict(Vec<String>),
}

/// Result of policy evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    /// The final decision
    pub decision: PolicyEffect,
    /// Policies that were applied
    pub applied_policies: Vec<String>,
    /// Reasons for the decision
    pub reasons: Vec<String>,
    /// Additional requirements
    pub requirements: Vec<String>,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f32,
    /// Any restrictions applied
    pub restrictions: Vec<String>,
    /// Time limit if applicable (in seconds)
    pub time_limit: Option<u64>,
}

/// Trait for policy evaluation engines
#[async_trait]
pub trait PolicyEngine: Send + Sync {
    /// Evaluate policies for an authorization request
    async fn evaluate_policies(
        &self,
        subject: &Subject,
        action: &Action,
        resource: &Resource,
        context: &RequestContext,
    ) -> Result<PolicyDecision>;
    
    /// Get applicable policies for a request
    async fn get_applicable_policies(
        &self,
        subject: &Subject,
        action: &Action,
        resource: &Resource,
    ) -> Result<Vec<HealthcarePolicy>>;
    
    /// Add a new policy
    async fn add_policy(&self, policy: HealthcarePolicy) -> Result<()>;
    
    /// Remove a policy
    async fn remove_policy(&self, policy_id: &str) -> Result<()>;
    
    /// Update an existing policy
    async fn update_policy(&self, policy: HealthcarePolicy) -> Result<()>;
    
    /// Get a policy by ID
    async fn get_policy(&self, policy_id: &str) -> Result<Option<HealthcarePolicy>>;
    
    /// List all active policies
    async fn list_active_policies(&self) -> Result<Vec<HealthcarePolicy>>;
}

/// Default implementation of the policy engine
pub struct HimsPolicyEngine {
    policies: Vec<HealthcarePolicy>,
}

impl HimsPolicyEngine {
    /// Create a new policy engine with default healthcare policies
    pub fn new() -> Self {
        Self {
            policies: Self::default_healthcare_policies(),
        }
    }
    
    /// Create a policy engine with custom policies
    pub fn with_policies(policies: Vec<HealthcarePolicy>) -> Self {
        Self { policies }
    }
    
    /// Get default healthcare policies
    fn default_healthcare_policies() -> Vec<HealthcarePolicy> {
        vec![
            // Emergency break-glass policy
            HealthcarePolicy {
                id: "emergency-break-glass".to_string(),
                name: "Emergency Break-Glass Access".to_string(),
                description: "Allow emergency access with proper justification and audit".to_string(),
                policy_type: PolicyType::EmergencyAccess,
                conditions: vec![
                    PolicyCondition::EmergencyDeclared,
                    PolicyCondition::ReasonRequired,
                    PolicyCondition::AuditTrailRequired,
                ],
                effect: PolicyEffect::Allow,
                priority: 100,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                metadata: HashMap::new(),
            },
            
            // Business hours policy
            HealthcarePolicy {
                id: "business-hours-access".to_string(),
                name: "Standard Business Hours Access".to_string(),
                description: "Allow standard access during business hours".to_string(),
                policy_type: PolicyType::BusinessHours,
                conditions: vec![
                    PolicyCondition::TimeOfDay { 
                        start: "08:00".to_string(), 
                        end: "18:00".to_string() 
                    },
                    PolicyCondition::DayOfWeek(vec![
                        "Monday".to_string(),
                        "Tuesday".to_string(),
                        "Wednesday".to_string(),
                        "Thursday".to_string(),
                        "Friday".to_string(),
                    ]),
                ],
                effect: PolicyEffect::Allow,
                priority: 50,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                metadata: HashMap::new(),
            },
            
            // After hours restricted access
            HealthcarePolicy {
                id: "after-hours-restricted".to_string(),
                name: "After Hours Restricted Access".to_string(),
                description: "Require additional approval for after-hours access".to_string(),
                policy_type: PolicyType::TimeBased,
                conditions: vec![
                    PolicyCondition::AfterHours,
                ],
                effect: PolicyEffect::RequireApproval,
                priority: 80,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                metadata: HashMap::new(),
            },
            
            // Critical patient access
            HealthcarePolicy {
                id: "critical-patient-access".to_string(),
                name: "Critical Patient Enhanced Access".to_string(),
                description: "Enhanced access for critical patients with audit".to_string(),
                policy_type: PolicyType::ClinicalProtocol,
                conditions: vec![
                    PolicyCondition::UrgencyLevel(UrgencyLevel::Critical),
                    PolicyCondition::RequireRole("physician".to_string()),
                ],
                effect: PolicyEffect::AuditOnly,
                priority: 90,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                metadata: HashMap::new(),
            },
            
            // Sensitive data audit requirement
            HealthcarePolicy {
                id: "sensitive-data-audit".to_string(),
                name: "Sensitive Data Access Audit".to_string(),
                description: "Require audit for all sensitive healthcare data access".to_string(),
                policy_type: PolicyType::AuditRequired,
                conditions: vec![
                    PolicyCondition::DataClassification("sensitive".to_string()),
                ],
                effect: PolicyEffect::AuditOnly,
                priority: 70,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                metadata: HashMap::new(),
            },
            
            // Remote access security
            HealthcarePolicy {
                id: "remote-access-security".to_string(),
                name: "Remote Access Security Requirements".to_string(),
                description: "Require secure connection and MFA for remote access".to_string(),
                policy_type: PolicyType::LocationBased,
                conditions: vec![
                    PolicyCondition::RemoteAccess,
                ],
                effect: PolicyEffect::RequireSecondFactor,
                priority: 85,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                metadata: HashMap::new(),
            },
            
            // Patient consent requirement
            HealthcarePolicy {
                id: "patient-consent-required".to_string(),
                name: "Patient Consent Required".to_string(),
                description: "Require patient consent for non-emergency access".to_string(),
                policy_type: PolicyType::PatientConsent,
                conditions: vec![
                    PolicyCondition::PatientRelated,
                ],
                effect: PolicyEffect::Conditional(vec![
                    PolicyCondition::PatientConsent
                ]),
                priority: 60,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                metadata: HashMap::new(),
            },
        ]
    }
    
    /// Evaluate a single condition against the request context
    async fn evaluate_condition(
        &self,
        condition: &PolicyCondition,
        context: &RequestContext,
    ) -> Result<bool> {
        match condition {
            PolicyCondition::TimeOfDay { start, end } => {
                let current_time = context.timestamp.time();
                let start_time = chrono::NaiveTime::parse_from_str(start, "%H:%M")
                    .map_err(|e| anyhow!("Invalid start time format: {}", e))?;
                let end_time = chrono::NaiveTime::parse_from_str(end, "%H:%M")
                    .map_err(|e| anyhow!("Invalid end time format: {}", e))?;
                Ok(current_time >= start_time && current_time <= end_time)
            },
            
            PolicyCondition::DayOfWeek(allowed_days) => {
                let current_day = context.timestamp.weekday().to_string();
                Ok(allowed_days.contains(&current_day))
            },
            
            PolicyCondition::AfterHours => {
                Ok(context.is_after_hours())
            },
            
            PolicyCondition::Weekend => {
                Ok(context.is_weekend())
            },
            
            PolicyCondition::EmergencyDeclared => {
                Ok(context.is_emergency())
            },
            
            PolicyCondition::UrgencyLevel(required_level) => {
                let current_level = context.get_urgency_level();
                Ok(current_level >= *required_level)
            },
            
            PolicyCondition::RemoteAccess => {
                Ok(context.is_remote_access())
            },
            
            PolicyCondition::SecureConnection => {
                let security_level = context.get_security_level();
                Ok(matches!(security_level, SecurityLevel::High | SecurityLevel::Maximum))
            },
            
            PolicyCondition::MinimumSecurityLevel(required_level) => {
                let current_level = context.get_security_level();
                Ok(Self::security_level_value(&current_level) >= Self::security_level_value(required_level))
            },
            
            PolicyCondition::RequireLocation(allowed_locations) => {
                if let Some(location) = &context.location {
                    Ok(allowed_locations.contains(&location.hospital_id.to_string()))
                } else {
                    Ok(false)
                }
            },
            
            PolicyCondition::AuditTrailRequired => {
                Ok(!context.audit_trail.is_empty())
            },
            
            PolicyCondition::ReasonRequired => {
                // Check if justification is provided in emergency context
                if let Some(emergency) = &context.emergency {
                    Ok(emergency.justification.is_some())
                } else {
                    Ok(false)
                }
            },
            
            PolicyCondition::PatientRelated => {
                if let Some(clinical) = &context.clinical {
                    Ok(clinical.patient_id.is_some())
                } else {
                    Ok(false)
                }
            },
            
            PolicyCondition::PatientConsent => {
                // In a real implementation, this would check actual consent records
                // For now, we'll assume consent is always available during emergencies
                Ok(context.is_emergency())
            },
            
            PolicyCondition::BreakGlassActivated => {
                if let Some(emergency) = &context.emergency {
                    Ok(emergency.emergency_type.as_ref().map_or(false, |t| {
                        matches!(t, EmergencyType::BreakGlass)
                    }))
                } else {
                    Ok(false)
                }
            },
            
            // For unimplemented conditions, default to true
            _ => Ok(true),
        }
    }
    
    /// Get numeric value for security level comparison
    fn security_level_value(level: &SecurityLevel) -> u8 {
        match level {
            SecurityLevel::Low => 1,
            SecurityLevel::Medium => 2,
            SecurityLevel::High => 3,
            SecurityLevel::Maximum => 4,
        }
    }
    
    /// Combine multiple policy effects into a single decision
    fn combine_effects(effects: Vec<(PolicyEffect, String, i32)>) -> PolicyEffect {
        // Sort by priority (highest first)
        let mut sorted_effects = effects;
        sorted_effects.sort_by(|a, b| b.2.cmp(&a.2));
        
        // Apply the highest priority effect
        if let Some((effect, _, _)) = sorted_effects.first() {
            effect.clone()
        } else {
            PolicyEffect::Deny
        }
    }
}

#[async_trait]
impl PolicyEngine for HimsPolicyEngine {
    async fn evaluate_policies(
        &self,
        _subject: &Subject,
        _action: &Action,
        _resource: &Resource,
        context: &RequestContext,
    ) -> Result<PolicyDecision> {
        let mut applicable_policies = Vec::new();
        let mut applicable_effects = Vec::new();
        let mut reasons = Vec::new();
        let mut requirements = Vec::new();
        let mut restrictions = Vec::new();
        let mut time_limit = None;
        
        for policy in &self.policies {
            if !policy.is_active {
                continue;
            }
            
            // Check if all conditions are met
            let mut all_conditions_met = true;
            for condition in &policy.conditions {
                if !self.evaluate_condition(condition, context).await? {
                    all_conditions_met = false;
                    break;
                }
            }
            
            if all_conditions_met {
                applicable_policies.push(policy.id.clone());
                applicable_effects.push((policy.effect.clone(), policy.name.clone(), policy.priority));
                reasons.push(format!("Policy '{}' applied", policy.name));
                
                // Extract requirements and restrictions from effects
                match &policy.effect {
                    PolicyEffect::RequireApproval => {
                        requirements.push("Secondary approval required".to_string());
                    },
                    PolicyEffect::RequireSecondFactor => {
                        requirements.push("Multi-factor authentication required".to_string());
                    },
                    PolicyEffect::TimeLimit(seconds) => {
                        time_limit = Some(*seconds);
                    },
                    PolicyEffect::Restrict(restriction_list) => {
                        restrictions.extend(restriction_list.clone());
                    },
                    PolicyEffect::Conditional(conditions) => {
                        for condition in conditions {
                            if !self.evaluate_condition(condition, context).await? {
                                requirements.push(format!("Condition not met: {:?}", condition));
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
        
        let final_decision = if applicable_effects.is_empty() {
            PolicyEffect::Deny
        } else {
            Self::combine_effects(applicable_effects)
        };
        
        let confidence = if !applicable_policies.is_empty() { 0.9 } else { 0.1 };
        
        Ok(PolicyDecision {
            decision: final_decision,
            applied_policies: applicable_policies,
            reasons,
            requirements,
            confidence,
            restrictions,
            time_limit,
        })
    }
    
    async fn get_applicable_policies(
        &self,
        _subject: &Subject,
        _action: &Action,
        _resource: &Resource,
    ) -> Result<Vec<HealthcarePolicy>> {
        Ok(self.policies.clone())
    }
    
    async fn add_policy(&self, _policy: HealthcarePolicy) -> Result<()> {
        // In a real implementation, this would persist to database
        Ok(())
    }
    
    async fn remove_policy(&self, _policy_id: &str) -> Result<()> {
        // In a real implementation, this would remove from database
        Ok(())
    }
    
    async fn update_policy(&self, _policy: HealthcarePolicy) -> Result<()> {
        // In a real implementation, this would update in database
        Ok(())
    }
    
    async fn get_policy(&self, policy_id: &str) -> Result<Option<HealthcarePolicy>> {
        Ok(self.policies.iter().find(|p| p.id == policy_id).cloned())
    }
    
    async fn list_active_policies(&self) -> Result<Vec<HealthcarePolicy>> {
        Ok(self.policies.iter()
            .filter(|p| p.is_active)
            .cloned()
            .collect())
    }
}

impl Default for HimsPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}