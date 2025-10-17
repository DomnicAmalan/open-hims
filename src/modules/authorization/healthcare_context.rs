// src/modules/authorization/healthcare_context.rs
//! Healthcare-specific context for authorization decisions
//! 
//! This module provides context structures that capture the healthcare-specific
//! information needed to make informed authorization decisions, including clinical
//! context, emergency situations, and location information.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Timelike, Datelike};
use std::collections::HashMap;

/// Main request context that encapsulates all contextual information for an authorization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    /// Unique session identifier
    pub session_id: Option<String>,
    /// Client IP address
    pub ip_address: Option<String>,
    /// User agent string
    pub user_agent: Option<String>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Geographic/physical location context
    pub location: Option<LocationContext>,
    /// Clinical context information
    pub clinical: Option<ClinicalContext>,
    /// Emergency context if applicable
    pub emergency: Option<EmergencyContext>,
    /// Audit trail entries
    pub audit_trail: Vec<String>,
    /// Additional request headers
    pub headers: HashMap<String, String>,
    /// Request path/endpoint
    pub endpoint: Option<String>,
    /// HTTP method
    pub method: Option<String>,
}

/// Location context for geographic and facility-based authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationContext {
    /// Hospital/facility identifier
    pub hospital_id: Uuid,
    /// Department within the facility
    pub department_id: Option<Uuid>,
    /// Building identifier
    pub building: Option<String>,
    /// Floor number or identifier
    pub floor: Option<String>,
    /// Room number or identifier
    pub room: Option<String>,
    /// GPS coordinates if available
    pub coordinates: Option<(f64, f64)>,
    /// Time zone of the location
    pub timezone: Option<String>,
    /// Whether this is a remote access location
    pub is_remote: bool,
    /// VPN or secure connection information
    pub connection_info: Option<ConnectionInfo>,
}

/// Clinical context that provides healthcare-specific information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalContext {
    /// Current patient being accessed
    pub patient_id: Option<Uuid>,
    /// Current encounter/visit
    pub encounter_id: Option<Uuid>,
    /// Active care plan
    pub care_plan_id: Option<Uuid>,
    /// Clinical status of the patient
    pub clinical_status: Option<String>,
    /// Urgency level of the request
    pub urgency_level: UrgencyLevel,
    /// Care team members involved
    pub care_team_members: Vec<Uuid>,
    /// Clinical protocols in effect
    pub active_protocols: Vec<String>,
    /// Specialty or department context
    pub specialty: Option<String>,
    /// Whether this is part of a clinical workflow
    pub workflow_context: Option<WorkflowContext>,
}

/// Emergency context for break-glass and emergency access scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyContext {
    /// Whether this is an emergency situation
    pub is_emergency: bool,
    /// Type of emergency
    pub emergency_type: Option<EmergencyType>,
    /// Who declared the emergency
    pub declared_by: Option<Uuid>,
    /// When the emergency was declared
    pub declared_at: Option<DateTime<Utc>>,
    /// Justification for emergency access
    pub justification: Option<String>,
    /// Whether secondary approval is required
    pub approval_required: bool,
    /// Approving authority if needed
    pub approved_by: Option<Uuid>,
    /// When approval was granted
    pub approved_at: Option<DateTime<Utc>>,
    /// Emergency access duration limit
    pub expires_at: Option<DateTime<Utc>>,
}

/// Connection information for remote or VPN access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    /// VPN connection status
    pub is_vpn: bool,
    /// VPN provider if applicable
    pub vpn_provider: Option<String>,
    /// SSL/TLS certificate information
    pub tls_info: Option<String>,
    /// Network security level
    pub security_level: SecurityLevel,
}

/// Workflow context for clinical processes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {
    /// Workflow identifier
    pub workflow_id: String,
    /// Current step in the workflow
    pub current_step: String,
    /// Workflow initiator
    pub initiated_by: Uuid,
    /// Workflow priority
    pub priority: WorkflowPriority,
    /// Related workflow instances
    pub related_workflows: Vec<String>,
}

/// Urgency levels for clinical situations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UrgencyLevel {
    /// Routine, non-urgent access
    Routine,
    /// Urgent but not critical
    Urgent,
    /// Emergency situation
    Emergency,
    /// Life-threatening critical situation
    Critical,
}

/// Types of emergency situations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyType {
    /// Medical emergency with patient
    MedicalEmergency,
    /// Code blue/cardiac arrest
    CodeBlue,
    /// Code red/fire emergency
    CodeRed,
    /// System failure requiring immediate access
    SystemFailure,
    /// Security breach response
    SecurityBreach,
    /// Natural disaster response
    NaturalDisaster,
    /// General break-glass access
    BreakGlass,
    /// After-hours emergency access
    AfterHoursEmergency,
    /// Mass casualty incident
    MassCasualty,
    /// Custom emergency type
    Custom(String),
}

/// Network security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Low security (public networks)
    Low,
    /// Medium security (corporate networks)
    Medium,
    /// High security (VPN, encrypted)
    High,
    /// Maximum security (dedicated lines)
    Maximum,
}

/// Workflow priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for RequestContext {
    fn default() -> Self {
        Self {
            session_id: None,
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now(),
            location: None,
            clinical: None,
            emergency: None,
            audit_trail: Vec::new(),
            headers: HashMap::new(),
            endpoint: None,
            method: None,
        }
    }
}

