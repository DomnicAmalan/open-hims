import { createSlice, PayloadAction } from '@reduxjs/toolkit';

// Compliance status types
export type ComplianceStatus = 'compliant' | 'non-compliant' | 'warning' | 'pending' | 'unknown';

// Regulation interfaces
export interface HealthcareRegulation {
  id: string;
  name: string;
  description: string;
  country: string;
  state?: string;
  category: 'privacy' | 'security' | 'data-protection' | 'medical-device' | 'clinical' | 'administrative';
  mandatory: boolean;
  effectiveDate: string;
  lastUpdated: string;
  requirements: RegulationRequirement[];
}

export interface RegulationRequirement {
  id: string;
  title: string;
  description: string;
  status: ComplianceStatus;
  evidence?: string[];
  dueDate?: string;
  lastAssessment?: string;
  assessor?: string;
  notes?: string;
}

// Compliance audit interfaces
export interface ComplianceAudit {
  id: string;
  regulationId: string;
  auditDate: string;
  auditor: string;
  status: ComplianceStatus;
  score: number; // 0-100
  findings: AuditFinding[];
  recommendations: string[];
  nextAuditDate: string;
  reportUrl?: string;
}

export interface AuditFinding {
  id: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  category: string;
  description: string;
  remediation: string;
  dueDate: string;
  status: 'open' | 'in-progress' | 'resolved' | 'deferred';
  assignee?: string;
}

// Country/State selection
export interface CountryState {
  country: string;
  countryName: string;
  state?: string;
  stateName?: string;
}

// Compliance state interface
export interface ComplianceState {
  selectedLocation: CountryState;
  availableLocations: CountryState[];
  regulations: HealthcareRegulation[];
  selectedRegulation: HealthcareRegulation | null;
  audits: ComplianceAudit[];
  overallComplianceScore: number;
  loading: {
    fetchRegulations: boolean;
    fetchAudits: boolean;
    updateCompliance: boolean;
    generateReport: boolean;
  };
  error: string | null;
  filters: {
    category: string;
    status: ComplianceStatus | '';
    mandatory: boolean | null;
  };
}

// Mock data
const mockLocations: CountryState[] = [
  { country: 'US', countryName: 'United States', state: 'CA', stateName: 'California' },
  { country: 'US', countryName: 'United States', state: 'NV', stateName: 'Nevada' },
  { country: 'US', countryName: 'United States', state: 'TX', stateName: 'Texas' },
  { country: 'IN', countryName: 'India', state: 'MH', stateName: 'Maharashtra' },
  { country: 'IN', countryName: 'India', state: 'KA', stateName: 'Karnataka' },
  { country: 'GB', countryName: 'United Kingdom' },
  { country: 'DE', countryName: 'Germany' },
];

const mockRegulations: HealthcareRegulation[] = [
  {
    id: 'hipaa-us',
    name: 'HIPAA (Health Insurance Portability and Accountability Act)',
    description: 'Federal law protecting sensitive patient health information',
    country: 'US',
    category: 'privacy',
    mandatory: true,
    effectiveDate: '1996-08-21',
    lastUpdated: '2024-01-15',
    requirements: [
      {
        id: 'hipaa-privacy-rule',
        title: 'Privacy Rule Compliance',
        description: 'Protect patient health information privacy',
        status: 'compliant',
        evidence: ['Privacy policy documentation', 'Staff training records'],
        lastAssessment: '2024-09-15',
        assessor: 'John Smith, Privacy Officer'
      },
      {
        id: 'hipaa-security-rule',
        title: 'Security Rule Compliance',
        description: 'Secure electronic protected health information',
        status: 'warning',
        evidence: ['Encryption documentation', 'Access control logs'],
        lastAssessment: '2024-09-15',
        assessor: 'Jane Doe, CISO',
        notes: 'Need to update encryption protocols'
      }
    ]
  },
  {
    id: 'gdpr-eu',
    name: 'GDPR (General Data Protection Regulation)',
    description: 'EU regulation on data protection and privacy',
    country: 'EU',
    category: 'data-protection',
    mandatory: true,
    effectiveDate: '2018-05-25',
    lastUpdated: '2024-01-10',
    requirements: [
      {
        id: 'gdpr-consent',
        title: 'Data Subject Consent',
        description: 'Obtain explicit consent for data processing',
        status: 'compliant',
        evidence: ['Consent management system', 'Audit logs'],
        lastAssessment: '2024-08-20',
        assessor: 'Maria Garcia, DPO'
      },
      {
        id: 'gdpr-breach-notification',
        title: 'Breach Notification',
        description: '72-hour breach notification requirement',
        status: 'pending',
        evidence: [],
        dueDate: '2024-11-01',
        notes: 'Implementing automated breach detection'
      }
    ]
  },
  {
    id: 'abdm-in',
    name: 'ABDM (Ayushman Bharat Digital Mission)',
    description: 'Digital health mission for interoperability in India',
    country: 'IN',
    category: 'clinical',
    mandatory: true,
    effectiveDate: '2021-09-27',
    lastUpdated: '2024-02-01',
    requirements: [
      {
        id: 'abdm-health-id',
        title: 'Health ID Integration',
        description: 'Support for Ayushman Bharat Health Account (ABHA)',
        status: 'non-compliant',
        evidence: [],
        dueDate: '2024-12-31',
        notes: 'Integration in progress'
      },
      {
        id: 'abdm-fhir',
        title: 'FHIR Compliance',
        description: 'Support FHIR R4 for health data exchange',
        status: 'compliant',
        evidence: ['FHIR implementation guide', 'API documentation'],
        lastAssessment: '2024-09-10',
        assessor: 'Raj Patel, Technical Lead'
      }
    ]
  }
];

