// src/modules/authorization/engine.rs
//! Main authorization engine implementing Zanzibar-style relationship-based access control
//! 
//! This module contains the core logic for evaluating authorization requests based on
//! relationships, policies, and contextual information.

use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use anyhow::Result;
use uuid::Uuid;
use tokio::time::Duration;

use super::relations::{Subject, Resource, Action, HealthcareRelation, RelationshipTuple};
use super::policies::{PolicyEngine, PolicyDecision, PolicyEffect, HimsPolicyEngine};
use super::healthcare_context::RequestContext;
use super::storage::{AuthorizationStorage, RelationStorage, PostgresAuthorizationStorage};
use super::audit::{AuditManager, AuditEntry, AccessDecision, AuthorizationAudit};
use super::error::{AuthError, AuthResult};
use super::{AuthorizationConfig, AuthorizationRequest};

/// Response from authorization evaluation
#[derive(Debug, Clone)]
pub struct AuthorizationResponse {
    /// Whether access is allowed
    pub allowed: bool,
    /// Decision type
    pub decision: AccessDecision,
    /// Reasons for the decision
    pub reasons: Vec<String>,
    /// Additional requirements (e.g., MFA, approval)
    pub requirements: Vec<String>,
    /// Time limit for access (if applicable)
    pub time_limit: Option<Duration>,
    /// Restrictions on access
    pub restrictions: Vec<String>,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f32,
    /// Evaluation time in milliseconds
    pub evaluation_time_ms: u64,
    /// Request ID
    pub request_id: Option<String>,
}

/// Trait for authorization engines
#[async_trait]
pub trait AuthorizationEngine: Send + Sync {
    /// Evaluate an authorization request
    async fn check(&self, request: AuthorizationRequest) -> AuthResult<AuthorizationResponse>;
    
    /// Expand relationships to find all subjects with a given relation to a resource
    async fn expand(
        &self,
        resource: Resource,
        relation: HealthcareRelation,
    ) -> AuthResult<Vec<Subject>>;
    
    /// List all resources of a given type that a subject can access with a given action
    async fn list_objects(
        &self,
        subject: Subject,
        action: Action,
        resource_type: String,
    ) -> AuthResult<Vec<Resource>>;
    
    /// Add a relationship
    async fn add_relationship(&self, tuple: RelationshipTuple) -> AuthResult<()>;
    
    /// Remove a relationship
    async fn remove_relationship(&self, tuple: RelationshipTuple) -> AuthResult<()>;
    
    /// Check if a specific relationship exists
    async fn has_relationship(
        &self,
        object: Resource,
        relation: HealthcareRelation,
        subject: Subject,
    ) -> AuthResult<bool>;
}

/// Main implementation of the healthcare authorization engine
pub struct HimsAuthorizationEngine {
    storage: Arc<PostgresAuthorizationStorage>,
    policy_engine: Arc<HimsPolicyEngine>,
    audit_manager: Arc<AuditManager>,
    config: AuthorizationConfig,
    relation_cache: Arc<tokio::sync::RwLock<HashMap<String, (Vec<Subject>, Instant)>>>,
}

