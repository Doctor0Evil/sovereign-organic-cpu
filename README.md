# Sovereign Organic CPU

A host-sovereign cybernetic identity framework grounded in biophysical constraints and machine-enforced law. This repository implements the OrganicCPU guard layer as the primary enforcement surface for neurorights, biophysical envelopes, and DID-bound augmented citizen identity.

## Tripartite Stack Priority

1. **OrganicCPU Guards** (this repository) - Cryptographic interactions, chat-RPC routing, device behavior
2. **Biophysical Envelopes** - RoH ≤ 0.3, ROD, LifeforceBand, EcoImpactScore
3. **Legal/DID** - Bostrom DID binding, reality.os integration, neurorights as kernel invariants

## Core Components

| Component | Purpose | Status |
|-----------|---------|--------|
| `organic_cpu_guard` | Rust guard crates for all neural interactions | Alpha |
| `biophysical_envelopes` | RoH, ROD, LifeforceBand calculation & enforcement | Alpha |
| `neurorights_kernel` | Non-derogable invariants compiled to machine code | Design |
| `aln_protocol` | ALN-governed networking stack with corridor-safe semantics | Design |
| `did_sovereignty` | DID-bound boot chains, EVOLVE-token-gated updates | Design |

## Security Model

- **Inner Domain**: BCI/EEG/nanoswarm channels never exposed externally
- **Outer Domain**: Only corridor-safe scalars (RoH, BCI bands, LifeforceBand) exported
- **Proposal-Only**: All external interactions are signed proposals, never direct control
- **Offline-Functional**: All critical decisions execute host-local; network is audit-only

## Evidence Bundle Requirement

Every evolution, upgrade, or high-impact decision requires a 10-tag EvidenceBundle:

| Hex | Tag | Constraint |
|-----|-----|------------|
| `0xpow20e7` | Brain Power Envelope | ≤20W cortical CMRO2 |
| `0xtherm3c` | Thermal Safety | ΔT ≤ 0.3–0.5°C |
| `0xcyt0a9` | Cytokine Thresholds | IL-6, CRP within bounds |
| `0xcload7f` | Cognitive Load | EEG alpha/gamma ratios |
| `0xlyaduty` | ML Duty Cycle | Lyapunov stability |
| `0xrights1c` | Neurorights | rollbackanytime, noraweegexport |
| `0xroh025c` | OrganicCPU Scheduler | RoH ≤ 0.25–0.3 |
| `0xeco9b2` | Eco-Monotonicity | CEIM/NanoKarma non-regressive |
| `0xdevcorr` | Device Corridor | BCI/EEG thermodynamic envelope |
| `0xclin10a` | Clinical Evidence | Guideline/trial cluster |

## Getting Started

```bash
# Clone repository
git clone https://github.com/Doctor0Evil/sovereign-organic-cpu.git
cd sovereign-organic-cpu

# Build guard crates
cargo build --release

# Run envelope calibration tests
cargo test --package biophysical_envelopes

# Setup DID boot chain (requires existing Bostrom DID)
./scripts/boot_chain_setup.sh --did did:bostrom:bostrom18...

Integration with reality.os
This repository provides the guard layer that reality.os enforces at the kernel level. See reality_os/integration/ for kernel module bindings and syscall wrappers.
Legal Framework
Neurorights encoded in this system align with:
California SB 1223 (neural data as sensitive personal information)
Colorado HB 24-1058 (cognitive liberty protections)
EU AI Act Article 5 (ban on subliminal/manipulative AI)
License
MIT License with Non-Commercial Neural Data Clause (see LICENSE)
Citation
If you use this framework in research, please cite:
@software{sovereign_organic_cpu,
  author = {Doctor0Evil},
  title = {Sovereign Organic CPU: A Biophysical Sovereignty Framework},
  year = {2026},
  url = {https://github.com/Doctor0Evil/sovereign-organic-cpu}
}