impl RequestContext {
    /// Create a new request context
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add session information
    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }
    
    /// Add IP address
    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip_address = Some(ip);
        self
    }
    
    /// Add user agent
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }
    
    /// Add location context
    pub fn with_location(mut self, location: LocationContext) -> Self {
        self.location = Some(location);
        self
    }
    
    /// Add clinical context
    pub fn with_clinical(mut self, clinical: ClinicalContext) -> Self {
        self.clinical = Some(clinical);
        self
    }
    
    /// Add emergency context
    pub fn with_emergency(mut self, emergency: EmergencyContext) -> Self {
        self.emergency = Some(emergency);
        self
    }
    
    /// Add an audit trail entry
    pub fn add_audit_entry(mut self, entry: String) -> Self {
        self.audit_trail.push(entry);
        self
    }
    
    /// Add a header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    /// Set the endpoint
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
    
    /// Set the HTTP method
    pub fn with_method(mut self, method: String) -> Self {
        self.method = Some(method);
        self
    }
    
    /// Check if this is an emergency situation
    pub fn is_emergency(&self) -> bool {
        self.emergency.as_ref().map_or(false, |e| e.is_emergency)
    }
    
    /// Get the urgency level
    pub fn get_urgency_level(&self) -> UrgencyLevel {
        self.clinical.as_ref()
            .map(|c| c.urgency_level.clone())
            .unwrap_or(UrgencyLevel::Routine)
    }
    
    /// Check if this is a remote access request
    pub fn is_remote_access(&self) -> bool {
        self.location.as_ref().map_or(false, |l| l.is_remote)
    }
    
    /// Check if this is after hours access
    pub fn is_after_hours(&self) -> bool {
        let hour = self.timestamp.hour();
        hour < 8 || hour > 18 // Simple after-hours check (8 AM - 6 PM)
    }
    
    /// Check if this is weekend access
    pub fn is_weekend(&self) -> bool {
        let weekday = self.timestamp.weekday();
        weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun
    }
    
    /// Get security level based on connection and location
    pub fn get_security_level(&self) -> SecurityLevel {
        if let Some(location) = &self.location {
            if let Some(conn_info) = &location.connection_info {
                conn_info.security_level.clone()
            } else if location.is_remote {
                SecurityLevel::Medium
            } else {
                SecurityLevel::High
            }
        } else {
            SecurityLevel::Low
        }
    }
    
    /// Validate the context for completeness and consistency
    pub fn validate(&self) -> Result<(), String> {
        // Check for emergency context consistency
        if let Some(emergency) = &self.emergency {
            if emergency.is_emergency && emergency.justification.is_none() {
                return Err("Emergency access requires justification".to_string());
            }
            
            if emergency.approval_required && emergency.approved_by.is_none() {
                return Err("Emergency access requires approval".to_string());
            }
            
            if let Some(expires_at) = emergency.expires_at {
                if expires_at <= self.timestamp {
                    return Err("Emergency access has expired".to_string());
                }
            }
        }
        
        // Check for clinical context consistency
        if let Some(clinical) = &self.clinical {
            if clinical.urgency_level == UrgencyLevel::Critical && !self.is_emergency() {
                return Err("Critical urgency level requires emergency context".to_string());
            }
        }
        
        Ok(())
    }
}

impl Default for UrgencyLevel {
    fn default() -> Self {
        UrgencyLevel::Routine
    }
}

impl LocationContext {
    /// Create a new location context
    pub fn new(hospital_id: Uuid) -> Self {
        Self {
            hospital_id,
            department_id: None,
            building: None,
            floor: None,
            room: None,
            coordinates: None,
            timezone: None,
            is_remote: false,
            connection_info: None,
        }
    }
    
    /// Mark as remote access
    pub fn as_remote(mut self) -> Self {
        self.is_remote = true;
        self
    }
    
    /// Add connection information
    pub fn with_connection_info(mut self, connection_info: ConnectionInfo) -> Self {
        self.connection_info = Some(connection_info);
        self
    }
}

impl ClinicalContext {
    /// Create a new clinical context
    pub fn new() -> Self {
        Self {
            patient_id: None,
            encounter_id: None,
            care_plan_id: None,
            clinical_status: None,
            urgency_level: UrgencyLevel::Routine,
            care_team_members: Vec::new(),
            active_protocols: Vec::new(),
            specialty: None,
            workflow_context: None,
        }
    }
    
    /// Set the patient
    pub fn with_patient(mut self, patient_id: Uuid) -> Self {
        self.patient_id = Some(patient_id);
        self
    }
    
    /// Set urgency level
    pub fn with_urgency(mut self, urgency_level: UrgencyLevel) -> Self {
        self.urgency_level = urgency_level;
        self
    }
    
    /// Add a care team member
    pub fn add_care_team_member(mut self, member_id: Uuid) -> Self {
        self.care_team_members.push(member_id);
        self
    }
}

impl EmergencyContext {
    /// Create a new emergency context
    pub fn new(emergency_type: EmergencyType, declared_by: Uuid, justification: String) -> Self {
        Self {
            is_emergency: true,
            emergency_type: Some(emergency_type),
            declared_by: Some(declared_by),
            declared_at: Some(Utc::now()),
            justification: Some(justification),
            approval_required: false,
            approved_by: None,
            approved_at: None,
            expires_at: None,
        }
    }
    
    /// Require approval for this emergency access
    pub fn require_approval(mut self) -> Self {
        self.approval_required = true;
        self
    }
    
    /// Set approval information
    pub fn with_approval(mut self, approved_by: Uuid) -> Self {
        self.approved_by = Some(approved_by);
        self.approved_at = Some(Utc::now());
        self
    }
    
    /// Set expiration time
    pub fn with_expiration(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
}