impl HimsAuthorizationEngine {
    /// Create a new authorization engine
    pub fn new(
        storage: Arc<PostgresAuthorizationStorage>,
        policy_engine: Arc<HimsPolicyEngine>,
        audit_manager: Arc<AuditManager>,
        config: AuthorizationConfig,
    ) -> Self {
        Self {
            storage,
            policy_engine,
            audit_manager,
            config,
            relation_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
    
    /// Validate the request context
    async fn validate_context(&self, context: &RequestContext) -> AuthResult<()> {
        context.validate()
            .map_err(|e| AuthError::ContextValidation(e.to_string()))?;
        Ok(())
    }
    
    /// Check emergency access scenarios
    async fn check_emergency_access(&self, request: &AuthorizationRequest) -> AuthResult<bool> {
        if !self.config.enable_emergency_access {
            return Ok(false);
        }
        
        if let Some(emergency) = &request.context.emergency {
            if emergency.is_emergency {
                // Validate emergency context
                if emergency.justification.is_none() {
                    return Err(AuthError::Engine(
                        "Emergency access requires justification".to_string()
                    ));
                }
                
                if emergency.approval_required && emergency.approved_by.is_none() {
                    return Err(AuthError::Engine(
                        "Emergency access requires approval".to_string()
                    ));
                }
                
                // Check if emergency access is still valid
                if let Some(expires_at) = emergency.expires_at {
                    if expires_at <= request.context.timestamp {
                        return Err(AuthError::Engine(
                            "Emergency access expired".to_string()
                        ));
                    }
                }
                
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Resolve relationships using graph traversal
    fn resolve_relationships<'a>(
        &'a self,
        resource: &'a Resource,
        subject: &'a Subject,
        relation: &'a HealthcareRelation,
        visited: &'a mut HashSet<String>,
        depth: u8,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AuthResult<bool>> + Send + 'a>> {
        Box::pin(async move {
        if depth >= self.config.max_relation_depth {
            return Err(AuthError::MaxDepthExceeded);
        }
        
        let cache_key = format!("{}#{}#{}", resource, relation, subject);
        
        // Check for circular dependencies
        if visited.contains(&cache_key) {
            return Err(AuthError::CircularDependency);
        }
        visited.insert(cache_key.clone());
        
        // Check cache first if enabled
        if self.config.enable_caching {
            let cache = self.relation_cache.read().await;
            if let Some((subjects, cached_at)) = cache.get(&format!("{}#{}", resource, relation)) {
                let cache_age = cached_at.elapsed();
                if cache_age < Duration::from_secs(self.config.cache_ttl_seconds) {
                    return Ok(subjects.contains(subject));
                }
            }
        }
        
        // Check direct relationship
        if self.storage.has_relationship(resource, relation, subject).await? {
            return Ok(true);
        }
        
        // Check inherited relationships
        if let Some(parent_relation) = self.get_parent_relation(relation) {
            if self.resolve_relationships(resource, subject, &parent_relation, visited, depth + 1).await? {
                return Ok(true);
            }
        }
        
        // Check computed relationships based on organizational hierarchy
        match relation {
            HealthcareRelation::DepartmentMember => {
                if let Subject::User(user_id) = subject {
                    if self.check_department_membership(resource, *user_id).await? {
                        return Ok(true);
                    }
                }
            },
            HealthcareRelation::CareTeamMember => {
                if let Subject::User(user_id) = subject {
                    if self.check_care_team_membership(resource, *user_id).await? {
                        return Ok(true);
                    }
                }
            },
            _ => {}
        }
        
        Ok(false)
        })
    }
    
    /// Get parent relation for inheritance
    fn get_parent_relation(&self, relation: &HealthcareRelation) -> Option<HealthcareRelation> {
        match relation {
            HealthcareRelation::DepartmentHead => Some(HealthcareRelation::DepartmentMember),
            HealthcareRelation::ConsultingPhysician => Some(HealthcareRelation::PrimaryPhysician),
            HealthcareRelation::SupervisingPhysician => Some(HealthcareRelation::TreatingPhysician),
            _ => None,
        }
    }
    
    /// Check department membership
    async fn check_department_membership(&self, resource: &Resource, user_id: Uuid) -> AuthResult<bool> {
        // This would check if the user is a member of the department associated with the resource
        // For now, return false to avoid complex implementation
        Ok(false)
    }
    
    /// Check care team membership
    async fn check_care_team_membership(&self, resource: &Resource, user_id: Uuid) -> AuthResult<bool> {
        // This would check if the user is part of the patient's care team
        // For now, return false to avoid complex implementation
        Ok(false)
    }
    
    /// Get required relations for an action
    async fn get_required_relations(&self, action: &Action, resource: &Resource) -> AuthResult<Vec<HealthcareRelation>> {
        match (action, resource) {
            (Action::Read, Resource::Patient(_)) => Ok(vec![
                HealthcareRelation::PrimaryPhysician,
                HealthcareRelation::ConsultingPhysician,
                HealthcareRelation::AttendingNurse,
                HealthcareRelation::CareTeamMember,
            ]),
            (Action::Write, Resource::Patient(_)) => Ok(vec![
                HealthcareRelation::PrimaryPhysician,
                HealthcareRelation::TreatingPhysician,
            ]),
            (Action::Prescribe, Resource::Patient(_)) => Ok(vec![
                HealthcareRelation::PrimaryPhysician,
                HealthcareRelation::ConsultingPhysician,
            ]),
            (Action::Schedule, Resource::Appointment(_)) => Ok(vec![
                HealthcareRelation::PrimaryPhysician,
                HealthcareRelation::AttendingNurse,
                HealthcareRelation::DepartmentMember,
            ]),
            (Action::ViewBilling, Resource::Billing(_)) => Ok(vec![
                HealthcareRelation::BillingAccess,
                HealthcareRelation::HospitalAdmin,
            ]),
            _ => Ok(vec![]), // Default to no specific relations required
        }
    }
    
    /// Audit the authorization decision
    async fn audit_decision(
        &self,
        request: &AuthorizationRequest,
        response: &AuthorizationResponse,
        evaluation_time_ms: u64,
    ) -> AuthResult<()> {
        if !self.audit_manager.is_enabled() {
            return Ok(());
        }
        
        // Extract user ID from subject
        let user_id = match &request.subject {
            Subject::User(id) => Some(*id),
            _ => None,
        };
        
        // Create audit entry
        let audit_entry = self.audit_manager.create_audit_entry(
            user_id,
            request.action.clone(),
            &request.resource,
            response.decision.clone(),
            Some(request.context.clone()),
        )
        .add_reason(response.reasons.join("; "))
        .with_metadata("evaluation_time_ms".to_string(), evaluation_time_ms.to_string())
        .with_metadata("confidence".to_string(), response.confidence.to_string());
        
        // Store audit entry
        self.storage.store_audit_entry(&audit_entry).await?;
        
        Ok(())
    }
    
    /// Update relationship cache
    async fn update_cache(&self, resource: &Resource, relation: &HealthcareRelation, subjects: Vec<Subject>) {
        if !self.config.enable_caching {
            return;
        }
        
        let cache_key = format!("{}#{}", resource, relation);
        let mut cache = self.relation_cache.write().await;
        
        // Clean up cache if it's getting too large
        if cache.len() >= self.config.max_cache_size {
            cache.clear();
        }
        
        cache.insert(cache_key, (subjects, Instant::now()));
    }
}

#[async_trait]
impl AuthorizationEngine for HimsAuthorizationEngine {
    async fn check(&self, request: AuthorizationRequest) -> AuthResult<AuthorizationResponse> {
        let start_time = Instant::now();
        let mut reasons = Vec::new();
        let mut requirements = Vec::new();
        let mut restrictions = Vec::new();
        let mut time_limit = None;
        let mut decision = AccessDecision::Deny;
        let mut confidence = 0.0;
        
        // Validate request context
        self.validate_context(&request.context).await?;
        
        // Check emergency access first
        if self.check_emergency_access(&request).await? {
            decision = AccessDecision::EmergencyAccess;
            reasons.push("Emergency access granted".to_string());
            confidence = 1.0;
        } else {
            // Evaluate policies
            let policy_decision = self.policy_engine.evaluate_policies(
                &request.subject,
                &request.action,
                &request.resource,
                &request.context,
            ).await.map_err(|e| AuthError::PolicyEvaluation(e.to_string()))?;
            
            match policy_decision.decision {
                PolicyEffect::Allow => {
                    decision = AccessDecision::Allow;
                    reasons.extend(policy_decision.reasons);
                    confidence = policy_decision.confidence;
                },
                PolicyEffect::Deny => {
                    decision = AccessDecision::Deny;
                    reasons.extend(policy_decision.reasons);
                    confidence = policy_decision.confidence;
                },
                PolicyEffect::RequireApproval => {
                    decision = AccessDecision::RequireApproval;
                    requirements.extend(policy_decision.requirements);
                    confidence = policy_decision.confidence;
                },
                PolicyEffect::RequireSecondFactor => {
                    decision = AccessDecision::RequireMFA;
                    requirements.push("Multi-factor authentication required".to_string());
                    confidence = policy_decision.confidence;
                },
                PolicyEffect::AuditOnly => {
                    // Continue with relationship checks
                    let required_relations = self.get_required_relations(&request.action, &request.resource).await?;
                    let mut relation_found = false;
                    
                    for relation in required_relations {
                        let mut visited = HashSet::new();
                        if self.resolve_relationships(
                            &request.resource,
                            &request.subject,
                            &relation,
                            &mut visited,
                            0
                        ).await? {
                            decision = AccessDecision::Allow;
                            reasons.push(format!("Access granted via {} relationship", relation));
                            confidence = 0.8;
                            relation_found = true;
                            break;
                        }
                    }
                    
                    if !relation_found {
                        decision = AccessDecision::Deny;
                        reasons.push("No valid relationship found".to_string());
                        confidence = 0.9;
                    }
                },
                PolicyEffect::TimeLimit(seconds) => {
                    time_limit = Some(Duration::from_secs(seconds));
                    decision = AccessDecision::AllowWithRestrictions;
                    restrictions.push(format!("Time limit: {} seconds", seconds));
                    confidence = policy_decision.confidence;
                },
                PolicyEffect::Restrict(restriction_list) => {
                    decision = AccessDecision::AllowWithRestrictions;
                    restrictions.extend(restriction_list);
                    confidence = policy_decision.confidence;
                },
                PolicyEffect::Conditional(_) => {
                    // Handle conditional policies based on additional requirements
                    requirements.extend(policy_decision.requirements);
                    if requirements.is_empty() {
                        decision = AccessDecision::Allow;
                    } else {
                        decision = AccessDecision::RequireApproval;
                    }
                    confidence = policy_decision.confidence;
                },
            }
        }
        
        let evaluation_time_ms = start_time.elapsed().as_millis() as u64;
        
        let response = AuthorizationResponse {
            allowed: matches!(decision, AccessDecision::Allow | AccessDecision::EmergencyAccess | AccessDecision::AllowWithRestrictions),
            decision: decision.clone(),
            reasons,
            requirements,
            time_limit,
            restrictions,
            confidence,
            evaluation_time_ms,
            request_id: request.request_id.clone(),
        };
        
        // Audit the decision
        self.audit_decision(&request, &response, evaluation_time_ms).await?;
        
        Ok(response)
    }
    
    async fn expand(
        &self,
        resource: Resource,
        relation: HealthcareRelation,
    ) -> AuthResult<Vec<Subject>> {
        // PostgresAuthorizationStorage implements RelationStorage, so we can call directly
        let subjects = self.storage.find_direct_relationships(&resource, &relation).await?;
        
        // Update cache
        self.update_cache(&resource, &relation, subjects.clone()).await;
        
        Ok(subjects)
    }
    
    async fn list_objects(
        &self,
        subject: Subject,
        _action: Action,
        _resource_type: String,
    ) -> AuthResult<Vec<Resource>> {
        // Get all relationships for the subject
        let relationships = self.storage.get_relationships_for_subject(&subject).await?;
        
        // Extract resources from relationships
        let resources: Vec<Resource> = relationships.into_iter()
            .map(|tuple| tuple.object)
            .collect();
        
        Ok(resources)
    }
    
    async fn add_relationship(&self, tuple: RelationshipTuple) -> AuthResult<()> {
        self.storage.store_relationship(&tuple).await?;
        Ok(())
    }
    
    async fn remove_relationship(&self, tuple: RelationshipTuple) -> AuthResult<()> {
        self.storage.remove_relationship(&tuple).await?;
        Ok(())
    }
    
    async fn has_relationship(
        &self,
        object: Resource,
        relation: HealthcareRelation,
        subject: Subject,
    ) -> AuthResult<bool> {
        let result = self.storage.has_relationship(&object, &relation, &subject).await?;
        Ok(result)
    }
}