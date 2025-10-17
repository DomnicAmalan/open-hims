-- Create authorization tables migration
-- Migration: 2024_create_authorization_tables.sql

-- Create authorization_relations table for storing relationship tuples
CREATE TABLE authorization_relations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID NOT NULL,
    relation VARCHAR(100) NOT NULL,
    subject_type VARCHAR(100) NOT NULL,
    subject_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    metadata JSONB DEFAULT '{}',
    expires_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT true,
    
    -- Indexes for performance
    CONSTRAINT fk_authorization_relations_created_by 
        FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE RESTRICT
);

-- Unique constraint to prevent duplicate relationships
CREATE UNIQUE INDEX idx_authorization_relations_unique 
    ON authorization_relations (resource_type, resource_id, relation, subject_type, subject_id)
    WHERE is_active = true;

-- Indexes for common queries
CREATE INDEX idx_authorization_relations_resource 
    ON authorization_relations (resource_type, resource_id);
    
CREATE INDEX idx_authorization_relations_subject 
    ON authorization_relations (subject_type, subject_id);
    
CREATE INDEX idx_authorization_relations_relation 
    ON authorization_relations (relation);
    
CREATE INDEX idx_authorization_relations_created_at 
    ON authorization_relations (created_at);
    
CREATE INDEX idx_authorization_relations_expires_at 
    ON authorization_relations (expires_at) 
    WHERE expires_at IS NOT NULL;

-- Create authorization_policies table for storing access policies
CREATE TABLE authorization_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    resource_type VARCHAR(100),
    resource_id UUID,
    action VARCHAR(100),
    subject_type VARCHAR(100),
    subject_id UUID,
    effect VARCHAR(50) NOT NULL CHECK (effect IN ('allow', 'deny', 'require_approval', 'require_second_factor', 'audit_only', 'time_limit', 'restrict', 'conditional')),
    conditions JSONB DEFAULT '{}',
    priority INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_by UUID NOT NULL,
    
    CONSTRAINT fk_authorization_policies_created_by 
        FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE RESTRICT,
    CONSTRAINT fk_authorization_policies_updated_by 
        FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE RESTRICT
);

-- Indexes for policy queries
CREATE INDEX idx_authorization_policies_resource 
    ON authorization_policies (resource_type, resource_id);
    
CREATE INDEX idx_authorization_policies_action 
    ON authorization_policies (action);
    
CREATE INDEX idx_authorization_policies_subject 
    ON authorization_policies (subject_type, subject_id);
    
CREATE INDEX idx_authorization_policies_effect 
    ON authorization_policies (effect);
    
CREATE INDEX idx_authorization_policies_priority 
    ON authorization_policies (priority DESC);
    
CREATE INDEX idx_authorization_policies_active 
    ON authorization_policies (is_active) 
    WHERE is_active = true;

-- Create authorization_audit_log table for storing access decisions
CREATE TABLE authorization_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    user_id UUID,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID NOT NULL,
    decision VARCHAR(50) NOT NULL CHECK (decision IN ('allow', 'deny', 'emergency_access', 'require_approval', 'require_mfa', 'allow_with_restrictions')),
    confidence DECIMAL(3,2) CHECK (confidence >= 0.0 AND confidence <= 1.0),
    reasons TEXT[],
    requirements TEXT[],
    restrictions TEXT[],
    request_id VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    session_id VARCHAR(255),
    location_data JSONB,
    context_data JSONB,
    metadata JSONB DEFAULT '{}',
    evaluation_time_ms INTEGER,
    
    -- Foreign key constraints
    CONSTRAINT fk_authorization_audit_log_user_id 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Indexes for audit log queries
CREATE INDEX idx_authorization_audit_log_timestamp 
    ON authorization_audit_log (timestamp DESC);
    
CREATE INDEX idx_authorization_audit_log_user_id 
    ON authorization_audit_log (user_id);
    
CREATE INDEX idx_authorization_audit_log_action 
    ON authorization_audit_log (action);
    
CREATE INDEX idx_authorization_audit_log_resource 
    ON authorization_audit_log (resource_type, resource_id);
    
CREATE INDEX idx_authorization_audit_log_decision 
    ON authorization_audit_log (decision);
    
CREATE INDEX idx_authorization_audit_log_request_id 
    ON authorization_audit_log (request_id);
    
-- GIN index for JSONB columns
CREATE INDEX idx_authorization_audit_log_context_data 
    ON authorization_audit_log USING GIN (context_data);
    
CREATE INDEX idx_authorization_audit_log_metadata 
    ON authorization_audit_log USING GIN (metadata);

