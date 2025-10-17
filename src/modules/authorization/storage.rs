// src/modules/authorization/storage.rs
//! Storage layer for authorization data
//! 
//! This module provides database storage and retrieval functionality for
//! authorization relationships, policies, and audit logs.

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::relations::{RelationshipTuple, Subject, Resource, HealthcareRelation};
use super::policies::HealthcarePolicy;
use super::audit::AuditEntry;
use super::error::AuthError;
use super::authorization_sql;

/// Trait for authorization data storage
#[async_trait]
pub trait AuthorizationStorage: Send + Sync {
    /// Store a relationship tuple
    async fn store_relationship(&self, tuple: &RelationshipTuple) -> Result<(), AuthError>;
    
    /// Remove a relationship tuple
    async fn remove_relationship(&self, tuple: &RelationshipTuple) -> Result<(), AuthError>;
    
    /// Check if a relationship exists
    async fn has_relationship(
        &self,
        object: &Resource,
        relation: &HealthcareRelation,
        subject: &Subject,
    ) -> Result<bool, AuthError>;
    
    /// Get all relationships for a resource
    async fn get_relationships_for_resource(
        &self,
        resource: &Resource,
    ) -> Result<Vec<RelationshipTuple>, AuthError>;
    
    /// Get all relationships for a subject
    async fn get_relationships_for_subject(
        &self,
        subject: &Subject,
    ) -> Result<Vec<RelationshipTuple>, AuthError>;
    
    /// Store a policy
    async fn store_policy(&self, policy: &HealthcarePolicy) -> Result<(), AuthError>;
    
    /// Get a policy by ID
    async fn get_policy(&self, policy_id: &str) -> Result<Option<HealthcarePolicy>, AuthError>;
    
    /// Get all active policies
    async fn get_active_policies(&self) -> Result<Vec<HealthcarePolicy>, AuthError>;
    
    /// Store an audit entry
    async fn store_audit_entry(&self, entry: &AuditEntry) -> Result<(), AuthError>;
    
    /// Clean up expired relationships
    async fn cleanup_expired_relationships(&self) -> Result<u64, AuthError>;
}

/// Trait for relationship-specific storage operations
#[async_trait]
pub trait RelationStorage: Send + Sync {
    /// Find direct relationships
    async fn find_direct_relationships(
        &self,
        object: &Resource,
        relation: &HealthcareRelation,
    ) -> Result<Vec<Subject>, AuthError>;
    
    /// Find inherited relationships through hierarchy
    async fn find_inherited_relationships(
        &self,
        object: &Resource,
        relation: &HealthcareRelation,
        max_depth: u8,
    ) -> Result<Vec<Subject>, AuthError>;
    
    /// Get relationship hierarchy for a subject
    async fn get_subject_hierarchy(&self, subject: &Subject) -> Result<Vec<Subject>, AuthError>;
}

/// Trait for policy-specific storage operations
#[async_trait]
pub trait PolicyStorage: Send + Sync {
    /// Get policies by type
    async fn get_policies_by_type(&self, policy_type: &str) -> Result<Vec<HealthcarePolicy>, AuthError>;
    
    /// Update policy status
    async fn update_policy_status(&self, policy_id: &str, is_active: bool) -> Result<(), AuthError>;
    
    /// Get policy usage statistics
    async fn get_policy_usage_stats(&self, policy_id: &str) -> Result<PolicyUsageStats, AuthError>;
}

/// PostgreSQL implementation of authorization storage
pub struct PostgresAuthorizationStorage {
    pool: PgPool,
}

/// Policy usage statistics
#[derive(Debug, Clone)]
pub struct PolicyUsageStats {
    pub policy_id: String,
    pub total_evaluations: i64,
    pub allow_decisions: i64,
    pub deny_decisions: i64,
    pub last_used: Option<DateTime<Utc>>,
}

impl PostgresAuthorizationStorage {
    /// Create a new PostgreSQL authorization storage
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Convert Resource enum to namespace and ID
    fn resource_to_parts(resource: &Resource) -> (String, String) {
        match resource {
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
        }
    }
    
