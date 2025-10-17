import type { 
  FhirPatient, 
  FhirObservation, 
  FhirIdentifier, 
  FhirHumanName,
  Hl7Message,
  Hl7PatientInfo,
  ComplianceStatus,
  HealthcareRegulation 
} from '@open-hims/types';

// FHIR utility functions
export function extractPatientName(patient: FhirPatient): string {
  if (!patient.name || patient.name.length === 0) {
    return 'Unknown Patient';
  }
  
  const name = patient.name[0];
  const given = name.given?.join(' ') || '';
  const family = name.family || '';
  
  return `${given} ${family}`.trim() || 'Unknown Patient';
}

export function extractPatientIdentifier(patient: FhirPatient, system?: string): string | undefined {
  if (!patient.identifier || patient.identifier.length === 0) {
    return undefined;
  }
  
  if (system) {
    const identifier = patient.identifier.find(id => id.system === system);
    return identifier?.value;
  }
  
  return patient.identifier[0]?.value;
}

export function getPatientAge(patient: FhirPatient): number | null {
  if (!patient.birthDate) return null;
  
  const birthDate = new Date(patient.birthDate);
  const today = new Date();
  let age = today.getFullYear() - birthDate.getFullYear();
  const monthDiff = today.getMonth() - birthDate.getMonth();
  
  if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
    age--;
  }
  
  return age;
}

export function formatFhirDate(dateString: string): string {
  try {
    const date = new Date(dateString);
    return date.toLocaleDateString();
  } catch {
    return dateString;
  }
}

export function createFhirIdentifier(system: string, value: string): FhirIdentifier {
  return {
    system,
    value,
  };
}

export function createFhirHumanName(family: string, given: string[]): FhirHumanName {
  return {
    family,
    given,
    use: 'official',
  };
}

// HL7 utility functions
export function parseHl7MessageType(message: Hl7Message): string {
  return `${message.messageType}^${message.segments.find(s => s.segmentType === 'MSH')?.fields[8] || 'Unknown'}`;
}

export function extractHl7PatientId(patientInfo: Hl7PatientInfo): string {
  return patientInfo.patientId;
}

export function formatHl7PatientName(patientInfo: Hl7PatientInfo): string {
  const { family, given, middle } = patientInfo.name;
  return [given.join(' '), middle, family].filter(Boolean).join(' ');
}

export function convertHl7ToFhirGender(hl7Gender: string): 'male' | 'female' | 'other' | 'unknown' {
  switch (hl7Gender.toUpperCase()) {
    case 'M': return 'male';
    case 'F': return 'female';
    case 'O': return 'other';
    default: return 'unknown';
  }
}

// Medical Record Number utilities
export function generateMRN(prefix: string = 'MRN', institutionCode?: string): string {
  const timestamp = Date.now().toString().slice(-6);
  const random = Math.floor(Math.random() * 1000).toString().padStart(3, '0');
  const institution = institutionCode ? `${institutionCode}-` : '';
  
  return `${prefix}${institution}${timestamp}${random}`;
}

export function validateMRN(mrn: string, pattern?: RegExp): boolean {
  if (pattern) {
    return pattern.test(mrn);
  }
  
  // Default MRN validation - alphanumeric, 6-20 characters
  const defaultPattern = /^[A-Za-z0-9]{6,20}$/;
  return defaultPattern.test(mrn);
}

// Healthcare data validation
export function validatePatientData(patient: Partial<FhirPatient>): { isValid: boolean; errors: string[] } {
  const errors: string[] = [];
  
  if (!patient.name || patient.name.length === 0) {
    errors.push('Patient name is required');
  }
  
  if (!patient.birthDate) {
    errors.push('Patient birth date is required');
  } else {
    const birthDate = new Date(patient.birthDate);
    const today = new Date();
    if (birthDate > today) {
      errors.push('Birth date cannot be in the future');
    }
  }
  
  if (patient.gender && !['male', 'female', 'other', 'unknown'].includes(patient.gender)) {
    errors.push('Invalid gender value');
  }
  
  if (patient.identifier && patient.identifier.length > 0) {
    for (const identifier of patient.identifier) {
      if (!identifier.value) {
        errors.push('Identifier value is required');
      }
    }
  }
  
  return {
    isValid: errors.length === 0,
    errors,
  };
}

// Compliance utilities
export function calculateComplianceScore(regulations: HealthcareRegulation[]): number {
  if (regulations.length === 0) return 0;
  
  const totalRequirements = regulations.reduce((total, reg) => total + reg.requirements.length, 0);
  const compliantRequirements = regulations.reduce(
    (total, reg) => total + reg.requirements.filter(req => req.status === 'compliant').length,
    0
  );
  
  return totalRequirements > 0 ? Math.round((compliantRequirements / totalRequirements) * 100) : 0;
}

