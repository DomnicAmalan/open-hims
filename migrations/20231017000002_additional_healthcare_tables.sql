-- Add additional healthcare-specific tables and optimizations

-- Create practitioners table for healthcare providers
CREATE TABLE practitioners (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identifier JSONB, -- Array of FHIR Identifier
    active BOOLEAN NOT NULL DEFAULT true,
    name JSONB NOT NULL, -- Array of FHIR HumanName
    telecom JSONB, -- Array of FHIR ContactPoint
    address JSONB, -- Array of FHIR Address
    gender VARCHAR(10),
    birth_date DATE,
    qualification JSONB, -- Array of PractitionerQualification
    communication JSONB, -- Array of FHIR CodeableConcept for languages
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_practitioner_gender CHECK (gender IN ('male', 'female', 'other', 'unknown'))
);

-- Create encounters table for healthcare visits
CREATE TABLE encounters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identifier JSONB, -- Array of FHIR Identifier
    status VARCHAR(20) NOT NULL,
    status_history JSONB, -- Array of EncounterStatusHistory
    class JSONB NOT NULL, -- FHIR Coding
    type JSONB, -- Array of FHIR CodeableConcept
    service_type JSONB, -- FHIR CodeableConcept
    priority JSONB, -- FHIR CodeableConcept
    subject UUID NOT NULL REFERENCES patients(id),
    participant JSONB, -- Array of EncounterParticipant
    period JSONB, -- FHIR Period (start/end times)
    length JSONB, -- FHIR Duration
    reason_code JSONB, -- Array of FHIR CodeableConcept
    diagnosis JSONB, -- Array of EncounterDiagnosis
    hospitalization JSONB, -- EncounterHospitalization
    location JSONB, -- Array of EncounterLocation
    service_provider UUID REFERENCES organizations(id),
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_encounter_status CHECK (status IN ('planned', 'arrived', 'triaged', 'in-progress', 'onleave', 'finished', 'cancelled', 'entered-in-error', 'unknown'))
);

-- Create observations table for clinical measurements
CREATE TABLE observations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identifier JSONB, -- Array of FHIR Identifier
    status VARCHAR(20) NOT NULL,
    category JSONB, -- Array of FHIR CodeableConcept
    code JSONB NOT NULL, -- FHIR CodeableConcept (what was observed)
    subject UUID NOT NULL REFERENCES patients(id),
    encounter UUID REFERENCES encounters(id),
    effective_datetime TIMESTAMP WITH TIME ZONE,
    effective_period JSONB, -- FHIR Period
    issued TIMESTAMP WITH TIME ZONE,
    performer JSONB, -- Array of FHIR Reference
    value_quantity JSONB, -- FHIR Quantity
    value_codeable_concept JSONB, -- FHIR CodeableConcept
    value_string TEXT,
    value_boolean BOOLEAN,
    value_integer INTEGER,
    value_range JSONB, -- FHIR Range
    value_ratio JSONB, -- FHIR Ratio
    value_sampled_data JSONB, -- FHIR SampledData
    value_time TIME,
    value_datetime TIMESTAMP WITH TIME ZONE,
    value_period JSONB, -- FHIR Period
    data_absent_reason JSONB, -- FHIR CodeableConcept
    interpretation JSONB, -- Array of FHIR CodeableConcept
    note JSONB, -- Array of FHIR Annotation
    body_site JSONB, -- FHIR CodeableConcept
    method JSONB, -- FHIR CodeableConcept
    device UUID, -- Reference to Device
    reference_range JSONB, -- Array of ObservationReferenceRange
    has_member JSONB, -- Array of FHIR Reference
    derived_from JSONB, -- Array of FHIR Reference
    component JSONB, -- Array of ObservationComponent
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_observation_status CHECK (status IN ('registered', 'preliminary', 'final', 'amended', 'corrected', 'cancelled', 'entered-in-error', 'unknown'))
);

-- Create medications table
CREATE TABLE medications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identifier JSONB, -- Array of FHIR Identifier
    code JSONB, -- FHIR CodeableConcept
    status VARCHAR(20),
    manufacturer UUID REFERENCES organizations(id),
    form JSONB, -- FHIR CodeableConcept
    amount JSONB, -- FHIR Ratio
    ingredient JSONB, -- Array of MedicationIngredient
    batch JSONB, -- MedicationBatch
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_medication_status CHECK (status IN ('active', 'inactive', 'entered-in-error'))
);