    /// Convert Subject enum to namespace and ID
    fn subject_to_parts(subject: &Subject) -> (String, String) {
        match subject {
            Subject::User(id) => ("user".to_string(), id.to_string()),
            Subject::Role(role) => ("role".to_string(), role.clone()),
            Subject::Department(id) => ("department".to_string(), id.to_string()),
            Subject::Organization(id) => ("organization".to_string(), id.to_string()),
            Subject::System(name) => ("system".to_string(), name.clone()),
            Subject::Group(id) => ("group".to_string(), id.to_string()),
        }
    }
    
    /// Convert namespace and ID back to Resource
    fn parts_to_resource(namespace: &str, id: &str) -> Result<Resource, AuthError> {
        match namespace {
            "patient" => Ok(Resource::Patient(Uuid::parse_str(id)?)),
            "medical_record" => Ok(Resource::MedicalRecord(Uuid::parse_str(id)?)),
            "appointment" => Ok(Resource::Appointment(Uuid::parse_str(id)?)),
            "department" => Ok(Resource::Department(Uuid::parse_str(id)?)),
            "organization" => Ok(Resource::Organization(Uuid::parse_str(id)?)),
            "prescription" => Ok(Resource::Prescription(Uuid::parse_str(id)?)),
            "lab_result" => Ok(Resource::LabResult(Uuid::parse_str(id)?)),
            "imaging_study" => Ok(Resource::ImagingStudy(Uuid::parse_str(id)?)),
            "report" => Ok(Resource::Report(Uuid::parse_str(id)?)),
            "billing" => Ok(Resource::Billing(Uuid::parse_str(id)?)),
            "care_plan" => Ok(Resource::CarePlan(Uuid::parse_str(id)?)),
            "encounter" => Ok(Resource::Encounter(Uuid::parse_str(id)?)),
            "clinical_decision_support" => Ok(Resource::ClinicalDecisionSupport(Uuid::parse_str(id)?)),
            "research_data" => Ok(Resource::ResearchData(Uuid::parse_str(id)?)),
            "system_config" => Ok(Resource::SystemConfig(id.to_string())),
            _ => Err(AuthError::Storage(anyhow::anyhow!("Unknown resource namespace: {}", namespace))),
        }
    }
    
    /// Convert namespace and ID back to Subject
    fn parts_to_subject(namespace: &str, id: &str) -> Result<Subject, AuthError> {
        match namespace {
            "user" => Ok(Subject::User(Uuid::parse_str(id)?)),
            "role" => Ok(Subject::Role(id.to_string())),
            "department" => Ok(Subject::Department(Uuid::parse_str(id)?)),
            "organization" => Ok(Subject::Organization(Uuid::parse_str(id)?)),
            "system" => Ok(Subject::System(id.to_string())),
            "group" => Ok(Subject::Group(Uuid::parse_str(id)?)),
            _ => Err(AuthError::Storage(anyhow::anyhow!("Unknown subject namespace: {}", namespace))),
        }
    }
    
    /// Parse effect string from database
    fn parse_effect(effect_str: &str) -> super::policies::PolicyEffect {
        match effect_str {
            "allow" => super::policies::PolicyEffect::Allow,
            "deny" => super::policies::PolicyEffect::Deny,
            "require_approval" => super::policies::PolicyEffect::RequireApproval,
            "require_second_factor" => super::policies::PolicyEffect::RequireSecondFactor,
            "audit_only" => super::policies::PolicyEffect::AuditOnly,
            "time_limit" => super::policies::PolicyEffect::TimeLimit(3600), // default 1 hour
            "restrict" => super::policies::PolicyEffect::Restrict(vec![]),
            "conditional" => super::policies::PolicyEffect::Conditional(vec![]),
            _ => super::policies::PolicyEffect::Deny,
        }
    }
}

