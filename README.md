# HIMS Core SDK - Rust & React Native

A comprehensive Healthcare Information Management System (HIMS) core SDK built in Rust with React Native bindings for mobile and cross-platform healthcare applications.

## 🏥 Features

### Healthcare Standards Support
- **FHIR R4/R5** - Complete healthcare data exchange capabilities
- **HL7v2** - Legacy system integration (ADT, ORU, ORM message handling)
- **DICOM** - Medical imaging metadata parsing and web client support
- **ABDM** - Ayushman Bharat Digital Mission integration for Indian healthcare
- **Terminology Services** - LOINC, SNOMED CT, ICD-10 code mapping

### Country & State-Specific Compliance (Object-Oriented Inheritance)
- **Inheritance-Based Configuration** - States inherit from federal/national regulations
- **Override System** - States can override or extend federal requirements
- **Multi-Level Validation** - Federal, state, and local compliance checking
- **Regulation Groups** - Group similar states with common laws (e.g., CCPA-like states)
- **Dynamic Compliance** - Real-time compliance validation based on location

#### Supported Countries & Regions
- **United States** - Federal + all 50 states with inheritance chains
  - California (CCPA, CPRA) → Nevada, Virginia (inherit privacy laws)
  - Texas → Similar southern states
  - New York → Northeast states with similar regulations
- **India** - Central + state-wise with ABDM integration
  - Maharashtra → Other ABDM-integrated states
  - Karnataka → Tech-forward states
- **European Union** - GDPR base + country-specific implementations
- **Canada** - Federal + provincial healthcare regulations
- **Australia** - National + state/territory specific rules
- **United Kingdom** - NHS + devolved nation specifics

### Security & Compliance
- **HIPAA Audit Logging** - Comprehensive audit trail for healthcare data access
- **GDPR Consent Management** - Data subject rights and consent tracking
- **ISO 27001 Logging** - Security event logging and monitoring
- **Hash Chain Logs** - Tamper-proof audit trails

### Multi-Platform Support
- **Rust Core** - High-performance, memory-safe healthcare data processing
- **Desktop (Tauri)** - Cross-platform desktop apps with native performance
- **Mobile (React Native)** - iOS and Android with UniFFI bindings
- **Web (React/Vite)** - Browser-based applications with WebAssembly
- **API Backend** - REST/GraphQL server implementations

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- Node.js 18+
- React Native development environment

### Installation

#### Rust Library
```bash
cargo add hims-core-sdk
```

#### React Native Package
```bash
npm install hims-core-sdk-react-native
# or
yarn add hims-core-sdk-react-native
```

### Basic Usage

#### Rust
```rust
use hims_core_sdk::*;

#[tokio::main]
async fn main() -> Result<(), HimsError> {
    // Initialize HIMS SDK with country/state configuration
    let config = HimsConfig {
        api_endpoint: "https://api.healthcare.org/fhir".to_string(),
        auth_token: Some("your-jwt-token".to_string()),
        enable_logging: true,
        country_code: Some("US".to_string()),
        state_code: Some("CA".to_string()), // California
    };
    
    let hims = HimsCore::new(config);
    hims.initialize()?;
    
    // Validate compliance for specific operation
    let compliant = hims.validate_compliance("US", Some("CA"), "patient_access")?;
    println!("California compliance: {}", compliant);
    
    // Get compliance requirements
    let requirements = hims.get_compliance_requirements("US", Some("CA"))?;
    for req in requirements {
        println!("Level: {}, Authority: {}", req.level, req.authority);
    }
    
    // Create a FHIR client with state-specific validation
    let fhir_client = FhirClient::new(
        "https://api.healthcare.org/fhir".to_string(),
        Some("auth-token".to_string())
    );
    
    // Create a patient (automatically validates against CA requirements)
    let mut patient = Patient::new();
    patient.name.push(HumanName {
        use_type: Some("official".to_string()),
        family: Some("Doe".to_string()),
        given: vec!["John".to_string()],
    });
    
    let created_patient = fhir_client.create_patient(&patient).await?;
    println!("Created patient: {:?}", created_patient);
    
    Ok(())
}
```

#### React Native (Mobile)
```typescript
import HimsCoreSDK, { HimsConfig } from 'hims-core-sdk-react-native';

const config: HimsConfig = {
  apiEndpoint: 'https://api.healthcare.org/fhir',
  authToken: 'your-jwt-token',
  enableLogging: true,
  countryCode: 'US',
  stateCode: 'CA', // California-specific compliance
};

const hims = new HimsCoreSDK(config);

// Initialize with location-based compliance
await hims.initialize();

// Check supported countries and states
const countries = await hims.getSupportedCountries();
const states = await hims.getSupportedStates('US');

// Validate compliance for specific operation
const isCompliant = await hims.validateCompliance('US', 'CA', 'patient_access');
console.log('Compliant with CA regulations:', isCompliant);

// Create a patient with automatic compliance validation
const patientData = {
  name: [{ family: 'Doe', given: ['John'] }],
  gender: 'male',
  birthDate: '1990-01-01',
};

const patient = await hims.createPatient(patientData);
console.log('Created patient:', patient);
```

