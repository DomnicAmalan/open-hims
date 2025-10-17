// HL7 v2 Message Types
export interface Hl7Message {
  messageType: string;
  messageControlId: string;
  timestamp: string;
  sendingApplication: string;
  sendingFacility: string;
  receivingApplication: string;
  receivingFacility: string;
  versionId: string;
  segments: Hl7Segment[];
  raw?: string;
}

export interface Hl7Segment {
  segmentType: string;
  sequenceNumber?: number;
  fields: Hl7Field[];
  raw: string;
}

export interface Hl7Field {
  position: number;
  value: string | Hl7Component[];
  dataType?: string;
}

export interface Hl7Component {
  position: number;
  value: string | Hl7SubComponent[];
}

export interface Hl7SubComponent {
  position: number;
  value: string;
}

// Common HL7 Message Types
export interface Hl7AdtMessage extends Hl7Message {
  messageType: 'ADT';
  eventType: 'A01' | 'A02' | 'A03' | 'A04' | 'A05' | 'A06' | 'A07' | 'A08' | 'A09' | 'A10' | 'A11' | 'A12' | 'A13' | 'A14' | 'A15' | 'A16' | 'A17' | 'A18' | 'A19' | 'A20' | 'A21' | 'A22' | 'A23' | 'A24' | 'A25' | 'A26' | 'A27' | 'A28' | 'A29' | 'A30' | 'A31' | 'A32' | 'A33' | 'A34' | 'A35' | 'A36' | 'A37' | 'A38' | 'A39' | 'A40' | 'A41' | 'A42' | 'A43' | 'A44' | 'A45' | 'A46' | 'A47' | 'A48' | 'A49' | 'A50' | 'A51' | 'A52' | 'A53' | 'A54' | 'A55';
  patient: Hl7PatientInfo;
  visit?: Hl7VisitInfo;
}

export interface Hl7OrmMessage extends Hl7Message {
  messageType: 'ORM';
  eventType: 'O01' | 'O02' | 'O03';
  patient: Hl7PatientInfo;
  orders: Hl7OrderInfo[];
}

export interface Hl7OruMessage extends Hl7Message {
  messageType: 'ORU';
  eventType: 'R01' | 'R02' | 'R03';
  patient: Hl7PatientInfo;
  observations: Hl7ObservationInfo[];
}

// HL7 Data Structures
export interface Hl7PatientInfo {
  patientId: string;
  patientIdList?: string[];
  name: {
    family: string;
    given: string[];
    middle?: string;
    prefix?: string;
    suffix?: string;
  };
  birthDate: string;
  gender: 'M' | 'F' | 'O' | 'U';
  race?: string;
  address?: {
    street: string[];
    city: string;
    state: string;
    zip: string;
    country: string;
    type?: 'home' | 'work' | 'mailing';
  };
  phone?: string[];
  maritalStatus?: string;
  ssn?: string;
  mothersMaidenName?: string;
  nationality?: string;
  language?: string;
}

export interface Hl7VisitInfo {
  visitNumber: string;
  patientClass: 'E' | 'I' | 'O' | 'P' | 'R' | 'B' | 'N';
  assignedPatientLocation?: {
    pointOfCare: string;
    room: string;
    bed: string;
    facility: string;
    locationStatus?: string;
    personLocationType?: string;
    building?: string;
    floor?: string;
  };
  admissionType?: string;
  preadmitNumber?: string;
  priorPatientLocation?: string;
  attendingDoctor?: Hl7Provider;
  referringDoctor?: Hl7Provider;
  consultingDoctor?: Hl7Provider[];
  hospitalService?: string;
  temporaryLocation?: string;
  preadmitTestIndicator?: string;
  readmissionIndicator?: string;
  admitSource?: string;
  ambulatoryStatus?: string[];
  vipIndicator?: string;
  admittingDoctor?: Hl7Provider;
  patientType?: string;
  visitNumber2?: string;
  admitDateTime?: string;
  dischargeDateTime?: string;
  currentPatientBalance?: number;
  totalCharges?: number;
  totalAdjustments?: number;
  totalPayments?: number;
  alternateVisitId?: string;
}

export interface Hl7Provider {
  id: string;
  familyName: string;
  givenName: string;
  middleName?: string;
  suffix?: string;
  prefix?: string;
  degree?: string;
  sourceTable?: string;
  assigningAuthority?: string;
  nameTypeCode?: string;
  identifierCheckDigit?: string;
  checkDigitScheme?: string;
  identifierTypeCode?: string;
  assigningFacility?: string;
}

export interface Hl7OrderInfo {
  orderControlCode: string;
  placerOrderNumber?: string;
  fillerOrderNumber?: string;
  placerGroupNumber?: string;
  orderStatus?: string;
  responseFlag?: string;
  quantityTiming?: Hl7QuantityTiming;
  parent?: string;
  dateTimeOfTransaction?: string;
  enteredBy?: Hl7Provider;
  verifiedBy?: Hl7Provider;
  orderingProvider?: Hl7Provider;
  enterersLocation?: string;
  callBackPhoneNumber?: string[];
  orderEffectiveDateTime?: string;
  orderControlCodeReason?: string;
  enteringOrganization?: string;
  enteringDevice?: string;
  actionBy?: Hl7Provider;
  orderDetail?: Hl7OrderDetail;
}

