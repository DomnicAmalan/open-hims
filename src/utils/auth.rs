// src/utils/auth.rs
//! Authentication and session management utilities
//! 
//! This module provides utilities for extracting user information from HTTP headers,
//! managing session context, and integrating with the authorization system.

use axum::http::HeaderMap;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc, Timelike};
use std::net::IpAddr;
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};

use crate::modules::authorization::{
    RequestContext, ClinicalContext, EmergencyContext, LocationContext, 
    UrgencyLevel, EmergencyType
};

/// Extract user ID from HTTP headers
/// 
/// This function extracts the user ID from various sources:
/// - Authorization header (JWT token)
/// - X-User-ID header (for development/testing)
/// - Session cookies
pub fn extract_user_from_headers(headers: &HeaderMap) -> Result<Uuid> {
    // Try Authorization header first (JWT)
    if let Some(auth_header) = headers.get("authorization") {
        let auth_str = auth_header.to_str()
            .map_err(|_| anyhow!("Invalid authorization header encoding"))?;
        
        if auth_str.starts_with("Bearer ") {
            let token = &auth_str[7..]; // Remove "Bearer " prefix
            return extract_user_from_jwt(token);
        }
    }
    
    // Try X-User-ID header (for development/testing)
    if let Some(user_header) = headers.get("x-user-id") {
        let user_str = user_header.to_str()
            .map_err(|_| anyhow!("Invalid user ID header encoding"))?;
        
        return Uuid::from_str(user_str)
            .map_err(|_| anyhow!("Invalid user ID format"));
    }
    
    // Try session cookies
    if let Some(cookie_header) = headers.get("cookie") {
        let cookie_str = cookie_header.to_str()
            .map_err(|_| anyhow!("Invalid cookie header encoding"))?;
        
        if let Some(user_id) = extract_user_from_cookies(cookie_str)? {
            return Ok(user_id);
        }
    }
    
    Err(anyhow!("No valid authentication found in headers"))
}

/// Extract user ID from JWT token
fn extract_user_from_jwt(token: &str) -> Result<Uuid> {
    // This is a simplified implementation
    // In a real application, you would:
    // 1. Validate the JWT signature
    // 2. Check expiration
    // 3. Extract claims
    
    // For now, we'll decode the payload (base64 encoded JSON)
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(anyhow!("Invalid JWT format"));
    }
    
    // Decode the payload (second part)
    let payload = parts[1];
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(payload)
        .map_err(|_| anyhow!("Failed to decode JWT payload"))?;
    
    let payload_str = String::from_utf8(decoded)
        .map_err(|_| anyhow!("Invalid UTF-8 in JWT payload"))?;
    
    // Parse JSON to extract user ID
    let json: serde_json::Value = serde_json::from_str(&payload_str)
        .map_err(|_| anyhow!("Invalid JSON in JWT payload"))?;
    
    let user_id_str = json.get("sub")
        .or_else(|| json.get("user_id"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("No user ID found in JWT"))?;
    
    Uuid::from_str(user_id_str)
        .map_err(|_| anyhow!("Invalid user ID format in JWT"))
}

/// Extract user ID from cookies
fn extract_user_from_cookies(cookie_str: &str) -> Result<Option<Uuid>> {
    for cookie in cookie_str.split(';') {
        let cookie = cookie.trim();
        if let Some(session_id) = cookie.strip_prefix("session_id=") {
            // Look up session in database/cache
            if let Some(user_id) = lookup_session_user(session_id)? {
                return Ok(Some(user_id));
            }
        }
    }
    Ok(None)
}

/// Look up user ID from session ID
/// 
/// This is a placeholder implementation that should be replaced with
/// actual session storage lookup (Redis, database, etc.)
fn lookup_session_user(_session_id: &str) -> Result<Option<Uuid>> {
    // In a real implementation, this would:
    // 1. Query the session store (Redis, database, etc.)
    // 2. Validate session expiration
    // 3. Return the associated user ID
    
    // For now, return None to indicate no session found
    Ok(None)
}