-- Create authorization_emergency_access table for tracking emergency access
CREATE TABLE authorization_emergency_access (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID NOT NULL,
    urgency_level VARCHAR(50) NOT NULL CHECK (urgency_level IN ('low', 'medium', 'high', 'critical')),
    justification TEXT NOT NULL,
    approval_required BOOLEAN DEFAULT false,
    approved_by UUID,
    approved_at TIMESTAMP WITH TIME ZONE,
    requested_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_active BOOLEAN DEFAULT true,
    access_granted BOOLEAN DEFAULT false,
    metadata JSONB DEFAULT '{}',
    
    -- Foreign key constraints
    CONSTRAINT fk_authorization_emergency_access_user_id 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_authorization_emergency_access_approved_by 
        FOREIGN KEY (approved_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Indexes for emergency access
CREATE INDEX idx_authorization_emergency_access_user_id 
    ON authorization_emergency_access (user_id);
    
CREATE INDEX idx_authorization_emergency_access_resource 
    ON authorization_emergency_access (resource_type, resource_id);
    
CREATE INDEX idx_authorization_emergency_access_expires_at 
    ON authorization_emergency_access (expires_at);
    
CREATE INDEX idx_authorization_emergency_access_active 
    ON authorization_emergency_access (is_active) 
    WHERE is_active = true;

-- Create authorization_session_context table for tracking user sessions
CREATE TABLE authorization_session_context (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id VARCHAR(255) NOT NULL UNIQUE,
    user_id UUID NOT NULL,
    department_id UUID,
    location_id UUID,
    shift_id UUID,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_active BOOLEAN DEFAULT true,
    mfa_verified BOOLEAN DEFAULT false,
    risk_score DECIMAL(3,2) DEFAULT 0.0 CHECK (risk_score >= 0.0 AND risk_score <= 1.0),
    context_data JSONB DEFAULT '{}',
    
    -- Foreign key constraints
    CONSTRAINT fk_authorization_session_context_user_id 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_authorization_session_context_department_id 
        FOREIGN KEY (department_id) REFERENCES departments(id) ON DELETE SET NULL,
    CONSTRAINT fk_authorization_session_context_location_id 
        FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
);

-- Indexes for session context
CREATE INDEX idx_authorization_session_context_session_id 
    ON authorization_session_context (session_id);
    
CREATE INDEX idx_authorization_session_context_user_id 
    ON authorization_session_context (user_id);
    
CREATE INDEX idx_authorization_session_context_expires_at 
    ON authorization_session_context (expires_at);
    
CREATE INDEX idx_authorization_session_context_active 
    ON authorization_session_context (is_active) 
    WHERE is_active = true;
    
CREATE INDEX idx_authorization_session_context_last_activity 
    ON authorization_session_context (last_activity);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger for authorization_policies table
CREATE TRIGGER update_authorization_policies_updated_at 
    BEFORE UPDATE ON authorization_policies 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Create function to clean up expired relations
CREATE OR REPLACE FUNCTION cleanup_expired_authorization_relations()
RETURNS void AS $$
BEGIN
    UPDATE authorization_relations 
    SET is_active = false 
    WHERE expires_at IS NOT NULL 
    AND expires_at < CURRENT_TIMESTAMP 
    AND is_active = true;
END;
$$ LANGUAGE plpgsql;

-- Create function to clean up expired emergency access
CREATE OR REPLACE FUNCTION cleanup_expired_emergency_access()
RETURNS void AS $$
BEGIN
    UPDATE authorization_emergency_access 
    SET is_active = false 
    WHERE expires_at < CURRENT_TIMESTAMP 
    AND is_active = true;
END;
$$ LANGUAGE plpgsql;

-- Create function to clean up expired sessions
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS void AS $$
BEGIN
    UPDATE authorization_session_context 
    SET is_active = false 
    WHERE expires_at < CURRENT_TIMESTAMP 
    AND is_active = true;
END;
$$ LANGUAGE plpgsql;

-- Create views for common queries

-- View for active relationships
CREATE VIEW active_authorization_relations AS
SELECT 
    id,
    resource_type,
    resource_id,
    relation,
    subject_type,
    subject_id,
    created_at,
    created_by,
    metadata,
    expires_at
FROM authorization_relations
WHERE is_active = true
AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP);

-- View for effective policies (active and ordered by priority)
CREATE VIEW effective_authorization_policies AS
SELECT 
    id,
    name,
    description,
    resource_type,
    resource_id,
    action,
    subject_type,
    subject_id,
    effect,
    conditions,
    priority,
    created_at,
    updated_at
FROM authorization_policies
WHERE is_active = true
ORDER BY priority DESC, created_at ASC;

-- View for recent audit entries
CREATE VIEW recent_authorization_audit AS
SELECT 
    id,
    timestamp,
    user_id,
    action,
    resource_type,
    resource_id,
    decision,
    confidence,
    reasons,
    requirements,
    restrictions,
    request_id,
    evaluation_time_ms
FROM authorization_audit_log
WHERE timestamp > CURRENT_TIMESTAMP - INTERVAL '30 days'
ORDER BY timestamp DESC;

-- View for active emergency access
CREATE VIEW active_emergency_access AS
SELECT 
    id,
    user_id,
    resource_type,
    resource_id,
    urgency_level,
    justification,
    approval_required,
    approved_by,
    approved_at,
    requested_at,
    expires_at,
    access_granted,
    metadata
FROM authorization_emergency_access
WHERE is_active = true
AND expires_at > CURRENT_TIMESTAMP;

-- View for active sessions
CREATE VIEW active_authorization_sessions AS
SELECT 
    id,
    session_id,
    user_id,
    department_id,
    location_id,
    shift_id,
    ip_address,
    created_at,
    last_activity,
    expires_at,
    mfa_verified,
    risk_score,
    context_data
FROM authorization_session_context
WHERE is_active = true
AND expires_at > CURRENT_TIMESTAMP;

-- Insert default policies for healthcare access

-- Policy: Primary physicians can read/write patient data
INSERT INTO authorization_policies (
    name,
    description,
    resource_type,
    action,
    subject_type,
    effect,
    conditions,
    priority,
    created_by,
    updated_by
) VALUES (
    'primary_physician_patient_access',
    'Primary physicians have full access to their patients',
    'patient',
    'read',
    'user',
    'allow',
    '{"requires_relationship": "primary_physician"}',
    100,
    '00000000-0000-0000-0000-000000000001', -- System user
    '00000000-0000-0000-0000-000000000001'
);

-- Policy: Nurses can read patient data during shift hours
INSERT INTO authorization_policies (
    name,
    description,
    resource_type,
    action,
    subject_type,
    effect,
    conditions,
    priority,
    created_by,
    updated_by
) VALUES (
    'nurse_patient_read_access',
    'Nurses can read patient data during their shift',
    'patient',
    'read',
    'user',
    'allow',
    '{"requires_relationship": "attending_nurse", "shift_hours": true}',
    80,
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000001'
);

-- Policy: Emergency access for critical situations
INSERT INTO authorization_policies (
    name,
    description,
    effect,
    conditions,
    priority,
    created_by,
    updated_by
) VALUES (
    'emergency_access_policy',
    'Allow emergency access for critical patient situations',
    'allow',
    '{"emergency_access": true, "urgency_level": ["high", "critical"], "requires_justification": true}',
    200,
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000001'
);

-- Policy: Audit all administrative access
INSERT INTO authorization_policies (
    name,
    description,
    resource_type,
    subject_type,
    effect,
    conditions,
    priority,
    created_by,
    updated_by
) VALUES (
    'admin_audit_policy',
    'Audit all administrative access to sensitive resources',
    'billing',
    'user',
    'audit_only',
    '{"requires_role": "admin", "audit_level": "detailed"}',
    150,
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000001'
);

-- Policy: Restrict access to patient financial data
INSERT INTO authorization_policies (
    name,
    description,
    resource_type,
    action,
    effect,
    conditions,
    priority,
    created_by,
    updated_by
) VALUES (
    'financial_data_restriction',
    'Restrict access to patient financial data to authorized personnel only',
    'billing',
    'read',
    'require_approval',
    '{"requires_role": ["billing_admin", "finance_manager"], "approval_required": true}',
    120,
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000001'
);

-- Add comments to tables
COMMENT ON TABLE authorization_relations IS 'Stores relationship tuples for Zanzibar-style authorization';
COMMENT ON TABLE authorization_policies IS 'Stores access policies with conditions and effects';
COMMENT ON TABLE authorization_audit_log IS 'Comprehensive audit log for all authorization decisions';
COMMENT ON TABLE authorization_emergency_access IS 'Tracks emergency access requests and approvals';
COMMENT ON TABLE authorization_session_context IS 'Stores session context for risk-based authorization';

COMMENT ON COLUMN authorization_relations.metadata IS 'Additional metadata about the relationship (JSON)';
COMMENT ON COLUMN authorization_policies.conditions IS 'Policy conditions in JSON format';
COMMENT ON COLUMN authorization_audit_log.context_data IS 'Request context data (JSON)';
COMMENT ON COLUMN authorization_audit_log.confidence IS 'Authorization confidence level (0.0 to 1.0)';
COMMENT ON COLUMN authorization_session_context.risk_score IS 'Session risk score (0.0 to 1.0)';