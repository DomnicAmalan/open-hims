// src/modules/authorization/relations.rs
//! Core types and relationships for healthcare authorization
//! 
//! This module defines the fundamental building blocks of the authorization system:
//! subjects (who), resources (what), actions (how), and relationships between them.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt::Display;
use std::str::FromStr;
use std::collections::HashMap;

/// Represents entities that can perform actions (subjects in authorization)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subject {
    /// Individual user (doctor, nurse, admin, etc.)
    User(Uuid),
    /// Role-based subject (e.g., "physician", "nurse", "admin")
    Role(String),
    /// Department-based subject
    Department(Uuid),
    /// Organization-based subject
    Organization(Uuid),
    /// System service subject
    System(String),
    /// Group of users
    Group(Uuid),
}

/// Represents resources that can be acted upon
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resource {
    /// Patient record
    Patient(Uuid),
    /// Medical record/document
    MedicalRecord(Uuid),
    /// Appointment
    Appointment(Uuid),
    /// Department
    Department(Uuid),
    /// Organization
    Organization(Uuid),
    /// Prescription
    Prescription(Uuid),
    /// Laboratory result
    LabResult(Uuid),
    /// Medical imaging study
    ImagingStudy(Uuid),
    /// Report
    Report(Uuid),
    /// Billing information
    Billing(Uuid),
    /// Care plan
    CarePlan(Uuid),
    /// Encounter/visit
    Encounter(Uuid),
    /// Clinical decision support
    ClinicalDecisionSupport(Uuid),
    /// Research data
    ResearchData(Uuid),
    /// System configuration
    SystemConfig(String),
}

/// Actions that can be performed on resources
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    // FHIR-standard operations
    Read,
    Write,
    Create,
    Delete,
    Search,
    Update,
    
    // Clinical actions
    Prescribe,
    Diagnose,
    OrderTest,
    ViewResults,
    ModifyTreatment,
    ApproveTest,
    
    // Administrative actions
    Schedule,
    Cancel,
    Approve,
    Reject,
    Audit,
    Configure,
    
    // Emergency actions
    EmergencyAccess,
    BreakGlass,
    
    // Reporting and analytics
    GenerateReport,
    ExportData,
    ViewAnalytics,
    
    // Billing and financial
    ViewBilling,
    ProcessPayment,
    AdjustBilling,
    
    // Research and compliance
    ResearchAccess,
    DeIdentify,
    
    // System administration
    ManageUsers,
    ManageRoles,
    ManagePermissions,
    BackupData,
    RestoreData,
    
    // Custom action for extensibility
    Custom(String),
}

/// Healthcare-specific relationships between subjects and resources
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealthcareRelation {
    // Patient-Provider relationships
    PrimaryPhysician,
    ConsultingPhysician,
    SpecialistReferral,
    AttendingNurse,
    CareTeamMember,
    EmergencyContact,
    Guardian,
    NextOfKin,
    
    // Organizational relationships
    DepartmentHead,
    DepartmentMember,
    HospitalAdmin,
    SystemAdmin,
    ChiefOfStaff,
    MedicalDirector,
    
    // Treatment relationships
    TreatingPhysician,
    OrderingPhysician,
    SupervisingPhysician,
    ConsultingSpecialist,
    SecondOpinion,
    
    // Access control relationships
    ProxyAccess,
    DelegatedAccess,
    TemporaryAccess,
    ResearchAccess,
    BillingAccess,
    AuditAccess,
    
    // Location-based relationships
    LocationAccess,
    CrossLocationAccess,
    RemoteAccess,
    
    // Hierarchical relationships
    Manager,
    Subordinate,
    Peer,
    Colleague,
    
    // Workflow relationships
    Approver,
    Reviewer,
    Supervisor,
    Delegate,
    
    // Data relationships
    DataOwner,
    DataProcessor,
    DataController,
    DataSubject,
    
    // Custom relationship for extensibility
    Custom(String),
}

