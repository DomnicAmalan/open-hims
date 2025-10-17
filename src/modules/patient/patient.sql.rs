/// SQL queries for patient operations
/// This file contains all SQL queries used by the patient service

/// Insert a new patient into the database
pub const INSERT_PATIENT: &str = r#"
    INSERT INTO patients (
        id, active, name, telecom, gender, birth_date, 
        address, marital_status, contact, communication, meta
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
"#;

/// Get patient by ID
pub const GET_PATIENT_BY_ID: &str = r#"
    SELECT id, active, name, telecom, gender, birth_date,
           address, marital_status, contact, communication, 
           managing_organization, meta
    FROM patients 
    WHERE id = $1 AND active = true
"#;

/// Search patients with pagination
pub const SEARCH_PATIENTS: &str = r#"
    SELECT id, active, name, telecom, gender, birth_date,
           address, marital_status, contact, communication, 
           managing_organization, meta
    FROM patients 
    WHERE active = COALESCE($1, active)
    ORDER BY id
    LIMIT $2 OFFSET $3
"#;

/// Update patient information
pub const UPDATE_PATIENT: &str = r#"
    UPDATE patients 
    SET name = $2, telecom = $3, gender = $4, birth_date = $5,
        address = $6, marital_status = $7, contact = $8, 
        communication = $9, meta = jsonb_set(meta, '{lastUpdated}', $10)
    WHERE id = $1
"#;

/// Soft delete patient (set active = false)
pub const DELETE_PATIENT: &str = r#"
    UPDATE patients SET active = false WHERE id = $1 AND active = true
"#;

/// Insert audit log entry
pub const INSERT_AUDIT_LOG: &str = r#"
    INSERT INTO audit_logs (
        id, event_type, user_id, patient_id, resource_type, 
        resource_id, action, outcome, timestamp, details
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
"#;