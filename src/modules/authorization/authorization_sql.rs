// src/modules/authorization/authorization.sql.rs
//! SQL queries for the authorization module
//! 
//! This file contains all SQL queries used by the authorization storage layer,
//! following the pattern of other modules in the system.

/// SQL queries for relationship management
pub mod relationships {
    /// Insert a new relationship tuple
    pub const INSERT_RELATIONSHIP: &str = r#"
        INSERT INTO authorization_relations (
            resource_type, resource_id, relation, subject_type, subject_id, 
            created_by, metadata, expires_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, created_at
    "#;

    /// Remove a relationship tuple
    pub const REMOVE_RELATIONSHIP: &str = r#"
        UPDATE authorization_relations 
        SET is_active = false 
        WHERE resource_type = $1 
        AND resource_id = $2 
        AND relation = $3 
        AND subject_type = $4 
        AND subject_id = $5 
        AND is_active = true
    "#;

    /// Check if a relationship exists
    pub const CHECK_RELATIONSHIP: &str = r#"
        SELECT EXISTS(
            SELECT 1 FROM authorization_relations 
            WHERE resource_type = $1 
            AND resource_id = $2 
            AND relation = $3 
            AND subject_type = $4 
            AND subject_id = $5 
            AND is_active = true
            AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)
        )
    "#;

    /// Find direct relationships for a resource
    pub const FIND_RELATIONSHIPS_FOR_RESOURCE: &str = r#"
        SELECT subject_type, subject_id, relation, created_at, metadata
        FROM authorization_relations 
        WHERE resource_type = $1 
        AND resource_id = $2 
        AND relation = $3
        AND is_active = true
        AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)
        ORDER BY created_at DESC
    "#;

    /// Find all relationships for a subject
    pub const FIND_RELATIONSHIPS_FOR_SUBJECT: &str = r#"
        SELECT resource_type, resource_id, relation, created_at, metadata, expires_at
        FROM authorization_relations 
        WHERE subject_type = $1 
        AND subject_id = $2 
        AND is_active = true
        AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)
        ORDER BY created_at DESC
    "#;

    /// Get all active relationships (for admin/debugging)
    pub const GET_ALL_ACTIVE_RELATIONSHIPS: &str = r#"
        SELECT id, resource_type, resource_id, relation, subject_type, subject_id, 
               created_at, created_by, metadata, expires_at
        FROM authorization_relations 
        WHERE is_active = true
        AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    "#;

    /// Clean up expired relationships
    pub const CLEANUP_EXPIRED_RELATIONSHIPS: &str = r#"
        UPDATE authorization_relations 
        SET is_active = false 
        WHERE expires_at IS NOT NULL 
        AND expires_at < CURRENT_TIMESTAMP 
        AND is_active = true
    "#;
}

