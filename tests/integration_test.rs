use hims_core_sdk::fhir::{FhirClient, FhirPatient, FhirBundle};
use hims_core_sdk::hl7::{Hl7Parser, Hl7Message, Hl7AdtMessage};
use hims_core_sdk::security::{AuditLogger, SecurityEvent};
use tokio_test;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[cfg(test)]
mod fhir_tests {
    use super::*;

    #[tokio::test]
    async fn test_fhir_patient_creation() {
        let patient = FhirPatient::builder()
            .id("test-patient-001")
            .family_name("Doe")
            .given_names(vec!["John".to_string(), "William".to_string()])
            .birth_date("1990-01-15")
            .gender("male")
            .build();

        assert_eq!(patient.id.as_ref().unwrap(), "test-patient-001");
        assert_eq!(patient.name[0].family.as_ref().unwrap(), "Doe");
        assert_eq!(patient.birth_date.as_ref().unwrap(), "1990-01-15");
    }

    #[tokio::test]
    async fn test_fhir_bundle_creation() {
        let patient1 = FhirPatient::builder()
            .id("patient-001")
            .family_name("Smith")
            .given_names(vec!["Jane".to_string()])
            .build();

        let patient2 = FhirPatient::builder()
            .id("patient-002")
            .family_name("Johnson")
            .given_names(vec!["Bob".to_string()])
            .build();

        let bundle = FhirBundle::builder()
            .bundle_type("collection")
            .add_resource(patient1)
            .add_resource(patient2)
            .build();

        assert_eq!(bundle.total.unwrap(), 2);
        assert_eq!(bundle.entry.len(), 2);
    }

    #[tokio::test]
    async fn test_fhir_client_get_patient() {
        let mock_server = MockServer::start().await;
        
        let patient_response = r#"{
            "resourceType": "Patient",
            "id": "test-patient",
            "name": [{
                "family": "Doe",
                "given": ["John"]
            }],
            "birthDate": "1990-01-15",
            "gender": "male"
        }"#;

        Mock::given(method("GET"))
            .and(path("/Patient/test-patient"))
            .respond_with(ResponseTemplate::new(200).set_body_string(patient_response))
            .mount(&mock_server)
            .await;

        let client = FhirClient::new(&mock_server.uri());
        let patient = client.get_patient("test-patient").await.unwrap();

        assert_eq!(patient.id.as_ref().unwrap(), "test-patient");
        assert_eq!(patient.name[0].family.as_ref().unwrap(), "Doe");
    }

    #[tokio::test]
    async fn test_fhir_patient_search() {
        let mock_server = MockServer::start().await;
        
        let search_response = r#"{
            "resourceType": "Bundle",
            "type": "searchset",
            "total": 1,
            "entry": [{
                "resource": {
                    "resourceType": "Patient",
                    "id": "test-patient",
                    "name": [{"family": "Smith", "given": ["John"]}]
                }
            }]
        }"#;

        Mock::given(method("GET"))
            .and(path("/Patient"))
            .respond_with(ResponseTemplate::new(200).set_body_string(search_response))
            .mount(&mock_server)
            .await;

        let client = FhirClient::new(&mock_server.uri());
        let bundle = client.search_patients(&[("family", "Smith")]).await.unwrap();

        assert_eq!(bundle.total.unwrap(), 1);
        assert_eq!(bundle.entry.len(), 1);
    }
}

#[cfg(test)]
mod hl7_tests {
    use super::*;

    #[test]
    fn test_hl7_adt_message_parsing() {
        let hl7_message = r#"MSH|^~\&|SENDING_APP|SENDING_FACILITY|RECEIVING_APP|RECEIVING_FACILITY|20231016140000||ADT^A01|12345|P|2.5
EVN|A01|20231016140000
PID|1||123456^^^MR||DOE^JOHN^WILLIAM||19900115|M|||123 MAIN ST^^ANYTOWN^NY^12345||555-123-4567|555-123-4568|EN|M|CHR|123456789
PV1|1|I|ICU^101^1|||ATT001^DOCTOR^ATTENDING|||ICU||||A|||ATT001^DOCTOR^ATTENDING|IP|CHR|||||||||||||||||||||20231016140000"#;

        let parser = Hl7Parser::new();
        let result = parser.parse(hl7_message).unwrap();

        assert_eq!(result.message_type, "ADT");
        assert_eq!(result.message_control_id, "12345");
        assert_eq!(result.sending_application, "SENDING_APP");

        if let Hl7Message::Adt(adt_msg) = result {
            assert_eq!(adt_msg.event_type, "A01");
            assert_eq!(adt_msg.patient.patient_id, "123456");
            assert_eq!(adt_msg.patient.name.family, "DOE");
            assert_eq!(adt_msg.patient.name.given[0], "JOHN");
        }
    }

