# Complete Deployment Checklist

## Repository Setup

- [ ] Clone `sovereign-organic-cpu` repository
- [ ] Verify Rust version ≥ 1.75
- [ ] Install Python 3.10+ for verification scripts
- [ ] Generate Bostrom DID (`did:bostrom:...`)

---

## Build & Test

- [ ] `cargo build --release` (all crates compile)
- [ ] `cargo test --workspace` (all tests pass)
- [ ] `cargo bench` (benchmarks run successfully)
- [ ] `python3 scripts/verify_evidence_bundle.py` (script executes)

---

## Identity & Boot Chain

- [ ] Run `did_bootstrap` with valid DID
- [ ] Verify `~/.sovereign_identity.json` created
- [ ] Confirm boot chain genesis hash generated
- [ ] EVOLVE token minted for initial upgrade

---

## Biophysical Calibration

- [ ] Run `envelope_calibrator --mode baseline`
- [ ] Verify `~/.biophysical_envelope.json` created
- [ ] Confirm RoH baseline < 0.3
- [ ] Confirm ROD baseline < 0.5
- [ ] Confirm LifeforceBand = Baseline

---

## reality.os Integration

- [ ] Build kernel module with `--features kernel-module`
- [ ] Load kernel module (`sudo insmod reality_os_kernel.ko`)
- [ ] Verify guard service status = Active
- [ ] Confirm 4 guards active (roh, rod, lifeforce, neurorights)
- [ ] Verify violation count = 0

---

## ALN Protocol Stack

- [ ] Start `aln_router --mode sovereign`
- [ ] Configure legacy shims (if needed)
- [ ] Verify inner domain devices classified correctly
- [ ] Confirm outer domain only exports corridor-safe scalars

---

## Evidence Bundle Verification

- [ ] Create test EvidenceBundle with 10 tags
- [ ] Run `verify_evidence_bundle.py` on test bundle
- [ ] Confirm all verification checks pass
- [ ] Verify computed hash matches expected format

---

## Audit & Anchoring

- [ ] Enable audit log in reality.os
- [ ] Anchor initial audit entries to Organichain/Bostrom
- [ ] Verify anchor hash recorded
- [ ] Confirm audit trail exportable

---

## Security Verification

- [ ] Test inner domain access blocked from userland
- [ ] Verify raw socket creation blocked
- [ ] Confirm mmap to inner domain blocked
- [ ] Test neurorights violation logged correctly
- [ ] Verify EVOLVE token required for upgrades

---

## Monitoring & Operations

- [ ] Start `guard_scheduler --interval 60s`
- [ ] Configure metrics collection
- [ ] Set up alerts for RoH > 0.25
- [ ] Set up alerts for ROD > 0.7
- [ ] Set up alerts for LifeforceBand = HardStop

---

## Documentation Review

- [ ] Read `docs/tripartite_architecture.md`
- [ ] Read `docs/deployment_guide.md`
- [ ] Read `docs/legal_framework.md`
- [ ] Review `LICENSE` terms
- [ ] Review `README.md` quickstart

---

## Final Verification

- [ ] All 54 files present in repository
- [ ] All crates compile without warnings
- [ ] All tests pass
- [ ] Evidence bundle verification passes
- [ ] Audit log anchoring works
- [ ] Security guarantees verified

---

## Sign-Off

**Deployed By:** _______________________

**DID:** `did:bostrom:...`

**Date:** _______________________

**OrganicCPU Instance ID:** _______________________

**Audit Log Anchor Hash:** _______________________

---

## Next Steps

1. Begin production deployment
2. Monitor guard metrics continuously
3. Anchor audit logs regularly (daily recommended)
4. Review and update EvidenceBundles for each evolution
5. Report issues via GitHub Issues
