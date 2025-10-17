use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::types::*;

/// User model for system authentication and authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub active: bool,
    pub name: HumanName,
    pub practitioner_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password_hash: String,
        role: UserRole,
        name: HumanName,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            role,
            active: true,
            name,
            practitioner_id: None,
            organization_id: None,
            created_at: now,
            updated_at: now,
            last_login: None,
        }
    }

    pub fn update_last_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.updated_at = Utc::now();
    }

    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
        self.updated_at = Utc::now();
    }

    pub fn update_profile(&mut self, name: HumanName, email: String) {
        self.name = name;
        self.email = email;
        self.updated_at = Utc::now();
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }

    pub fn is_healthcare_provider(&self) -> bool {
        matches!(self.role, UserRole::Doctor | UserRole::Nurse | UserRole::Technician)
    }

    pub fn can_access_patient_data(&self) -> bool {
        matches!(
            self.role,
            UserRole::Admin | UserRole::Doctor | UserRole::Nurse | UserRole::Receptionist
        )
    }
}