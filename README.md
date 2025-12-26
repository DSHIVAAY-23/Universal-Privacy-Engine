# Universal Privacy Engine (VeriVault Core)

> ⚠️ **Status: Alpha Research Prototype**  
> This allows developers to build and test ZK verification flows.  
> **Not production-ready.** No current hardware enclave (TEE) dependency.

## Overview

The **Universal Privacy Engine** is a modular framework for building verifiable computation systems. It abstracts the complexity of different proving backends (like SP1 zkVM) behind a unified Rust trait, allowing developers to focus on business logic.

**Current Capabilities**:
- **ZK-VM Integration**: Run Rust code inside the SP1 zkVM and generate Groth16/PLONK proofs.
- **Recorded Data Verification**: Deterministically verify that JSON inputs match pre-recorded TLS response fixtures.
- **Smart Contract Generation**: Auto-generate Solidity verifiers for your specific guest programs.

---

## Security & Trust (Current State)

This Alpha release is a **client-side research prototype**.

- **Authenticity**: External data is verified against **Recorded TLS Evidence** (fixtures), ensuring the input matches a specific historical SHA256 hash and domain.
- **Integrity**: The SP1 zkVM proves that the computation was performed correctly on the given inputs.
- **Confidentiality**: **None in Alpha**. The prover has full visibility of the data. Confidentiality features (like TEEs) are planned for future releases.

See [TRUST_MODEL.md](TRUST_MODEL.md) for a detailed breakdown.

---

## Key Features

### 🏗️ **Modular Core Architecture**
- **Hexagonal Pattern**: `PrivacyEngine` trait decouples logic from backends.
- **Swappable Backends**: Currently supports `Sp1Backend` (ZK) and a local Dev/Mock backend.
- **Extensible**: Designed to support future backends (RISC0, TEEs) without rewriting app logic.

### 📊 **Data Ingestion with "Recorded zkTLS"**
- **HttpProvider**: Fetches data and validates it against cryptographic fixtures.
- **Fixture System**: proves "This JSON came from `example.com` at time `T`" (using recorded proof).
- **ZkInputBuilder**: Helper to serialize public and private inputs for the zkVM.

### 🔐 **Smart Contract Generator**
- CLI tool to generate `UniversalVerifier.sol`.
- Embeds your program's specific Verification Key (VKey).
- Ready for deployment to EVM chains (Sepolia, etc.).

---

## Quick Start

### Installation

```bash
git clone https://github.com/your-org/universal-privacy-engine
cd universal-privacy-engine
cargo build --workspace
```

### Usage Example

```rust
use universal_privacy_engine_core::{
    PrivacyEngine,
    data_source::{HttpProvider, DataProvider, ZkInputBuilder}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Fetch data (Simulated zkTLS using fixtures for 'example.com')
    let provider = HttpProvider::new();
    let balance = provider.fetch(
        "https://example.com", 
        "data.balance"
    ).await?;
    
    // 2. Build ZK input
    let mut builder = ZkInputBuilder::new();
    builder.add_public_data(balance);
    
    let zk_input = builder.build();
    
    // 3. Generate Proof (SP1)
    // In dev, use MockBackend for speed. In prod, use Sp1Backend.
    let backend = universal_privacy_engine_core::mock::MockBackend::new();
    let receipt = backend.prove(&zk_input)?;
    
    // 4. Verify
    assert!(backend.verify(&receipt)?);
    
    Ok(())
}
```

### Run Tests & Benchmarks

```bash
# Verify the ZK logic
cargo test --workspace

# Run benchmarks
cargo bench -p universal-privacy-engine-core
```

---

## Documentation

- **[TRUST_MODEL.md](TRUST_MODEL.md)**: Security assumptions and Alpha limitations.
- **[ARCHITECTURE.md](ARCHITECTURE.md)**: System design and hexagonal patterns.
- **[FUTURE_WORK.md](docs/FUTURE_WORK.md)**: Roadmap for zkTLS and TEEs.
- **[ZKTLS_RECORDED_PROOF.md](docs/ZKTLS_RECORDED_PROOF.md)**: Details on the fixture verification system.

---

## Project Structure

```
universal-privacy-engine/
├── core/                      # Core abstraction (PrivacyEngine trait)
│   ├── src/data_source/       # HttpProvider & Recorded zkTLS
├── adapters/
│   ├── sp1/                   # SP1 zkVM integration
│   └── tee/                   # (Experimental) Future TEE adapters
├── contracts/                 # Solidity templates
├── fixtures/                  # zkTLS recordings for testing
└── guest/                     # Rust code that runs inside ZK-VM
```

---

## Disclaimer

**Research Software**.
This codebase is for experimental and development use only.
It provides **integrity** proofs (ZK) but does not yet provide **confidentiality** guarantees (TEEs/MPC).
Do not use with real value.
