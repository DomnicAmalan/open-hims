# HIMS Core SDK - Extensibility Guide

## 🌍 Hierarchical Healthcare System Architecture

This SDK is designed to be **infinitely extensible** at every level of healthcare governance. Anyone can contribute new regions, countries, states, districts, or local governing bodies.

---

## 📐 Hierarchy Levels

The SDK supports multiple levels of healthcare governance:

```
Continent
  ├── Country
  │     ├── State/Province/Region
  │     │     ├── District/County
  │     │     │     ├── Panchayat/Municipality/City
  │     │     │     │     ├── Healthcare Facility
  │     │     │     │     │     ├── Department
```

Each level can have:
- **Regulatory requirements**
- **Compliance standards**
- **Accreditation bodies**
- **Data privacy rules**
- **Healthcare standards** (FHIR, HL7v2, DICOM, etc.)

---

## 🚀 Quick Start: Adding New Regions

### 1. Adding a New Country

**Step 1**: Create country module structure
```bash
mkdir -p src/countries/brazil
mkdir -p src/countries/brazil/states
```

**Step 2**: Create `src/countries/brazil/mod.rs`
```rust
pub mod states;
pub mod federal;

use crate::countries::{CountryConfig, RegulatoryFramework, AuditRequirements};

pub fn get_brazil_config() -> CountryConfig {
    CountryConfig {
        country_code: "BR".to_string(),
        country_name: "Brazil".to_string(),
        regulatory_framework: RegulatoryFramework {
            primary_authority: "ANVISA - Agência Nacional de Vigilância Sanitária".to_string(),
            compliance_standards: vec![
                "LGPD".to_string(), // Lei Geral de Proteção de Dados
                "RNDS".to_string(), // Rede Nacional de Dados em Saúde
            ],
            audit_requirements: AuditRequirements {
                retention_period_years: 20,
                real_time_monitoring: true,
                third_party_audit_required: true,
            },
        },
        data_localization_required: true,
        supported_standards: vec![
            "FHIR R4".to_string(),
            "HL7v2".to_string(),
            "TISS".to_string(), // Troca de Informação em Saúde Suplementar
        ],
        privacy_regulations: vec![
            "LGPD".to_string(),
            "HIPAA Equivalent".to_string(),
        ],
    }
}

pub use states::*;
pub use federal::*;
```

**Step 3**: Register in `src/countries/mod.rs`
```rust
pub mod brazil;
pub use brazil::*;

// In initialize_default_countries()
self.register_country(brazil::get_brazil_config());
```

---

### 2. Adding a New State/Province

**Step 1**: Create state module
```bash
mkdir -p src/countries/usa/states/texas
touch src/countries/usa/states/texas/mod.rs
```

**Step 2**: Implement `src/countries/usa/states/texas/mod.rs`
```rust
use crate::core::{HimsError, ComplianceCheck, ComplianceStatus};

pub struct TexasHealthcare {
    pub name: String,
    pub regulations: Vec<String>,
}

impl TexasHealthcare {
    pub fn new() -> Self {
        Self {
            name: "Texas".to_string(),
            regulations: vec![
                "Texas Health and Safety Code".to_string(),
                "HIPAA".to_string(),
                "Texas Medical Records Privacy Act".to_string(),
            ],
        }
    }

    pub fn validate_facility(&self, facility_id: &str) -> Result<ComplianceCheck, HimsError> {
        // Implement Texas-specific facility validation
        Ok(ComplianceCheck {
            status: ComplianceStatus::Compliant,
            requirements_met: vec!["Texas License Verified".to_string()],
            requirements_pending: vec![],
            notes: format!("Facility {} validated for Texas", facility_id),
        })
    }

    pub fn get_accreditation_bodies(&self) -> Vec<String> {
        vec![
            "Texas Medical Board".to_string(),
            "Texas Department of State Health Services".to_string(),
            "Joint Commission".to_string(),
        ]
    }
}
```

**Step 3**: Register in `src/countries/usa/states/mod.rs`
```rust
pub mod texas;
pub use texas::*;
```

---

### 3. Adding District/County Level

**Step 1**: Create district structure
```bash
mkdir -p src/countries/india/states/maharashtra/districts
mkdir -p src/countries/india/states/maharashtra/districts/pune
```