#### Desktop (Tauri)
```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Initialize HIMS with country/state configuration
const config = {
  api_endpoint: 'https://api.healthcare.org/fhir',
  auth_token: 'your-jwt-token',
  enable_logging: true,
  country_code: 'US',
  state_code: 'CA',
};

await invoke('initialize_hims', { config });

// Validate compliance
const compliant = await invoke('validate_compliance', {
  countryCode: 'US',
  stateCode: 'CA'
});

// Parse HL7 message
const hl7Result = await invoke('parse_hl7_message', {
  message: 'MSH|^~\\&|EPIC|HOSPITAL|||...'
});

// Generate audit report
const auditReport = await invoke('generate_audit_report', {
  startDate: '2024-01-01',
  endDate: '2024-12-31'
});
```

#### Web (React/Vite with WebAssembly)
```typescript
import init, { HimsCore, HimsConfig } from 'hims-core-sdk-wasm';

// Initialize WASM module
await init();

const config: HimsConfig = {
  api_endpoint: 'https://api.healthcare.org/fhir',
  auth_token: 'your-jwt-token',
  enable_logging: true,
  country_code: 'IN',
  state_code: 'MH', // Maharashtra, India
};

const hims = new HimsCore(config);
await hims.initialize();

// Get ABDM-specific compliance for Maharashtra
const compliance = await hims.get_compliance_requirements('IN', 'MH');
console.log('Maharashtra compliance:', compliance);
```

## 📁 Project Structure

```
hims-core-sdk/
├── src/
│   ├── standards/           # Healthcare standards implementation
│   │   ├── fhir/           # FHIR R4/R5 support
│   │   │   ├── models/     # Patient, Observation, Bundle, etc.
│   │   │   ├── client/     # FHIR server communication
│   │   │   ├── transformers/ # Data format conversions
│   │   │   └── validators/ # Data validation
│   │   ├── hl7v2/          # HL7v2 message processing
│   │   │   ├── parser/     # ADT, ORU, ORM handlers
│   │   │   ├── mapper/     # HL7→FHIR conversion
│   │   │   └── generator/  # Message generation
│   │   ├── dicom/          # DICOM support
│   │   │   ├── dicomweb-client # QIDO, WADO, STOW
│   │   │   ├── metadata-parser
│   │   │   └── viewer-integration
│   │   ├── terminology/    # Code systems and mappings
│   │   ├── abdm/           # Indian healthcare standards
│   │   └── accreditation/  # JCI, NABH, NABL standards
│   ├── countries/          # Country & state-specific regulations
│   │   ├── common/         # Base inheritance system
│   │   ├── usa/            # US federal + all states
│   │   │   ├── federal/    # Federal regulations (HIPAA, HITECH)
│   │   │   └── states/     # State-specific (CA, TX, NY, FL...)
│   │   ├── india/          # India central + states
│   │   │   ├── central/    # Central regulations (DPDP Act, ABDM)
│   │   │   └── states/     # State-specific (MH, KA, TN...)
│   │   ├── eu/             # GDPR + country implementations
│   │   ├── canada/         # Federal + provincial
│   │   └── australia/      # National + state/territory
│   ├── security/           # Security and compliance
│   │   ├── hipaa-audit/    # HIPAA audit logging
│   │   ├── gdpr-consent/   # GDPR compliance
│   │   ├── iso27001-logging/ # Security logging
│   │   └── hash-chain-logs/ # Immutable audit trails
│   ├── core/               # Core utilities
│   │   ├── auth/           # JWT, OAuth2, SMART-on-FHIR
│   │   ├── config/         # Configuration management
│   │   ├── errors/         # Error handling
│   │   ├── logger/         # Structured logging
│   │   └── utils/          # Common utilities
│   └── exporters/          # Data export formats
│       ├── pdf/            # PDF generation
│       ├── x12-edi/        # EDI transaction sets
│       ├── csv-fhir-import/ # CSV to FHIR conversion
│       └── api-adapters/   # Third-party API integrations
├── apps/                   # Multi-platform applications
│   ├── desktop-tauri/      # Tauri desktop app
│   │   ├── src-tauri/      # Rust backend
│   │   └── src/            # React frontend
│   ├── web-react/          # React/Vite web app
│   │   ├── src/            # React components
│   │   └── public/         # Static assets
│   └── mobile-rn/          # React Native mobile app
│       ├── ios/            # iOS project
│       ├── android/        # Android project
│       └── src/            # React Native code
├── bindings/               # Cross-platform bindings
│   ├── react-native/       # React Native (UniFFI)
│   ├── web-wasm/          # WebAssembly bindings
│   └── python/            # Python bindings (optional)
├── examples/               # Usage examples
├── docs/                   # API documentation
└── tests/                  # Test suites
```

