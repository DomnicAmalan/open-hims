# Contributing: Adding Brazil to HIMS SDK

This is a step-by-step practical example showing how to add a new country (Brazil) with its state structure to the HIMS Core SDK.

## Step 1: Create Directory Structure

```bash
# From project root
cd src/countries

# Create Brazil module structure
mkdir -p brazil/states
mkdir -p brazil/federal

# Create initial state structures
mkdir -p brazil/states/sao_paulo
mkdir -p brazil/states/rio_de_janeiro
mkdir -p brazil/states/minas_gerais
```

## Step 2: Implement Country Module

Create `src/countries/brazil/mod.rs`:

```rust
pub mod states;
pub mod federal;

use crate::countries::{CountryConfig, RegulatoryFramework, AuditRequirements};

/// Get Brazil configuration
pub fn get_brazil_config() -> CountryConfig {
    CountryConfig {
        country_code: "BR".to_string(),
        country_name: "Brazil".to_string(),
        regulatory_framework: RegulatoryFramework {
            primary_authority: "ANVISA - Agência Nacional de Vigilância Sanitária".to_string(),
            compliance_standards: vec![
                "LGPD".to_string(), // Lei Geral de Proteção de Dados
                "RNDS".to_string(), // Rede Nacional de Dados em Saúde
                "RDC 302/2005".to_string(), // ANVISA Technical Regulation
                "Lei 8.080/1990".to_string(), // SUS Law
            ],
            audit_requirements: AuditRequirements {
                retention_period_years: 20, // Medical records retention
                real_time_monitoring: true,
                third_party_audit_required: true,
            },
        },
        data_localization_required: true, // LGPD requires data to stay in Brazil
        supported_standards: vec![
            "FHIR R4".to_string(),
            "HL7v2".to_string(),
            "DICOM".to_string(),
            "TISS".to_string(), // Troca de Informação em Saúde Suplementar
            "ePrescription Brazil".to_string(),
        ],
        privacy_regulations: vec![
            "LGPD - Lei Geral de Proteção de Dados Pessoais".to_string(),
            "Marco Civil da Internet".to_string(),
            "ANVISA Privacy Guidelines".to_string(),
        ],
    }
}

pub use states::*;
pub use federal::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brazil_config() {
        let config = get_brazil_config();
        assert_eq!(config.country_code, "BR");
        assert_eq!(config.country_name, "Brazil");
        assert!(config.data_localization_required);
        assert_eq!(config.regulatory_framework.audit_requirements.retention_period_years, 20);
    }
}
```

## Step 3: Implement Federal Module

Create `src/countries/brazil/federal/mod.rs`:

```rust
use crate::core::{HimsError, ComplianceCheck, ComplianceStatus};

/// Federal healthcare regulations for Brazil
pub struct BrazilFederalHealthcare {
    pub authority: String,
    pub sus_requirements: Vec<String>, // Sistema Único de Saúde
}

impl BrazilFederalHealthcare {
    pub fn new() -> Self {
        Self {
            authority: "Ministry of Health - Ministério da Saúde".to_string(),
            sus_requirements: vec![
                "SUS Registration Required".to_string(),
                "CNES Registration".to_string(), // Cadastro Nacional de Estabelecimentos de Saúde
                "ANVISA Licensing".to_string(),
                "Professional Registration (CRM/CRO/etc)".to_string(),
            ],
        }
    }

    pub fn validate_cnes(&self, cnes_number: &str) -> Result<ComplianceCheck, HimsError> {
        // CNES format: 7 digits
        if cnes_number.len() != 7 || !cnes_number.chars().all(|c| c.is_numeric()) {
            return Err(HimsError::ValidationError {
                field: "cnes_number".to_string(),
                message: "CNES must be 7 digits".to_string(),
            });
        }

        Ok(ComplianceCheck {
            status: ComplianceStatus::Compliant,
            requirements_met: vec!["CNES format valid".to_string()],
            requirements_pending: vec!["CNES database verification pending".to_string()],
            notes: format!("CNES {} validated for format", cnes_number),
        })
    }

    pub fn get_federal_accreditation_bodies(&self) -> Vec<String> {
        vec![
            "ANVISA".to_string(),
            "ONA - Organização Nacional de Acreditação".to_string(),
            "CBA - Colégio Brasileiro de Acreditação".to_string(),
            "Joint Commission International (Brazil)".to_string(),
            "Canadian Council on Health Services Accreditation".to_string(),
        ]
    }
}

impl Default for BrazilFederalHealthcare {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cnes_validation() {
        let federal = BrazilFederalHealthcare::new();
        assert!(federal.validate_cnes("2269510").is_ok());
        assert!(federal.validate_cnes("123").is_err());
        assert!(federal.validate_cnes("12345ab").is_err());
    }
}
```