/// A relationship tuple representing a connection between a subject and resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipTuple {
    /// The resource being accessed
    pub object: Resource,
    /// The type of relationship
    pub relation: HealthcareRelation,
    /// The subject having the relationship
    pub subject: Subject,
    /// Optional context for the relationship
    pub context: Option<String>,
    /// When this relationship expires (if applicable)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Who created this relationship
    pub created_by: Option<Uuid>,
    /// When this relationship was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl RelationshipTuple {
    /// Create a new relationship tuple
    pub fn new(
        object: Resource,
        relation: HealthcareRelation,
        subject: Subject,
    ) -> Self {
        Self {
            object,
            relation,
            subject,
            context: None,
            expires_at: None,
            created_by: None,
            created_at: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add context to the relationship
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
    
    /// Set expiration time for the relationship
    pub fn with_expiration(mut self, expires_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
    
    /// Set who created this relationship
    pub fn with_creator(mut self, created_by: Uuid) -> Self {
        self.created_by = Some(created_by);
        self
    }
    
    /// Add metadata to the relationship
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    /// Check if the relationship has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }
    
    /// Get a string representation of this tuple for caching/logging
    pub fn to_string_key(&self) -> String {
        format!("{}#{}#{}", self.object, self.relation, self.subject)
    }
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::User(id) => write!(f, "user:{}", id),
            Subject::Role(role) => write!(f, "role:{}", role),
            Subject::Department(id) => write!(f, "department:{}", id),
            Subject::Organization(id) => write!(f, "organization:{}", id),
            Subject::System(name) => write!(f, "system:{}", name),
            Subject::Group(id) => write!(f, "group:{}", id),
        }
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::Patient(id) => write!(f, "patient:{}", id),
            Resource::MedicalRecord(id) => write!(f, "medical_record:{}", id),
            Resource::Appointment(id) => write!(f, "appointment:{}", id),
            Resource::Department(id) => write!(f, "department:{}", id),
            Resource::Organization(id) => write!(f, "organization:{}", id),
            Resource::Prescription(id) => write!(f, "prescription:{}", id),
            Resource::LabResult(id) => write!(f, "lab_result:{}", id),
            Resource::ImagingStudy(id) => write!(f, "imaging_study:{}", id),
            Resource::Report(id) => write!(f, "report:{}", id),
            Resource::Billing(id) => write!(f, "billing:{}", id),
            Resource::CarePlan(id) => write!(f, "care_plan:{}", id),
            Resource::Encounter(id) => write!(f, "encounter:{}", id),
            Resource::ClinicalDecisionSupport(id) => write!(f, "clinical_decision_support:{}", id),
            Resource::ResearchData(id) => write!(f, "research_data:{}", id),
            Resource::SystemConfig(name) => write!(f, "system_config:{}", name),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Read => write!(f, "read"),
            Action::Write => write!(f, "write"),
            Action::Create => write!(f, "create"),
            Action::Delete => write!(f, "delete"),
            Action::Search => write!(f, "search"),
            Action::Update => write!(f, "update"),
            Action::Prescribe => write!(f, "prescribe"),
            Action::Diagnose => write!(f, "diagnose"),
            Action::OrderTest => write!(f, "order_test"),
            Action::ViewResults => write!(f, "view_results"),
            Action::ModifyTreatment => write!(f, "modify_treatment"),
            Action::ApproveTest => write!(f, "approve_test"),
            Action::Schedule => write!(f, "schedule"),
            Action::Cancel => write!(f, "cancel"),
            Action::Approve => write!(f, "approve"),
            Action::Reject => write!(f, "reject"),
            Action::Audit => write!(f, "audit"),
            Action::Configure => write!(f, "configure"),
            Action::EmergencyAccess => write!(f, "emergency_access"),
            Action::BreakGlass => write!(f, "break_glass"),
            Action::GenerateReport => write!(f, "generate_report"),
            Action::ExportData => write!(f, "export_data"),
            Action::ViewAnalytics => write!(f, "view_analytics"),
            Action::ViewBilling => write!(f, "view_billing"),
            Action::ProcessPayment => write!(f, "process_payment"),
            Action::AdjustBilling => write!(f, "adjust_billing"),
            Action::ResearchAccess => write!(f, "research_access"),
            Action::DeIdentify => write!(f, "deidentify"),
            Action::ManageUsers => write!(f, "manage_users"),
            Action::ManageRoles => write!(f, "manage_roles"),
            Action::ManagePermissions => write!(f, "manage_permissions"),
            Action::BackupData => write!(f, "backup_data"),
            Action::RestoreData => write!(f, "restore_data"),
            Action::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl Display for HealthcareRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthcareRelation::PrimaryPhysician => write!(f, "primary_physician"),
            HealthcareRelation::ConsultingPhysician => write!(f, "consulting_physician"),
            HealthcareRelation::SpecialistReferral => write!(f, "specialist_referral"),
            HealthcareRelation::AttendingNurse => write!(f, "attending_nurse"),
            HealthcareRelation::CareTeamMember => write!(f, "care_team_member"),
            HealthcareRelation::EmergencyContact => write!(f, "emergency_contact"),
            HealthcareRelation::Guardian => write!(f, "guardian"),
            HealthcareRelation::NextOfKin => write!(f, "next_of_kin"),
            HealthcareRelation::DepartmentHead => write!(f, "department_head"),
            HealthcareRelation::DepartmentMember => write!(f, "department_member"),
            HealthcareRelation::HospitalAdmin => write!(f, "hospital_admin"),
            HealthcareRelation::SystemAdmin => write!(f, "system_admin"),
            HealthcareRelation::ChiefOfStaff => write!(f, "chief_of_staff"),
            HealthcareRelation::MedicalDirector => write!(f, "medical_director"),
            HealthcareRelation::TreatingPhysician => write!(f, "treating_physician"),
            HealthcareRelation::OrderingPhysician => write!(f, "ordering_physician"),
            HealthcareRelation::SupervisingPhysician => write!(f, "supervising_physician"),
            HealthcareRelation::ConsultingSpecialist => write!(f, "consulting_specialist"),
            HealthcareRelation::SecondOpinion => write!(f, "second_opinion"),
            HealthcareRelation::ProxyAccess => write!(f, "proxy_access"),
            HealthcareRelation::DelegatedAccess => write!(f, "delegated_access"),
            HealthcareRelation::TemporaryAccess => write!(f, "temporary_access"),
            HealthcareRelation::ResearchAccess => write!(f, "research_access"),
            HealthcareRelation::BillingAccess => write!(f, "billing_access"),
            HealthcareRelation::AuditAccess => write!(f, "audit_access"),
            HealthcareRelation::LocationAccess => write!(f, "location_access"),
            HealthcareRelation::CrossLocationAccess => write!(f, "cross_location_access"),
            HealthcareRelation::RemoteAccess => write!(f, "remote_access"),
            HealthcareRelation::Manager => write!(f, "manager"),
            HealthcareRelation::Subordinate => write!(f, "subordinate"),
            HealthcareRelation::Peer => write!(f, "peer"),
            HealthcareRelation::Colleague => write!(f, "colleague"),
            HealthcareRelation::Approver => write!(f, "approver"),
            HealthcareRelation::Reviewer => write!(f, "reviewer"),
            HealthcareRelation::Supervisor => write!(f, "supervisor"),
            HealthcareRelation::Delegate => write!(f, "delegate"),
            HealthcareRelation::DataOwner => write!(f, "data_owner"),
            HealthcareRelation::DataProcessor => write!(f, "data_processor"),
            HealthcareRelation::DataController => write!(f, "data_controller"),
            HealthcareRelation::DataSubject => write!(f, "data_subject"),
            HealthcareRelation::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Helper functions for relationship management
impl HealthcareRelation {
    /// Get the permissions typically associated with this relationship
    pub fn default_permissions(&self) -> Vec<Action> {
        match self {
            HealthcareRelation::PrimaryPhysician => vec![
                Action::Read, Action::Write, Action::Update, Action::Prescribe, 
                Action::Diagnose, Action::ModifyTreatment, Action::OrderTest,
                Action::ViewResults, Action::Schedule
            ],
            HealthcareRelation::ConsultingPhysician => vec![
                Action::Read, Action::Prescribe, Action::Diagnose, Action::ViewResults
            ],
            HealthcareRelation::AttendingNurse => vec![
                Action::Read, Action::Update, Action::Schedule, Action::ViewResults
            ],
            HealthcareRelation::EmergencyContact => vec![
                Action::EmergencyAccess, Action::Read
            ],
            HealthcareRelation::DepartmentHead => vec![
                Action::Read, Action::Write, Action::Update, Action::Approve, 
                Action::GenerateReport, Action::ViewAnalytics, Action::ManageUsers
            ],
            HealthcareRelation::HospitalAdmin => vec![
                Action::Read, Action::Write, Action::Update, Action::Configure,
                Action::ManageUsers, Action::ManageRoles, Action::Audit
            ],
            HealthcareRelation::SystemAdmin => vec![
                Action::Configure, Action::ManageUsers, Action::ManageRoles,
                Action::ManagePermissions, Action::BackupData, Action::RestoreData
            ],
            HealthcareRelation::BillingAccess => vec![
                Action::ViewBilling, Action::ProcessPayment, Action::AdjustBilling
            ],
            HealthcareRelation::ResearchAccess => vec![
                Action::ResearchAccess, Action::DeIdentify, Action::ExportData
            ],
            HealthcareRelation::AuditAccess => vec![
                Action::Audit, Action::ViewAnalytics, Action::GenerateReport
            ],
            _ => vec![Action::Read], // Default to read-only for other relations
        }
    }
    
    /// Check if this relationship can inherit from another relationship
    pub fn can_inherit_from(&self, other: &HealthcareRelation) -> bool {
        match (self, other) {
            // Department heads can inherit from department members
            (HealthcareRelation::DepartmentHead, HealthcareRelation::DepartmentMember) => true,
            // Primary physicians can delegate to consulting physicians
            (HealthcareRelation::ConsultingPhysician, HealthcareRelation::PrimaryPhysician) => true,
            // Supervisors can inherit from subordinates
            (HealthcareRelation::Supervisor, HealthcareRelation::Subordinate) => true,
            // System admins can inherit from hospital admins
            (HealthcareRelation::SystemAdmin, HealthcareRelation::HospitalAdmin) => true,
            _ => false,
        }
    }
    
    /// Get the inverse relationship if it exists
    pub fn inverse(&self) -> Option<HealthcareRelation> {
        match self {
            HealthcareRelation::Manager => Some(HealthcareRelation::Subordinate),
            HealthcareRelation::Subordinate => Some(HealthcareRelation::Manager),
            HealthcareRelation::Supervisor => Some(HealthcareRelation::Subordinate),
            HealthcareRelation::DataController => Some(HealthcareRelation::DataSubject),
            HealthcareRelation::DataSubject => Some(HealthcareRelation::DataController),
            _ => None,
        }
    }
}

/// Parse HealthcareRelation from string
impl FromStr for HealthcareRelation {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "primary_physician" => Ok(HealthcareRelation::PrimaryPhysician),
            "consulting_physician" => Ok(HealthcareRelation::ConsultingPhysician),
            "specialist_referral" => Ok(HealthcareRelation::SpecialistReferral),
            "attending_nurse" => Ok(HealthcareRelation::AttendingNurse),
            "care_team_member" => Ok(HealthcareRelation::CareTeamMember),
            "emergency_contact" => Ok(HealthcareRelation::EmergencyContact),
            "guardian" => Ok(HealthcareRelation::Guardian),
            "next_of_kin" => Ok(HealthcareRelation::NextOfKin),
            "department_head" => Ok(HealthcareRelation::DepartmentHead),
            "department_member" => Ok(HealthcareRelation::DepartmentMember),
            "hospital_admin" => Ok(HealthcareRelation::HospitalAdmin),
            "system_admin" => Ok(HealthcareRelation::SystemAdmin),
            "chief_of_staff" => Ok(HealthcareRelation::ChiefOfStaff),
            "medical_director" => Ok(HealthcareRelation::MedicalDirector),
            "treating_physician" => Ok(HealthcareRelation::TreatingPhysician),
            "ordering_physician" => Ok(HealthcareRelation::OrderingPhysician),
            "supervising_physician" => Ok(HealthcareRelation::SupervisingPhysician),
            "consulting_specialist" => Ok(HealthcareRelation::ConsultingSpecialist),
            "second_opinion" => Ok(HealthcareRelation::SecondOpinion),
            "proxy_access" => Ok(HealthcareRelation::ProxyAccess),
            "delegated_access" => Ok(HealthcareRelation::DelegatedAccess),
            "temporary_access" => Ok(HealthcareRelation::TemporaryAccess),
            "research_access" => Ok(HealthcareRelation::ResearchAccess),
            "billing_access" => Ok(HealthcareRelation::BillingAccess),
            "audit_access" => Ok(HealthcareRelation::AuditAccess),
            "location_access" => Ok(HealthcareRelation::LocationAccess),
            "cross_location_access" => Ok(HealthcareRelation::CrossLocationAccess),
            "remote_access" => Ok(HealthcareRelation::RemoteAccess),
            "manager" => Ok(HealthcareRelation::Manager),
            "subordinate" => Ok(HealthcareRelation::Subordinate),
            "peer" => Ok(HealthcareRelation::Peer),
            "colleague" => Ok(HealthcareRelation::Colleague),
            "approver" => Ok(HealthcareRelation::Approver),
            "reviewer" => Ok(HealthcareRelation::Reviewer),
            "supervisor" => Ok(HealthcareRelation::Supervisor),
            "delegate" => Ok(HealthcareRelation::Delegate),
            "data_owner" => Ok(HealthcareRelation::DataOwner),
            "data_processor" => Ok(HealthcareRelation::DataProcessor),
            "data_controller" => Ok(HealthcareRelation::DataController),
            "data_subject" => Ok(HealthcareRelation::DataSubject),
            other => Ok(HealthcareRelation::Custom(other.to_string())),
        }
    }
}