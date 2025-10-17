use crate::core::HimsError;
use crate::standards::fhir::models::*;
use reqwest::Client;

/// FHIR client for communicating with FHIR servers
pub struct FhirClient {
    base_url: String,
    client: Client,
    auth_token: Option<String>,
}

impl FhirClient {
    pub fn new(base_url: String, auth_token: Option<String>) -> Self {
        Self {
            base_url,
            client: Client::new(),
            auth_token,
        }
    }

    /// Create a new patient resource
    pub async fn create_patient(&self, patient: &Patient) -> Result<Patient, HimsError> {
        patient.validate()?;
        
        let url = format!("{}/Patient", self.base_url);
        let mut request = self.client.post(&url).json(patient);
        
        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }
        
        let response = request.send().await.map_err(|e| HimsError::NetworkError {
            message: e.to_string(),
        })?;
        
        if !response.status().is_success() {
            return Err(HimsError::NetworkError {
                message: format!("Failed to create patient: {}", response.status()),
            });
        }
        
        let created_patient: Patient = response.json().await.map_err(|e| HimsError::InternalError {
            message: e.to_string(),
        })?;
        
        Ok(created_patient)
    }

    /// Get a patient by ID
    pub async fn get_patient(&self, id: &str) -> Result<Patient, HimsError> {
        let url = format!("{}/Patient/{}", self.base_url, id);
        let mut request = self.client.get(&url);
        
        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }
        
        let response = request.send().await.map_err(|e| HimsError::NetworkError {
            message: e.to_string(),
        })?;
        
        if response.status() == 404 {
            return Err(HimsError::ValidationError {
                message: format!("Patient with ID {} not found", id),
            });
        }
        
        if !response.status().is_success() {
            return Err(HimsError::NetworkError {
                message: format!("Failed to get patient: {}", response.status()),
            });
        }
        
        let patient: Patient = response.json().await.map_err(|e| HimsError::InternalError {
            message: e.to_string(),
        })?;
        
        Ok(patient)
    }

    /// Search for patients
    pub async fn search_patients(&self, query: &str) -> Result<Bundle, HimsError> {
        let url = format!("{}/Patient?{}", self.base_url, query);
        let mut request = self.client.get(&url);
        
        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }
        
        let response = request.send().await.map_err(|e| HimsError::NetworkError {
            message: e.to_string(),
        })?;
        
        if !response.status().is_success() {
            return Err(HimsError::NetworkError {
                message: format!("Failed to search patients: {}", response.status()),
            });
        }
        
        let bundle: Bundle = response.json().await.map_err(|e| HimsError::InternalError {
            message: e.to_string(),
        })?;
        
        Ok(bundle)
    }

    /// Create an observation
    pub async fn create_observation(&self, observation: &Observation) -> Result<Observation, HimsError> {
        let url = format!("{}/Observation", self.base_url);
        let mut request = self.client.post(&url).json(observation);
        
        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }
        
        let response = request.send().await.map_err(|e| HimsError::NetworkError {
            message: e.to_string(),
        })?;
        
        if !response.status().is_success() {
            return Err(HimsError::NetworkError {
                message: format!("Failed to create observation: {}", response.status()),
            });
        }
        
        let created_observation: Observation = response.json().await.map_err(|e| HimsError::InternalError {
            message: e.to_string(),
        })?;
        
        Ok(created_observation)
    }
}