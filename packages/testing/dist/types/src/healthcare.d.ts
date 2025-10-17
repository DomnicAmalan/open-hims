export type ComplianceStatus = 'compliant' | 'non-compliant' | 'warning' | 'pending' | 'unknown';
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
    inheritsFrom?: string;
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
    severity: 'low' | 'medium' | 'high' | 'critical';
}
export interface ComplianceAudit {
    id: string;
    regulationId: string;
    auditDate: string;
    auditor: string;
    status: ComplianceStatus;
    score: number;
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
export interface CountryRegulation {
    country: string;
    countryName: string;
    regulations: HealthcareRegulation[];
    states?: StateRegulation[];
}
export interface StateRegulation {
    state: string;
    stateName: string;
    regulations: HealthcareRegulation[];
    inheritsFrom: string;
}
export interface AuditLog {
    id: string;
    timestamp: string;
    userId: string;
    userRole: string;
    action: AuditAction;
    resourceType: string;
    resourceId: string;
    description: string;
    ipAddress: string;
    userAgent: string;
    outcome: 'success' | 'failure' | 'unknown';
    details?: Record<string, any>;
    sensitivityLevel: 'low' | 'medium' | 'high' | 'critical';
    dataCategory: 'phi' | 'pii' | 'financial' | 'administrative' | 'clinical';
}
export type AuditAction = 'create' | 'read' | 'update' | 'delete' | 'login' | 'logout' | 'export' | 'import' | 'share' | 'print' | 'backup' | 'restore' | 'admin_action' | 'system_access' | 'data_breach' | 'security_incident';
export interface SecurityIncident {
    id: string;
    timestamp: string;
    type: 'data_breach' | 'unauthorized_access' | 'malware' | 'phishing' | 'system_failure' | 'other';
    severity: 'low' | 'medium' | 'high' | 'critical';
    status: 'open' | 'investigating' | 'mitigated' | 'resolved' | 'false_positive';
    description: string;
    affectedSystems: string[];
    affectedPatients?: string[];
    reportedBy: string;
    assignedTo?: string;
    mitigationSteps: string[];
    resolutionDate?: string;
    notificationRequired: boolean;
    notificationsSent: NotificationSent[];
    regulatoryReporting: RegulatoryReporting[];
}
export interface NotificationSent {
    type: 'patient' | 'regulator' | 'media' | 'law_enforcement' | 'insurance';
    recipient: string;
    method: 'email' | 'mail' | 'phone' | 'fax' | 'portal';
    sentDate: string;
    deliveryConfirmed: boolean;
}
export interface RegulatoryReporting {
    regulator: string;
    reportDate: string;
    reportId: string;
    status: 'submitted' | 'acknowledged' | 'under_review' | 'approved' | 'rejected';
    followUpRequired: boolean;
    followUpDueDate?: string;
}
export interface DataTransferRequest {
    id: string;
    requestId: string;
    timestamp: string;
    source: DataSource;
    destination: DataDestination;
    dataType: 'fhir' | 'hl7' | 'dicom' | 'ccda' | 'csv' | 'json' | 'xml';
    transferType: 'real-time' | 'batch' | 'scheduled';
    status: 'pending' | 'in-progress' | 'completed' | 'failed' | 'cancelled';
    priority: 'low' | 'normal' | 'high' | 'urgent';
    patientIds?: string[];
    dateRange?: {
        startDate: string;
        endDate: string;
    };
    filters?: Record<string, any>;
    encryption: boolean;
    compressionType?: 'gzip' | 'zip' | 'none';
    estimatedSize?: number;
    actualSize?: number;
    checksum?: string;
    error?: DataTransferError;
    progress?: DataTransferProgress;
}
export interface DataSource {
    type: 'ehr' | 'his' | 'lis' | 'ris' | 'pacs' | 'external_api' | 'file_upload';
    name: string;
    endpoint?: string;
    credentials?: {
        type: 'api_key' | 'oauth' | 'certificate' | 'basic_auth';
        details: Record<string, any>;
    };
    configuration?: Record<string, any>;
}
export interface DataDestination {
    type: 'ehr' | 'his' | 'lis' | 'ris' | 'pacs' | 'external_api' | 'file_export' | 'cloud_storage';
    name: string;
    endpoint?: string;
    credentials?: {
        type: 'api_key' | 'oauth' | 'certificate' | 'basic_auth';
        details: Record<string, any>;
    };
    configuration?: Record<string, any>;
}
export interface DataTransferError {
    code: string;
    message: string;
    details?: string;
    timestamp: string;
    retryable: boolean;
    retryCount: number;
    maxRetries: number;
}
export interface DataTransferProgress {
    totalRecords: number;
    processedRecords: number;
    successfulRecords: number;
    failedRecords: number;
    percentage: number;
    estimatedTimeRemaining?: number;
    throughputRecordsPerSecond?: number;
}
export interface HealthcareWorkflow {
    id: string;
    name: string;
    description: string;
    version: string;
    country: string;
    state?: string;
    category: 'admission' | 'discharge' | 'transfer' | 'medication' | 'lab_order' | 'imaging_order' | 'consultation' | 'billing';
    steps: WorkflowStep[];
    triggers: WorkflowTrigger[];
    conditions: WorkflowCondition[];
    notifications: WorkflowNotification[];
    escalations: WorkflowEscalation[];
    complianceRequirements: string[];
    createdBy: string;
    createdDate: string;
    lastModified: string;
    status: 'active' | 'inactive' | 'draft' | 'deprecated';
}
export interface WorkflowStep {
    id: string;
    name: string;
    description: string;
    type: 'manual' | 'automated' | 'decision' | 'notification' | 'integration';
    order: number;
    assignedRole: string;
    estimatedDuration: number;
    requiredFields: string[];
    validations: WorkflowValidation[];
    actions: WorkflowAction[];
    nextSteps: WorkflowTransition[];
}
export interface WorkflowTrigger {
    id: string;
    type: 'event' | 'schedule' | 'manual' | 'api_call';
    event?: string;
    schedule?: string;
    conditions: WorkflowCondition[];
}
export interface WorkflowCondition {
    field: string;
    operator: 'equals' | 'not_equals' | 'greater_than' | 'less_than' | 'contains' | 'exists' | 'in_list';
    value: any;
    logicalOperator?: 'and' | 'or';
}
export interface WorkflowValidation {
    field: string;
    type: 'required' | 'format' | 'range' | 'custom';
    parameters?: Record<string, any>;
    errorMessage: string;
}
export interface WorkflowAction {
    id: string;
    type: 'api_call' | 'email' | 'sms' | 'update_record' | 'create_record' | 'generate_document' | 'compliance_check';
    parameters: Record<string, any>;
    timeout?: number;
    retryPolicy?: {
        maxRetries: number;
        retryDelay: number;
    };
}
export interface WorkflowTransition {
    stepId: string;
    condition?: WorkflowCondition;
    probability?: number;
}
export interface WorkflowNotification {
    id: string;
    trigger: string;
    recipients: string[];
    method: 'email' | 'sms' | 'push' | 'in_app';
    template: string;
    priority: 'low' | 'normal' | 'high' | 'urgent';
}
export interface WorkflowEscalation {
    id: string;
    trigger: string;
    condition: WorkflowCondition;
    escalationLevel: number;
    recipients: string[];
    timeoutMinutes: number;
}
export interface WorkflowExecution {
    id: string;
    workflowId: string;
    patientId?: string;
    triggeredBy: string;
    startTime: string;
    endTime?: string;
    status: 'running' | 'completed' | 'failed' | 'cancelled' | 'paused';
    currentStep: string;
    stepHistory: WorkflowStepExecution[];
    variables: Record<string, any>;
    error?: string;
}
export interface WorkflowStepExecution {
    stepId: string;
    startTime: string;
    endTime?: string;
    status: 'pending' | 'in_progress' | 'completed' | 'failed' | 'skipped';
    assignedTo?: string;
    inputData: Record<string, any>;
    outputData?: Record<string, any>;
    notes?: string;
    error?: string;
}