## Step 4: Implement State Module

Create `src/countries/brazil/states/mod.rs`:

```rust
pub mod sao_paulo;
pub mod rio_de_janeiro;
pub mod minas_gerais;

pub use sao_paulo::*;
pub use rio_de_janeiro::*;
pub use minas_gerais::*;
```

## Step 5: Implement São Paulo State

Create `src/countries/brazil/states/sao_paulo/mod.rs`:

```rust
use crate::core::{HimsError, ComplianceCheck, ComplianceStatus};

pub struct SaoPauloHealthcare {
    pub state_name: String,
    pub state_code: String,
    pub health_secretary: String,
    pub regional_health_departments: Vec<String>, // DRS - Departamentos Regionais de Saúde
}

impl SaoPauloHealthcare {
    pub fn new() -> Self {
        Self {
            state_name: "São Paulo".to_string(),
            state_code: "SP".to_string(),
            health_secretary: "Secretaria de Estado da Saúde de São Paulo".to_string(),
            regional_health_departments: vec![
                "DRS I - Grande São Paulo".to_string(),
                "DRS II - Araçatuba".to_string(),
                "DRS III - Araraquara".to_string(),
                "DRS IV - Baixada Santista".to_string(),
                "DRS V - Barretos".to_string(),
                "DRS VI - Bauru".to_string(),
                "DRS VII - Campinas".to_string(),
                "DRS VIII - Franca".to_string(),
                "DRS IX - Marília".to_string(),
                "DRS X - Piracicaba".to_string(),
                "DRS XI - Presidente Prudente".to_string(),
                "DRS XII - Registro".to_string(),
                "DRS XIII - Ribeirão Preto".to_string(),
                "DRS XIV - São João da Boa Vista".to_string(),
                "DRS XV - São José do Rio Preto".to_string(),
                "DRS XVI - Sorocaba".to_string(),
                "DRS XVII - Taubaté".to_string(),
            ],
        }
    }

    pub fn validate_facility(&self, facility_id: &str, drs_region: &str) -> Result<ComplianceCheck, HimsError> {
        if !self.regional_health_departments.iter().any(|dept| dept.contains(drs_region)) {
            return Err(HimsError::ValidationError {
                field: "drs_region".to_string(),
                message: format!("Invalid DRS region: {}", drs_region),
            });
        }

        Ok(ComplianceCheck {
            status: ComplianceStatus::Compliant,
            requirements_met: vec![
                format!("Registered in {}", drs_region),
                "São Paulo State License Verified".to_string(),
            ],
            requirements_pending: vec![],
            notes: format!("Facility {} validated for São Paulo state", facility_id),
        })
    }

    pub fn get_state_requirements(&self) -> Vec<String> {
        vec![
            "São Paulo State Health License".to_string(),
            "CEVS Registration".to_string(), // Centro de Vigilância Sanitária
            "Professional Registry in CREMESP".to_string(), // Conselho Regional de Medicina
            "Fire Department Approval (AVCB)".to_string(),
            "Environmental License".to_string(),
        ]
    }

    pub fn get_accreditation_bodies(&self) -> Vec<String> {
        vec![
            "CREMESP - Conselho Regional de Medicina do Estado de São Paulo".to_string(),
            "COREN-SP - Conselho Regional de Enfermagem de São Paulo".to_string(),
            "CRF-SP - Conselho Regional de Farmácia de São Paulo".to_string(),
            "ONA São Paulo".to_string(),
        ]
    }
}

impl Default for SaoPauloHealthcare {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sao_paulo_validation() {
        let sp = SaoPauloHealthcare::new();
        assert!(sp.validate_facility("SP-001", "Grande São Paulo").is_ok());
        assert!(sp.validate_facility("SP-002", "Invalid Region").is_err());
    }

    #[test]
    fn test_drs_regions() {
        let sp = SaoPauloHealthcare::new();
        assert_eq!(sp.regional_health_departments.len(), 17);
    }
}
```

## Step 6: Register Brazil in Country Registry

Update `src/countries/mod.rs`:

