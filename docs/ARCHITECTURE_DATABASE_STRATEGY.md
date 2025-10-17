# HIMS Core SDK - Database Architecture Strategy

## 🤔 The Question: Code vs Database?

**Should healthcare compliance configurations be stored in:**
- **Code** (Rust modules, compile-time)
- **Database** (PostgreSQL, runtime)
- **Hybrid** (Both)

---

## 🎯 Recommended Approach: **Hybrid Architecture**

### Core Principle: **Configuration in Code, State in Database**

```
┌─────────────────────────────────────────────────────────────┐
│                    HIMS Core SDK                             │
├─────────────────────────────────────────────────────────────┤
│  Rust Modules (Compile-time)                                │
│  ├── Regulatory frameworks (HIPAA, LGPD, ABDM)             │
│  ├── Compliance standards (immutable rules)                 │
│  ├── Validation logic (unchanging algorithms)               │
│  └── Healthcare standards (FHIR, HL7v2, DICOM)             │
├─────────────────────────────────────────────────────────────┤
│  PostgreSQL Database (Runtime)                               │
│  ├── Facility registrations                                 │
│  ├── Accreditation status (changes over time)               │
│  ├── License expiration dates                               │
│  ├── Audit logs                                             │
│  ├── User preferences                                       │
│  └── Dynamic configuration overrides                        │
└─────────────────────────────────────────────────────────────┘
```

---

## 📊 What Goes Where?

### ✅ **In Code (Rust Modules)** - Static, Regulatory Knowledge

| Data Type | Example | Why Code? |
|-----------|---------|-----------|
| **Regulatory Standards** | HIPAA requirements, LGPD articles | Legally defined, rarely change, version controlled |
| **Validation Rules** | CNES format (7 digits), NPI format (10 digits) | Business logic, testable, type-safe |
| **Healthcare Standards** | FHIR R4 resources, HL7v2 segments | Standardized specs, need compile-time guarantees |
| **Compliance Checklists** | JCI accreditation criteria, NABH standards | Official documents, audit trail needed |
| **Geographical Hierarchy** | Country→State→District→Panchayat structure | Organizational model, rarely changes |

**Benefits:**
- ✅ Version control (Git history)
- ✅ Code review for compliance changes
- ✅ Compile-time type safety
- ✅ No database dependency for core logic
- ✅ Faster execution (no DB queries)
- ✅ Can be audited and certified

### ✅ **In Database (PostgreSQL)** - Dynamic, Operational Data

| Data Type | Example | Why Database? |
|-----------|---------|---------------|
| **Facility Information** | Hospital name, address, contact | Changes frequently, user-managed |
| **License Status** | Valid from/until dates, renewal status | Time-sensitive, needs updates |
| **Accreditation Records** | JCI certified 2024-2027 | Expires, needs tracking |
| **Audit Logs** | Who checked what compliance when | Legal requirement, append-only |
| **User Configurations** | Preferred states, notification settings | Per-user, per-tenant |
| **Custom Extensions** | Organization-specific rules | Client customizations |

**Benefits:**
- ✅ Real-time updates without redeployment
- ✅ Multi-tenant isolation
- ✅ Query flexibility
- ✅ Scalability for large datasets
- ✅ Backup and disaster recovery
- ✅ Integration with existing systems

---

## 🏗️ Proposed Database Schema

### Core Tables

