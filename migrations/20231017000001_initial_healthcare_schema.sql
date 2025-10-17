-- Create extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create organizations table for healthcare facilities
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    identifier VARCHAR(100) UNIQUE,
    type VARCHAR(50) NOT NULL, -- hospital, clinic, pharmacy, etc.
    address JSONB,
    contact JSONB,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create users table for authentication
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL, -- admin, doctor, nurse, receptionist, patient, technician
    active BOOLEAN NOT NULL DEFAULT true,
    name JSONB NOT NULL, -- FHIR HumanName structure
    practitioner_id UUID,
    organization_id UUID REFERENCES organizations(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_login TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT valid_role CHECK (role IN ('admin', 'doctor', 'nurse', 'receptionist', 'patient', 'technician'))
);

-- Create patients table with FHIR R4 compliance
CREATE TABLE patients (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    active BOOLEAN NOT NULL DEFAULT true,
    name JSONB NOT NULL, -- Array of FHIR HumanName
    telecom JSONB, -- Array of FHIR ContactPoint
    gender VARCHAR(10) NOT NULL,
    birth_date DATE,
    deceased BOOLEAN,
    address JSONB, -- Array of FHIR Address
    marital_status JSONB, -- FHIR CodeableConcept
    contact JSONB, -- Array of PatientContact
    communication JSONB, -- Array of PatientCommunication
    managing_organization UUID REFERENCES organizations(id),
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_gender CHECK (gender IN ('male', 'female', 'other', 'unknown'))
);

-- Create appointments table with FHIR R4 compliance
CREATE TABLE appointments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    status VARCHAR(20) NOT NULL,
    service_category JSONB, -- Array of CodeableConcept
    service_type JSONB, -- Array of CodeableConcept
    specialty JSONB, -- Array of CodeableConcept
    appointment_type JSONB, -- CodeableConcept
    reason_code JSONB, -- Array of CodeableConcept
    priority INTEGER,
    description TEXT,
    start_time TIMESTAMP WITH TIME ZONE,
    end_time TIMESTAMP WITH TIME ZONE,
    minutes_duration INTEGER,
    participant JSONB NOT NULL, -- Array of AppointmentParticipant
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_status CHECK (status IN ('proposed', 'pending', 'booked', 'arrived', 'fulfilled', 'cancelled', 'noshow', 'entered-in-error', 'checked-in', 'waitlist')),
    CONSTRAINT valid_duration CHECK (minutes_duration > 0 AND minutes_duration <= 480)
);

-- Create medical records table for clinical documentation
CREATE TABLE medical_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    patient_id UUID NOT NULL REFERENCES patients(id),
    encounter_id UUID,
    record_type VARCHAR(30) NOT NULL,
    status VARCHAR(20) NOT NULL,
    subject JSONB NOT NULL, -- FHIR Reference
    author JSONB NOT NULL, -- Array of FHIR Reference
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_record_type CHECK (record_type IN ('progress-note', 'discharge-summary', 'operative-note', 'consultation', 'diagnostic-report')),
    CONSTRAINT valid_status CHECK (status IN ('preliminary', 'final', 'amended', 'entered-in-error'))
);

-- Create audit logs table for healthcare compliance
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_type VARCHAR(30) NOT NULL,
    user_id UUID REFERENCES users(id),
    patient_id UUID REFERENCES patients(id),
    resource_type VARCHAR(50) NOT NULL,
    resource_id UUID,
    action VARCHAR(20) NOT NULL,
    outcome VARCHAR(20) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    source_ip INET,
    user_agent TEXT,
    details TEXT,
    
    CONSTRAINT valid_event_type CHECK (event_type IN ('patient-access', 'data-modification', 'authentication', 'export', 'system-access')),
    CONSTRAINT valid_action CHECK (action IN ('create', 'read', 'update', 'delete', 'execute')),
    CONSTRAINT valid_outcome CHECK (outcome IN ('success', 'minor-failure', 'serious-failure', 'major-failure'))
);

-- Create indexes for performance optimization

-- Users indexes
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_organization ON users(organization_id);
CREATE INDEX idx_users_active ON users(active);

-- Patients indexes
CREATE INDEX idx_patients_active ON patients(active);
CREATE INDEX idx_patients_gender ON patients(gender);
CREATE INDEX idx_patients_birth_date ON patients(birth_date);
CREATE INDEX idx_patients_organization ON patients(managing_organization);
CREATE INDEX idx_patients_name_gin ON patients USING GIN(name); -- For JSONB searches

-- Appointments indexes
CREATE INDEX idx_appointments_status ON appointments(status);
CREATE INDEX idx_appointments_start_time ON appointments(start_time);
CREATE INDEX idx_appointments_end_time ON appointments(end_time);
CREATE INDEX idx_appointments_participant_gin ON appointments USING GIN(participant);

-- Medical records indexes
CREATE INDEX idx_medical_records_patient ON medical_records(patient_id);
CREATE INDEX idx_medical_records_type ON medical_records(record_type);
CREATE INDEX idx_medical_records_status ON medical_records(status);
CREATE INDEX idx_medical_records_created ON medical_records(created_at);
CREATE INDEX idx_medical_records_encounter ON medical_records(encounter_id);

-- Audit logs indexes
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp);
CREATE INDEX idx_audit_logs_user ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_patient ON audit_logs(patient_id);
CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at columns
CREATE TRIGGER update_organizations_updated_at BEFORE UPDATE ON organizations FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_medical_records_updated_at BEFORE UPDATE ON medical_records FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();