export interface Hl7QuantityTiming {
  quantity?: string;
  interval?: string;
  duration?: string;
  startDateTime?: string;
  endDateTime?: string;
  priority?: string;
  condition?: string;
  text?: string;
  conjunction?: string;
  orderSequencing?: string;
  occurrenceDuration?: string;
  totalOccurrences?: string;
}

export interface Hl7OrderDetail {
  segmentType: 'OBR' | 'RXO' | 'RXE' | 'RXD';
  setId?: string;
  placerOrderNumber?: string;
  fillerOrderNumber?: string;
  universalServiceIdentifier: {
    identifier: string;
    text: string;
    nameOfCodingSystem: string;
    alternateIdentifier?: string;
    alternateText?: string;
    nameOfAlternateCodingSystem?: string;
  };
  priority?: string;
  requestedDateTime?: string;
  observationDateTime?: string;
  observationEndDateTime?: string;
  collectionVolume?: string;
  collectorIdentifier?: Hl7Provider;
  specimenActionCode?: string;
  dangerCode?: string;
  relevantClinicalInformation?: string;
  specimenReceivedDateTime?: string;
  specimenSource?: string;
  orderingProvider?: Hl7Provider;
  orderCallbackPhoneNumber?: string[];
  placerField1?: string;
  placerField2?: string;
  fillerField1?: string;
  fillerField2?: string;
  resultsRptStatusChngDateTime?: string;
  chargetoPractice?: string;
  diagnosticServSectId?: string;
  resultStatus?: string;
  parentResult?: string;
  transportationMode?: string;
  reasonForStudy?: string[];
  principalResultInterpreter?: Hl7Provider;
  assistantResultInterpreter?: Hl7Provider[];
  technician?: Hl7Provider[];
  transcriptionist?: Hl7Provider[];
  scheduledDateTime?: string;
  numberOfSampleContainers?: string;
  transportLogisticsOfCollectedSample?: string[];
  collectorsComment?: string[];
  transportArrangementResponsibility?: string;
  transportArranged?: string;
  escortRequired?: string;
  plannedPatientTransportComment?: string[];
}

export interface Hl7ObservationInfo {
  setId?: string;
  valueType: string;
  observationIdentifier: {
    identifier: string;
    text: string;
    nameOfCodingSystem: string;
    alternateIdentifier?: string;
    alternateText?: string;
    nameOfAlternateCodingSystem?: string;
  };
  observationSubId?: string;
  observationValue?: string | number | Hl7CodedElement;
  units?: Hl7CodedElement;
  referencesRange?: string;
  abnormalFlags?: string[];
  probability?: number;
  natureOfAbnormalTest?: string[];
  observationResultStatus: 'C' | 'D' | 'F' | 'I' | 'N' | 'O' | 'P' | 'R' | 'S' | 'U' | 'W' | 'X';
  effectiveDateOfReferenceRange?: string;
  userDefinedAccessChecks?: string;
  dateTimeOfObservation?: string;
  producersId?: string;
  responsibleObserver?: Hl7Provider;
  observationMethod?: Hl7CodedElement[];
  equipmentInstanceIdentifier?: string[];
  dateTimeOfAnalysis?: string;
}

export interface Hl7CodedElement {
  identifier: string;
  text: string;
  nameOfCodingSystem: string;
  alternateIdentifier?: string;
  alternateText?: string;
  nameOfAlternateCodingSystem?: string;
}

// HL7 Parsing and Validation
export interface Hl7ParseResult {
  success: boolean;
  message?: Hl7Message;
  errors: Hl7ParseError[];
  warnings: Hl7ParseWarning[];
}

export interface Hl7ParseError {
  severity: 'error' | 'fatal';
  code: string;
  message: string;
  line?: number;
  column?: number;
  segmentType?: string;
  fieldPosition?: number;
}

export interface Hl7ParseWarning {
  code: string;
  message: string;
  line?: number;
  column?: number;
  segmentType?: string;
  fieldPosition?: number;
}

// HL7 Configuration
export interface Hl7Configuration {
  fieldSeparator: string;
  componentSeparator: string;
  repetitionSeparator: string;
  escapeCharacter: string;
  subcomponentSeparator: string;
  truncationCharacter?: string;
  dateFormat: string;
  timeFormat: string;
  timestampFormat: string;
  validateRequired: boolean;
  validateDataTypes: boolean;
  strictParsing: boolean;
}

export const DEFAULT_HL7_CONFIG: Hl7Configuration = {
  fieldSeparator: '|',
  componentSeparator: '^',
  repetitionSeparator: '~',
  escapeCharacter: '\\',
  subcomponentSeparator: '&',
  dateFormat: 'YYYYMMDD',
  timeFormat: 'HHMMSS',
  timestampFormat: 'YYYYMMDDHHMMSS',
  validateRequired: true,
  validateDataTypes: true,
  strictParsing: false,
};