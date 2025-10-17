export interface FhirResource {
    resourceType: string;
    id?: string;
    meta?: FhirMeta;
    implicitRules?: string;
    language?: string;
}
export interface FhirMeta {
    versionId?: string;
    lastUpdated?: string;
    source?: string;
    profile?: string[];
    security?: FhirCoding[];
    tag?: FhirCoding[];
}
export interface FhirCoding {
    system?: string;
    version?: string;
    code?: string;
    display?: string;
    userSelected?: boolean;
}
export interface FhirIdentifier {
    use?: 'usual' | 'official' | 'temp' | 'secondary' | 'old';
    type?: FhirCodeableConcept;
    system?: string;
    value?: string;
    period?: FhirPeriod;
    assigner?: FhirReference;
}
export interface FhirCodeableConcept {
    coding?: FhirCoding[];
    text?: string;
}
export interface FhirPeriod {
    start?: string;
    end?: string;
}
export interface FhirReference {
    reference?: string;
    type?: string;
    identifier?: FhirIdentifier;
    display?: string;
}
export interface FhirHumanName {
    use?: 'usual' | 'official' | 'temp' | 'nickname' | 'anonymous' | 'old' | 'maiden';
    text?: string;
    family?: string;
    given?: string[];
    prefix?: string[];
    suffix?: string[];
    period?: FhirPeriod;
}
export interface FhirContactPoint {
    system?: 'phone' | 'fax' | 'email' | 'pager' | 'url' | 'sms' | 'other';
    value?: string;
    use?: 'home' | 'work' | 'temp' | 'old' | 'mobile';
    rank?: number;
    period?: FhirPeriod;
}
export interface FhirAddress {
    use?: 'home' | 'work' | 'temp' | 'old' | 'billing';
    type?: 'postal' | 'physical' | 'both';
    text?: string;
    line?: string[];
    city?: string;
    district?: string;
    state?: string;
    postalCode?: string;
    country?: string;
    period?: FhirPeriod;
}
export interface FhirPatient extends FhirResource {
    resourceType: 'Patient';
    identifier?: FhirIdentifier[];
    active?: boolean;
    name?: FhirHumanName[];
    telecom?: FhirContactPoint[];
    gender?: 'male' | 'female' | 'other' | 'unknown';
    birthDate?: string;
    deceasedBoolean?: boolean;
    deceasedDateTime?: string;
    address?: FhirAddress[];
    maritalStatus?: FhirCodeableConcept;
    multipleBirthBoolean?: boolean;
    multipleBirthInteger?: number;
    photo?: FhirAttachment[];
    contact?: FhirPatientContact[];
    communication?: FhirPatientCommunication[];
    generalPractitioner?: FhirReference[];
    managingOrganization?: FhirReference;
    link?: FhirPatientLink[];
}
export interface FhirAttachment {
    contentType?: string;
    language?: string;
    data?: string;
    url?: string;
    size?: number;
    hash?: string;
    title?: string;
    creation?: string;
}
export interface FhirPatientContact {
    relationship?: FhirCodeableConcept[];
    name?: FhirHumanName;
    telecom?: FhirContactPoint[];
    address?: FhirAddress;
    gender?: 'male' | 'female' | 'other' | 'unknown';
    organization?: FhirReference;
    period?: FhirPeriod;
}
export interface FhirPatientCommunication {
    language: FhirCodeableConcept;
    preferred?: boolean;
}
export interface FhirPatientLink {
    other: FhirReference;
    type: 'replaced-by' | 'replaces' | 'refer' | 'seealso';
}
export interface FhirObservation extends FhirResource {
    resourceType: 'Observation';
    identifier?: FhirIdentifier[];
    basedOn?: FhirReference[];
    partOf?: FhirReference[];
    status: 'registered' | 'preliminary' | 'final' | 'amended' | 'corrected' | 'cancelled' | 'entered-in-error' | 'unknown';
    category?: FhirCodeableConcept[];
    code: FhirCodeableConcept;
    subject?: FhirReference;
    focus?: FhirReference[];
    encounter?: FhirReference;
    effectiveDateTime?: string;
    effectivePeriod?: FhirPeriod;
    issued?: string;
    performer?: FhirReference[];
    valueQuantity?: FhirQuantity;
    valueCodeableConcept?: FhirCodeableConcept;
    valueString?: string;
    valueBoolean?: boolean;
    valueInteger?: number;
    valueRange?: FhirRange;
    valueRatio?: FhirRatio;
    valueSampledData?: FhirSampledData;
    valueTime?: string;
    valueDateTime?: string;
    valuePeriod?: FhirPeriod;
    dataAbsentReason?: FhirCodeableConcept;
    interpretation?: FhirCodeableConcept[];
    note?: FhirAnnotation[];
    bodySite?: FhirCodeableConcept;
    method?: FhirCodeableConcept;
    specimen?: FhirReference;
    device?: FhirReference;
    referenceRange?: FhirObservationReferenceRange[];
    hasMember?: FhirReference[];
    derivedFrom?: FhirReference[];
    component?: FhirObservationComponent[];
}
export interface FhirQuantity {
    value?: number;
    comparator?: '<' | '<=' | '>=' | '>';
    unit?: string;
    system?: string;
    code?: string;
}
export interface FhirRange {
    low?: FhirQuantity;
    high?: FhirQuantity;
}
export interface FhirRatio {
    numerator?: FhirQuantity;
    denominator?: FhirQuantity;
}
export interface FhirSampledData {
    origin: FhirQuantity;
    period: number;
    factor?: number;
    lowerLimit?: number;
    upperLimit?: number;
    dimensions: number;
    data?: string;
}
export interface FhirAnnotation {
    authorReference?: FhirReference;
    authorString?: string;
    time?: string;
    text: string;
}
export interface FhirObservationReferenceRange {
    low?: FhirQuantity;
    high?: FhirQuantity;
    type?: FhirCodeableConcept;
    appliesTo?: FhirCodeableConcept[];
    age?: FhirRange;
    text?: string;
}
export interface FhirObservationComponent {
    code: FhirCodeableConcept;
    valueQuantity?: FhirQuantity;
    valueCodeableConcept?: FhirCodeableConcept;
    valueString?: string;
    valueBoolean?: boolean;
    valueInteger?: number;
    valueRange?: FhirRange;
    valueRatio?: FhirRatio;
    valueSampledData?: FhirSampledData;
    valueTime?: string;
    valueDateTime?: string;
    valuePeriod?: FhirPeriod;
    dataAbsentReason?: FhirCodeableConcept;
    interpretation?: FhirCodeableConcept[];
    referenceRange?: FhirObservationReferenceRange[];
}
export interface FhirEncounter extends FhirResource {
    resourceType: 'Encounter';
    identifier?: FhirIdentifier[];
    status: 'planned' | 'arrived' | 'triaged' | 'in-progress' | 'onleave' | 'finished' | 'cancelled' | 'entered-in-error' | 'unknown';
    statusHistory?: FhirEncounterStatusHistory[];
    class: FhirCoding;
    classHistory?: FhirEncounterClassHistory[];
    type?: FhirCodeableConcept[];
    serviceType?: FhirCodeableConcept;
    priority?: FhirCodeableConcept;
    subject?: FhirReference;
    episodeOfCare?: FhirReference[];
    basedOn?: FhirReference[];
    participant?: FhirEncounterParticipant[];
    appointment?: FhirReference[];
    period?: FhirPeriod;
    length?: FhirQuantity;
    reasonCode?: FhirCodeableConcept[];
    reasonReference?: FhirReference[];
    diagnosis?: FhirEncounterDiagnosis[];
    account?: FhirReference[];
    hospitalization?: FhirEncounterHospitalization;
    location?: FhirEncounterLocation[];
    serviceProvider?: FhirReference;
    partOf?: FhirReference;
}
export interface FhirEncounterStatusHistory {
    status: string;
    period: FhirPeriod;
}
export interface FhirEncounterClassHistory {
    class: FhirCoding;
    period: FhirPeriod;
}
export interface FhirEncounterParticipant {
    type?: FhirCodeableConcept[];
    period?: FhirPeriod;
    individual?: FhirReference;
}
export interface FhirEncounterDiagnosis {
    condition: FhirReference;
    use?: FhirCodeableConcept;
    rank?: number;
}
export interface FhirEncounterHospitalization {
    preAdmissionIdentifier?: FhirIdentifier;
    origin?: FhirReference;
    admitSource?: FhirCodeableConcept;
    reAdmission?: FhirCodeableConcept;
    dietPreference?: FhirCodeableConcept[];
    specialCourtesy?: FhirCodeableConcept[];
    specialArrangement?: FhirCodeableConcept[];
    destination?: FhirReference;
    dischargeDisposition?: FhirCodeableConcept;
}
export interface FhirEncounterLocation {
    location: FhirReference;
    status?: 'planned' | 'active' | 'reserved' | 'completed';
    physicalType?: FhirCodeableConcept;
    period?: FhirPeriod;
}
export interface FhirBundle extends FhirResource {
    resourceType: 'Bundle';
    identifier?: FhirIdentifier;
    type: 'document' | 'message' | 'transaction' | 'transaction-response' | 'batch' | 'batch-response' | 'history' | 'searchset' | 'collection';
    timestamp?: string;
    total?: number;
    link?: FhirBundleLink[];
    entry?: FhirBundleEntry[];
    signature?: FhirSignature;
}
export interface FhirBundleLink {
    relation: string;
    url: string;
}
export interface FhirBundleEntry {
    link?: FhirBundleLink[];
    fullUrl?: string;
    resource?: FhirResource;
    search?: FhirBundleEntrySearch;
    request?: FhirBundleEntryRequest;
    response?: FhirBundleEntryResponse;
}
export interface FhirBundleEntrySearch {
    mode?: 'match' | 'include' | 'outcome';
    score?: number;
}
export interface FhirBundleEntryRequest {
    method: 'GET' | 'HEAD' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
    url: string;
    ifNoneMatch?: string;
    ifModifiedSince?: string;
    ifMatch?: string;
    ifNoneExist?: string;
}
export interface FhirBundleEntryResponse {
    status: string;
    location?: string;
    etag?: string;
    lastModified?: string;
    outcome?: FhirResource;
}
export interface FhirSignature {
    type: FhirCoding[];
    when: string;
    who: FhirReference;
    onBehalfOf?: FhirReference;
    targetFormat?: string;
    sigFormat?: string;
    data?: string;
}
export interface FhirSearchParams {
    _count?: number;
    _offset?: number;
    _sort?: string;
    _include?: string[];
    _revinclude?: string[];
    _summary?: 'true' | 'text' | 'data' | 'count' | 'false';
    _elements?: string[];
    _format?: 'json' | 'xml';
    [key: string]: any;
}
export interface FhirOperationOutcome extends FhirResource {
    resourceType: 'OperationOutcome';
    issue: FhirOperationOutcomeIssue[];
}
export interface FhirOperationOutcomeIssue {
    severity: 'fatal' | 'error' | 'warning' | 'information';
    code: string;
    details?: FhirCodeableConcept;
    diagnostics?: string;
    location?: string[];
    expression?: string[];
}
export interface HimsPatientExtension {
    mrn?: string;
    nationalId?: string;
    insuranceInfo?: {
        provider: string;
        policyNumber: string;
        groupNumber: string;
        effectiveDate?: string;
        expirationDate?: string;
    };
    emergencyContact?: {
        name: string;
        relationship: string;
        phone: string;
        email?: string;
    };
    preferences?: {
        language: string;
        communicationMethod: 'phone' | 'email' | 'sms' | 'mail';
        appointmentReminders: boolean;
    };
}
//# sourceMappingURL=fhir.d.ts.map