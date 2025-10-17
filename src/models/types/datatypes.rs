use serde::{Deserialize, Serialize};

/// FHIR HumanName data type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanName {
    pub use_type: Option<NameUse>,
    pub text: Option<String>,
    pub family: Option<String>,
    pub given: Vec<String>,
    pub prefix: Vec<String>,
    pub suffix: Vec<String>,
}

/// FHIR ContactPoint data type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPoint {
    pub system: ContactPointSystem,
    pub value: String,
    pub use_type: Option<ContactPointUse>,
    pub rank: Option<i32>,
}

/// FHIR Address data type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub use_type: Option<AddressUse>,
    pub address_type: Option<AddressType>,
    pub text: Option<String>,
    pub line: Vec<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

// Enums for data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NameUse {
    #[serde(rename = "usual")]
    Usual,
    #[serde(rename = "official")]
    Official,
    #[serde(rename = "temp")]
    Temp,
    #[serde(rename = "nickname")]
    Nickname,
    #[serde(rename = "anonymous")]
    Anonymous,
    #[serde(rename = "old")]
    Old,
    #[serde(rename = "maiden")]
    Maiden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactPointSystem {
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "fax")]
    Fax,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "pager")]
    Pager,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactPointUse {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "temp")]
    Temp,
    #[serde(rename = "old")]
    Old,
    #[serde(rename = "mobile")]
    Mobile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddressUse {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "temp")]
    Temp,
    #[serde(rename = "old")]
    Old,
    #[serde(rename = "billing")]
    Billing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddressType {
    #[serde(rename = "postal")]
    Postal,
    #[serde(rename = "physical")]
    Physical,
    #[serde(rename = "both")]
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "unknown")]
    Unknown,
}