## 🔧 Development

### Building the Rust Library
```bash
cargo build --release
```

### Generating React Native Bindings
```bash
cargo run --bin uniffi-bindgen generate src/hims_core_sdk.udl --language kotlin --out-dir bindings/android
cargo run --bin uniffi-bindgen generate src/hims_core_sdk.udl --language swift --out-dir bindings/ios
```

### Running Tests
```bash
cargo test
```

### React Native Development
```bash
cd bindings/react-native
npm install
npm run build
```

## 📋 API Reference

### Core SDK

#### HimsCore
Main SDK interface for initialization and configuration.

```rust
impl HimsCore {
    pub fn new(config: HimsConfig) -> Self
    pub fn initialize(&self) -> Result<String, HimsError>
}
```

### FHIR Support

#### FhirClient
Complete FHIR R4/R5 client implementation.

```rust
impl FhirClient {
    pub async fn create_patient(&self, patient: &Patient) -> Result<Patient, HimsError>
    pub async fn get_patient(&self, id: &str) -> Result<Patient, HimsError>
    pub async fn search_patients(&self, query: &str) -> Result<Bundle, HimsError>
    pub async fn create_observation(&self, observation: &Observation) -> Result<Observation, HimsError>
}
```

### HL7v2 Support

#### Hl7Parser
Parse and process HL7v2 messages.

```rust
impl Hl7Parser {
    pub fn parse_message(&self, message: &str) -> Result<Hl7Message, HimsError>
    pub fn parse_adt_message(&self, message: &str) -> Result<AdtMessage, HimsError>
}
```

### Security & Compliance

#### HipaaAuditLogger
HIPAA-compliant audit logging.

```rust
impl HipaaAuditLogger {
    pub async fn log_patient_access(&self, user_id: String, patient_id: String, action: AuditAction, ip_address: String, outcome: AuditOutcome) -> Result<(), HimsError>
    pub async fn generate_audit_report(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<HipaaAuditEntry>, HimsError>
}
```

#### GdprConsentManager
GDPR consent and data subject rights management.

```rust
impl GdprConsentManager {
    pub async fn record_consent(&self, user_id: String, consent_type: ConsentType, purpose: String, legal_basis: LegalBasis, granted: bool) -> Result<GdprConsent, HimsError>
    pub async fn create_data_subject_request(&self, user_id: String, request_type: DataSubjectRightType) -> Result<DataSubjectRequest, HimsError>
}
```

## 🛡️ Security

This SDK implements comprehensive security measures for healthcare data:

- **End-to-end Encryption** - All data in transit and at rest
- **Audit Logging** - Complete audit trails for all data access
- **Access Controls** - Role-based access control (RBAC)
- **Data Anonymization** - Built-in data de-identification
- **Consent Management** - GDPR-compliant consent tracking

## 🏥 Healthcare Standards Compliance

- **HIPAA** - Health Insurance Portability and Accountability Act
- **GDPR** - General Data Protection Regulation
- **FHIR R4/R5** - Fast Healthcare Interoperability Resources
- **HL7v2** - Health Level Seven International
- **DICOM** - Digital Imaging and Communications in Medicine
- **IHE** - Integrating the Healthcare Enterprise profiles
- **ABDM** - Ayushman Bharat Digital Mission (India)

## 🌍 International Support

### Indian Healthcare (ABDM)
- Health ID integration
- Consent management
- Health Information Provider (HIP) integration
- Health Information User (HIU) integration

### Accreditation Standards
- **JCI** - Joint Commission International
- **NABH** - National Accreditation Board for Hospitals
- **NABL** - National Accreditation Board for Testing and Calibration Laboratories

## 📱 React Native Integration

The SDK provides seamless React Native integration through UniFFI-generated bindings:

```typescript
// Import the SDK
import HimsCoreSDK from 'hims-core-sdk-react-native';

// Initialize with configuration
const sdk = new HimsCoreSDK({
  apiEndpoint: 'https://your-fhir-server.com',
  authToken: 'your-auth-token',
  enableLogging: true,
});

// Use healthcare standards
await sdk.createPatient(patientData);
await sdk.parseHL7Message(hl7String);
await sdk.parseDicomMetadata(dicomFile);
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- 📧 Email: support@hims-sdk.org
- 📖 Documentation: [https://docs.hims-sdk.org](https://docs.hims-sdk.org)
- 🐛 Issues: [GitHub Issues](https://github.com/hims/hims-core-sdk/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/hims/hims-core-sdk/discussions)

## 🙏 Acknowledgments

- FHIR Foundation for healthcare interoperability standards
- HL7 International for messaging standards
- Rust community for excellent tooling and libraries
- React Native community for mobile development framework

---

Built with ❤️ for the healthcare community