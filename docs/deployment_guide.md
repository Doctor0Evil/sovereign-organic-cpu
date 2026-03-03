# Deployment Guide

## Prerequisites

- Rust 1.75+ (edition 2024)
- Python 3.10+ (for verification scripts)
- Bostrom DID (did:bostrom:...)
- OrganicCPU instance ID
- Access to biophysical telemetry (EEG, HRV, thermal)

---

## Step 1: Clone and Build

```bash
# Clone repository
git clone https://github.com/Doctor0Evil/sovereign-organic-cpu.git
cd sovereign-organic-cpu

# Build all crates
cargo build --release

# Run tests
cargo test --workspace

# Run benchmarks (optional)
cargo bench --package guard_verification_tests
```

---

## Step 2: Initialize DID Identity

```bash
# Run DID bootstrap
cargo run --bin did_bootstrap -- --did did:bostrom:bostrom18...

# This creates:
# - SovereignIdentity binding
# - Boot chain genesis hash
# - Initial EVOLVE token mint
```

**Output:** `~/.sovereign_identity.json`

---

## Step 3: Configure Biophysical Envelopes

```bash
# Run envelope calibrator
cargo run --bin envelope_calibrator -- --mode baseline

# Calibrates:
# - RoH baseline (EEG, HRV, thermal)
# - ROD initial state
# - LifeforceBand thresholds
# - EcoImpactScore baseline
```

**Output:** `~/.biophysical_envelope.json`

---

## Step 4: Deploy reality.os Integration

```bash
# Build kernel module
cargo build --features kernel-module --release

# Load kernel module (requires root)
sudo insmod reality_os_kernel.ko

# Verify guard service is active
cargo run --bin reality_kernel_module -- --status
```

**Expected Output:**
```
Guard Service: Active
Active Guards: 4 (roh, rod, lifeforce, neurorights)
Violation Count: 0
```

---

## Step 5: Configure ALN Protocol Stack

```bash
# Start ALN router
cargo run --bin aln_router -- --mode sovereign

# Configure legacy shims (optional)
cargo run --bin gateway_shim -- --protocol ros2 --topic /aln/export
```

**Network Endpoints:**
- Inner Domain: `/dev/bci_inner_*` (host-only)
- Outer Domain: libp2p `/aln/sovereign/1.0.0`
- Legacy Shim: ROS2, BLE, CAN (translated only)

---

## Step 6: Verify Evidence Bundles

```bash
# For each evolution/upgrade, verify evidence bundle
python3 scripts/verify_evidence_bundle.py path/to/bundle.json

# Expected output:
# ✅ ALL VERIFICATION CHECKS PASSED
```

---

## Step 7: Anchor to Organichain/Bostrom

```bash
# Anchor audit log to chain
cargo run --bin reality_kernel_module -- --anchor

# This creates:
# - Hash-linked audit entries
# - DID-signed attestations
# - On-chain verification records
```

---

## Step 8: Continuous Monitoring

```bash
# Start guard heartbeat monitor
cargo run --bin guard_scheduler -- --interval 60s

# Monitor rejection rate
cargo run --bin guard_validator -- --metrics
```

**Key Metrics to Track:**
- RoH average (target: < 0.2)
- ROD accumulation (target: < 0.5)
- Syscall rejection rate (target: < 5%)
- Evidence bundle completeness (target: 100%)

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Guard service not active | Check kernel module loaded: `lsmod \| grep reality_os` |
| RoH threshold exceeded | Reduce workload, check telemetry calibration |
| Evidence bundle incomplete | Ensure all 10 tags present in bundle JSON |
| DID binding failed | Verify DID format: `did:bostrom:...` |
| Boot chain verification failed | Re-run `did_bootstrap` with correct keys |

---

## Security Checklist

- [ ] DID bound to OrganicCPU instance
- [ ] Boot chain verified at startup
- [ ] All 4 guard services active
- [ ] Inner domain devices classified correctly
- [ ] Evidence bundles contain 10 tags
- [ ] Audit log anchoring enabled
- [ ] EVOLVE tokens required for upgrades
- [ ] Legacy shims restricted to export-only

---

## Support

- GitHub Issues: https://github.com/Doctor0Evil/sovereign-organic-cpu/issues
- Documentation: `/docs` directory
- Verification Scripts: `/scripts` directory
