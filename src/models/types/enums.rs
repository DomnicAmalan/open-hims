use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Appointment status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum AppointmentStatus {
    #[serde(rename = "proposed")]
    #[default]
    Proposed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "booked")]
    Booked,
    #[serde(rename = "arrived")]
    Arrived,
    #[serde(rename = "fulfilled")]
    Fulfilled,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "noshow")]
    NoShow,
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    #[serde(rename = "checked-in")]
    CheckedIn,
    #[serde(rename = "waitlist")]
    Waitlist,
}

/// Participant requirement level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRequired {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "information-only")]
    InformationOnly,
}

/// Participation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipationStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "tentative")]
    Tentative,
    #[serde(rename = "needs-action")]
    NeedsAction,
}

/// Medical record type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MedicalRecordType {
    #[serde(rename = "progress-note")]
    ProgressNote,
    #[serde(rename = "discharge-summary")]
    DischargeSummary,
    #[serde(rename = "operative-note")]
    OperativeNote,
    #[serde(rename = "consultation")]
    Consultation,
    #[serde(rename = "diagnostic-report")]
    #[default]
    DiagnosticReport,
}

/// Document status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DocumentStatus {
    #[serde(rename = "preliminary")]
    #[default]
    Preliminary,
    #[serde(rename = "final")]
    Final,
    #[serde(rename = "amended")]
    Amended,
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}

impl Display for AppointmentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let status = match self {
            AppointmentStatus::Proposed => "proposed",
            AppointmentStatus::Pending => "pending",
            AppointmentStatus::Booked => "booked",
            AppointmentStatus::Arrived => "arrived",
            AppointmentStatus::Fulfilled => "fulfilled",
            AppointmentStatus::Cancelled => "cancelled",
            AppointmentStatus::NoShow => "noshow",
            AppointmentStatus::EnteredInError => "entered-in-error",
            AppointmentStatus::CheckedIn => "checked-in",
            AppointmentStatus::Waitlist => "waitlist",
        };
        write!(f, "{}", status)
    }
}

impl FromStr for AppointmentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "proposed" => Ok(AppointmentStatus::Proposed),
            "pending" => Ok(AppointmentStatus::Pending),
            "booked" => Ok(AppointmentStatus::Booked),
            "arrived" => Ok(AppointmentStatus::Arrived),
            "fulfilled" => Ok(AppointmentStatus::Fulfilled),
            "cancelled" => Ok(AppointmentStatus::Cancelled),
            "noshow" => Ok(AppointmentStatus::NoShow),
            "entered-in-error" => Ok(AppointmentStatus::EnteredInError),
            "checked-in" => Ok(AppointmentStatus::CheckedIn),
            "waitlist" => Ok(AppointmentStatus::Waitlist),
            _ => Err(format!("Unknown appointment status: {}", s)),
        }
    }
}

impl MedicalRecordType {
    pub fn from_string(s: &str) -> Self {
        match s {
            "progress-note" => MedicalRecordType::ProgressNote,
            "discharge-summary" => MedicalRecordType::DischargeSummary,
            "operative-note" => MedicalRecordType::OperativeNote,
            "consultation" => MedicalRecordType::Consultation,
            "diagnostic-report" => MedicalRecordType::DiagnosticReport,
            _ => MedicalRecordType::default(),
        }
    }
}

impl DocumentStatus {
    pub fn from_string(s: &str) -> Self {
        match s {
            "preliminary" => DocumentStatus::Preliminary,
            "final" => DocumentStatus::Final,
            "amended" => DocumentStatus::Amended,
            "entered-in-error" => DocumentStatus::EnteredInError,
            _ => DocumentStatus::default(),
        }
    }
}