```sql
-- Countries (seeded from Rust code on startup)
CREATE TABLE countries (
    id SERIAL PRIMARY KEY,
    country_code VARCHAR(2) UNIQUE NOT NULL, -- ISO 3166-1
    country_name VARCHAR(100) NOT NULL,
    regulatory_authority TEXT,
    data_localization_required BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- States/Provinces (seeded from Rust code)
CREATE TABLE states (
    id SERIAL PRIMARY KEY,
    country_id INTEGER REFERENCES countries(id),
    state_code VARCHAR(10) NOT NULL,
    state_name VARCHAR(100) NOT NULL,
    regulatory_authority TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(country_id, state_code)
);

-- Districts (seeded from Rust code)
CREATE TABLE districts (
    id SERIAL PRIMARY KEY,
    state_id INTEGER REFERENCES states(id),
    district_code VARCHAR(20) NOT NULL,
    district_name VARCHAR(100) NOT NULL,
    health_officer TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(state_id, district_code)
);

-- Healthcare Facilities (user-managed)
CREATE TABLE facilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    facility_name VARCHAR(255) NOT NULL,
    facility_type VARCHAR(50), -- Hospital, Clinic, PHC, etc.
    country_id INTEGER REFERENCES countries(id),
    state_id INTEGER REFERENCES states(id),
    district_id INTEGER REFERENCES districts(id),
    address TEXT,
    license_number VARCHAR(100),
    cnes_number VARCHAR(7), -- Brazil
    npi_number VARCHAR(10), -- USA
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Facility Licenses (time-sensitive)
CREATE TABLE facility_licenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    facility_id UUID REFERENCES facilities(id) ON DELETE CASCADE,
    license_type VARCHAR(100) NOT NULL, -- State, Federal, Accreditation
    license_number VARCHAR(100) NOT NULL,
    issued_by VARCHAR(255),
    issued_date DATE,
    expiry_date DATE,
    status VARCHAR(20) DEFAULT 'active', -- active, expired, suspended
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Accreditation Records
CREATE TABLE accreditations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    facility_id UUID REFERENCES facilities(id) ON DELETE CASCADE,
    accreditation_body VARCHAR(100), -- JCI, NABH, ONA, etc.
    accreditation_level VARCHAR(50), -- Full, Provisional, Conditional
    certified_date DATE,
    expiry_date DATE,
    certificate_number VARCHAR(100),
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Compliance Checks (audit trail)
CREATE TABLE compliance_checks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    facility_id UUID REFERENCES facilities(id),
    check_type VARCHAR(100), -- HIPAA, LGPD, ABDM, etc.
    check_level VARCHAR(50), -- Federal, State, District
    status VARCHAR(20), -- compliant, non_compliant, pending
    checked_at TIMESTAMPTZ DEFAULT NOW(),
    checked_by UUID, -- User ID
    requirements_met TEXT[], -- Array of met requirements
    requirements_pending TEXT[], -- Array of pending requirements
    notes TEXT
);

-- Custom Compliance Rules (per-organization)
CREATE TABLE custom_compliance_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    country_id INTEGER REFERENCES countries(id),
    state_id INTEGER REFERENCES states(id),
    rule_name VARCHAR(255) NOT NULL,
    rule_description TEXT,
    validation_logic JSONB, -- Store custom validation rules
    active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Audit Log (immutable)
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type VARCHAR(50), -- facility, license, accreditation
    entity_id UUID,
    action VARCHAR(50), -- create, update, delete, check
    performed_by UUID,
    changes JSONB, -- Store before/after state
    timestamp TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_facilities_country ON facilities(country_id);
CREATE INDEX idx_facilities_state ON facilities(state_id);
CREATE INDEX idx_compliance_checks_facility ON compliance_checks(facility_id);
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_licenses_expiry ON facility_licenses(expiry_date) WHERE status = 'active';
```

---

## 🔄 Data Flow Architecture

### 1. **Startup Seeding**

```rust
// On application startup, seed reference data from Rust code
async fn seed_reference_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Seed countries from Rust modules
    let countries = vec![
        ("US", "United States"),
        ("IN", "India"),
        ("BR", "Brazil"),
    ];
    
    for (code, name) in countries {
        sqlx::query!(
            "INSERT INTO countries (country_code, country_name) 
             VALUES ($1, $2) ON CONFLICT (country_code) DO NOTHING",
            code, name
        )
        .execute(pool)
        .await?;
    }
    
    // Seed states
    let states = vec![
        ("US", "CA", "California"),
        ("US", "TX", "Texas"),
        ("IN", "MH", "Maharashtra"),
        ("BR", "SP", "São Paulo"),
    ];
    
    for (country_code, state_code, state_name) in states {
        sqlx::query!(
            "INSERT INTO states (country_id, state_code, state_name)
             SELECT id, $2, $3 FROM countries WHERE country_code = $1
             ON CONFLICT (country_id, state_code) DO NOTHING",
            country_code, state_code, state_name
        )
        .execute(pool)
        .await?;
    }
    
    Ok(())
}
```

### 2. **Runtime Validation**

```rust
// Combine code-based rules with database state
pub async fn validate_facility_compliance(
    facility_id: Uuid,
    pool: &PgPool
) -> Result<ComplianceCheck, HimsError> {
    // 1. Get facility details from database
    let facility = sqlx::query!(
        "SELECT f.*, c.country_code, s.state_code 
         FROM facilities f
         JOIN countries c ON f.country_id = c.id
         JOIN states s ON f.state_id = s.id
         WHERE f.id = $1",
        facility_id
    )
    .fetch_one(pool)
    .await?;
    
    // 2. Get validation rules from Rust code
    let rules = match facility.country_code.as_str() {
        "US" => usa::get_state_requirements(&facility.state_code),
        "IN" => india::get_state_requirements(&facility.state_code),
        "BR" => brazil::get_state_requirements(&facility.state_code),
        _ => return Err(HimsError::ConfigurationError {
            message: "Unsupported country".to_string()
        }),
    };
    
    // 3. Check licenses in database
    let licenses = sqlx::query!(
        "SELECT * FROM facility_licenses 
         WHERE facility_id = $1 AND status = 'active' AND expiry_date > NOW()",
        facility_id
    )
    .fetch_all(pool)
    .await?;
    
    // 4. Combine code rules + database state
    let mut requirements_met = vec![];
    let mut requirements_pending = vec![];
    
    for rule in rules {
        if licenses.iter().any(|l| l.license_type == rule.license_type) {
            requirements_met.push(rule.name);
        } else {
            requirements_pending.push(rule.name);
        }
    }
    
    // 5. Save audit trail
    sqlx::query!(
        "INSERT INTO compliance_checks 
         (facility_id, check_type, status, requirements_met, requirements_pending)
         VALUES ($1, $2, $3, $4, $5)",
        facility_id,
        facility.country_code,
        if requirements_pending.is_empty() { "compliant" } else { "pending" },
        &requirements_met,
        &requirements_pending
    )
    .execute(pool)
    .await?;
    
    Ok(ComplianceCheck {
        status: if requirements_pending.is_empty() { 
            ComplianceStatus::Compliant 
        } else { 
            ComplianceStatus::Pending 
        },
        requirements_met,
        requirements_pending,
        notes: format!("Checked {} licenses", licenses.len()),
    })
}
```