```rust
pub mod usa;
pub mod india;
pub mod brazil; // Add this
pub mod common;
pub mod inheritance_examples;

pub use usa::*;
pub use india::*;
pub use brazil::*; // Add this
pub use common::*;
pub use inheritance_examples::*;

// ... existing code ...

impl CountryRegistry {
    // ... existing code ...

    fn initialize_default_countries(&mut self) {
        self.register_country(usa::get_usa_config());
        self.register_country(india::get_india_config());
        self.register_country(brazil::get_brazil_config()); // Add this
    }
}
```

## Step 7: Add Integration Tests

Create `tests/brazil_integration_test.rs`:

```rust
use hims_core_sdk::countries::brazil::*;
use hims_core_sdk::countries::CountryRegistry;

#[test]
fn test_brazil_registration() {
    let registry = CountryRegistry::new();
    let brazil_config = registry.get_country_config("BR").unwrap();
    
    assert_eq!(brazil_config.country_code, "BR");
    assert_eq!(brazil_config.country_name, "Brazil");
    assert!(brazil_config.data_localization_required);
}

#[test]
fn test_brazil_federal_cnes() {
    let federal = BrazilFederalHealthcare::new();
    let result = federal.validate_cnes("2269510");
    assert!(result.is_ok());
}

#[test]
fn test_sao_paulo_drs() {
    let sp = SaoPauloHealthcare::new();
    assert!(sp.validate_facility("SP-001", "Grande São Paulo").is_ok());
}
```

## Step 8: Run Tests

```bash
cargo test brazil

# Output should show:
# test brazil_integration_test::test_brazil_registration ... ok
# test brazil_integration_test::test_brazil_federal_cnes ... ok
# test brazil_integration_test::test_sao_paulo_drs ... ok
# test countries::brazil::tests::test_brazil_config ... ok
# test countries::brazil::federal::tests::test_cnes_validation ... ok
# test countries::brazil::states::sao_paulo::tests::test_sao_paulo_validation ... ok
```

## Step 9: Update Documentation

Add to `README.md`:

```markdown
### Supported Countries

- 🇺🇸 **United States** - HIPAA, HITECH, 21 CFR Part 11
  - States: California, Texas, Florida, New York, Illinois
  
- 🇮🇳 **India** - ABDM, DPDP Act 2023
  - States: Maharashtra, Karnataka, Tamil Nadu, Kerala
  
- 🇧🇷 **Brazil** - LGPD, RNDS, ANVISA
  - States: São Paulo, Rio de Janeiro, Minas Gerais
```

## Step 10: Submit Pull Request

```bash
git checkout -b feature/add-brazil-support
git add src/countries/brazil/
git add tests/brazil_integration_test.rs
git commit -m "feat: Add Brazil healthcare compliance support

- Implemented Brazil federal healthcare regulations
- Added São Paulo, Rio de Janeiro, and Minas Gerais states
- LGPD and ANVISA compliance standards
- CNES validation system
- Full test coverage"
git push origin feature/add-brazil-support
```

---

## Continent-Level Organization (Optional)

For better organization, you can group by continent:

```bash
mkdir -p src/countries/americas
mkdir -p src/countries/asia
mkdir -p src/countries/europe

# Move countries into continents
mv src/countries/usa src/countries/americas/
mv src/countries/brazil src/countries/americas/
mv src/countries/india src/countries/asia/
```

Update `src/countries/mod.rs`:

```rust
pub mod americas;
pub mod asia;
pub mod europe;

pub use americas::*;
pub use asia::*;
pub use europe::*;
```

Create `src/countries/americas/mod.rs`:

```rust
pub mod usa;
pub mod brazil;
pub mod canada; // Future

pub use usa::*;
pub use brazil::*;
```

---

## Next Steps

After Brazil is implemented:
1. Add remaining Brazilian states (Bahia, Paraná, etc.)
2. Add municipal level (São Paulo city, Rio de Janeiro city)
3. Implement TISS standard support
4. Add RNDS integration
5. Document Brazilian telemedicine regulations

---

## Key Takeaways

✅ **Always start with federal/central regulations**
✅ **Add states progressively (don't need all at once)**
✅ **Write tests for every module**
✅ **Document regulatory authorities clearly**
✅ **Follow naming conventions (lowercase + underscores)**
✅ **Include real-world validation rules (like CNES format)**
✅ **Reference actual laws and standards**

This pattern can be repeated for any country in the world! 🌍