/// Get comprehensive session context for a user
/// 
/// This function builds the RequestContext needed for authorization decisions
/// by gathering information about the user's current session, location, and context.
pub async fn get_user_session_context(
    user_id: Uuid,
    headers: &HeaderMap,
) -> Result<RequestContext> {
    let timestamp = Utc::now();
    
    // Extract IP address
    let ip_address = extract_ip_address(headers);
    
    // Extract user agent
    let user_agent = headers.get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    // Get session ID
    let session_id = extract_session_id(headers);
    
    // Check if this is an emergency context
    let emergency_context = check_emergency_context(headers)?;
    
    // Build clinical context if available
    let clinical_context = build_clinical_context(user_id, headers).await?;
    
    // Build location context
    let location_context = build_location_context(user_id, headers).await?;
    
    Ok(RequestContext {
        session_id,
        ip_address: ip_address.map(|ip| ip.to_string()),
        user_agent,
        timestamp,
        location: location_context,
        clinical: clinical_context,
        emergency: emergency_context,
        audit_trail: vec![format!("Context built for user {}", user_id)],
        headers: extract_additional_metadata(headers),
        endpoint: headers.get("x-endpoint").and_then(|h| h.to_str().ok()).map(|s| s.to_string()),
        method: headers.get("x-method").and_then(|h| h.to_str().ok()).map(|s| s.to_string()),
    })
}

/// Extract IP address from headers
fn extract_ip_address(headers: &HeaderMap) -> Option<IpAddr> {
    // Try various headers in order of preference
    let ip_headers = [
        "x-forwarded-for",
        "x-real-ip",
        "x-client-ip",
        "cf-connecting-ip", // Cloudflare
    ];
    
    for header_name in &ip_headers {
        if let Some(header_value) = headers.get(*header_name) {
            if let Ok(ip_str) = header_value.to_str() {
                // X-Forwarded-For can have multiple IPs, take the first one
                let ip_str = ip_str.split(',').next().unwrap_or(ip_str).trim();
                if let Ok(ip) = ip_str.parse::<IpAddr>() {
                    return Some(ip);
                }
            }
        }
    }
    
    None
}

/// Extract session ID from headers
fn extract_session_id(headers: &HeaderMap) -> Option<String> {
    // Try to get session ID from cookies
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some(session_id) = cookie.strip_prefix("session_id=") {
                    return Some(session_id.to_string());
                }
            }
        }
    }
    
    // Try X-Session-ID header
    headers.get("x-session-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}

/// Check if MFA is verified for this session
fn check_mfa_status(headers: &HeaderMap) -> Result<bool> {
    // Check for MFA verification header
    if let Some(mfa_header) = headers.get("x-mfa-verified") {
        let mfa_str = mfa_header.to_str()
            .map_err(|_| anyhow!("Invalid MFA header encoding"))?;
        return Ok(mfa_str.to_lowercase() == "true");
    }
    
    // In a real implementation, this would check the session store
    Ok(false)
}

/// Check for emergency context in headers
fn check_emergency_context(headers: &HeaderMap) -> Result<Option<EmergencyContext>> {
    if let Some(emergency_header) = headers.get("x-emergency-access") {
        let emergency_str = emergency_header.to_str()
            .map_err(|_| anyhow!("Invalid emergency header encoding"))?;
        
        if emergency_str.to_lowercase() == "true" {
            // Extract emergency details from other headers
            let urgency_level = headers.get("x-emergency-urgency")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("medium")
                .to_string();
            
            let justification = headers.get("x-emergency-justification")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());
            
            return Ok(Some(EmergencyContext {
                is_emergency: true,
                emergency_type: Some(EmergencyType::BreakGlass),
                declared_by: None, // Would be set based on authentication context
                declared_at: Some(Utc::now()),
                justification,
                approval_required: false, // Set based on policy
                approved_by: None,
                approved_at: None,
                expires_at: Some(Utc::now() + chrono::Duration::hours(1)), // 1 hour emergency access
            }));
        }
    }
    
    Ok(None)
}

