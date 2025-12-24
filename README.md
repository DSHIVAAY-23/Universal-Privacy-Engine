# ZK Compliance Execution Prototype (Research)

> **Status**: Early-stage research prototype. NOT production-ready.

This repository demonstrates zero-knowledge proof generation and verification for compliance-style predicates across different execution environments. It is a **research artifact** exploring the technical feasibility of privacy-preserving compliance proofs, not a finished protocol or institutional product.

---

## Current Focus

This prototype uses **SP1 (RISC-V zkVM)** to generate proofs of compliance predicates (e.g., "balance ≥ threshold") without revealing private data. Any grant-funded work is scoped to **one specific chain** (e.g., Fuse/EVM or Stellar), with other chain implementations serving as exploratory research artifacts to validate cross-VM feasibility.

**Primary Research Question**: Can we generate succinct, verifiable proofs of compliance predicates that preserve privacy while being economically viable on-chain?

---

## What This Repo Is / Is Not

### ✅ This Repository IS:

- A **ZK execution prototype** demonstrating SP1 zkVM integration
- **Verifier experiments** for multiple execution environments (EVM, Soroban, CosmWasm)
- **Compliance-style circuits** showing threshold checks and Merkle inclusion
- **Research code** exploring cross-chain ZK verification patterns

### ❌ This Repository IS NOT:

- A production system ready for institutional use
- A universal privacy adapter for all blockchains
- A legally compliant RWA verification protocol
- An audited or security-reviewed codebase
- A product with real-world users or partners

---

## Known Limitations & Open Problems

### Critical Trust Assumptions

1. **Prover Sees Private Data**
   - The SP1 prover currently runs on the user's machine and has access to all private inputs
   - No TEE (Trusted Execution Environment) isolation yet
   - This is a fundamental limitation for institutional use cases

2. **No Authenticated Data Ingestion**
   - No zkTLS or HTTPS-based data authenticity
   - No cryptographic proof that input data came from a legitimate source
   - Merkle trees and signatures are self-generated, not from real institutions

3. **No Legal Attestation Framework**
   - No regulatory compliance validation
   - No legal entity attestation
   - No connection to real-world identity or KYC systems

4. **No Revocation or Dispute Mechanism**
   - Proofs are one-time, non-revocable
   - No way to invalidate a proof if circumstances change
   - No dispute resolution process

5. **No Security Audit**
   - Code has not been professionally audited
   - Cryptographic implementations may have vulnerabilities
   - Smart contracts have not been formally verified

6. **Economic Viability Uncertain**
   - Proof generation costs (time, compute) not optimized
   - Gas costs for verification not benchmarked at scale
   - No analysis of economic attack vectors

### Technical Gaps

- Guest program uses placeholder Ed25519 verification (not full SP1 precompile integration)
- Merkle tree implementation not optimized for production scale
- No proof aggregation or batching
- No key management or rotation strategy
- No monitoring or observability infrastructure

---

## Architecture Overview

```
User → SP1 Prover → Groth16 SNARK → On-Chain Verifier
  │         │            │                 │
  │         │            │                 ├─ Solana (Research)
  │         │            │                 ├─ Stellar (Research)
  │         │            │                 └─ Mantra (Research)
  │         │            │
  │         │            └─ ~300 bytes proof
  │         │
  │         └─ RISC-V zkVM execution
  │
  └─ Private inputs (balance, signature, Merkle proof)
```

**Key Components**:
- **Guest Program** (`guest/rwa_compliance`): RISC-V binary running in SP1 zkVM
- **SP1 Adapter** (`adapters/sp1`): Proof generation and verification
- **Verifiers** (`verifiers/`): On-chain verification contracts (research prototypes)
- **CLI** (`cli`): Command-line interface for proof generation

---

## Workspace Structure

```
├── core/                    # Trait definitions and shared types
├── adapters/sp1/            # SP1 zkVM integration
├── guest/rwa_compliance/    # RISC-V guest program (compliance circuit)
├── verifiers/               # On-chain verifiers (research prototypes)
│   ├── solana/              # Anchor program (research)
│   ├── stellar/             # Soroban contract (research)
│   └── mantra/              # CosmWasm contract (research)
├── cli/                     # Command-line tools
├── scripts/                 # Test data generation
└── docs/                    # Documentation
```

**Note**: Verifier implementations for Solana, Stellar, and Mantra are **research prototypes** demonstrating cross-VM feasibility. They are not production-ready and have not been audited.

---

## Quick Start (Research Demo)

### Prerequisites

- Rust 1.75+
- SP1 toolchain (see [SP1 docs](https://docs.succinct.xyz/))

### Generate Test Credentials

```bash
# Generate cryptographically valid test data
cargo run --bin generate_inputs -- --output rwa_creds.bin
```

**What this does**: Creates a simulated institutional credential with Ed25519 signature and Merkle proof. This is **test data only** and does not represent real institutional assets.

### Build Guest Program

```bash
cd guest/rwa_compliance
cargo build --release
```

### Run Tests

```bash
cargo test --workspace
```

**Current Status**: 25/25 tests passing (unit tests only, no integration tests)

---

## Planned Research Directions

> **Note**: These are research directions, not commitments or timelines.

### Short-term (Exploratory)

- **TEE-hosted SP1 Prover**: Investigate running prover in SGX/Nitro/SEV enclave
- **zkTLS Integration**: Explore HTTPS-based data authenticity proofs
- **Chain-specific Vertical MVP**: Focus on one use case (e.g., payroll compliance on Fuse)

### Medium-term (Uncertain)

- Proof aggregation and batching
- Key management and rotation
- Economic attack analysis
- Formal verification of circuits

### Long-term (Speculative)

- Legal attestation framework
- Regulatory compliance validation
- Multi-party computation for distributed proving
- Hardware acceleration

---

## Grant Application Context

If this repository is part of a grant application, the scope is limited to:

1. **One specific chain** (e.g., Fuse/EVM, Stellar, or Mantra)
2. **One specific use case** (e.g., payroll compliance, asset verification)
3. **Research deliverables**, not production deployment

Other chain implementations in this repository are **research artifacts** demonstrating technical feasibility and are not in scope for any single grant.

---

## Contributing

This is a research prototype. Contributions are welcome, but please understand:

- No production use cases are supported
- No guarantees of backward compatibility
- Code may change significantly as research evolves
- Security issues should be reported via GitHub issues (no bug bounty)

---

## License

MIT OR Apache-2.0

---

## Disclaimer

**THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND.**

This is experimental research software. It has not been audited, is not production-ready, and should not be used for any real-world financial or compliance purposes. The authors make no claims about the security, correctness, or suitability of this code for any particular use case.

**No institutional partnerships, users, or real-world deployments exist.**

---

## References

- [SP1 Documentation](https://docs.succinct.xyz/)
- [Groth16 Paper](https://eprint.iacr.org/2016/260.pdf)
- [Zero-Knowledge Proofs: An Introduction](https://z.cash/technology/zksnarks/)

---

## Contact

For research collaboration or technical questions, please open a GitHub issue.

**No commercial inquiries. No partnership requests. Research only.**
