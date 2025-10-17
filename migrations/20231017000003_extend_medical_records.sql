-- Extend medical_records table with additional FHIR fields
ALTER TABLE medical_records 
ADD COLUMN category JSONB,
ADD COLUMN code JSONB,
ADD COLUMN encounter JSONB,
ADD COLUMN effective_date_time TIMESTAMP WITH TIME ZONE,
ADD COLUMN issued TIMESTAMP WITH TIME ZONE,
ADD COLUMN performer JSONB,
ADD COLUMN value_quantity JSONB,
ADD COLUMN value_codeable_concept JSONB,
ADD COLUMN value_string TEXT,
ADD COLUMN value_boolean BOOLEAN,
ADD COLUMN value_integer INTEGER,
ADD COLUMN value_range JSONB,
ADD COLUMN value_ratio JSONB,
ADD COLUMN value_sampled_data JSONB,
ADD COLUMN value_time TIME,
ADD COLUMN value_date_time TIMESTAMP WITH TIME ZONE,
ADD COLUMN value_period JSONB,
ADD COLUMN data_absent_reason JSONB,
ADD COLUMN interpretation JSONB,
ADD COLUMN note JSONB,
ADD COLUMN body_site JSONB,
ADD COLUMN method JSONB,
ADD COLUMN specimen UUID,
ADD COLUMN device UUID,
ADD COLUMN reference_range JSONB,
ADD COLUMN has_member JSONB,
ADD COLUMN derived_from JSONB,
ADD COLUMN component JSONB,
ADD COLUMN deleted_at TIMESTAMP WITH TIME ZONE;

-- Add indexes for new fields
CREATE INDEX idx_medical_records_category ON medical_records USING GIN(category);
CREATE INDEX idx_medical_records_code ON medical_records USING GIN(code);
CREATE INDEX idx_medical_records_effective_date ON medical_records(effective_date_time);
CREATE INDEX idx_medical_records_issued ON medical_records(issued);
CREATE INDEX idx_medical_records_deleted ON medical_records(deleted_at);

-- Add audit logs appointment_id field that the service expects
ALTER TABLE audit_logs ADD COLUMN appointment_id UUID REFERENCES appointments(id);
CREATE INDEX idx_audit_logs_appointment ON audit_logs(appointment_id);