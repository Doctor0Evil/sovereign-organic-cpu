# Tripartite Architecture Documentation

## Overview

The Sovereign Organic CPU framework implements a tripartite stack for host-sovereign cybernetic identity. This architecture ensures that augmented citizens retain full control over their neural data, biophysical state, and augmentation continuity.

## Stack Priority Order

### 1. OrganicCPU Guards (Foundation Layer)

**Purpose:** Immediate cryptographic and behavioral enforcement

**Components:**
- `organic_cpu_guard` crate
- GuardContext, NeurorightsGuard, BiophysicalGuard
- All neural interactions routed through guard checks

**Key Invariants:**
- RoH ≤ 0.3 (Risk of Harm ceiling)
- ROD < 1.0 (Risk of Danger HardStop)
- No inner-domain exposure to external networks

**Implementation Status:** ✅ Complete

---

### 2. Biophysical Envelopes (Measurement Layer)

**Purpose:** Quantifiable, measurable safety constraints

**Components:**
- `biophysical_envelopes` crate
- RoH, ROD, LifeforceBand, EcoImpactScore calculators
- Telemetry ingestion (EEG, HRV, thermal, cytokines)

**Key Metrics:**

| Metric | Constraint | Source |
|--------|------------|--------|
| RoH | ≤ 0.3 | EEG + HRV + thermal + duty |
| ROD | < 1.0 | Pain debt + neurorights budget |
| LifeforceBand | Baseline/Warn/Stop | Cytokines + HRV + subjective |
| EcoImpact | Δ ≤ 0.0 | CEIM + NanoKarma |

**Implementation Status:** ✅ Complete

---

### 3. Legal/DID (Identity Layer)

**Purpose:** Bind technical invariants to permanent identity

**Components:**
- `did_sovereignty` crate
- `neurorights_kernel` crate
- Bostrom DID binding, EVOLVE tokens, boot chains

**Key Rights:**
- Cognitive Liberty (no non-consensual modulation)
- Mental Privacy (no raw neural export)
- Augmentation Continuity (no guard removal)
- Project Continuity (no downgrade without consent)

**Legal Alignment:**
- California SB 1223 (neural data as sensitive)
- Colorado HB 24-1058 (cognitive liberty)
- EU AI Act Article 5 (manipulation ban)

**Implementation Status:** ✅ Complete

---

## Data Flow Diagram
