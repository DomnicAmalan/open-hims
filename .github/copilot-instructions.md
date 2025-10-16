# HIMS Core SDK - Rust & React Native

This is a Rust-based Healthcare Information Management System (HIMS) core SDK with React Native bindings.

## Project Structure
- `src/` - Rust source code for healthcare standards
- `bindings/` - React Native FFI bindings
- `examples/` - Usage examples for each module
- `docs/` - API documentation

## Development Guidelines
- Use Rust for core business logic and performance-critical operations
- Expose APIs through uniffi for React Native integration
- Follow HIPAA, GDPR, and healthcare data security standards
- Implement comprehensive error handling and logging
- Write tests for all public APIs

## Standards Supported
- FHIR R4/R5 for healthcare data exchange
- HL7v2 for legacy system integration  
- DICOM for medical imaging
- ABDM (Ayushman Bharat Digital Mission) for Indian healthcare
- Various accreditation standards (JCI, NABH, NABL)