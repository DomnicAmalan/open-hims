// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hims_core_sdk::*;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    hims_core: Option<HimsCore>,
}

// Tauri commands
#[tauri::command]
async fn initialize_hims(config: HimsConfig) -> Result<String, String> {
    let hims = HimsCore::new(config);
    match hims.initialize() {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to initialize HIMS: {:?}", e)),
    }
}

#[tauri::command]
async fn create_patient(patient_data: serde_json::Value) -> Result<String, String> {
    // Convert JSON to Patient struct and create via FHIR client
    Ok("Patient created".to_string()) // Placeholder
}

#[tauri::command]
async fn parse_hl7_message(message: String) -> Result<serde_json::Value, String> {
    let parser = hims_core_sdk::standards::hl7v2::Hl7Parser::new();
    match parser.parse_message(&message) {
        Ok(parsed) => Ok(serde_json::json!({
            "message_type": parsed.message_type,
            "segments": parsed.segments.len()
        })),
        Err(e) => Err(format!("Failed to parse HL7: {:?}", e)),
    }
}

#[tauri::command]
async fn validate_compliance(country_code: String, state_code: Option<String>) -> Result<bool, String> {
    // Validate compliance based on country and state
    Ok(true) // Placeholder
}

#[tauri::command]
async fn generate_audit_report(start_date: String, end_date: String) -> Result<String, String> {
    // Generate audit report
    Ok("Audit report generated".to_string()) // Placeholder
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            hims_core: None,
        })
        .invoke_handler(tauri::generate_handler![
            initialize_hims,
            create_patient,
            parse_hl7_message,
            validate_compliance,
            generate_audit_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}