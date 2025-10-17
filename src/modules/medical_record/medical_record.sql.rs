/// Medical Record SQL Queries
/// 
/// This file contains all SQL queries used by the medical record service
/// for clean separation of concerns and better maintainability.

/// Insert a new medical record
pub const INSERT_MEDICAL_RECORD: &str = r#"
    INSERT INTO medical_records (
        id, patient_id, encounter_id, record_type, status, subject,
        author, content, created_at, updated_at, meta
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
    )
"#;

/// Get medical record by ID
pub const GET_MEDICAL_RECORD_BY_ID: &str = r#"
    SELECT id, patient_id, encounter_id, record_type, status, subject,
           author, content, created_at, updated_at, meta
    FROM medical_records 
    WHERE id = $1 AND deleted_at IS NULL
"#;

/// Get medical records by patient ID
pub const GET_MEDICAL_RECORDS_BY_PATIENT: &str = r#"
    SELECT id, patient_id, encounter_id, record_type, status, subject,
           author, content, created_at, updated_at, meta
    FROM medical_records 
    WHERE patient_id = $1 AND deleted_at IS NULL
    ORDER BY created_at DESC
"#;

/// Soft delete medical record
pub const SOFT_DELETE_MEDICAL_RECORD: &str = r#"
    UPDATE medical_records 
    SET deleted_at = $1 
    WHERE id = $2 AND deleted_at IS NULL
"#;

/// Update medical record content
pub const UPDATE_MEDICAL_RECORD: &str = r#"
    UPDATE medical_records 
    SET content = $1, updated_at = $2, meta = $3
    WHERE id = $4 AND deleted_at IS NULL
"#;

/// Search medical records with filters
pub const SEARCH_MEDICAL_RECORDS: &str = r#"
    SELECT id, patient_id, encounter_id, record_type, status, subject,
           author, content, created_at, updated_at, meta
    FROM medical_records 
    WHERE deleted_at IS NULL
        AND ($1::uuid IS NULL OR patient_id = $1)
        AND ($2::text IS NULL OR record_type ILIKE '%' || $2 || '%')
        AND ($3::text IS NULL OR status ILIKE '%' || $3 || '%')
    ORDER BY created_at DESC
    LIMIT $4 OFFSET $5
"#;