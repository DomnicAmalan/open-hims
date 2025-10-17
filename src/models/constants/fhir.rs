/// FHIR R4 Base URL
pub const FHIR_BASE_URL: &str = "http://hl7.org/fhir";

/// FHIR Resource Types
pub const PATIENT_RESOURCE_TYPE: &str = "Patient";
pub const APPOINTMENT_RESOURCE_TYPE: &str = "Appointment";
pub const BUNDLE_RESOURCE_TYPE: &str = "Bundle";
pub const PRACTITIONER_RESOURCE_TYPE: &str = "Practitioner";
pub const ORGANIZATION_RESOURCE_TYPE: &str = "Organization";

/// FHIR Profile URLs
pub const PATIENT_PROFILE: &str = "http://hl7.org/fhir/StructureDefinition/Patient";
pub const APPOINTMENT_PROFILE: &str = "http://hl7.org/fhir/StructureDefinition/Appointment";
pub const BUNDLE_PROFILE: &str = "http://hl7.org/fhir/StructureDefinition/Bundle";

/// FHIR Code Systems
pub const GENDER_CODE_SYSTEM: &str = "http://hl7.org/fhir/administrative-gender";
pub const CONTACT_POINT_SYSTEM: &str = "http://hl7.org/fhir/contact-point-system";
pub const CONTACT_POINT_USE: &str = "http://hl7.org/fhir/contact-point-use";
pub const ADDRESS_USE: &str = "http://hl7.org/fhir/address-use";
pub const ADDRESS_TYPE: &str = "http://hl7.org/fhir/address-type";
pub const NAME_USE: &str = "http://hl7.org/fhir/name-use";

/// FHIR Value Sets
pub const APPOINTMENT_STATUS_VALUE_SET: &str = "http://hl7.org/fhir/ValueSet/appointmentstatus";
pub const PARTICIPATION_STATUS_VALUE_SET: &str = "http://hl7.org/fhir/ValueSet/participationstatus";
pub const PARTICIPANT_REQUIRED_VALUE_SET: &str = "http://hl7.org/fhir/ValueSet/participantrequired";

/// FHIR Extensions
pub const PATIENT_RELIGION_EXTENSION: &str = "http://hl7.org/fhir/StructureDefinition/patient-religion";
pub const PATIENT_BIRTHPLACE_EXTENSION: &str = "http://hl7.org/fhir/StructureDefinition/patient-birthPlace";
pub const PATIENT_NATIONALITY_EXTENSION: &str = "http://hl7.org/fhir/StructureDefinition/patient-nationality";