#[async_trait]
impl AuthorizationStorage for PostgresAuthorizationStorage {
    async fn store_relationship(&self, tuple: &RelationshipTuple) -> Result<(), AuthError> {
        let (resource_type, resource_id) = Self::resource_to_parts(&tuple.object);
        let (subject_type, subject_id) = Self::subject_to_parts(&tuple.subject);
        let resource_id_uuid = Uuid::parse_str(&resource_id)?;
        let subject_id_uuid = Uuid::parse_str(&subject_id)?;
        
        sqlx::query(authorization_sql::relationships::INSERT_RELATIONSHIP)
            .bind(&resource_type)
            .bind(&resource_id_uuid)
            .bind(&tuple.relation.to_string())
            .bind(&subject_type)
            .bind(&subject_id_uuid)
            .bind(&tuple.created_by)
            .bind(&serde_json::to_value(&tuple.metadata).unwrap_or_default())
            .bind(&tuple.expires_at)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    async fn remove_relationship(&self, tuple: &RelationshipTuple) -> Result<(), AuthError> {
        let (resource_type, resource_id) = Self::resource_to_parts(&tuple.object);
        let (subject_type, subject_id) = Self::subject_to_parts(&tuple.subject);
        let resource_id_uuid = Uuid::parse_str(&resource_id)?;
        let subject_id_uuid = Uuid::parse_str(&subject_id)?;
        
        sqlx::query(authorization_sql::relationships::REMOVE_RELATIONSHIP)
            .bind(&resource_type)
            .bind(&resource_id_uuid)
            .bind(&tuple.relation.to_string())
            .bind(&subject_type)
            .bind(&subject_id_uuid)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    async fn has_relationship(
        &self,
        object: &Resource,
        relation: &HealthcareRelation,
        subject: &Subject,
    ) -> Result<bool, AuthError> {
        let (resource_type, resource_id) = Self::resource_to_parts(object);
        let (subject_type, subject_id) = Self::subject_to_parts(subject);
        let resource_id_uuid = Uuid::parse_str(&resource_id)?;
        let subject_id_uuid = Uuid::parse_str(&subject_id)?;
        
        let result = sqlx::query_scalar::<_, bool>(authorization_sql::relationships::CHECK_RELATIONSHIP)
            .bind(&resource_type)
            .bind(&resource_id_uuid)
            .bind(&relation.to_string())
            .bind(&subject_type)
            .bind(&subject_id_uuid)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(result)
    }
    
    async fn get_relationships_for_resource(
        &self,
        resource: &Resource,
    ) -> Result<Vec<RelationshipTuple>, AuthError> {
        let (resource_type, resource_id) = Self::resource_to_parts(resource);
        let resource_id_uuid = Uuid::parse_str(&resource_id)?;
        
        let rows = sqlx::query_as::<_, (String, Uuid, String, String, Uuid, Option<serde_json::Value>, Option<DateTime<Utc>>, Option<Uuid>, DateTime<Utc>)>(
            r#"
            SELECT resource_type, resource_id, relation, subject_type, subject_id,
                   metadata, expires_at, created_by, created_at
            FROM authorization_relations
            WHERE resource_type = $1 AND resource_id = $2
            AND is_active = true
            AND (expires_at IS NULL OR expires_at > NOW())
            "#
        )
        .bind(&resource_type)
        .bind(&resource_id_uuid)
        .fetch_all(&self.pool)
        .await?;
        
        let mut relationships = Vec::new();
        for (resource_type_str, resource_id_val, relation_str, subject_type_str, subject_id_val, metadata_val, expires_at_val, created_by_val, created_at_val) in rows {
            let object = Self::parts_to_resource(&resource_type_str, &resource_id_val.to_string())?;
            let subject = Self::parts_to_subject(&subject_type_str, &subject_id_val.to_string())?;
            let relation = relation_str.parse().map_err(|_| {
                AuthError::Storage(anyhow::anyhow!("Invalid relation type: {}", relation_str))
            })?;
            
            let metadata: HashMap<String, String> = metadata_val
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default();
            
            relationships.push(RelationshipTuple {
                object,
                relation,
                subject,
                context: None,
                expires_at: expires_at_val,
                created_by: created_by_val,
                created_at: created_at_val,
                metadata,
            });
        }
        
        Ok(relationships)
    }
    
    async fn get_relationships_for_subject(
        &self,
        subject: &Subject,
    ) -> Result<Vec<RelationshipTuple>, AuthError> {
        let (subject_type, subject_id) = Self::subject_to_parts(subject);
        let subject_id_uuid = Uuid::parse_str(&subject_id)?;
        
        let rows = sqlx::query_as::<_, (String, Uuid, String, String, Uuid, Option<serde_json::Value>, Option<DateTime<Utc>>, Option<Uuid>, DateTime<Utc>)>(
            r#"
            SELECT resource_type, resource_id, relation, subject_type, subject_id,
                   metadata, expires_at, created_by, created_at
            FROM authorization_relations
            WHERE subject_type = $1 AND subject_id = $2
            AND is_active = true
            AND (expires_at IS NULL OR expires_at > NOW())
            "#
        )
        .bind(&subject_type)
        .bind(&subject_id_uuid)
        .fetch_all(&self.pool)
        .await?;
        
        let mut relationships = Vec::new();
        for (resource_type_str, resource_id_val, relation_str, subject_type_str, subject_id_val, metadata_val, expires_at_val, created_by_val, created_at_val) in rows {
            let object = Self::parts_to_resource(&resource_type_str, &resource_id_val.to_string())?;
            let subject = Self::parts_to_subject(&subject_type_str, &subject_id_val.to_string())?;
            let relation = relation_str.parse().map_err(|_| {
                AuthError::Storage(anyhow::anyhow!("Invalid relation type: {}", relation_str))
            })?;
            
            let metadata: HashMap<String, String> = metadata_val
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default();
            
            relationships.push(RelationshipTuple {
                object,
                relation,
                subject,
                context: None,
                expires_at: expires_at_val,
                created_by: created_by_val,
                created_at: created_at_val,
                metadata,
            });
        }
        
        Ok(relationships)
    }
    
    async fn store_policy(&self, policy: &HealthcarePolicy) -> Result<(), AuthError> {
        // Extract policy_type and effect as strings for the database
        let effect_str = match &policy.effect {
            super::policies::PolicyEffect::Allow => "allow",
            super::policies::PolicyEffect::Deny => "deny",
            super::policies::PolicyEffect::RequireApproval => "require_approval",
            super::policies::PolicyEffect::RequireSecondFactor => "require_second_factor",
            super::policies::PolicyEffect::AuditOnly => "audit_only",
            super::policies::PolicyEffect::TimeLimit(_) => "time_limit",
            super::policies::PolicyEffect::Restrict(_) => "restrict",
            super::policies::PolicyEffect::Conditional(_) => "conditional",
        };
        
        let policy_id_uuid = Uuid::parse_str(&policy.id)?;
        
        // For now, we'll store the full policy details in the conditions JSON
        // and use dummy values for created_by and updated_by
        let system_user_id = Uuid::nil();
        
        sqlx::query(
            r#"
            INSERT INTO authorization_policies 
            (id, name, description, conditions, effect, priority, is_active, 
             created_by, updated_by, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (name)
            DO UPDATE SET
                description = EXCLUDED.description,
                conditions = EXCLUDED.conditions,
                effect = EXCLUDED.effect,
                priority = EXCLUDED.priority,
                is_active = EXCLUDED.is_active,
                updated_by = EXCLUDED.updated_by,
                metadata = EXCLUDED.metadata
            "#
        )
        .bind(&policy_id_uuid)
        .bind(&policy.name)
        .bind(&policy.description)
        .bind(&serde_json::to_value(&policy.conditions).unwrap_or_default())
        .bind(effect_str)
        .bind(&policy.priority)
        .bind(&policy.is_active)
        .bind(&system_user_id)
        .bind(&system_user_id)
        .bind(&serde_json::to_value(&policy.metadata).unwrap_or_default())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_policy(&self, policy_id: &str) -> Result<Option<HealthcarePolicy>, AuthError> {
        let policy_id_uuid = Uuid::parse_str(policy_id)?;
        
        let row = sqlx::query_as::<_, (Uuid, String, Option<String>, Option<serde_json::Value>, String, Option<i32>, Option<bool>, Option<DateTime<Utc>>, Option<DateTime<Utc>>, Option<serde_json::Value>)>(
            r#"
            SELECT id, name, description, conditions, effect, priority, is_active,
                   created_at, updated_at, metadata
            FROM authorization_policies
            WHERE id = $1
            "#
        )
        .bind(&policy_id_uuid)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some((id, name, description, conditions_val, effect_str, priority_val, is_active_val, created_at_val, updated_at_val, metadata_val)) = row {
            let conditions = serde_json::from_value(conditions_val.unwrap_or(serde_json::json!([]))).unwrap_or_default();
            let effect = Self::parse_effect(&effect_str);
            let metadata = serde_json::from_value(metadata_val.unwrap_or(serde_json::json!({}))).unwrap_or_default();
            
            Ok(Some(HealthcarePolicy {
                id: id.to_string(),
                name,
                description: description.unwrap_or_default(),
                policy_type: super::policies::PolicyType::Default, // Default value since it's not in DB
                conditions,
                effect,
                priority: priority_val.unwrap_or(0),
                is_active: is_active_val.unwrap_or(false),
                created_at: created_at_val.unwrap_or_else(chrono::Utc::now),
                updated_at: updated_at_val.unwrap_or_else(chrono::Utc::now),
                metadata,
            }))
        } else {
            Ok(None)
        }
    }
    
    async fn get_active_policies(&self) -> Result<Vec<HealthcarePolicy>, AuthError> {
        let rows = sqlx::query_as::<_, (Uuid, String, Option<String>, Option<serde_json::Value>, String, Option<i32>, Option<bool>, Option<DateTime<Utc>>, Option<DateTime<Utc>>, Option<serde_json::Value>)>(
            r#"
            SELECT id, name, description, conditions, effect, priority, is_active,
                   created_at, updated_at, metadata
            FROM authorization_policies
            WHERE is_active = true
            ORDER BY priority DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut policies = Vec::new();
        for (id, name, description, conditions_val, effect_str, priority_val, is_active_val, created_at_val, updated_at_val, metadata_val) in rows {
            let conditions = serde_json::from_value(conditions_val.unwrap_or(serde_json::json!([]))).unwrap_or_default();
            let effect = Self::parse_effect(&effect_str);
            let metadata = serde_json::from_value(metadata_val.unwrap_or(serde_json::json!({}))).unwrap_or_default();
            
            policies.push(HealthcarePolicy {
                id: id.to_string(),
                name,
                description: description.unwrap_or_default(),
                policy_type: super::policies::PolicyType::Default, // Default value since it's not in DB
                conditions,
                effect,
                priority: priority_val.unwrap_or(0),
                is_active: is_active_val.unwrap_or(false),
                created_at: created_at_val.unwrap_or_else(chrono::Utc::now),
                updated_at: updated_at_val.unwrap_or_else(chrono::Utc::now),
                metadata,
            });
        }
        
        Ok(policies)
    }
    
    async fn store_audit_entry(&self, entry: &AuditEntry) -> Result<(), AuthError> {
        let decision_str = match &entry.decision {
            super::audit::AccessDecision::Allow => "allow",
            super::audit::AccessDecision::Deny => "deny",
            super::audit::AccessDecision::RequireApproval => "require_approval",
            super::audit::AccessDecision::RequireMFA => "require_mfa",
            super::audit::AccessDecision::AllowWithRestrictions => "allow_with_restrictions",
            super::audit::AccessDecision::EmergencyAccess => "emergency_access",
            super::audit::AccessDecision::BreakGlassAccess => "emergency_access", // Map to emergency_access
        };
        
        let resource_id_uuid = Uuid::parse_str(&entry.resource_id)?;
        
        // Parse IP address string to IpAddr type if present
        let ip_addr = entry.ip_address.as_ref()
            .and_then(|ip_str| ip_str.parse::<std::net::IpAddr>().ok());
        
        // Convert IP to sqlx-compatible type (use sqlx::types::ipnetwork::IpNetwork or just String)
        let ip_str = ip_addr.as_ref().map(|ip| ip.to_string());
        
        // Convert reasons Vec to SQL array
        let reasons_array: Vec<String> = entry.reasons.clone();
        
        sqlx::query(
            r#"
            INSERT INTO authorization_audit_log 
            (user_id, action, resource_type, resource_id, decision, reasons,
             ip_address, user_agent, session_id, context_data, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7::inet, $8, $9, $10, $11)
            "#
        )
        .bind(&entry.user_id)
        .bind(&entry.action.to_string())
        .bind(&entry.resource_namespace)
        .bind(&resource_id_uuid)
        .bind(decision_str)
        .bind(&reasons_array[..])
        .bind(&ip_str)
        .bind(&entry.user_agent)
        .bind(&entry.session_id)
        .bind(&serde_json::to_value(&entry.request_context).unwrap_or_default())
        .bind(&serde_json::to_value(&entry.metadata).unwrap_or_default())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn cleanup_expired_relationships(&self) -> Result<u64, AuthError> {
        let result = sqlx::query(
            r#"
            UPDATE authorization_relations 
            SET is_active = false 
            WHERE expires_at IS NOT NULL AND expires_at <= NOW()
            AND is_active = true
            "#
        )
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected())
    }
}

#[async_trait]
impl RelationStorage for PostgresAuthorizationStorage {
    async fn find_direct_relationships(
        &self,
        object: &Resource,
        relation: &HealthcareRelation,
    ) -> Result<Vec<Subject>, AuthError> {
        let (resource_type, resource_id) = Self::resource_to_parts(object);
        
        let rows = sqlx::query_as::<_, (String, uuid::Uuid, String, chrono::DateTime<chrono::Utc>, serde_json::Value)>(authorization_sql::relationships::FIND_RELATIONSHIPS_FOR_RESOURCE)
            .bind(&resource_type)
            .bind(&Uuid::parse_str(&resource_id)?)
            .bind(&relation.to_string())
            .fetch_all(&self.pool)
            .await?;
        
        let mut subjects = Vec::new();
        for (subject_type, subject_id, _relation, _created_at, _metadata) in rows {
            let subject = Self::parts_to_subject(&subject_type, &subject_id.to_string())?;
            subjects.push(subject);
        }
        
        Ok(subjects)
    }
    
    async fn find_inherited_relationships(
        &self,
        _object: &Resource,
        _relation: &HealthcareRelation,
        _max_depth: u8,
    ) -> Result<Vec<Subject>, AuthError> {
        // This would implement recursive relationship resolution
        // For now, return empty to avoid complexity
        Ok(Vec::new())
    }
    
    async fn get_subject_hierarchy(&self, _subject: &Subject) -> Result<Vec<Subject>, AuthError> {
        // This would implement subject hierarchy traversal
        // For now, return empty to avoid complexity
        Ok(Vec::new())
    }
}

#[async_trait]
impl PolicyStorage for PostgresAuthorizationStorage {
    async fn get_policies_by_type(&self, policy_type: &str) -> Result<Vec<HealthcarePolicy>, AuthError> {
        // For now, return active policies and filter by name/description containing the type
        let rows = sqlx::query(authorization_sql::policies::GET_ACTIVE_POLICIES)
            .fetch_all(&self.pool)
            .await?;

        // Simple filtering and conversion - implement proper HealthcarePolicy conversion based on your struct
        let policies = vec![]; // Placeholder - implement proper row to HealthcarePolicy conversion
        Ok(policies)
    }
    
    async fn update_policy_status(&self, policy_id: &str, is_active: bool) -> Result<(), AuthError> {
        let policy_id_uuid = Uuid::parse_str(policy_id)?;
        
        sqlx::query(
            r#"
            UPDATE authorization_policies
            SET is_active = $1, updated_at = NOW()
            WHERE id = $2
            "#
        )
        .bind(&is_active)
        .bind(&policy_id_uuid)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_policy_usage_stats(&self, policy_id: &str) -> Result<PolicyUsageStats, AuthError> {
        let row = sqlx::query_as::<_, (i64, i64, i64, Option<DateTime<Utc>>)>(
            r#"
            SELECT 
                COUNT(*) as total_evaluations,
                SUM(CASE WHEN decision = 'allow' THEN 1 ELSE 0 END) as allow_decisions,
                SUM(CASE WHEN decision = 'deny' THEN 1 ELSE 0 END) as deny_decisions,
                MAX(timestamp) as last_used
            FROM authorization_audit_log
            WHERE $1 = ANY(reasons)
            "#
        )
        .bind(policy_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(PolicyUsageStats {
            policy_id: policy_id.to_string(),
            total_evaluations: row.0,
            allow_decisions: row.1,
            deny_decisions: row.2,
            last_used: row.3,
        })
    }
}