/// Build clinical context from user session
async fn build_clinical_context(_user_id: Uuid, _headers: &HeaderMap) -> Result<Option<ClinicalContext>> {
    // This would typically query the database for:
    // - Current patient assignments
    // - Active care teams
    // - Department schedules
    // - Compliance requirements
    
    // For now, return a basic context
    Ok(Some(ClinicalContext {
        patient_id: None,
        encounter_id: None,
        care_plan_id: None,
        clinical_status: None,
        urgency_level: UrgencyLevel::Routine,
        care_team_members: vec![],
        active_protocols: vec![],
        specialty: None,
        workflow_context: None,
    }))
}

/// Build location context from user session
async fn build_location_context(_user_id: Uuid, _headers: &HeaderMap) -> Result<Option<LocationContext>> {
    // This would typically query the user's current location from:
    // - Badge scan data
    // - Workstation assignments
    // - Department schedules
    
    // For now, return None
    Ok(None)
}

/// Get user's current department and location
async fn get_user_department_and_location(_user_id: Uuid) -> Result<(Option<Uuid>, Option<Uuid>)> {
    // This would typically query the user's profile or current session
    // For now, return None for both
    Ok((None, None))
}

/// Get current shift ID for a user
async fn get_current_shift_id(_user_id: Uuid) -> Result<Option<Uuid>> {
    // This would query the scheduling system
    Ok(None)
}

/// Calculate risk score for the session
async fn calculate_risk_score(
    _user_id: Uuid,
    ip_address: &Option<IpAddr>,
    user_agent: &Option<String>,
    clinical_context: &Option<ClinicalContext>,
) -> Result<f32> {
    let mut risk_score: f32 = 0.0;
    
    // Check for unusual IP address
    if let Some(ip) = ip_address {
        if is_unusual_ip_for_user(_user_id, ip).await? {
            risk_score += 0.3;
        }
    }
    
    // Check for unusual user agent
    if let Some(ua) = user_agent {
        if is_unusual_user_agent_for_user(_user_id, ua).await? {
            risk_score += 0.2;
        }
    }
    
    // Check clinical context risks
    if let Some(clinical) = clinical_context {
        if clinical.urgency_level >= UrgencyLevel::Emergency {
            risk_score += 0.1;
        }
    }
    
    // Check time-based risks (off-hours access)
    let now = Utc::now();
    let hour = now.hour();
    if hour < 6 || hour > 22 {
        risk_score += 0.1;
    }
    
    // Ensure risk score is between 0.0 and 1.0
    Ok(risk_score.min(1.0))
}

/// Check if IP address is unusual for this user
async fn is_unusual_ip_for_user(_user_id: Uuid, _ip: &IpAddr) -> Result<bool> {
    // This would check against the user's historical IP addresses
    Ok(false)
}

/// Check if user agent is unusual for this user
async fn is_unusual_user_agent_for_user(_user_id: Uuid, _user_agent: &str) -> Result<bool> {
    // This would check against the user's historical user agents
    Ok(false)
}

/// Extract additional metadata from headers
fn extract_additional_metadata(headers: &HeaderMap) -> std::collections::HashMap<String, String> {
    let mut metadata = std::collections::HashMap::new();
    
    // Extract various headers that might be useful for authorization
    if let Some(referer) = headers.get("referer").and_then(|h| h.to_str().ok()) {
        metadata.insert("referer".to_string(), referer.to_string());
    }
    
    if let Some(accept_language) = headers.get("accept-language").and_then(|h| h.to_str().ok()) {
        metadata.insert("accept_language".to_string(), accept_language.to_string());
    }
    
    if let Some(timezone) = headers.get("x-timezone").and_then(|h| h.to_str().ok()) {
        metadata.insert("timezone".to_string(), timezone.to_string());
    }
    
    metadata
}