const mockAudits: ComplianceAudit[] = [
  {
    id: 'audit-hipaa-2024-q3',
    regulationId: 'hipaa-us',
    auditDate: '2024-09-15',
    auditor: 'Healthcare Compliance Associates',
    status: 'compliant',
    score: 85,
    findings: [
      {
        id: 'finding-1',
        severity: 'medium',
        category: 'Security',
        description: 'Outdated encryption protocols in use',
        remediation: 'Upgrade to AES-256 encryption',
        dueDate: '2024-11-30',
        status: 'in-progress',
        assignee: 'IT Security Team'
      }
    ],
    recommendations: [
      'Implement regular security training',
      'Update incident response procedures',
      'Enhance access logging'
    ],
    nextAuditDate: '2025-03-15'
  },
  {
    id: 'audit-gdpr-2024-q2',
    regulationId: 'gdpr-eu',
    auditDate: '2024-08-20',
    auditor: 'EU Privacy Consultants',
    status: 'warning',
    score: 78,
    findings: [
      {
        id: 'finding-2',
        severity: 'high',
        category: 'Data Processing',
        description: 'Missing data retention policies',
        remediation: 'Develop and implement data retention schedule',
        dueDate: '2024-12-15',
        status: 'open',
        assignee: 'Legal Team'
      }
    ],
    recommendations: [
      'Implement automated data deletion',
      'Enhanced consent management',
      'Regular staff training on GDPR'
    ],
    nextAuditDate: '2025-02-20'
  }
];

// Initial state
const initialState: ComplianceState = {
  selectedLocation: mockLocations[0],
  availableLocations: mockLocations,
  regulations: mockRegulations,
  selectedRegulation: null,
  audits: mockAudits,
  overallComplianceScore: 81,
  loading: {
    fetchRegulations: false,
    fetchAudits: false,
    updateCompliance: false,
    generateReport: false,
  },
  error: null,
  filters: {
    category: '',
    status: '',
    mandatory: null,
  },
};