export function getComplianceStatusColor(status: ComplianceStatus): string {
  const colors = {
    compliant: '#10B981', // green
    warning: '#F59E0B', // yellow
    'non-compliant': '#EF4444', // red
    pending: '#6B7280', // gray
    unknown: '#9CA3AF', // light gray
  };
  
  return colors[status] || colors.unknown;
}

export function getComplianceStatusLabel(status: ComplianceStatus): string {
  const labels = {
    compliant: 'Compliant',
    warning: 'Needs Attention',
    'non-compliant': 'Non-Compliant',
    pending: 'Under Review',
    unknown: 'Unknown',
  };
  
  return labels[status] || labels.unknown;
}

// Clinical data utilities
export function categorizeVitalSigns(observation: FhirObservation): 'normal' | 'abnormal' | 'critical' | 'unknown' {
  if (!observation.code?.coding?.[0]?.code || !observation.valueQuantity?.value) {
    return 'unknown';
  }
  
  const code = observation.code.coding[0].code;
  const value = observation.valueQuantity.value;
  
  // Basic vital signs categorization (simplified)
  switch (code) {
    case '8480-6': // Systolic BP
      if (value < 90 || value > 180) return 'critical';
      if (value < 120) return 'normal';
      return 'abnormal';
      
    case '8462-4': // Diastolic BP
      if (value < 60 || value > 120) return 'critical';
      if (value < 80) return 'normal';
      return 'abnormal';
      
    case '8867-4': // Heart rate
      if (value < 40 || value > 120) return 'critical';
      if (value >= 60 && value <= 100) return 'normal';
      return 'abnormal';
      
    case '8310-5': // Body temperature
      if (value < 35 || value > 40) return 'critical';
      if (value >= 36.1 && value <= 37.2) return 'normal';
      return 'abnormal';
      
    default:
      return 'unknown';
  }
}

export function formatObservationValue(observation: FhirObservation): string {
  if (observation.valueQuantity) {
    const value = observation.valueQuantity.value;
    const unit = observation.valueQuantity.unit || '';
    return `${value} ${unit}`.trim();
  }
  
  if (observation.valueString) {
    return observation.valueString;
  }
  
  if (observation.valueCodeableConcept) {
    return observation.valueCodeableConcept.text || 
           observation.valueCodeableConcept.coding?.[0]?.display || 
           'Unknown';
  }
  
  if (observation.valueBoolean !== undefined) {
    return observation.valueBoolean ? 'Yes' : 'No';
  }
  
  return 'No value';
}

// Country/State healthcare regulation utilities
export function getRegulationsByCountry(regulations: HealthcareRegulation[], country: string): HealthcareRegulation[] {
  return regulations.filter(reg => reg.country === country);
}

export function getRegulationsByState(
  regulations: HealthcareRegulation[], 
  country: string, 
  state: string
): HealthcareRegulation[] {
  return regulations.filter(reg => 
    reg.country === country && (!reg.state || reg.state === state)
  );
}

export function getMandatoryRegulations(regulations: HealthcareRegulation[]): HealthcareRegulation[] {
  return regulations.filter(reg => reg.mandatory);
}

export function getRegulationsByCategory(
  regulations: HealthcareRegulation[], 
  category: string
): HealthcareRegulation[] {
  return regulations.filter(reg => reg.category === category);
}

// Privacy and security utilities
export function maskSensitiveData(data: string, type: 'ssn' | 'phone' | 'email' | 'custom', visibleChars?: number): string {
  if (!data) return '';
  
  switch (type) {
    case 'ssn':
      return data.replace(/\d(?=\d{4})/g, '*');
      
    case 'phone':
      return data.replace(/\d(?=\d{4})/g, '*');
      
    case 'email':
      const [local, domain] = data.split('@');
      if (!domain) return data;
      const maskedLocal = local.charAt(0) + '*'.repeat(Math.max(0, local.length - 2)) + local.slice(-1);
      return `${maskedLocal}@${domain}`;
      
    case 'custom':
      const visible = visibleChars || 4;
      if (data.length <= visible) return data;
      return data.slice(0, Math.ceil(visible / 2)) + '*'.repeat(data.length - visible) + data.slice(-Math.floor(visible / 2));
      
    default:
      return data;
  }
}

export function sanitizeForDisplay(text: string): string {
  return text
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#x27;')
    .replace(/\//g, '&#x2F;');
}