**Step 2**: Implement `src/countries/india/states/maharashtra/districts/pune/mod.rs`
```rust
use crate::core::{HimsError, ComplianceCheck};

pub struct PuneDistrictHealthcare {
    pub district_name: String,
    pub district_code: String,
    pub health_officer: String,
    pub primary_health_centers: Vec<String>,
}

impl PuneDistrictHealthcare {
    pub fn new() -> Self {
        Self {
            district_name: "Pune".to_string(),
            district_code: "MH-13".to_string(),
            health_officer: "District Health Officer, Pune".to_string(),
            primary_health_centers: vec![
                "PHC Hadapsar".to_string(),
                "PHC Kothrud".to_string(),
                "PHC Aundh".to_string(),
            ],
        }
    }

    pub fn validate_phc(&self, phc_name: &str) -> Result<bool, HimsError> {
        Ok(self.primary_health_centers.contains(&phc_name.to_string()))
    }

    pub fn get_district_requirements(&self) -> Vec<String> {
        vec![
            "Maharashtra State Health License".to_string(),
            "Pune District Registration".to_string(),
            "ABDM Health ID Integration".to_string(),
        ]
    }
}
```

**Step 3**: Register in parent state module
```rust
// src/countries/india/states/maharashtra/mod.rs
pub mod districts;
pub use districts::*;

// src/countries/india/states/maharashtra/districts/mod.rs
pub mod pune;
pub use pune::*;
```

---

### 4. Adding Panchayat/Municipality Level

**Step 1**: Create local governance structure
```bash
mkdir -p src/countries/india/states/kerala/districts/ernakulam/panchayats
touch src/countries/india/states/kerala/districts/ernakulam/panchayats/vypin.rs
```

**Step 2**: Implement panchayat module
```rust
use crate::core::HimsError;

pub struct VypinPanchayat {
    pub name: String,
    pub panchayat_code: String,
    pub health_centers: Vec<HealthCenter>,
}

pub struct HealthCenter {
    pub name: String,
    pub center_type: String, // PHC, CHC, Sub-Center
    pub license_number: String,
}

impl VypinPanchayat {
    pub fn new() -> Self {
        Self {
            name: "Vypin Panchayat".to_string(),
            panchayat_code: "KL-07-VYP".to_string(),
            health_centers: vec![
                HealthCenter {
                    name: "Vypin PHC".to_string(),
                    center_type: "Primary Health Center".to_string(),
                    license_number: "KL-PHC-001".to_string(),
                },
            ],
        }
    }

    pub fn validate_health_center(&self, license: &str) -> Result<bool, HimsError> {
        Ok(self.health_centers.iter().any(|hc| hc.license_number == license))
    }

    pub fn get_panchayat_regulations(&self) -> Vec<String> {
        vec![
            "Kerala Panchayat Health Rules".to_string(),
            "Kerala State Health Policy".to_string(),
            "Swasthya Kerala Mission Guidelines".to_string(),
        ]
    }
}
```

---

## 📋 File Structure Template

For any new region, follow this structure:

```
src/countries/
├── [continent]/                    # Optional: asia/, europe/, americas/
│   └── mod.rs
├── [country]/
│   ├── mod.rs                      # Country config & exports
│   ├── federal/                    # Central/Federal regulations
│   │   └── mod.rs
│   └── states/                     # State/Province level
│       ├── mod.rs                  # State exports
│       ├── [state_name]/
│       │   ├── mod.rs              # State config
│       │   ├── accreditation/      # State accreditation bodies
│       │   │   └── mod.rs
│       │   └── districts/          # District/County level
│       │       ├── mod.rs
│       │       └── [district_name]/
│       │           ├── mod.rs
│       │           ├── panchayats/ # Local governance (optional)
│       │           │   └── mod.rs
│       │           └── municipalities/
│       │               └── mod.rs
```

---

## 🎯 Best Practices

### 1. **Naming Conventions**
- Use lowercase with underscores: `new_york`, `maharashtra`, `sao_paulo`
- Country codes: ISO 3166-1 alpha-2 (`US`, `IN`, `BR`, `UK`)
- State codes: Country-specific standards (e.g., `CA` for California, `MH` for Maharashtra)

