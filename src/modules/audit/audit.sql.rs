/// Audit SQL Queries
/// 
/// This file contains all SQL queries used by the audit service
/// for clean separation of concerns and better maintainability.

/// Insert a new audit log entry
pub const INSERT_AUDIT_LOG: &str = r#"
    INSERT INTO audit_logs (
        id, event_type, user_id, patient_id, appointment_id, resource_type,
        resource_id, action, outcome, timestamp, source_ip, user_agent, details
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
    )
"#;

/// Get audit logs by user ID
pub const GET_AUDIT_LOGS_BY_USER: &str = r#"
    SELECT 
        id, event_type, user_id, patient_id, appointment_id,
        resource_type, resource_id, action, outcome, timestamp,
        source_ip, user_agent, details
    FROM audit_logs
    WHERE user_id = $1
    ORDER BY timestamp DESC
    LIMIT $2 OFFSET $3
"#;

/// Get audit log by ID
pub const GET_AUDIT_LOG_BY_ID: &str = r#"
    SELECT 
        id, event_type, user_id, patient_id, appointment_id,
        resource_type, resource_id, action, outcome, timestamp,
        source_ip, user_agent, details
    FROM audit_logs
    WHERE id = $1
"#;

/// Get access statistics for HIPAA compliance
pub const GET_ACCESS_STATISTICS: &str = r#"
    SELECT 
        COUNT(*) as total_access_events,
        COUNT(DISTINCT user_id) as unique_users
    FROM audit_logs 
    WHERE event_type IN ('patient-access', 'read', 'access')
        AND timestamp >= CURRENT_DATE - INTERVAL '30 days'
"#;

/// Get failed access attempts
pub const GET_FAILED_ACCESS_ATTEMPTS: &str = r#"
    SELECT COUNT(*) as failed_attempts
    FROM audit_logs 
    WHERE outcome != 'success'
        AND timestamp >= CURRENT_DATE - INTERVAL '30 days'
"#;

/// Get data modifications count
pub const GET_DATA_MODIFICATIONS: &str = r#"
    SELECT COUNT(*) as total_modifications
    FROM audit_logs 
    WHERE action IN ('create', 'update', 'delete')
        AND timestamp >= CURRENT_DATE - INTERVAL '30 days'
"#;

/// Get user activity for report
pub const GET_USER_ACTIVITY: &str = r#"
    SELECT 
        event_type,
        action,
        timestamp
    FROM audit_logs 
    WHERE user_id = $1
        AND timestamp >= CURRENT_DATE - INTERVAL '30 days'
    ORDER BY timestamp DESC
"#;

/// Get audit logs by patient ID
pub const GET_AUDIT_LOGS_BY_PATIENT: &str = r#"
    SELECT 
        id, event_type, user_id, patient_id, appointment_id,
        resource_type, resource_id, action, outcome, timestamp,
        source_ip, user_agent, details
    FROM audit_logs
    WHERE patient_id = $1
    ORDER BY timestamp DESC
    LIMIT $2 OFFSET $3
"#;