    #[test]
    fn test_hl7_validation() {
        let invalid_message = "INVALID|MESSAGE|FORMAT";
        
        let parser = Hl7Parser::new();
        let result = parser.parse(invalid_message);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid HL7 message format"));
    }

    #[test]
    fn test_hl7_segment_parsing() {
        let pid_segment = "PID|1||123456^^^MR||DOE^JOHN^WILLIAM||19900115|M|||123 MAIN ST^^ANYTOWN^NY^12345||555-123-4567|555-123-4568|EN|M|CHR|123456789";
        
        let parser = Hl7Parser::new();
        let segment = parser.parse_segment(pid_segment).unwrap();

        assert_eq!(segment.segment_type, "PID");
        assert_eq!(segment.fields.len(), 19);
        assert_eq!(segment.get_field(5).unwrap(), "DOE^JOHN^WILLIAM");
    }
}

#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_audit_log_creation() {
        let audit_logger = AuditLogger::new();
        
        let event = SecurityEvent::builder()
            .event_type("patient_access")
            .user_id("user123")
            .resource_type("Patient")
            .resource_id("patient456")
            .description("Patient record accessed")
            .build();

        let log_entry = audit_logger.log_event(event);

        assert_eq!(log_entry.event_type, "patient_access");
        assert_eq!(log_entry.user_id, "user123");
        assert_eq!(log_entry.resource_type, "Patient");
        assert!(log_entry.timestamp > 0);
    }

    #[test]
    fn test_data_encryption() {
        let sensitive_data = "Patient SSN: 123-45-6789";
        let encryption_key = "test-encryption-key-256bit";

        let encrypted = hims_core_sdk::security::encrypt(sensitive_data, encryption_key).unwrap();
        let decrypted = hims_core_sdk::security::decrypt(&encrypted, encryption_key).unwrap();

        assert_ne!(encrypted, sensitive_data);
        assert_eq!(decrypted, sensitive_data);
    }

    #[test]
    fn test_phi_detection() {
        let text_with_phi = "Patient John Doe, SSN: 123-45-6789, Phone: (555) 123-4567";
        
        let phi_detector = hims_core_sdk::security::PhiDetector::new();
        let detection_result = phi_detector.scan_text(text_with_phi);

        assert!(detection_result.contains_phi);
        assert!(detection_result.detected_types.contains(&"ssn"));
        assert!(detection_result.detected_types.contains(&"phone"));
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use criterion::{criterion_group, criterion_main, Criterion};

    fn benchmark_fhir_parsing(c: &mut Criterion) {
        let patient_json = r#"{
            "resourceType": "Patient",
            "id": "test-patient",
            "name": [{"family": "Doe", "given": ["John"]}],
            "birthDate": "1990-01-15",
            "gender": "male"
        }"#;

        c.bench_function("fhir_patient_parsing", |b| {
            b.iter(|| {
                let _patient: FhirPatient = serde_json::from_str(patient_json).unwrap();
            })
        });
    }

    fn benchmark_hl7_parsing(c: &mut Criterion) {
        let hl7_message = r#"MSH|^~\&|SENDING_APP|SENDING_FACILITY|RECEIVING_APP|RECEIVING_FACILITY|20231016140000||ADT^A01|12345|P|2.5
PID|1||123456^^^MR||DOE^JOHN^WILLIAM||19900115|M|||123 MAIN ST^^ANYTOWN^NY^12345||555-123-4567|555-123-4568|EN|M|CHR|123456789"#;

        c.bench_function("hl7_message_parsing", |b| {
            b.iter(|| {
                let parser = Hl7Parser::new();
                let _result = parser.parse(hl7_message).unwrap();
            })
        });
    }

    criterion_group!(benches, benchmark_fhir_parsing, benchmark_hl7_parsing);
}

criterion_main!(performance_tests::benches);