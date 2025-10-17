// SQL queries for appointment operations

pub const CREATE_APPOINTMENT: &str = r#"
    INSERT INTO appointments (
        id, patient_id, practitioner_id, start_time, end_time, 
        status, service_type, comment, created_at, updated_at
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
"#;

pub const GET_APPOINTMENT_BY_ID: &str = r#"
    SELECT id, patient_id, practitioner_id, start_time, end_time, 
           status, service_type, comment, created_at, updated_at
    FROM appointments 
    WHERE id = $1 AND deleted_at IS NULL
"#;

pub const SEARCH_APPOINTMENTS: &str = r#"
    SELECT id, patient_id, practitioner_id, start_time, end_time, 
           status, service_type, comment, created_at, updated_at
    FROM appointments 
    WHERE deleted_at IS NULL
    ORDER BY start_time DESC
    LIMIT $1 OFFSET $2
"#;

pub const UPDATE_APPOINTMENT_STATUS: &str = r#"
    UPDATE appointments 
    SET status = $1, updated_at = NOW() 
    WHERE id = $2
"#;

pub const UPDATE_APPOINTMENT: &str = r#"
    UPDATE appointments 
    SET start_time = $1, end_time = $2, status = $3, patient_id = $4,
        practitioner_id = $5, service_type = $6, comment = $7, updated_at = NOW()
    WHERE id = $8
"#;

pub const DELETE_APPOINTMENT: &str = r#"
    UPDATE appointments 
    SET deleted_at = NOW(), updated_at = NOW() 
    WHERE id = $1
"#;

pub const CREATE_AUDIT_LOG: &str = r#"
    INSERT INTO audit_logs (
        id, event_type, user_id, patient_id, appointment_id,
        resource_type, resource_id, action, outcome, timestamp,
        source_ip, user_agent, details
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
"#;