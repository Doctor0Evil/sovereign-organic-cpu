# Contributing to Sovereign Organic CPU

## Contributor License Agreement

By contributing to this repository, you agree that:

1. All contributions are licensed under MIT with Non-Commercial Neural Data Clause
2. Neurorights invariants cannot be modified or removed
3. All evolutions require 10-tag EvidenceBundles
4. No contribution can weaken security or sovereignty guarantees

---

## Development Workflow

### 1. Fork and Clone

```bash
git clone https://github.com/Doctor0Evil/sovereign-organic-cpu.git
cd sovereign-organic-cpu
```

### 2. Create Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Follow Rust coding standards
- Add tests for new functionality
- Update documentation as needed
- Ensure all existing tests pass

### 4. Create EvidenceBundle

For any evolution or significant change:

```bash
# Create evidence bundle JSON
# Include all 10 required tags
python3 scripts/verify_evidence_bundle.py path/to/bundle.json
```

### 5. Submit Pull Request

- Link EvidenceBundle in PR description
- Describe security implications
- Wait for review and approval

---

## Code Standards

### Rust

- Edition 2024
- `#![forbid(unsafe_code)]` required
- All public functions documented
- Tests required for new functionality

### Python

- Python 3.10+
- Type hints required
- PEP 8 compliant

### Documentation

- Markdown format
- Include examples where applicable
- Update changelog for significant changes

---

## Testing Requirements

- Unit tests for all new functions
- Property-based tests for guard logic
- Integration tests for cross-crate functionality
- Benchmarks for performance-critical code

```bash
# Run all tests
cargo test --workspace

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace
```

---

## Security Review

All contributions undergo security review:

1. Guard logic changes require formal verification
2. Cryptographic changes require external audit
3. Kernel module changes require additional review
4. EvidenceBundle schema changes require community consensus

---

## Neurorights Compliance

Contributions must not:

- Weaken neurorights invariants
- Expose inner-domain data
- Allow downgrade of safety envelopes
- Bypass EVOLVE token requirements
- Remove audit logging capabilities

---

## Contact

- GitHub Issues: https://github.com/Doctor0Evil/sovereign-organic-cpu/issues
- Email: [via GitHub]
- DID: `did:bostrom:bostrom18...`
```

**Thank you for building sovereign cybernetic infrastructure.** 🧠🔐
