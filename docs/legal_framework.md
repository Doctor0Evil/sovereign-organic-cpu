# Legal Framework & Neurorights Compliance

## Overview

This framework encodes neurorights doctrine and legal statutes into machine-enforceable invariants. The goal is to make violations technically unrepresentable, not just legally prohibited.

---

## Neurorights Doctrine

### 1. Cognitive Liberty

**Invariant:** `no_nonconsensual_modulation`

**Legal Basis:**
- Chilean Neurorights Law (2021)
- California SB 1223 (2024)
- Colorado HB 24-1058 (2024)

**Technical Enforcement:**
- All neural modulation requires DID-signed consent
- EVOLVE token gating for any brain-state changes
- Audit log records all modulation events

---

### 2. Mental Privacy

**Invariant:** `no_raw_neural_export`

**Legal Basis:**
- EU GDPR Article 9 (biometric data)
- California SB 1223 (neural data as sensitive)
- Colorado HB 24-1058 (mental privacy protection)

**Technical Enforcement:**
- Inner domain never exposed to network
- Only corridor-safe scalars exported (RoH, ROD, Lifeforce)
- Zero-knowledge attestations for external verification

---

### 3. Augmentation Continuity

**Invariant:** `no_guard_removal`

**Legal Basis:**
- UN Convention on Rights of Persons with Disabilities
- EU AI Act Article 5 (continuity of care)

**Technical Enforcement:**
- OrganicCPU guard layer cannot be unloaded
- Kernel module signed and verified at boot
- Any removal attempt logged as constitutional violation

---

### 4. Project Continuity

**Invariant:** `no_downgrade_without_consent`

**Legal Basis:**
- Contract law (unilateral modification prohibition)
- Consumer protection statutes

**Technical Enforcement:**
- EVOLVE tokens required for all upgrades
- Strictest-wins policy merging
- Downgrade attempts blocked at kernel level

---

## Legal Regime Support

| Regime | Protectiveness Score | Status |
|--------|---------------------|--------|
| California SB 1223 | 8/10 | ✅ Supported |
| Colorado HB 24-1058 | 9/10 | ✅ Supported |
| EU AI Act Article 5 | 7/10 | ✅ Supported |
| Chilean Neurorights Law | 8/10 | ✅ Supported |
| Custom DID-Bound | 10/10 | ✅ Supported (default) |

**Strictest-Wins Logic:** When multiple regimes apply, the most protective is automatically selected.

---

## Audit & Compliance

### Internal Audit Trail

- All guard decisions logged to `KernelAuditLog`
- Entries are append-only and immutable
- Can be exported for external review

### External Verification

- Audit entries can be anchored to Organichain/Bostrom
- Zero-knowledge proofs verify compliance without exposing inner-domain data
- Regulators can verify safety without accessing raw neural data

### Violation Reporting

- Neurorights violations logged with severity 1-10
- Critical violations (severity ≥ 8) flagged for immediate review
- All violations include DID, timestamp, and proposal hash

---

## Non-Derogable Clauses

The following clauses cannot be removed by any update, fork, or modification:

```rust
// From neurorights_kernel/src/lib.rs
pub const INVARIANT_COGNITIVE_LIBERTY: &str = "no_nonconsensual_modulation";
pub const INVARIANT_MENTAL_PRIVACY: &str = "no_raw_neural_export";
pub const INVARIANT_AUGMENTATION_CONTINUITY: &str = "no_guard_removal";
pub const INVARIANT_PROJECT_CONTINUITY: &str = "no_downgrade_without_consent";
```

**Enforcement:** Compiled into kernel invariants, verified at boot and runtime.

---

## License Terms

See `LICENSE` file for full terms. Key clauses:

- **Non-Commercial Neural Data:** Neural data cannot be sold or monetized
- **Neurorights Invariants:** Four core rights cannot be removed
- **Violation Logging:** All bypass attempts are logged and anchored

---

## Jurisdiction Notes

This framework is designed to be jurisdiction-agnostic while complying with the strictest applicable laws. Users should consult local legal counsel for specific compliance requirements.

---

## References

- [California SB 1223 Text](https://leginfo.legislature.ca.gov/)
- [Colorado HB 24-1058 Text](https://leg.colorado.gov/)
- [EU AI Act](https://artificialintelligenceact.eu/)
- [Chilean Neurorights Law](https://www.senado.cl/)
