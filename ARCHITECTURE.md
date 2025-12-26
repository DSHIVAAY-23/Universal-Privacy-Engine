# Architecture

## Overview

The Universal Privacy Engine follows a **Hexagonal Architecture** (Ports and Adapters pattern) to achieve backend independence and modularity. This design separation allows us to define the *shape* of a privacy application (the Port) independent of the specific proving technology (the Adapter).

---

## Hexagonal Design

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Application Layer                            │
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │     CLI      │  │     API      │  │ Data Ingest  │             │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘             │
│         │                  │                  │                      │
│         └──────────────────┼──────────────────┘                      │
│                            │                                         │
21└────────────────────────────┼─────────────────────────────────────────┘
                             │
                             ▼
              ┌──────────────────────────────┐
              │      PORT (Interface)        │
              │                              │
              │    ┌──────────────────┐     │
              │    │ PrivacyEngine    │     │
              │    │     Trait        │     │
              │    └──────────────────┘     │
              │                              │
              └──────────────┬───────────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
         ▼                   ▼                   ▼
┌────────────────┐  ┌────────────────┐  ┌────────────────┐
│   ADAPTER 1    │  │   ADAPTER 2    │  │   ADAPTER 3    │
│                │  │                │  │                │
│  Sp1Backend    │  │  MockBackend   │  │ (Future) TEEs  │
│                │  │                │  │                │
│  • SP1 SDK     │  │ • Dev/Test     │  │ • SGX / Nitro  │
│  • Groth16     │  │ • Fast Feedback│  │ • Attestation  │
└────────────────┘  └────────────────┘  └────────────────┘
     ZK-VM                Dev Tool           Hardening
```

---

## Core Abstraction: `PrivacyEngine` Trait

The `PrivacyEngine` trait is the core contract.

```rust
pub trait PrivacyEngine {
    /// Generate a cryptographic proof (ZK) or attestation
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError>;
    
    /// Verify a proof receipt
    fn verify(&self, receipt: &ProofReceipt) -> Result<bool, PrivacyEngineError>;
    
    /// Export verifier artifacts (e.g. Solidity contracts)
    fn export_verifier(&self, chain: ChainType) -> Result<Vec<u8>, PrivacyEngineError>;
}
```

### Design Goals
1.  **Backend Agnostic**: The application doesn't care if the proof comes from SP1, RISC0, or a future TEE.
2.  **Opaque Inputs**: Inputs are serialized bytes, allowing flexibility in data schemas.

---

## Data Ingestion: Recorded zkTLS

The `HttpProvider` implements a "Recorded Verification" flow for the Alpha release.

```
┌────────────────┐         ┌─────────────────────────┐
│ HttpProvider   │────────▶│ Fixture (metadata.json) │
│                │         └─────────────────────────┘
│ • fetch()      │                      ▲
│ • verify_tls() │──────────(Verifies)──┘
└────────────────┘
```

1.  **Fetch**: Retrieves data (simulated or real).
2.  **Verify**: Checks if the data matches the cryptographic hash of a known, trusted TLS recording (in `fixtures/`).
3.  **Result**: Returns verified bytes only if the proof holds.

*Note: In the future, this will be replaced by live TLSNotary proof generation.*

---

## Adapter Implementations

### 1. SP1 Adapter (`adapters/sp1`)
The primary ZK backend.
- **Micro-VM**: Uses SP1 (RISC-V zkVM) to execute Rust code.
- **Proof**: Generates a STARK -> SNARK (Groth16) proof.
- **Verification**: Can be verified on-chain (Ethereum) or off-chain.

### 2. Mock Adapter
Used for local testing and benchmarking.
- **Speed**: Instant "proof" generation.
- **Use Case**: CI/CD pipelines and unit tests.
- **Security**: None (do not use in production).

### 3. TEE Adapter (Future)
We have designed the system to support Trusted Execution Environments.
- **Role**: Optional hardening.
- **Status**: Planned roadmap item (see [FUTURE_WORK.md](../docs/FUTURE_WORK.md)).

---

## Verifier Generation

The system includes a CLI tool (`scripts`) to generate chain-specific verifiers.

1.  **Input**: Program Verification Key (Hash of the guest ELF).
2.  **Template**: `UniversalVerifier.sol.tera`.
3.  **Output**: A ready-to-deploy Solidity contract.

This allows for a seamless deploy flow: `Build Circuit -> Generate Verifier -> Deploy -> Verify Proofs`.