-- Create medication requests (prescriptions) table
CREATE TABLE medication_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identifier JSONB, -- Array of FHIR Identifier
    status VARCHAR(20) NOT NULL,
    status_reason JSONB, -- FHIR CodeableConcept
    intent VARCHAR(20) NOT NULL,
    category JSONB, -- Array of FHIR CodeableConcept
    priority VARCHAR(20),
    do_not_perform BOOLEAN,
    reported_boolean BOOLEAN,
    reported_reference UUID, -- Reference to Patient/Practitioner
    medication_codeable_concept JSONB, -- FHIR CodeableConcept
    medication_reference UUID REFERENCES medications(id),
    subject UUID NOT NULL REFERENCES patients(id),
    encounter UUID REFERENCES encounters(id),
    authored_on TIMESTAMP WITH TIME ZONE,
    requester UUID, -- Reference to Practitioner/Organization
    performer UUID, -- Reference to Practitioner/Organization
    performer_type JSONB, -- FHIR CodeableConcept
    recorder UUID, -- Reference to Practitioner
    reason_code JSONB, -- Array of FHIR CodeableConcept
    reason_reference JSONB, -- Array of FHIR Reference
    instantiates_canonical TEXT,
    instantiates_uri TEXT,
    based_on JSONB, -- Array of FHIR Reference
    group_identifier JSONB, -- FHIR Identifier
    course_of_therapy_type JSONB, -- FHIR CodeableConcept
    insurance JSONB, -- Array of FHIR Reference
    note JSONB, -- Array of FHIR Annotation
    dosage_instruction JSONB, -- Array of FHIR Dosage
    dispense_request JSONB, -- MedicationRequestDispenseRequest
    substitution JSONB, -- MedicationRequestSubstitution
    prior_prescription UUID, -- Reference to MedicationRequest
    detection_issue JSONB, -- Array of FHIR Reference
    event_history JSONB, -- Array of FHIR Reference
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_medication_request_status CHECK (status IN ('active', 'on-hold', 'cancelled', 'completed', 'entered-in-error', 'stopped', 'draft', 'unknown')),
    CONSTRAINT valid_medication_request_intent CHECK (intent IN ('proposal', 'plan', 'order', 'original-order', 'reflex-order', 'filler-order', 'instance-order', 'option')),
    CONSTRAINT valid_medication_request_priority CHECK (priority IN ('routine', 'urgent', 'asap', 'stat'))
);

-- Create diagnostic reports table
CREATE TABLE diagnostic_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identifier JSONB, -- Array of FHIR Identifier
    based_on JSONB, -- Array of FHIR Reference
    status VARCHAR(20) NOT NULL,
    category JSONB, -- Array of FHIR CodeableConcept
    code JSONB NOT NULL, -- FHIR CodeableConcept
    subject UUID NOT NULL REFERENCES patients(id),
    encounter UUID REFERENCES encounters(id),
    effective_datetime TIMESTAMP WITH TIME ZONE,
    effective_period JSONB, -- FHIR Period
    issued TIMESTAMP WITH TIME ZONE,
    performer JSONB, -- Array of FHIR Reference
    results_interpreter JSONB, -- Array of FHIR Reference
    specimen JSONB, -- Array of FHIR Reference
    result JSONB, -- Array of FHIR Reference to Observations
    imaging_study JSONB, -- Array of FHIR Reference
    media JSONB, -- Array of DiagnosticReportMedia
    conclusion TEXT,
    conclusion_code JSONB, -- Array of FHIR CodeableConcept
    presented_form JSONB, -- Array of FHIR Attachment
    meta JSONB NOT NULL, -- FHIR ResourceMeta
    
    CONSTRAINT valid_diagnostic_report_status CHECK (status IN ('registered', 'partial', 'preliminary', 'final', 'amended', 'corrected', 'appended', 'cancelled', 'entered-in-error', 'unknown'))
);

-- Add foreign key for medical_records to reference encounters
ALTER TABLE medical_records 
ADD CONSTRAINT fk_medical_records_encounter 
FOREIGN KEY (encounter_id) REFERENCES encounters(id);

-- Add indexes for new tables

-- Practitioners indexes
CREATE INDEX idx_practitioners_active ON practitioners(active);
CREATE INDEX idx_practitioners_name_gin ON practitioners USING GIN(name);
CREATE INDEX idx_practitioners_identifier_gin ON practitioners USING GIN(identifier);

-- Encounters indexes
CREATE INDEX idx_encounters_patient ON encounters(subject);
CREATE INDEX idx_encounters_status ON encounters(status);
CREATE INDEX idx_encounters_service_provider ON encounters(service_provider);
CREATE INDEX idx_encounters_period_gin ON encounters USING GIN(period);

-- Observations indexes
CREATE INDEX idx_observations_patient ON observations(subject);
CREATE INDEX idx_observations_encounter ON observations(encounter);
CREATE INDEX idx_observations_status ON observations(status);
CREATE INDEX idx_observations_effective_datetime ON observations(effective_datetime);
CREATE INDEX idx_observations_category_gin ON observations USING GIN(category);
CREATE INDEX idx_observations_code_gin ON observations USING GIN(code);

-- Medications indexes
CREATE INDEX idx_medications_status ON medications(status);
CREATE INDEX idx_medications_manufacturer ON medications(manufacturer);
CREATE INDEX idx_medications_code_gin ON medications USING GIN(code);

-- Medication requests indexes
CREATE INDEX idx_medication_requests_patient ON medication_requests(subject);
CREATE INDEX idx_medication_requests_encounter ON medication_requests(encounter);
CREATE INDEX idx_medication_requests_status ON medication_requests(status);
CREATE INDEX idx_medication_requests_authored_on ON medication_requests(authored_on);
CREATE INDEX idx_medication_requests_medication ON medication_requests(medication_reference);

-- Diagnostic reports indexes
CREATE INDEX idx_diagnostic_reports_patient ON diagnostic_reports(subject);
CREATE INDEX idx_diagnostic_reports_encounter ON diagnostic_reports(encounter);
CREATE INDEX idx_diagnostic_reports_status ON diagnostic_reports(status);
CREATE INDEX idx_diagnostic_reports_issued ON diagnostic_reports(issued);
CREATE INDEX idx_diagnostic_reports_effective_datetime ON diagnostic_reports(effective_datetime);