### 2. **Module Documentation**
Always document:
- Regulatory authority
- Compliance standards
- Data retention requirements
- Privacy regulations
- Accreditation bodies

### 3. **Error Handling**
Use `HimsError` for all errors:
```rust
return Err(HimsError::ConfigurationError {
    message: "Invalid facility license".to_string(),
});
```

### 4. **Testing**
Create tests for each level:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_validation() {
        let state = TexasHealthcare::new();
        assert!(state.validate_facility("TX-001").is_ok());
    }
}
```

---

## 🌐 Real-World Examples

### Example 1: India's Three-Tier System
```
India (Country)
├── Maharashtra (State)
│   ├── Pune (District)
│   │   ├── Pune Municipal Corporation (Municipal)
│   │   └── Baramati Panchayat (Panchayat)
│   └── Mumbai (District)
│       └── BMC (Municipal Corporation)
```

### Example 2: USA's Federal-State-County System
```
USA (Country)
├── California (State)
│   ├── Los Angeles County (County)
│   │   ├── LA City (Municipal)
│   │   └── Santa Monica (Municipal)
│   └── San Francisco County (County)
│       └── San Francisco City (City-County)
```

### Example 3: UK's Devolved System
```
UK (Country)
├── England (Nation)
│   └── NHS England Regions
│       └── South West
├── Scotland (Nation)
│   └── NHS Scotland Health Boards
│       └── NHS Lothian
├── Wales (Nation)
│   └── NHS Wales Health Boards
└── Northern Ireland (Nation)
    └── Health & Social Care Trusts
```

---

## 🔧 Configuration Options

Each level can define:

```rust
pub struct RegionalHealthConfig {
    // Identity
    pub region_code: String,
    pub region_name: String,
    pub parent_region: Option<String>,
    
    // Governance
    pub regulatory_authority: String,
    pub compliance_standards: Vec<String>,
    pub accreditation_bodies: Vec<String>,
    
    // Data & Privacy
    pub data_retention_years: u32,
    pub data_localization_required: bool,
    pub privacy_regulations: Vec<String>,
    
    // Healthcare Standards
    pub supported_fhir_versions: Vec<String>,
    pub supported_hl7_versions: Vec<String>,
    pub dicom_required: bool,
    pub local_standards: Vec<String>,
    
    // Audit & Monitoring
    pub real_time_monitoring: bool,
    pub audit_frequency_days: u32,
    pub third_party_audit_required: bool,
}
```

---

## 🤝 Contributing New Regions

### Pull Request Checklist:
- [ ] Created module structure following hierarchy
- [ ] Implemented configuration with all required fields
- [ ] Added regulatory authority information
- [ ] Documented compliance standards
- [ ] Added unit tests
- [ ] Updated parent module exports
- [ ] Added to country registry (if new country)
- [ ] Documented in this guide with example

### Contribution Process:
1. Fork the repository
2. Create feature branch: `git checkout -b feature/add-region-name`
3. Implement the region module
4. Add tests and documentation
5. Submit pull request with detailed description

---

## 📚 Additional Resources

- **FHIR Resources**: https://www.hl7.org/fhir/
- **HIPAA Compliance**: https://www.hhs.gov/hipaa
- **GDPR**: https://gdpr.eu/
- **ABDM (India)**: https://abdm.gov.in/
- **NHS Standards (UK)**: https://digital.nhs.uk/

---

## 💡 Extensibility Use Cases

1. **Multi-National Healthcare Chains**: Support facilities across different countries with unified compliance tracking
2. **Telemedicine Platforms**: Validate cross-border healthcare delivery compliance
3. **Health Insurance**: Track multi-regional policy requirements
4. **Medical Tourism**: Manage healthcare standards across patient origin and destination
5. **Research Networks**: Ensure data sharing compliance across jurisdictions
6. **Supply Chain**: Track medical device/pharma regulations by region
7. **Government Health Departments**: Manage hierarchical governance structures

---

**Questions?** Open an issue on GitHub or contact the maintainers.

**Ready to contribute?** Start by picking a region from the TODO list in `src/countries/mod.rs` or `src/countries/*/states/mod.rs`!
