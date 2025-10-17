use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hims_core_sdk::fhir::{FhirPatient, FhirBundle};
use hims_core_sdk::hl7::{Hl7Parser, Hl7Message};
use hims_core_sdk::security::{encrypt, decrypt, hash_data};

fn bench_fhir_operations(c: &mut Criterion) {
    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "bench-patient",
        "name": [{"family": "Benchmark", "given": ["Test", "User"]}],
        "birthDate": "1985-06-15",
        "gender": "male",
        "identifier": [{"system": "SSN", "value": "123-45-6789"}],
        "telecom": [{"system": "phone", "value": "555-123-4567"}],
        "address": [{
            "line": ["123 Test St"],
            "city": "Test City",
            "state": "TS",
            "postalCode": "12345",
            "country": "US"
        }]
    }"#;

    c.bench_function("fhir_patient_deserialization", |b| {
        b.iter(|| {
            let _patient: FhirPatient = serde_json::from_str(black_box(patient_json)).unwrap();
        })
    });

    let patient: FhirPatient = serde_json::from_str(patient_json).unwrap();
    c.bench_function("fhir_patient_serialization", |b| {
        b.iter(|| {
            let _json = serde_json::to_string(black_box(&patient)).unwrap();
        })
    });

    c.bench_function("fhir_bundle_creation", |b| {
        b.iter(|| {
            let patient1 = FhirPatient::builder()
                .id("patient-1")
                .family_name("Test")
                .given_names(vec!["User".to_string()])
                .build();
                
            let patient2 = FhirPatient::builder()
                .id("patient-2")
                .family_name("Another")
                .given_names(vec!["Test".to_string()])
                .build();

            let _bundle = FhirBundle::builder()
                .bundle_type("collection")
                .add_resource(patient1)
                .add_resource(patient2)
                .build();
        })
    });
}

fn bench_hl7_operations(c: &mut Criterion) {
    let adt_message = r#"MSH|^~\&|EPIC|EPICADT|SMS|SMSADT|20231016140000||ADT^A04|12345|P|2.5
EVN|A04|20231016140000|||ADMIT_USER|20231016140000
PID|1||MRN12345^^^MR^EPIC||DOE^JOHN^WILLIAM^JR^^||19850615|M||2106-3|123 MAIN ST^^ANYTOWN^NY^12345^USA|GL|(555)123-4567|(555)123-4568|EN|M|CHR|MRN12345|123-45-6789|||2186-5||||||20231016140000
NK1|1|DOE^JANE^||WIFE|123 MAIN ST^^ANYTOWN^NY^12345^USA|(555)123-4567|(555)123-4568
PV1|1|I|ICU^101^1^NURSING HOME||19|ATT001^DOCTOR^ATTENDING^MD^^UPIN^Y|REF001^DOCTOR^REFERRING^MD^^UPIN|SUR001^DOCTOR^SURGEON^MD^^UPIN||ICU||||R||ATT001^DOCTOR^ATTENDING^MD^^UPIN|IP|CHR|A|||||||||||||||||||||||||20231016140000|20231016140000||V
OBX|1|ST|1554-5^GLUCOSE^LN||182|mg/dl|70_105|H|||F"#;

    let parser = Hl7Parser::new();
    
    c.bench_function("hl7_message_parsing", |b| {
        b.iter(|| {
            let _result = parser.parse(black_box(adt_message)).unwrap();
        })
    });

    let parsed_message = parser.parse(adt_message).unwrap();
    c.bench_function("hl7_message_validation", |b| {
        b.iter(|| {
            let _is_valid = parsed_message.validate();
        })
    });

    c.bench_function("hl7_to_fhir_conversion", |b| {
        b.iter(|| {
            let _fhir_patient = parsed_message.to_fhir_patient();
        })
    });
}

fn bench_security_operations(c: &mut Criterion) {
    let sensitive_data = "Patient: John Doe, SSN: 123-45-6789, DOB: 1985-06-15, Address: 123 Main St, Anytown NY 12345";
    let encryption_key = "super-secret-256-bit-encryption-key-for-benchmarking-purposes";

    c.bench_function("data_encryption", |b| {
        b.iter(|| {
            let _encrypted = encrypt(black_box(sensitive_data), black_box(encryption_key)).unwrap();
        })
    });

    let encrypted_data = encrypt(sensitive_data, encryption_key).unwrap();
    c.bench_function("data_decryption", |b| {
        b.iter(|| {
            let _decrypted = decrypt(black_box(&encrypted_data), black_box(encryption_key)).unwrap();
        })
    });

    c.bench_function("data_hashing", |b| {
        b.iter(|| {
            let _hash = hash_data(black_box(sensitive_data));
        })
    });

    c.bench_function("phi_detection", |b| {
        b.iter(|| {
            let phi_detector = hims_core_sdk::security::PhiDetector::new();
            let _result = phi_detector.scan_text(black_box(sensitive_data));
        })
    });
}

fn bench_compliance_operations(c: &mut Criterion) {
    use hims_core_sdk::countries::usa::states::CaliforniaHealthcareConfig;
    use hims_core_sdk::countries::common::InheritableStateConfig;

    c.bench_function("config_instantiation", |b| {
        b.iter(|| {
            let _config = CaliforniaHealthcareConfig::new();
        })
    });

    let config = CaliforniaHealthcareConfig::new();
    c.bench_function("config_serialization", |b| {
        b.iter(|| {
            let _json = serde_json::to_string(black_box(&config)).unwrap();
        })
    });

    let config_json = serde_json::to_string(&config).unwrap();
    c.bench_function("config_deserialization", |b| {
        b.iter(|| {
            let _config: CaliforniaHealthcareConfig = serde_json::from_str(black_box(&config_json)).unwrap();
        })
    });

    c.bench_function("compliance_check", |b| {
        b.iter(|| {
            let _hipaa = config.is_hipaa_required();
            let _ccpa = config.is_ccpa_required();
            let _retention = config.get_data_retention_years();
        })
    });
}

criterion_group!(
    benches,
    bench_fhir_operations,
    bench_hl7_operations,
    bench_security_operations,
    bench_compliance_operations
);
criterion_main!(benches);