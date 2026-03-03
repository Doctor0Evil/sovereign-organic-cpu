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