---

## 🎨 Architecture Diagram

```
┌───────────────────────────────────────────────────────────────┐
│                     React Native App                          │
│                  (Mobile/Desktop/Web)                         │
└────────────────────────┬──────────────────────────────────────┘
                         │
                         │ UniFFI Bindings
                         ▼
┌───────────────────────────────────────────────────────────────┐
│                  HIMS Core SDK (Rust)                         │
├───────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Code-based Configuration                               │ │
│  │  • Regulatory standards (HIPAA, LGPD, ABDM)            │ │
│  │  • Validation algorithms (immutable)                   │ │
│  │  • Healthcare standards (FHIR, HL7, DICOM)            │ │
│  │  • Compliance checklists (JCI, NABH)                  │ │
│  └─────────────────────────────────────────────────────────┘ │
│                         │                                     │
│                         │ Seed on startup                     │
│                         ▼                                     │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Database Layer (SQLx)                                  │ │
│  │  • Reference data (countries, states, districts)       │ │
│  │  • Operational data (facilities, licenses)             │ │
│  │  • Audit trail (immutable logs)                       │ │
│  │  • Custom rules (per-organization)                     │ │
│  └─────────────────────────────────────────────────────────┘ │
└────────────────────────┬──────────────────────────────────────┘
                         │
                         ▼
┌───────────────────────────────────────────────────────────────┐
│                  PostgreSQL Database                          │
│  • Multi-tenant isolation                                     │
│  • Real-time updates                                          │
│  • Backup & disaster recovery                                 │
└───────────────────────────────────────────────────────────────┘
```

---

## 🚀 Implementation Roadmap

### Phase 1: Code-Only (Current) ✅
- Rust modules for all compliance rules
- No database dependency
- Suitable for simple use cases

### Phase 2: Add Database Layer (Recommended)
```bash
# Add SQLx with PostgreSQL
cargo add sqlx --features runtime-tokio-rustls,postgres,chrono,uuid,migrate

# Create migrations
sqlx migrate add create_compliance_tables
```

### Phase 3: Hybrid Architecture
- Seed reference data from Rust modules
- Store operational data in PostgreSQL
- Combine both for validation

### Phase 4: Multi-Tenant & Extensions
- Organization-specific customizations
- Real-time license tracking
- Advanced analytics

---

## 💡 Decision Matrix

| Scenario | Recommendation | Reason |
|----------|---------------|---------|
| **Open-source library** | Code only | Simpler adoption, no DB setup |
| **SaaS product** | Hybrid (Code + DB) | Need runtime state, multi-tenancy |
| **Enterprise on-premise** | Hybrid with DB priority | Integration with existing systems |
| **Mobile SDK** | Code only (SQLite embedded) | Offline-first, no server dependency |
| **Government portal** | Full database | Frequent regulation updates |

---

## 🎯 Final Recommendation for HIMS SDK

**Use Hybrid Architecture:**

1. **Keep in Code:**
   - Regulatory framework definitions
   - Validation logic
   - Compliance checklists
   - Healthcare standards

2. **Move to Database:**
   - Facility registrations
   - License tracking
   - Accreditation records
   - Audit logs
   - User preferences

3. **Benefits:**
   - ✅ Best of both worlds
   - ✅ Certifiable compliance logic (code)
   - ✅ Flexible operational data (database)
   - ✅ Can work offline (code) or online (DB)
   - ✅ Extensible without recompilation

---

## 📝 Example: HIPAA Compliance Check

**Code (Immutable Rules):**
```rust
// src/countries/usa/federal/hipaa.rs
pub fn get_hipaa_requirements() -> Vec<ComplianceRequirement> {
    vec![
        ComplianceRequirement {
            id: "HIPAA-164.308",
            name: "Administrative Safeguards",
            mandatory: true,
        },
        ComplianceRequirement {
            id: "HIPAA-164.310",
            name: "Physical Safeguards",
            mandatory: true,
        },
    ]
}
```

**Database (Operational State):**
```sql
-- Check if facility meets HIPAA requirements
SELECT 
    f.facility_name,
    COUNT(fl.id) as licenses_count,
    bool_and(fl.expiry_date > NOW()) as all_valid
FROM facilities f
LEFT JOIN facility_licenses fl ON f.id = fl.facility_id
WHERE f.id = '...' AND fl.license_type IN (
    'HIPAA Administrative Safeguards',
    'HIPAA Physical Safeguards'
)
GROUP BY f.id;
```

This approach gives you:
- **Auditable compliance logic** (Git history)
- **Real-time operational status** (Database)
- **Type-safe validation** (Rust)
- **Flexible queries** (SQL)

🎉 **Best of both worlds!**