/// SQL queries for policy management
pub mod policies {
    /// Store a new policy
    pub const INSERT_POLICY: &str = r#"
        INSERT INTO authorization_policies (
            name, description, resource_type, resource_id, action, 
            subject_type, subject_id, effect, conditions, priority, 
            created_by, updated_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id, created_at
    "#;

    /// Update an existing policy
    pub const UPDATE_POLICY: &str = r#"
        UPDATE authorization_policies 
        SET description = $2, resource_type = $3, resource_id = $4, 
            action = $5, subject_type = $6, subject_id = $7, 
            effect = $8, conditions = $9, priority = $10, 
            updated_by = $11, updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND is_active = true
    "#;

    /// Get a specific policy by ID
    pub const GET_POLICY_BY_ID: &str = r#"
        SELECT id, name, description, resource_type, resource_id, action, 
               subject_type, subject_id, effect, conditions, priority, 
               is_active, created_at, updated_at, created_by, updated_by
        FROM authorization_policies 
        WHERE id = $1
    "#;

    /// Get a policy by name
    pub const GET_POLICY_BY_NAME: &str = r#"
        SELECT id, name, description, resource_type, resource_id, action, 
               subject_type, subject_id, effect, conditions, priority, 
               is_active, created_at, updated_at, created_by, updated_by
        FROM authorization_policies 
        WHERE name = $1 AND is_active = true
    "#;

    /// Get all active policies
    pub const GET_ACTIVE_POLICIES: &str = r#"
        SELECT id, name, description, resource_type, resource_id, action, 
               subject_type, subject_id, effect, conditions, priority, 
               is_active, created_at, updated_at, created_by, updated_by
        FROM authorization_policies 
        WHERE is_active = true
        ORDER BY priority DESC, created_at ASC
    "#;

    /// Get applicable policies for a request
    pub const GET_APPLICABLE_POLICIES: &str = r#"
        SELECT id, name, description, resource_type, resource_id, action, 
               subject_type, subject_id, effect, conditions, priority, 
               is_active, created_at, updated_at, created_by, updated_by
        FROM authorization_policies 
        WHERE is_active = true
        AND (resource_type IS NULL OR resource_type = $1)
        AND (resource_id IS NULL OR resource_id = $2)
        AND (action IS NULL OR action = $3)
        AND (subject_type IS NULL OR subject_type = $4)
        AND (subject_id IS NULL OR subject_id = $5)
        ORDER BY priority DESC, created_at ASC
    "#;

    /// Soft delete a policy
    pub const DELETE_POLICY: &str = r#"
        UPDATE authorization_policies 
        SET is_active = false, updated_at = CURRENT_TIMESTAMP, updated_by = $2
        WHERE id = $1 AND is_active = true
    "#;

    /// Count active policies
    pub const COUNT_ACTIVE_POLICIES: &str = r#"
        SELECT COUNT(*) FROM authorization_policies WHERE is_active = true
    "#;
}

/// SQL queries for audit logging
pub mod audit {
    /// Store an audit entry
    pub const INSERT_AUDIT_ENTRY: &str = r#"
        INSERT INTO authorization_audit_log (
            user_id, action, resource_type, resource_id, decision, 
            confidence, reasons, requirements, restrictions, request_id, 
            ip_address, user_agent, session_id, location_data, 
            context_data, metadata, evaluation_time_ms
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING id, timestamp
    "#;

    /// Get audit entries for a user
    pub const GET_AUDIT_ENTRIES_FOR_USER: &str = r#"
        SELECT id, timestamp, user_id, action, resource_type, resource_id, 
               decision, confidence, reasons, requirements, restrictions, 
               request_id, ip_address, user_agent, session_id, location_data, 
               context_data, metadata, evaluation_time_ms
        FROM authorization_audit_log 
        WHERE user_id = $1 
        AND timestamp >= $2 
        AND timestamp <= $3
        ORDER BY timestamp DESC
        LIMIT $4 OFFSET $5
    "#;

    /// Get audit entries for a resource
    pub const GET_AUDIT_ENTRIES_FOR_RESOURCE: &str = r#"
        SELECT id, timestamp, user_id, action, resource_type, resource_id, 
               decision, confidence, reasons, requirements, restrictions, 
               request_id, ip_address, user_agent, session_id, location_data, 
               context_data, metadata, evaluation_time_ms
        FROM authorization_audit_log 
        WHERE resource_type = $1 
        AND resource_id = $2 
        AND timestamp >= $3 
        AND timestamp <= $4
        ORDER BY timestamp DESC
        LIMIT $5 OFFSET $6
    "#;

    /// Get recent failed access attempts
    pub const GET_RECENT_FAILED_ACCESS: &str = r#"
        SELECT id, timestamp, user_id, action, resource_type, resource_id, 
               decision, confidence, reasons, ip_address, user_agent, 
               session_id, context_data, metadata
        FROM authorization_audit_log 
        WHERE decision = 'deny' 
        AND timestamp >= $1
        ORDER BY timestamp DESC
        LIMIT $2
    "#;

    /// Get emergency access entries
    pub const GET_EMERGENCY_ACCESS_ENTRIES: &str = r#"
        SELECT id, timestamp, user_id, action, resource_type, resource_id, 
               decision, confidence, reasons, requirements, restrictions, 
               context_data, metadata
        FROM authorization_audit_log 
        WHERE decision IN ('emergency_access', 'break_glass_access')
        AND timestamp >= $1
        ORDER BY timestamp DESC
        LIMIT $2 OFFSET $3
    "#;

    /// Get suspicious activity patterns
    pub const GET_SUSPICIOUS_ACTIVITY: &str = r#"
        SELECT user_id, COUNT(*) as failed_attempts, 
               ARRAY_AGG(DISTINCT ip_address) as ip_addresses,
               MIN(timestamp) as first_attempt,
               MAX(timestamp) as last_attempt
        FROM authorization_audit_log 
        WHERE decision = 'deny' 
        AND timestamp >= $1
        GROUP BY user_id
        HAVING COUNT(*) >= $2
        ORDER BY failed_attempts DESC, last_attempt DESC
    "#;

    /// Get audit statistics
    pub const GET_AUDIT_STATISTICS: &str = r#"
        SELECT 
            decision,
            COUNT(*) as count,
            AVG(evaluation_time_ms) as avg_evaluation_time,
            MIN(timestamp) as earliest,
            MAX(timestamp) as latest
        FROM authorization_audit_log 
        WHERE timestamp >= $1 
        AND timestamp <= $2
        GROUP BY decision
        ORDER BY count DESC
    "#;

    /// Clean up old audit entries (for GDPR compliance)
    pub const CLEANUP_OLD_AUDIT_ENTRIES: &str = r#"
        DELETE FROM authorization_audit_log 
        WHERE timestamp < $1
    "#;
}

/// SQL queries for emergency access management
pub mod emergency {
    /// Create emergency access request
    pub const INSERT_EMERGENCY_ACCESS: &str = r#"
        INSERT INTO authorization_emergency_access (
            user_id, resource_type, resource_id, urgency_level, 
            justification, approval_required, requested_at, expires_at, metadata
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, requested_at
    "#;

    /// Approve emergency access
    pub const APPROVE_EMERGENCY_ACCESS: &str = r#"
        UPDATE authorization_emergency_access 
        SET approved_by = $2, approved_at = CURRENT_TIMESTAMP, access_granted = true
        WHERE id = $1 AND is_active = true
    "#;

    /// Get active emergency access for user
    pub const GET_ACTIVE_EMERGENCY_ACCESS: &str = r#"
        SELECT id, user_id, resource_type, resource_id, urgency_level, 
               justification, approval_required, approved_by, approved_at, 
               requested_at, expires_at, access_granted, metadata
        FROM authorization_emergency_access 
        WHERE user_id = $1 
        AND is_active = true 
        AND expires_at > CURRENT_TIMESTAMP
        ORDER BY requested_at DESC
    "#;

    /// Get pending emergency access requests
    pub const GET_PENDING_EMERGENCY_ACCESS: &str = r#"
        SELECT id, user_id, resource_type, resource_id, urgency_level, 
               justification, approval_required, requested_at, expires_at, metadata
        FROM authorization_emergency_access 
        WHERE approval_required = true 
        AND approved_by IS NULL 
        AND is_active = true 
        AND expires_at > CURRENT_TIMESTAMP
        ORDER BY urgency_level DESC, requested_at ASC
    "#;

    /// Expire emergency access
    pub const EXPIRE_EMERGENCY_ACCESS: &str = r#"
        UPDATE authorization_emergency_access 
        SET is_active = false 
        WHERE expires_at <= CURRENT_TIMESTAMP 
        AND is_active = true
    "#;
}

/// SQL queries for session management
pub mod sessions {
    /// Create or update session context
    pub const UPSERT_SESSION_CONTEXT: &str = r#"
        INSERT INTO authorization_session_context (
            session_id, user_id, department_id, location_id, shift_id,
            ip_address, user_agent, expires_at, mfa_verified, risk_score, context_data
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (session_id) 
        DO UPDATE SET 
            last_activity = CURRENT_TIMESTAMP,
            department_id = EXCLUDED.department_id,
            location_id = EXCLUDED.location_id,
            shift_id = EXCLUDED.shift_id,
            ip_address = EXCLUDED.ip_address,
            user_agent = EXCLUDED.user_agent,
            expires_at = EXCLUDED.expires_at,
            mfa_verified = EXCLUDED.mfa_verified,
            risk_score = EXCLUDED.risk_score,
            context_data = EXCLUDED.context_data
        RETURNING id, created_at, last_activity
    "#;

    /// Get session context
    pub const GET_SESSION_CONTEXT: &str = r#"
        SELECT id, session_id, user_id, department_id, location_id, shift_id,
               ip_address, user_agent, created_at, last_activity, expires_at,
               is_active, mfa_verified, risk_score, context_data
        FROM authorization_session_context 
        WHERE session_id = $1 
        AND is_active = true 
        AND expires_at > CURRENT_TIMESTAMP
    "#;

    /// Update session activity
    pub const UPDATE_SESSION_ACTIVITY: &str = r#"
        UPDATE authorization_session_context 
        SET last_activity = CURRENT_TIMESTAMP 
        WHERE session_id = $1 
        AND is_active = true
    "#;

    /// Invalidate session
    pub const INVALIDATE_SESSION: &str = r#"
        UPDATE authorization_session_context 
        SET is_active = false 
        WHERE session_id = $1
    "#;

    /// Clean up expired sessions
    pub const CLEANUP_EXPIRED_SESSIONS: &str = r#"
        UPDATE authorization_session_context 
        SET is_active = false 
        WHERE expires_at <= CURRENT_TIMESTAMP 
        AND is_active = true
    "#;

    /// Get user's active sessions
    pub const GET_USER_ACTIVE_SESSIONS: &str = r#"
        SELECT id, session_id, department_id, location_id, shift_id,
               ip_address, user_agent, created_at, last_activity, 
               expires_at, mfa_verified, risk_score
        FROM authorization_session_context 
        WHERE user_id = $1 
        AND is_active = true 
        AND expires_at > CURRENT_TIMESTAMP
        ORDER BY last_activity DESC
    "#;

    /// Get high-risk sessions
    pub const GET_HIGH_RISK_SESSIONS: &str = r#"
        SELECT id, session_id, user_id, department_id, location_id,
               ip_address, user_agent, created_at, last_activity, 
               risk_score, context_data
        FROM authorization_session_context 
        WHERE risk_score >= $1 
        AND is_active = true 
        AND expires_at > CURRENT_TIMESTAMP
        ORDER BY risk_score DESC, last_activity DESC
        LIMIT $2
    "#;
}

/// SQL queries for administrative operations
pub mod admin {
    /// Get authorization system statistics
    pub const GET_SYSTEM_STATISTICS: &str = r#"
        SELECT 
            'relationships' as entity_type,
            COUNT(*) as total_count,
            COUNT(*) FILTER (WHERE is_active = true) as active_count,
            COUNT(*) FILTER (WHERE expires_at IS NOT NULL AND expires_at > CURRENT_TIMESTAMP) as expiring_count
        FROM authorization_relations
        UNION ALL
        SELECT 
            'policies' as entity_type,
            COUNT(*) as total_count,
            COUNT(*) FILTER (WHERE is_active = true) as active_count,
            0 as expiring_count
        FROM authorization_policies
        UNION ALL
        SELECT 
            'audit_entries' as entity_type,
            COUNT(*) as total_count,
            COUNT(*) FILTER (WHERE timestamp >= CURRENT_DATE - INTERVAL '30 days') as active_count,
            COUNT(*) FILTER (WHERE timestamp >= CURRENT_DATE - INTERVAL '1 day') as expiring_count
        FROM authorization_audit_log
        UNION ALL
        SELECT 
            'emergency_access' as entity_type,
            COUNT(*) as total_count,
            COUNT(*) FILTER (WHERE is_active = true AND expires_at > CURRENT_TIMESTAMP) as active_count,
            COUNT(*) FILTER (WHERE expires_at <= CURRENT_TIMESTAMP + INTERVAL '1 hour') as expiring_count
        FROM authorization_emergency_access
        UNION ALL
        SELECT 
            'active_sessions' as entity_type,
            COUNT(*) as total_count,
            COUNT(*) FILTER (WHERE is_active = true AND expires_at > CURRENT_TIMESTAMP) as active_count,
            COUNT(*) FILTER (WHERE last_activity >= CURRENT_TIMESTAMP - INTERVAL '1 hour') as expiring_count
        FROM authorization_session_context
    "#;

    /// Health check query
    pub const HEALTH_CHECK: &str = r#"
        SELECT 
            'authorization_system' as component,
            CASE 
                WHEN COUNT(*) > 0 THEN 'healthy'
                ELSE 'degraded'
            END as status,
            COUNT(*) as table_count
        FROM information_schema.tables 
        WHERE table_name IN (
            'authorization_relations', 
            'authorization_policies', 
            'authorization_audit_log',
            'authorization_emergency_access',
            'authorization_session_context'
        )
    "#;

    /// Get recent authorization activity summary
    pub const GET_RECENT_ACTIVITY_SUMMARY: &str = r#"
        SELECT 
            DATE_TRUNC('hour', timestamp) as hour,
            decision,
            COUNT(*) as count
        FROM authorization_audit_log 
        WHERE timestamp >= $1
        GROUP BY DATE_TRUNC('hour', timestamp), decision
        ORDER BY hour DESC, count DESC
    "#;
}