// Compliance slice
const complianceSlice = createSlice({
  name: 'compliance',
  initialState,
  reducers: {
    // Location selection
    selectLocation: (state, action: PayloadAction<CountryState>) => {
      state.selectedLocation = action.payload;
      // Filter regulations based on selected location
      state.regulations = mockRegulations.filter(reg => 
        reg.country === action.payload.country || reg.country === 'EU'
      );
    },

    // Fetch regulations
    fetchRegulationsRequest: (state) => {
      state.loading.fetchRegulations = true;
      state.error = null;
    },
    fetchRegulationsSuccess: (state, action: PayloadAction<HealthcareRegulation[]>) => {
      state.loading.fetchRegulations = false;
      state.regulations = action.payload;
      state.error = null;
    },
    fetchRegulationsFailure: (state, action: PayloadAction<string>) => {
      state.loading.fetchRegulations = false;
      state.error = action.payload;
    },

    // Select regulation
    selectRegulation: (state, action: PayloadAction<HealthcareRegulation | null>) => {
      state.selectedRegulation = action.payload;
    },

    // Update compliance status
    updateComplianceRequest: (state, action: PayloadAction<{ regulationId: string; requirementId: string; status: ComplianceStatus }>) => {
      state.loading.updateCompliance = true;
      state.error = null;
    },
    updateComplianceSuccess: (state, action: PayloadAction<{ regulationId: string; requirementId: string; status: ComplianceStatus; evidence?: string[]; notes?: string }>) => {
      state.loading.updateCompliance = false;
      const { regulationId, requirementId, status, evidence, notes } = action.payload;
      
      // Update regulation requirement
      const regulation = state.regulations.find(r => r.id === regulationId);
      if (regulation) {
        const requirement = regulation.requirements.find(req => req.id === requirementId);
        if (requirement) {
          requirement.status = status;
          requirement.lastAssessment = new Date().toISOString();
          if (evidence) requirement.evidence = evidence;
          if (notes) requirement.notes = notes;
        }
      }

      // Update selected regulation if it matches
      if (state.selectedRegulation?.id === regulationId) {
        const requirement = state.selectedRegulation.requirements.find(req => req.id === requirementId);
        if (requirement) {
          requirement.status = status;
          requirement.lastAssessment = new Date().toISOString();
          if (evidence) requirement.evidence = evidence;
          if (notes) requirement.notes = notes;
        }
      }

      state.error = null;
    },
    updateComplianceFailure: (state, action: PayloadAction<string>) => {
      state.loading.updateCompliance = false;
      state.error = action.payload;
    },

    // Fetch audits
    fetchAuditsRequest: (state) => {
      state.loading.fetchAudits = true;
      state.error = null;
    },
    fetchAuditsSuccess: (state, action: PayloadAction<ComplianceAudit[]>) => {
      state.loading.fetchAudits = false;
      state.audits = action.payload;
      state.error = null;
    },
    fetchAuditsFailure: (state, action: PayloadAction<string>) => {
      state.loading.fetchAudits = false;
      state.error = action.payload;
    },

    // Update audit finding
    updateAuditFinding: (state, action: PayloadAction<{ auditId: string; findingId: string; updates: Partial<AuditFinding> }>) => {
      const { auditId, findingId, updates } = action.payload;
      const audit = state.audits.find(a => a.id === auditId);
      if (audit) {
        const finding = audit.findings.find(f => f.id === findingId);
        if (finding) {
          Object.assign(finding, updates);
        }
      }
    },

    // Generate compliance report
    generateReportRequest: (state, action: PayloadAction<{ regulationIds: string[]; format: 'pdf' | 'excel' | 'csv' }>) => {
      state.loading.generateReport = true;
      state.error = null;
    },
    generateReportSuccess: (state, action: PayloadAction<{ reportUrl: string }>) => {
      state.loading.generateReport = false;
      state.error = null;
      // Could add notification here
    },
    generateReportFailure: (state, action: PayloadAction<string>) => {
      state.loading.generateReport = false;
      state.error = action.payload;
    },

    // Update filters
    updateFilters: (state, action: PayloadAction<Partial<typeof initialState.filters>>) => {
      state.filters = { ...state.filters, ...action.payload };
    },

    // Calculate overall compliance score
    calculateOverallScore: (state) => {
      const totalRequirements = state.regulations.reduce((total, reg) => total + reg.requirements.length, 0);
      const compliantRequirements = state.regulations.reduce((total, reg) => 
        total + reg.requirements.filter(req => req.status === 'compliant').length, 0
      );
      
      state.overallComplianceScore = totalRequirements > 0 ? Math.round((compliantRequirements / totalRequirements) * 100) : 0;
    },

    // Clear error
    clearError: (state) => {
      state.error = null;
    },

    // Reset state
    resetComplianceState: () => initialState,
  },
});

// Export actions
export const {
  selectLocation,
  fetchRegulationsRequest,
  fetchRegulationsSuccess,
  fetchRegulationsFailure,
  selectRegulation,
  updateComplianceRequest,
  updateComplianceSuccess,
  updateComplianceFailure,
  fetchAuditsRequest,
  fetchAuditsSuccess,
  fetchAuditsFailure,
  updateAuditFinding,
  generateReportRequest,
  generateReportSuccess,
  generateReportFailure,
  updateFilters,
  calculateOverallScore,
  clearError,
  resetComplianceState,
} = complianceSlice.actions;

// Export default reducer
export default complianceSlice;